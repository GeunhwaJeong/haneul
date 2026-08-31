// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

use crate::crypto::BridgeAuthorityPublicKey;
use crate::error::{BridgeError, BridgeResult};
use crate::events::HaneulBridgeEvent;
use crate::metrics::BridgeMetrics;
use crate::retry_with_max_elapsed_time;
use crate::types::BridgeActionStatus;
use crate::types::HaneulEvents;
use crate::types::ParsedTokenTransferMessage;
use crate::types::{BridgeAction, BridgeAuthority, BridgeCommittee};
use async_trait::async_trait;
use core::panic;
use fastcrypto::traits::ToFromBytes;
use haneul_json_rpc_types::BcsEvent;
use haneul_json_rpc_types::HaneulEvent;
use haneul_json_rpc_types::HaneulExecutionStatus;
use haneul_rpc::field::{FieldMask, FieldMaskUtil};
use haneul_rpc::proto::haneul::rpc::v2::{
    Checkpoint, ExecuteTransactionRequest, ExecutedTransaction, GetCheckpointRequest,
    GetObjectRequest, GetServiceInfoRequest, GetTransactionRequest, Object,
    Transaction as ProtoTransaction, UserSignature as ProtoUserSignature,
};
use haneul_sdk_types::Address;
use haneul_types::BRIDGE_PACKAGE_ID;
use haneul_types::HANEUL_BRIDGE_OBJECT_ID;
use haneul_types::Identifier;
use haneul_types::TypeTag;
use haneul_types::base_types::ObjectID;
use haneul_types::base_types::ObjectRef;
use haneul_types::base_types::SequenceNumber;
use haneul_types::bridge::{
    BridgeSummary, BridgeWrapper, MoveTypeBridgeMessageKey, MoveTypeBridgeRecord,
};
use haneul_types::bridge::{BridgeTrait, BridgeTreasurySummary};
use haneul_types::bridge::{MoveTypeBridgeMessage, MoveTypeParsedTokenTransferMessage};
use haneul_types::bridge::{
    MoveTypeCommitteeMember, MoveTypeTokenTransferPayload, MoveTypeTokenTransferPayloadV2,
};
use haneul_types::collection_types::LinkedTableNode;
use haneul_types::digests::TransactionDigest;
use haneul_types::event::EventID;
use haneul_types::gas_coin::GasCoin;
use haneul_types::object::Owner;
use haneul_types::parse_haneul_type_tag;
use haneul_types::transaction::ObjectArg;
use haneul_types::transaction::SharedObjectMutability;
use haneul_types::transaction::Transaction;
use std::collections::HashMap;
use std::str::from_utf8;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::OnceCell;
use tracing::{error, warn};

pub struct HaneulClient<P> {
    inner: P,
    bridge_metrics: Arc<BridgeMetrics>,
}

pub type HaneulBridgeClient = HaneulClient<HaneulClientInternal>;

pub struct HaneulClientInternal {
    grpc_client: haneul_rpc_api::Client,
}

#[derive(Clone, Debug)]
pub struct ExecuteTransactionResult {
    pub status: HaneulExecutionStatus,
    pub events: Vec<HaneulEvent>,
}

impl HaneulBridgeClient {
    pub async fn new(rpc_url: &str, bridge_metrics: Arc<BridgeMetrics>) -> anyhow::Result<Self> {
        let grpc_client = haneul_rpc_api::Client::new(rpc_url)?;
        let inner = HaneulClientInternal { grpc_client };
        let self_ = Self {
            inner,
            bridge_metrics,
        };
        self_.describe().await.map_err(|e| anyhow::anyhow!("{e}"))?;
        Ok(self_)
    }

    pub fn grpc_client(&self) -> &haneul_rpc_api::Client {
        &self.inner.grpc_client
    }
}

impl<P> HaneulClient<P>
where
    P: HaneulClientInner,
{
    pub fn new_for_testing(inner: P) -> Self {
        Self {
            inner,
            bridge_metrics: Arc::new(BridgeMetrics::new_for_testing()),
        }
    }

    // TODO assert chain identifier
    async fn describe(&self) -> Result<(), BridgeError> {
        let chain_id = self.inner.get_chain_identifier().await?;
        let block_number = self.inner.get_latest_checkpoint_sequence_number().await?;
        tracing::info!(
            "HaneulClient is connected to chain {chain_id}, current block number: {block_number}"
        );
        Ok(())
    }

    /// Get the mutable bridge object arg on chain.
    // We retry a few times in case of errors. If it fails eventually, we panic.
    // In general it's safe to call in the beginning of the program.
    // After the first call, the result is cached since the value should never change.
    pub async fn get_mutable_bridge_object_arg_must_succeed(&self) -> ObjectArg {
        static ARG: OnceCell<ObjectArg> = OnceCell::const_new();
        *ARG.get_or_init(|| async move {
            let Ok(Ok(bridge_object_arg)) = retry_with_max_elapsed_time!(
                self.inner.get_mutable_bridge_object_arg(),
                Duration::from_secs(30)
            ) else {
                panic!("Failed to get bridge object arg after retries");
            };
            bridge_object_arg
        })
        .await
    }

    /// Returns BridgeAction from a Haneul Transaction with transaction hash
    /// and the event index. If event is declared in an unrecognized
    /// package, return error.
    pub async fn get_bridge_action_by_tx_digest_and_event_idx_maybe(
        &self,
        tx_digest: &TransactionDigest,
        event_idx: u16,
    ) -> BridgeResult<BridgeAction> {
        let events = self.inner.get_events_by_tx_digest(*tx_digest).await?;
        let event = events
            .events
            .get(event_idx as usize)
            .ok_or(BridgeError::NoBridgeEventsInTxPosition)?;
        if event.type_.address.as_ref() != BRIDGE_PACKAGE_ID.as_ref() {
            return Err(BridgeError::BridgeEventInUnrecognizedHaneulPackage);
        }
        let bridge_event = HaneulBridgeEvent::try_from_haneul_event(event)?
            .ok_or(BridgeError::NoBridgeEventsInTxPosition)?;

        bridge_event
            .try_into_bridge_action()
            .ok_or(BridgeError::BridgeEventNotActionable)
    }

    pub async fn get_bridge_summary(&self) -> BridgeResult<BridgeSummary> {
        self.inner.get_bridge_summary().await
    }

    pub async fn is_bridge_paused(&self) -> BridgeResult<bool> {
        self.get_bridge_summary()
            .await
            .map(|summary| summary.is_frozen)
    }

    pub async fn get_treasury_summary(&self) -> BridgeResult<BridgeTreasurySummary> {
        Ok(self.get_bridge_summary().await?.treasury)
    }

    pub async fn get_token_id_map(&self) -> BridgeResult<HashMap<u8, TypeTag>> {
        self.get_bridge_summary()
            .await?
            .treasury
            .id_token_type_map
            .into_iter()
            .map(|(id, name)| {
                parse_haneul_type_tag(&format!("0x{name}"))
                    .map(|name| (id, name))
                    .map_err(|e| {
                        BridgeError::InternalError(format!(
                            "Failed to retrieve token id mapping: {e}, type name: {name}"
                        ))
                    })
            })
            .collect()
    }

    pub async fn get_notional_values(&self) -> BridgeResult<HashMap<u8, u64>> {
        let bridge_summary = self.get_bridge_summary().await?;
        bridge_summary
            .treasury
            .id_token_type_map
            .iter()
            .map(|(id, type_name)| {
                bridge_summary
                    .treasury
                    .supported_tokens
                    .iter()
                    .find_map(|(tn, metadata)| {
                        if type_name == tn {
                            Some((*id, metadata.notional_value))
                        } else {
                            None
                        }
                    })
                    .ok_or(BridgeError::InternalError(
                        "Error encountered when retrieving token notional values.".into(),
                    ))
            })
            .collect()
    }

    pub async fn get_bridge_committee(&self) -> BridgeResult<BridgeCommittee> {
        let bridge_summary = self.inner.get_bridge_summary().await?;
        let move_type_bridge_committee = bridge_summary.committee;

        let mut authorities = vec![];
        // TODO: move this to MoveTypeBridgeCommittee
        for (_, member) in move_type_bridge_committee.members {
            let MoveTypeCommitteeMember {
                haneul_address,
                bridge_pubkey_bytes,
                voting_power,
                http_rest_url,
                blocklisted,
            } = member;
            let pubkey = BridgeAuthorityPublicKey::from_bytes(&bridge_pubkey_bytes)?;
            let base_url = from_utf8(&http_rest_url).unwrap_or_else(|_e| {
                warn!(
                    "Bridge authority address: {}, pubkey: {:?} has invalid http url: {:?}",
                    haneul_address, bridge_pubkey_bytes, http_rest_url
                );
                ""
            });
            authorities.push(BridgeAuthority {
                haneul_address,
                pubkey,
                voting_power,
                base_url: base_url.into(),
                is_blocklisted: blocklisted,
            });
        }
        BridgeCommittee::new(authorities)
    }

    pub async fn get_chain_identifier(&self) -> BridgeResult<String> {
        self.inner.get_chain_identifier().await
    }

    pub async fn get_reference_gas_price_until_success(&self) -> u64 {
        loop {
            let Ok(Ok(rgp)) = retry_with_max_elapsed_time!(
                self.inner.get_reference_gas_price(),
                Duration::from_secs(30)
            ) else {
                self.bridge_metrics
                    .haneul_rpc_errors
                    .with_label_values(&["get_reference_gas_price"])
                    .inc();
                error!("Failed to get reference gas price");
                continue;
            };
            return rgp;
        }
    }

    pub async fn get_latest_checkpoint_sequence_number(&self) -> BridgeResult<u64> {
        self.inner.get_latest_checkpoint_sequence_number().await
    }

    pub async fn execute_transaction_block_with_effects(
        &self,
        tx: haneul_types::transaction::Transaction,
    ) -> BridgeResult<ExecuteTransactionResult> {
        self.inner.execute_transaction_block_with_effects(tx).await
    }

    // TODO: this function is very slow (seconds) in tests, we need to optimize it
    pub async fn get_token_transfer_action_onchain_status_until_success(
        &self,
        source_chain_id: u8,
        seq_number: u64,
    ) -> BridgeActionStatus {
        loop {
            let bridge_object_arg = self.get_mutable_bridge_object_arg_must_succeed().await;
            let Ok(Ok(status)) = retry_with_max_elapsed_time!(
                self.inner.get_token_transfer_action_onchain_status(
                    bridge_object_arg,
                    source_chain_id,
                    seq_number
                ),
                Duration::from_secs(30)
            ) else {
                self.bridge_metrics
                    .haneul_rpc_errors
                    .with_label_values(&["get_token_transfer_action_onchain_status"])
                    .inc();
                error!(
                    source_chain_id,
                    seq_number, "Failed to get token transfer action onchain status"
                );
                continue;
            };
            return status;
        }
    }

    pub async fn get_token_transfer_action_onchain_signatures_until_success(
        &self,
        source_chain_id: u8,
        seq_number: u64,
    ) -> Option<Vec<Vec<u8>>> {
        loop {
            let bridge_object_arg = self.get_mutable_bridge_object_arg_must_succeed().await;
            let Ok(Ok(sigs)) = retry_with_max_elapsed_time!(
                self.inner.get_token_transfer_action_onchain_signatures(
                    bridge_object_arg,
                    source_chain_id,
                    seq_number
                ),
                Duration::from_secs(30)
            ) else {
                self.bridge_metrics
                    .haneul_rpc_errors
                    .with_label_values(&["get_token_transfer_action_onchain_signatures"])
                    .inc();
                error!(
                    source_chain_id,
                    seq_number, "Failed to get token transfer action onchain signatures"
                );
                continue;
            };
            return sigs;
        }
    }

    pub async fn get_parsed_token_transfer_message(
        &self,
        source_chain_id: u8,
        seq_number: u64,
    ) -> BridgeResult<Option<ParsedTokenTransferMessage>> {
        let bridge_object_arg = self.get_mutable_bridge_object_arg_must_succeed().await;
        let message = self
            .inner
            .get_parsed_token_transfer_message(bridge_object_arg, source_chain_id, seq_number)
            .await?;
        Ok(match message {
            Some(payload) => Some(ParsedTokenTransferMessage::try_from(payload)?),
            None => None,
        })
    }

    pub async fn get_bridge_record(
        &self,
        source_chain_id: u8,
        seq_number: u64,
    ) -> Result<Option<MoveTypeBridgeRecord>, BridgeError> {
        self.inner
            .get_bridge_record(source_chain_id, seq_number)
            .await
    }

    pub async fn get_gas_data_panic_if_not_gas(
        &self,
        gas_object_id: ObjectID,
    ) -> (GasCoin, ObjectRef, Owner) {
        self.inner
            .get_gas_data_panic_if_not_gas(gas_object_id)
            .await
    }

    pub async fn get_bridge_records_in_range(
        &self,
        source_chain_id: u8,
        start_seq_num: u64,
        end_seq_num: u64,
    ) -> Result<Vec<(u64, MoveTypeBridgeRecord)>, BridgeError> {
        self.inner
            .get_bridge_records_in_range(source_chain_id, start_seq_num, end_seq_num)
            .await
    }

    pub async fn get_token_transfer_next_seq_number(
        &self,
        source_chain_id: u8,
    ) -> Result<u64, BridgeError> {
        self.inner
            .get_token_transfer_next_seq_number(source_chain_id)
            .await
    }

    /// Temporary measure to get corresponding sequence number cursor from a Bridge Module EventID
    pub async fn get_sequence_number_from_event_id(
        &self,
        event_id: EventID,
    ) -> BridgeResult<Option<u64>> {
        let events = self
            .inner
            .get_events_by_tx_digest(event_id.tx_digest)
            .await?;

        let event = events
            .events
            .get(event_id.event_seq as usize)
            .ok_or(BridgeError::NoBridgeEventsInTxPosition)?;

        if event.type_.address.as_ref() != BRIDGE_PACKAGE_ID.as_ref() {
            return Ok(None);
        }

        let bridge_event = match HaneulBridgeEvent::try_from_haneul_event(event)? {
            Some(e) => e,
            None => return Ok(None),
        };

        match bridge_event {
            HaneulBridgeEvent::HaneulToEthTokenBridgeV1(event) => Ok(Some(event.nonce)),
            _ => Ok(None),
        }
    }
}

/// Use a trait to abstract over the HaneulSDKClient and HaneulMockClient for testing.
#[async_trait]
pub trait HaneulClientInner: Send + Sync {
    async fn get_events_by_tx_digest(
        &self,
        tx_digest: TransactionDigest,
    ) -> Result<HaneulEvents, BridgeError>;

    async fn get_chain_identifier(&self) -> Result<String, BridgeError>;

    async fn get_reference_gas_price(&self) -> Result<u64, BridgeError>;

    async fn get_latest_checkpoint_sequence_number(&self) -> Result<u64, BridgeError>;

    async fn get_mutable_bridge_object_arg(&self) -> Result<ObjectArg, BridgeError>;

    async fn get_bridge_summary(&self) -> Result<BridgeSummary, BridgeError>;

    async fn execute_transaction_block_with_effects(
        &self,
        tx: Transaction,
    ) -> Result<ExecuteTransactionResult, BridgeError>;

    async fn get_token_transfer_action_onchain_status(
        &self,
        bridge_object_arg: ObjectArg,
        source_chain_id: u8,
        seq_number: u64,
    ) -> Result<BridgeActionStatus, BridgeError>;

    async fn get_token_transfer_action_onchain_signatures(
        &self,
        bridge_object_arg: ObjectArg,
        source_chain_id: u8,
        seq_number: u64,
    ) -> Result<Option<Vec<Vec<u8>>>, BridgeError>;

    async fn get_parsed_token_transfer_message(
        &self,
        bridge_object_arg: ObjectArg,
        source_chain_id: u8,
        seq_number: u64,
    ) -> Result<Option<MoveTypeParsedTokenTransferMessage>, BridgeError>;

    async fn get_bridge_record(
        &self,
        source_chain_id: u8,
        seq_number: u64,
    ) -> Result<Option<MoveTypeBridgeRecord>, BridgeError>;

    async fn get_gas_data_panic_if_not_gas(
        &self,
        gas_object_id: ObjectID,
    ) -> (GasCoin, ObjectRef, Owner);

    async fn get_bridge_records_in_range(
        &self,
        source_chain_id: u8,
        start_seq_num: u64,
        end_seq_num: u64,
    ) -> Result<Vec<(u64, MoveTypeBridgeRecord)>, BridgeError>;

    async fn get_token_transfer_next_seq_number(
        &self,
        source_chain_id: u8,
    ) -> Result<u64, BridgeError>;
}

#[async_trait]
impl HaneulClientInner for haneul_rpc_api::Client {
    async fn get_events_by_tx_digest(
        &self,
        tx_digest: TransactionDigest,
    ) -> Result<HaneulEvents, BridgeError> {
        let mut client = self.clone();
        let resp = client
            .inner_mut()
            .ledger_client()
            .get_transaction(
                GetTransactionRequest::new(&(tx_digest.into())).with_read_mask(
                    FieldMask::from_paths([
                        ExecutedTransaction::path_builder().digest(),
                        ExecutedTransaction::path_builder().events().finish(),
                        ExecutedTransaction::path_builder().checkpoint(),
                        ExecutedTransaction::path_builder().timestamp(),
                    ]),
                ),
            )
            .await?
            .into_inner();
        let resp = resp.transaction();

        Ok(HaneulEvents {
            transaction_digest: tx_digest,
            checkpoint: resp.checkpoint_opt(),
            timestamp_ms: resp
                .timestamp_opt()
                .map(|timestamp| haneul_rpc::proto::proto_to_timestamp_ms(*timestamp))
                .transpose()?,
            events: resp
                .events()
                .events()
                .iter()
                .enumerate()
                .map(|(idx, event)| {
                    Ok(HaneulEvent {
                        id: EventID {
                            tx_digest,
                            event_seq: idx as u64,
                        },
                        package_id: event.package_id().parse()?,
                        transaction_module: Identifier::new(event.module())?,
                        sender: event.sender().parse()?,
                        type_: event.event_type().parse()?,
                        parsed_json: Default::default(),
                        bcs: BcsEvent::Base64 {
                            bcs: event.contents().value().into(),
                        },
                        timestamp_ms: None,
                    })
                })
                .collect::<Result<_, BridgeError>>()?,
        })
    }

    async fn get_chain_identifier(&self) -> Result<String, BridgeError> {
        let chain_id = self
            .clone()
            .inner_mut()
            .ledger_client()
            .get_service_info(GetServiceInfoRequest::default())
            .await?
            .into_inner()
            .chain_id()
            .parse::<haneul_types::digests::CheckpointDigest>()?;

        Ok(haneul_types::digests::ChainIdentifier::from(chain_id).to_string())
    }

    async fn get_reference_gas_price(&self) -> Result<u64, BridgeError> {
        let mut client = self.clone();
        haneul_rpc::Client::get_reference_gas_price(client.inner_mut())
            .await
            .map_err(Into::into)
    }

    async fn get_latest_checkpoint_sequence_number(&self) -> Result<u64, BridgeError> {
        let mut client = self.clone();
        let resp =
            client
                .inner_mut()
                .ledger_client()
                .get_checkpoint(GetCheckpointRequest::latest().with_read_mask(
                    FieldMask::from_paths([Checkpoint::path_builder().sequence_number()]),
                ))
                .await?
                .into_inner();
        Ok(resp.checkpoint().sequence_number())
    }

    async fn get_mutable_bridge_object_arg(&self) -> Result<ObjectArg, BridgeError> {
        let owner = self
            .clone()
            .inner_mut()
            .ledger_client()
            .get_object(
                GetObjectRequest::new(&(HANEUL_BRIDGE_OBJECT_ID.into())).with_read_mask(
                    FieldMask::from_paths([Object::path_builder().owner().finish()]),
                ),
            )
            .await?
            .into_inner()
            .object()
            .owner()
            .to_owned();
        Ok(ObjectArg::SharedObject {
            id: HANEUL_BRIDGE_OBJECT_ID,
            initial_shared_version: SequenceNumber::from_u64(owner.version()),
            mutability: SharedObjectMutability::Mutable,
        })
    }

    async fn get_bridge_summary(&self) -> Result<BridgeSummary, BridgeError> {
        static BRIDGE_VERSION_ID: tokio::sync::OnceCell<Address> =
            tokio::sync::OnceCell::const_new();

        let bridge_version_id = BRIDGE_VERSION_ID
            .get_or_try_init::<BridgeError, _, _>(|| async {
                let bridge_wrapper_bcs = self
                    .clone()
                    .inner_mut()
                    .ledger_client()
                    .get_object(
                        GetObjectRequest::new(&(HANEUL_BRIDGE_OBJECT_ID.into())).with_read_mask(
                            FieldMask::from_paths([Object::path_builder().contents().finish()]),
                        ),
                    )
                    .await?
                    .into_inner()
                    .object()
                    .contents()
                    .to_owned();

                let bridge_wrapper: BridgeWrapper = bcs::from_bytes(bridge_wrapper_bcs.value())?;

                Ok(bridge_wrapper.version.id.id.bytes.into())
            })
            .await?;

        let bridge_inner_id = bridge_version_id.derive_dynamic_child_id(
            &haneul_sdk_types::TypeTag::U64,
            &bcs::to_bytes(&1u64).unwrap(),
        );

        let field_bcs = self
            .clone()
            .inner_mut()
            .ledger_client()
            .get_object(GetObjectRequest::new(&bridge_inner_id).with_read_mask(
                FieldMask::from_paths([Object::path_builder().contents().finish()]),
            ))
            .await?
            .into_inner()
            .object()
            .contents()
            .to_owned();

        let field: haneul_types::dynamic_field::Field<u64, haneul_types::bridge::BridgeInnerV1> =
            bcs::from_bytes(field_bcs.value())?;
        let summary = field.value.try_into_bridge_summary()?;
        Ok(summary)
    }

    async fn get_token_transfer_action_onchain_status(
        &self,
        _bridge_object_arg: ObjectArg,
        source_chain_id: u8,
        seq_number: u64,
    ) -> Result<BridgeActionStatus, BridgeError> {
        let record = self.get_bridge_record(source_chain_id, seq_number).await?;
        let Some(record) = record else {
            return Ok(BridgeActionStatus::NotFound);
        };

        if record.claimed {
            Ok(BridgeActionStatus::Claimed)
        } else if record.verified_signatures.is_some() {
            Ok(BridgeActionStatus::Approved)
        } else {
            Ok(BridgeActionStatus::Pending)
        }
    }

    async fn get_token_transfer_action_onchain_signatures(
        &self,
        _bridge_object_arg: ObjectArg,
        source_chain_id: u8,
        seq_number: u64,
    ) -> Result<Option<Vec<Vec<u8>>>, BridgeError> {
        let record = self.get_bridge_record(source_chain_id, seq_number).await?;
        Ok(record.and_then(|record| record.verified_signatures))
    }

    async fn execute_transaction_block_with_effects(
        &self,
        tx: Transaction,
    ) -> Result<ExecuteTransactionResult, BridgeError> {
        use haneul_rpc::proto::haneul::rpc::v2::ExecutedTransaction as ProtoExecutedTransaction;
        use haneul_sdk_types::SignedTransaction;
        use move_core_types::language_storage::StructTag;

        let signed_tx: SignedTransaction = tx.try_into().map_err(|e| {
            BridgeError::HaneulTxFailureGeneric(format!("Failed to convert transaction: {:?}", e))
        })?;

        let proto_tx: ProtoTransaction = signed_tx.transaction.into();
        let proto_sigs: Vec<ProtoUserSignature> =
            signed_tx.signatures.into_iter().map(Into::into).collect();

        let request = ExecuteTransactionRequest::default()
            .with_transaction(proto_tx)
            .with_signatures(proto_sigs)
            .with_read_mask(FieldMask::from_paths([
                ProtoExecutedTransaction::path_builder()
                    .effects()
                    .status()
                    .finish(),
                ProtoExecutedTransaction::path_builder()
                    .events()
                    .events()
                    .finish(),
            ]));

        let response = self
            .clone()
            .inner_mut()
            .execution_client()
            .execute_transaction(request)
            .await
            .map_err(|e| {
                BridgeError::HaneulTxFailureGeneric(format!("gRPC execute failed: {:?}", e))
            })?
            .into_inner();

        let executed_tx = response.transaction();

        let effects = executed_tx.effects();
        let status = effects.status();

        let haneul_status = if status.success() {
            HaneulExecutionStatus::Success
        } else {
            let error = status.error();
            let description = error.description().to_string();

            let failure_msg = if !description.is_empty() {
                description
            } else {
                format!("{:?}", error.kind())
            };

            HaneulExecutionStatus::Failure { error: failure_msg }
        };

        let haneul_events: Vec<HaneulEvent> = executed_tx
            .events()
            .events()
            .iter()
            .filter_map(|event| {
                let package_id: ObjectID = event.package_id().parse().ok()?;
                let module = event.module().to_string();
                let sender: haneul_types::base_types::HaneulAddress =
                    event.sender().parse().ok()?;

                let event_type_tag: haneul_types::TypeTag =
                    parse_haneul_type_tag(event.event_type()).ok()?;
                let struct_tag: StructTag = match event_type_tag {
                    haneul_types::TypeTag::Struct(s) => *s,
                    _ => return None,
                };
                let contents = event.contents();
                let bcs_bytes = contents.value().to_vec();

                Some(HaneulEvent {
                    id: EventID {
                        tx_digest: TransactionDigest::default(),
                        event_seq: 0,
                    },
                    package_id,
                    transaction_module: Identifier::new(module).ok()?,
                    sender,
                    type_: struct_tag,
                    parsed_json: serde_json::Value::Null,
                    bcs: BcsEvent::new(bcs_bytes),
                    timestamp_ms: None,
                })
            })
            .collect();

        Ok(ExecuteTransactionResult {
            status: haneul_status,
            events: haneul_events,
        })
    }

    async fn get_parsed_token_transfer_message(
        &self,
        _bridge_object_arg: ObjectArg,
        source_chain_id: u8,
        seq_number: u64,
    ) -> Result<Option<MoveTypeParsedTokenTransferMessage>, BridgeError> {
        let record = self.get_bridge_record(source_chain_id, seq_number).await?;

        let Some(record) = record else {
            return Ok(None);
        };
        let MoveTypeBridgeMessage {
            message_type: _,
            message_version,
            seq_num,
            source_chain,
            payload,
        } = record.message;

        // Parse payload based on message version.
        let parsed_payload: MoveTypeTokenTransferPayload = if message_version == 2 {
            let mut v2: MoveTypeTokenTransferPayloadV2 = bcs::from_bytes(&payload)?;
            v2.amount = u64::from_be_bytes(v2.amount.to_le_bytes());
            v2.into()
        } else {
            let mut v1: MoveTypeTokenTransferPayload = bcs::from_bytes(&payload)?;
            v1.amount = u64::from_be_bytes(v1.amount.to_le_bytes());
            v1
        };

        Ok(Some(MoveTypeParsedTokenTransferMessage {
            message_version,
            seq_num,
            source_chain,
            payload,
            parsed_payload,
        }))
    }

    async fn get_bridge_record(
        &self,
        source_chain_id: u8,
        seq_number: u64,
    ) -> Result<Option<MoveTypeBridgeRecord>, BridgeError> {
        static BRIDGE_RECORDS_ID: tokio::sync::OnceCell<Address> =
            tokio::sync::OnceCell::const_new();

        let records_id = BRIDGE_RECORDS_ID
            .get_or_try_init(|| async {
                self.get_bridge_summary()
                    .await
                    .map(|summary| summary.bridge_records_id.into())
            })
            .await?;

        let record_id = {
            let key = MoveTypeBridgeMessageKey {
                source_chain: source_chain_id,
                message_type: crate::types::BridgeActionType::TokenTransfer as u8,
                bridge_seq_num: seq_number,
            };
            let key_bytes = bcs::to_bytes(&key)?;
            let key_type = haneul_sdk_types::StructTag::new(
                Address::from(BRIDGE_PACKAGE_ID),
                haneul_sdk_types::Identifier::from_static("message"),
                haneul_sdk_types::Identifier::from_static("BridgeMessageKey"),
                vec![],
            );

            records_id.derive_dynamic_child_id(&(key_type.into()), &key_bytes)
        };

        let response =
            match self
                .clone()
                .inner_mut()
                .ledger_client()
                .get_object(GetObjectRequest::new(&record_id).with_read_mask(
                    FieldMask::from_paths([Object::path_builder().contents().finish()]),
                ))
                .await
            {
                Ok(response) => response,
                Err(status) => {
                    if status.code() == tonic::Code::NotFound {
                        return Ok(None);
                    } else {
                        return Err(status.into());
                    }
                }
            };

        let field_bcs = response.into_inner().object().contents().to_owned();

        let field: haneul_types::dynamic_field::Field<
            MoveTypeBridgeMessageKey,
            LinkedTableNode<MoveTypeBridgeMessageKey, MoveTypeBridgeRecord>,
        > = bcs::from_bytes(field_bcs.value())?;

        Ok(Some(field.value.value))
    }

    async fn get_gas_data_panic_if_not_gas(
        &self,
        gas_object_id: ObjectID,
    ) -> (GasCoin, ObjectRef, Owner) {
        loop {
            let result = async {
                let resp = self
                    .clone()
                    .inner_mut()
                    .ledger_client()
                    .get_object(
                        GetObjectRequest::new(&(gas_object_id.into())).with_read_mask(
                            FieldMask::from_paths([Object::path_builder().bcs().finish()]),
                        ),
                    )
                    .await?
                    .into_inner();

                let obj = resp.object();
                let object: haneul_types::object::Object =
                    obj.bcs().deserialize().map_err(|e| {
                        BridgeError::Generic(format!("Failed to deserialize object from BCS: {e}"))
                    })?;

                let object_ref = object.compute_object_reference();
                let owner = object.owner().clone();
                let gas_coin = GasCoin::try_from(&object).map_err(|e| {
                    BridgeError::Generic(format!("Failed to convert object to gas coin: {e}"))
                })?;

                Ok::<_, BridgeError>((gas_coin, object_ref, owner))
            }
            .await;

            match result {
                Ok(data) => return data,
                Err(e) => {
                    warn!("Can't get gas object: {:?}: {:?}", gas_object_id, e);
                    tokio::time::sleep(Duration::from_secs(5)).await;
                }
            }
        }
    }

    async fn get_bridge_records_in_range(
        &self,
        source_chain_id: u8,
        start_seq_num: u64,
        end_seq_num: u64,
    ) -> Result<Vec<(u64, MoveTypeBridgeRecord)>, BridgeError> {
        let mut records = Vec::new();
        for seq_num in start_seq_num..=end_seq_num {
            if let Some(record) = self.get_bridge_record(source_chain_id, seq_num).await? {
                records.push((seq_num, record));
            }
        }
        Ok(records)
    }

    async fn get_token_transfer_next_seq_number(
        &self,
        _source_chain_id: u8,
    ) -> Result<u64, BridgeError> {
        let summary = self.get_bridge_summary().await?;
        // The bridge's sequence_nums is keyed by message_type
        const TOKEN_MESSAGE_TYPE: u8 = 0;
        let seq_num = summary
            .sequence_nums
            .iter()
            .find(|(msg_type, _)| *msg_type == TOKEN_MESSAGE_TYPE)
            .map(|(_, seq)| *seq)
            .unwrap_or(0);
        Ok(seq_num)
    }
}

#[async_trait]
impl HaneulClientInner for HaneulClientInternal {
    async fn get_events_by_tx_digest(
        &self,
        tx_digest: TransactionDigest,
    ) -> Result<HaneulEvents, BridgeError> {
        self.grpc_client.get_events_by_tx_digest(tx_digest).await
    }

    async fn get_chain_identifier(&self) -> Result<String, BridgeError> {
        HaneulClientInner::get_chain_identifier(&self.grpc_client).await
    }

    async fn get_reference_gas_price(&self) -> Result<u64, BridgeError> {
        HaneulClientInner::get_reference_gas_price(&self.grpc_client).await
    }

    async fn get_latest_checkpoint_sequence_number(&self) -> Result<u64, BridgeError> {
        self.grpc_client
            .get_latest_checkpoint_sequence_number()
            .await
    }

    async fn get_mutable_bridge_object_arg(&self) -> Result<ObjectArg, BridgeError> {
        self.grpc_client.get_mutable_bridge_object_arg().await
    }

    async fn get_bridge_summary(&self) -> Result<BridgeSummary, BridgeError> {
        self.grpc_client.get_bridge_summary().await
    }

    async fn get_token_transfer_action_onchain_status(
        &self,
        bridge_object_arg: ObjectArg,
        source_chain_id: u8,
        seq_number: u64,
    ) -> Result<BridgeActionStatus, BridgeError> {
        self.grpc_client
            .get_token_transfer_action_onchain_status(
                bridge_object_arg,
                source_chain_id,
                seq_number,
            )
            .await
    }

    async fn get_token_transfer_action_onchain_signatures(
        &self,
        bridge_object_arg: ObjectArg,
        source_chain_id: u8,
        seq_number: u64,
    ) -> Result<Option<Vec<Vec<u8>>>, BridgeError> {
        self.grpc_client
            .get_token_transfer_action_onchain_signatures(
                bridge_object_arg,
                source_chain_id,
                seq_number,
            )
            .await
    }

    async fn execute_transaction_block_with_effects(
        &self,
        tx: Transaction,
    ) -> Result<ExecuteTransactionResult, BridgeError> {
        self.grpc_client
            .execute_transaction_block_with_effects(tx)
            .await
    }

    async fn get_parsed_token_transfer_message(
        &self,
        bridge_object_arg: ObjectArg,
        source_chain_id: u8,
        seq_number: u64,
    ) -> Result<Option<MoveTypeParsedTokenTransferMessage>, BridgeError> {
        self.grpc_client
            .get_parsed_token_transfer_message(bridge_object_arg, source_chain_id, seq_number)
            .await
    }

    async fn get_bridge_record(
        &self,
        source_chain_id: u8,
        seq_number: u64,
    ) -> Result<Option<MoveTypeBridgeRecord>, BridgeError> {
        self.grpc_client
            .get_bridge_record(source_chain_id, seq_number)
            .await
    }

    async fn get_gas_data_panic_if_not_gas(
        &self,
        gas_object_id: ObjectID,
    ) -> (GasCoin, ObjectRef, Owner) {
        self.grpc_client
            .get_gas_data_panic_if_not_gas(gas_object_id)
            .await
    }

    async fn get_bridge_records_in_range(
        &self,
        source_chain_id: u8,
        start_seq_num: u64,
        end_seq_num: u64,
    ) -> Result<Vec<(u64, MoveTypeBridgeRecord)>, BridgeError> {
        self.grpc_client
            .get_bridge_records_in_range(source_chain_id, start_seq_num, end_seq_num)
            .await
    }

    async fn get_token_transfer_next_seq_number(
        &self,
        source_chain_id: u8,
    ) -> Result<u64, BridgeError> {
        self.grpc_client
            .get_token_transfer_next_seq_number(source_chain_id)
            .await
    }
}

#[cfg(test)]
mod tests {
    use crate::crypto::BridgeAuthorityKeyPair;
    use crate::e2e_tests::test_utils::TestClusterWrapperBuilder;
    use crate::types::HaneulToEthTokenTransfer;
    use crate::{
        events::{EmittedHaneulToEthTokenBridgeV1, MoveTokenDepositedEvent},
        haneul_mock_client::HaneulMockClient,
        test_utils::{
            approve_action_with_validator_secrets, bridge_token,
            get_test_eth_to_haneul_bridge_action, get_test_haneul_to_eth_bridge_action,
        },
    };
    use alloy::primitives::Address as EthAddress;
    use haneul_json_rpc_types::BcsEvent;
    use haneul_types::base_types::HaneulAddress;
    use haneul_types::bridge::{BridgeChainId, TOKEN_ID_HANEUL, TOKEN_ID_USDC};
    use haneul_types::crypto::get_key_pair;
    use move_core_types::account_address::AccountAddress;
    use serde::{Deserialize, Serialize};
    use std::str::FromStr;

    use super::*;
    use crate::events::{HaneulToEthTokenBridgeV1, init_all_struct_tags};

    #[tokio::test]
    async fn get_bridge_action_by_tx_digest_and_event_idx_maybe() {
        // Note: for random events generated in this test, we only care about
        // tx_digest and event_seq, so it's ok that package and module does
        // not match the query parameters.
        telemetry_subscribers::init_for_testing();
        let mock_client = HaneulMockClient::default();
        let haneul_client = HaneulClient::new_for_testing(mock_client.clone());
        let tx_digest = TransactionDigest::random();

        // Ensure all struct tags are inited
        init_all_struct_tags();

        let sanitized_event_1 = EmittedHaneulToEthTokenBridgeV1 {
            nonce: 1,
            haneul_chain_id: BridgeChainId::HaneulTestnet,
            haneul_address: HaneulAddress::random_for_testing_only(),
            eth_chain_id: BridgeChainId::EthSepolia,
            eth_address: EthAddress::random(),
            token_id: TOKEN_ID_HANEUL,
            amount_haneul_adjusted: 100,
        };
        let emitted_event_1 = MoveTokenDepositedEvent {
            seq_num: sanitized_event_1.nonce,
            source_chain: sanitized_event_1.haneul_chain_id as u8,
            sender_address: sanitized_event_1.haneul_address.to_vec(),
            target_chain: sanitized_event_1.eth_chain_id as u8,
            target_address: sanitized_event_1.eth_address.to_vec(),
            token_type: sanitized_event_1.token_id,
            amount_haneul_adjusted: sanitized_event_1.amount_haneul_adjusted,
        };

        let mut haneul_event_1 = HaneulEvent::random_for_testing();
        haneul_event_1.type_ = HaneulToEthTokenBridgeV1.get().unwrap().clone();
        haneul_event_1.bcs = BcsEvent::new(bcs::to_bytes(&emitted_event_1).unwrap());

        #[derive(Serialize, Deserialize)]
        struct RandomStruct {}

        let event_2: RandomStruct = RandomStruct {};
        // undeclared struct tag
        let mut haneul_event_2 = HaneulEvent::random_for_testing();
        haneul_event_2.type_ = HaneulToEthTokenBridgeV1.get().unwrap().clone();
        haneul_event_2.type_.module = Identifier::from_str("unrecognized_module").unwrap();
        haneul_event_2.bcs = BcsEvent::new(bcs::to_bytes(&event_2).unwrap());

        // Event 3 is defined in non-bridge package
        let mut haneul_event_3 = haneul_event_1.clone();
        haneul_event_3.type_.address = AccountAddress::random();

        mock_client.add_events_by_tx_digest(
            tx_digest,
            vec![
                haneul_event_1.clone(),
                haneul_event_2.clone(),
                haneul_event_1.clone(),
                haneul_event_3.clone(),
            ],
        );
        let expected_action = BridgeAction::HaneulToEthTokenTransfer(HaneulToEthTokenTransfer {
            nonce: sanitized_event_1.nonce,
            haneul_chain_id: sanitized_event_1.haneul_chain_id,
            eth_chain_id: sanitized_event_1.eth_chain_id,
            haneul_address: sanitized_event_1.haneul_address,
            eth_address: sanitized_event_1.eth_address,
            token_id: sanitized_event_1.token_id,
            amount_adjusted: sanitized_event_1.amount_haneul_adjusted,
        });
        assert_eq!(
            haneul_client
                .get_bridge_action_by_tx_digest_and_event_idx_maybe(&tx_digest, 0)
                .await
                .unwrap(),
            expected_action,
        );
        assert_eq!(
            haneul_client
                .get_bridge_action_by_tx_digest_and_event_idx_maybe(&tx_digest, 2)
                .await
                .unwrap(),
            expected_action,
        );
        assert!(matches!(
            haneul_client
                .get_bridge_action_by_tx_digest_and_event_idx_maybe(&tx_digest, 1)
                .await
                .unwrap_err(),
            BridgeError::NoBridgeEventsInTxPosition
        ),);
        assert!(matches!(
            haneul_client
                .get_bridge_action_by_tx_digest_and_event_idx_maybe(&tx_digest, 3)
                .await
                .unwrap_err(),
            BridgeError::BridgeEventInUnrecognizedHaneulPackage
        ),);
        assert!(matches!(
            haneul_client
                .get_bridge_action_by_tx_digest_and_event_idx_maybe(&tx_digest, 4)
                .await
                .unwrap_err(),
            BridgeError::NoBridgeEventsInTxPosition
        ),);

        // if the StructTag matches with unparsable bcs, it returns an error
        haneul_event_2.type_ = HaneulToEthTokenBridgeV1.get().unwrap().clone();
        mock_client.add_events_by_tx_digest(tx_digest, vec![haneul_event_2]);
        haneul_client
            .get_bridge_action_by_tx_digest_and_event_idx_maybe(&tx_digest, 2)
            .await
            .unwrap_err();
    }

    // Test get_action_onchain_status.
    // Use validator secrets to bridge USDC from Ethereum initially.
    // TODO: we need an e2e test for this with published solidity contract and committee with BridgeNodes
    #[tokio::test(flavor = "multi_thread", worker_threads = 8)]
    async fn test_get_action_onchain_status_for_haneul_to_eth_transfer() {
        telemetry_subscribers::init_for_testing();
        let mut bridge_keys = vec![];
        for _ in 0..=3 {
            let (_, kp): (_, BridgeAuthorityKeyPair) = get_key_pair();
            bridge_keys.push(kp);
        }
        let mut test_cluster = TestClusterWrapperBuilder::new()
            .with_bridge_authority_keys(bridge_keys)
            .with_deploy_tokens(true)
            .build()
            .await;

        let bridge_metrics = Arc::new(BridgeMetrics::new_for_testing());
        let haneul_client =
            HaneulClient::new(&test_cluster.inner.fullnode_handle.rpc_url, bridge_metrics)
                .await
                .unwrap();
        let bridge_authority_keys = test_cluster.authority_keys_clone();

        // Wait until committee is set up
        test_cluster
            .trigger_reconfiguration_if_not_yet_and_assert_bridge_committee_initialized()
            .await;
        let context = &mut test_cluster.inner.wallet;
        let sender = context.active_address().unwrap();
        let usdc_amount = 5000000;
        let bridge_object_arg = haneul_client
            .get_mutable_bridge_object_arg_must_succeed()
            .await;
        let id_token_map = haneul_client.get_token_id_map().await.unwrap();

        // 1. Create a Eth -> Haneul Transfer (recipient is sender address), approve with validator secrets and assert its status to be Claimed
        let action =
            get_test_eth_to_haneul_bridge_action(None, Some(usdc_amount), Some(sender), None);
        let usdc_object_ref = approve_action_with_validator_secrets(
            context,
            bridge_object_arg,
            action.clone(),
            &bridge_authority_keys,
            Some(sender),
            &id_token_map,
        )
        .await
        .unwrap();

        let status = haneul_client
            .inner
            .get_token_transfer_action_onchain_status(
                bridge_object_arg,
                action.chain_id() as u8,
                action.seq_number(),
            )
            .await
            .unwrap();
        assert_eq!(status, BridgeActionStatus::Claimed);

        // 2. Create a Haneul -> Eth Transfer, approve with validator secrets and assert its status to be Approved
        // We need to actually send tokens to bridge to initialize the record.
        let eth_recv_address = EthAddress::random();
        let bridge_event = bridge_token(
            context,
            eth_recv_address,
            usdc_object_ref,
            id_token_map.get(&TOKEN_ID_USDC).unwrap().clone(),
            bridge_object_arg,
        )
        .await;
        assert_eq!(bridge_event.nonce, 0);
        assert_eq!(bridge_event.haneul_chain_id, BridgeChainId::HaneulCustom);
        assert_eq!(bridge_event.eth_chain_id, BridgeChainId::EthCustom);
        assert_eq!(bridge_event.eth_address, eth_recv_address);
        assert_eq!(bridge_event.haneul_address, sender);
        assert_eq!(bridge_event.token_id, TOKEN_ID_USDC);
        assert_eq!(bridge_event.amount_haneul_adjusted, usdc_amount);

        let action = get_test_haneul_to_eth_bridge_action(
            None,
            None,
            Some(bridge_event.nonce),
            Some(bridge_event.amount_haneul_adjusted),
            Some(bridge_event.haneul_address),
            Some(bridge_event.eth_address),
            Some(TOKEN_ID_USDC),
        );
        let status = haneul_client
            .inner
            .get_token_transfer_action_onchain_status(
                bridge_object_arg,
                action.chain_id() as u8,
                action.seq_number(),
            )
            .await
            .unwrap();
        // At this point, the record is created and the status is Pending
        assert_eq!(status, BridgeActionStatus::Pending);

        // Approve it and assert its status to be Approved
        approve_action_with_validator_secrets(
            context,
            bridge_object_arg,
            action.clone(),
            &bridge_authority_keys,
            None,
            &id_token_map,
        )
        .await;

        let status = haneul_client
            .inner
            .get_token_transfer_action_onchain_status(
                bridge_object_arg,
                action.chain_id() as u8,
                action.seq_number(),
            )
            .await
            .unwrap();
        assert_eq!(status, BridgeActionStatus::Approved);

        // 3. Create a random action and assert its status as NotFound
        let action =
            get_test_haneul_to_eth_bridge_action(None, None, Some(100), None, None, None, None);
        let status = haneul_client
            .inner
            .get_token_transfer_action_onchain_status(
                bridge_object_arg,
                action.chain_id() as u8,
                action.seq_number(),
            )
            .await
            .unwrap();
        assert_eq!(status, BridgeActionStatus::NotFound);
    }
}
