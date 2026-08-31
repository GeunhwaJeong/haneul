// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

use super::governance_verifier::GovernanceVerifier;
use crate::crypto::{BridgeAuthorityKeyPair, BridgeAuthoritySignInfo};
use crate::error::{BridgeError, BridgeResult};
use crate::eth_client::EthClient;
use crate::haneul_client::{HaneulClient, HaneulClientInner};
use crate::types::{BridgeAction, SignedBridgeAction};
use alloy::primitives::TxHash;
use async_trait::async_trait;
use axum::Json;
use haneul_types::digests::TransactionDigest;
use std::str::FromStr;
use std::sync::Arc;
use tap::TapFallible;
use tracing::info;

#[async_trait]
pub trait BridgeRequestHandlerTrait {
    /// Handles a request to sign a BridgeAction that bridges assets
    /// from Ethereum to Haneul. The inputs are a transaction hash on Ethereum
    /// that emitted the bridge event and the Event index in that transaction
    async fn handle_eth_tx_hash(
        &self,
        tx_hash_hex: String,
        event_idx: u16,
    ) -> Result<Json<SignedBridgeAction>, BridgeError>;
    /// Handles a request to sign a BridgeAction that bridges assets
    /// from Haneul to Ethereum. The inputs are a transaction digest on Haneul
    /// that emitted the bridge event and the Event index in that transaction
    async fn handle_haneul_tx_digest(
        &self,
        tx_digest_base58: String,
        event_idx: u16,
    ) -> Result<Json<SignedBridgeAction>, BridgeError>;

    /// Handles a request to sign a BridgeAction that bridges assets
    /// from Haneul to Ethereum.
    async fn handle_haneul_token_transfer(
        &self,
        source_chain: u8,
        message_type: u8,
        bridge_seq_num: u64,
    ) -> Result<Json<SignedBridgeAction>, BridgeError>;

    /// Handles a request to sign a governance action.
    async fn handle_governance_action(
        &self,
        action: BridgeAction,
    ) -> Result<Json<SignedBridgeAction>, BridgeError>;
}

pub struct BridgeRequestHandler<SC> {
    signer: Arc<BridgeAuthorityKeyPair>,
    haneul_client: Arc<HaneulClient<SC>>,
    eth_client: Arc<EthClient>,
    governance_verifier: GovernanceVerifier,
}

impl<SC> BridgeRequestHandler<SC>
where
    SC: HaneulClientInner + Send + Sync + 'static,
{
    pub fn new(
        signer: BridgeAuthorityKeyPair,
        haneul_client: Arc<HaneulClient<SC>>,
        eth_client: Arc<EthClient>,
        approved_governance_actions: Vec<BridgeAction>,
    ) -> Self {
        let signer = Arc::new(signer);

        Self {
            signer,
            haneul_client,
            eth_client,
            governance_verifier: GovernanceVerifier::new(approved_governance_actions).unwrap(),
        }
    }

    fn sign(&self, bridge_action: BridgeAction) -> SignedBridgeAction {
        let sig = BridgeAuthoritySignInfo::new(&bridge_action, &self.signer);
        SignedBridgeAction::new_from_data_and_sig(bridge_action, sig)
    }

    async fn verify_eth(&self, key: (TxHash, u16)) -> BridgeResult<BridgeAction> {
        let (tx_hash, event_idx) = key;
        self.eth_client
            .get_finalized_bridge_action_maybe(tx_hash, event_idx)
            .await
            .tap_ok(|action| info!("Eth action found: {:?}", action))
    }

    async fn verify_haneul(&self, key: (TransactionDigest, u16)) -> BridgeResult<BridgeAction> {
        let (tx_digest, event_idx) = key;
        self.haneul_client
            .get_bridge_action_by_tx_digest_and_event_idx_maybe(&tx_digest, event_idx)
            .await
            .tap_ok(|action| info!("Haneul action found: {:?}", action))
    }

    async fn verify_haneul_message(
        &self,
        source_chain_id: u8,
        _message_type: u8,
        seq_number: u64,
    ) -> BridgeResult<BridgeAction> {
        let record = self
            .haneul_client
            .get_bridge_record(source_chain_id, seq_number)
            .await?
            .ok_or_else(|| BridgeError::Generic(format!("message {seq_number} not found")))?;
        if record.verified_signatures.is_some() {
            return Err(BridgeError::Generic(format!(
                "message {seq_number} already complete"
            )));
        }
        BridgeAction::try_from_bridge_record(&record)
            .tap_ok(|action| info!("Haneul action found: {:?}", action))
    }
}

#[async_trait]
impl<SC> BridgeRequestHandlerTrait for BridgeRequestHandler<SC>
where
    SC: HaneulClientInner + Send + Sync + 'static,
{
    async fn handle_eth_tx_hash(
        &self,
        tx_hash_hex: String,
        event_idx: u16,
    ) -> Result<Json<SignedBridgeAction>, BridgeError> {
        let tx_hash = TxHash::from_str(&tx_hash_hex).map_err(|_| BridgeError::InvalidTxHash)?;
        let bridge_action = self.verify_eth((tx_hash, event_idx)).await?;
        Ok(Json(self.sign(bridge_action)))
    }

    async fn handle_haneul_tx_digest(
        &self,
        tx_digest_base58: String,
        event_idx: u16,
    ) -> Result<Json<SignedBridgeAction>, BridgeError> {
        let tx_digest = TransactionDigest::from_str(&tx_digest_base58)
            .map_err(|_e| BridgeError::InvalidTxHash)?;

        let bridge_action = self.verify_haneul((tx_digest, event_idx)).await?;
        Ok(Json(self.sign(bridge_action)))
    }

    async fn handle_haneul_token_transfer(
        &self,
        source_chain: u8,
        message_type: u8,
        bridge_seq_num: u64,
    ) -> Result<Json<SignedBridgeAction>, BridgeError> {
        let bridge_action = self
            .verify_haneul_message(source_chain, message_type, bridge_seq_num)
            .await?;
        Ok(Json(self.sign(bridge_action)))
    }

    async fn handle_governance_action(
        &self,
        action: BridgeAction,
    ) -> Result<Json<SignedBridgeAction>, BridgeError> {
        if !action.is_governance_action() {
            return Err(BridgeError::ActionIsNotGovernanceAction(Box::new(action)));
        }
        let bridge_action = self.governance_verifier.verify(action).await?;
        Ok(Json(self.sign(bridge_action)))
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;
    use crate::{
        eth_mock_provider::EthMockService,
        events::{HaneulToEthTokenBridgeV1, MoveTokenDepositedEvent, init_all_struct_tags},
        haneul_mock_client::HaneulMockClient,
        test_utils::{
            get_test_haneul_to_eth_bridge_action, get_test_log_and_action,
            make_transaction_receipt, mock_last_finalized_block,
        },
        types::{EmergencyAction, EmergencyActionType, LimitUpdateAction},
    };
    use alloy::{primitives::Address as EthAddress, rpc::types::TransactionReceipt};
    use haneul_json_rpc_types::{BcsEvent, HaneulEvent};
    use haneul_types::bridge::{BridgeChainId, TOKEN_ID_USDC};
    use haneul_types::{base_types::HaneulAddress, crypto::get_key_pair};

    fn test_handler(
        approved_actions: Vec<BridgeAction>,
    ) -> (
        BridgeRequestHandler<HaneulMockClient>,
        HaneulMockClient,
        EthMockService,
        EthAddress,
    ) {
        let (_, kp): (_, BridgeAuthorityKeyPair) = get_key_pair();
        let haneul_client_mock = HaneulMockClient::default();

        let eth_mock_service = EthMockService::default();
        let contract_address = EthAddress::random();
        let eth_client = EthClient::new_mocked(
            eth_mock_service.clone(),
            HashSet::from_iter(vec![contract_address]),
        );

        let handler = BridgeRequestHandler::new(
            kp,
            Arc::new(HaneulClient::new_for_testing(haneul_client_mock.clone())),
            Arc::new(eth_client),
            approved_actions,
        );
        (
            handler,
            haneul_client_mock,
            eth_mock_service,
            contract_address,
        )
    }

    #[tokio::test]
    async fn test_haneul_verify() {
        let (handler, haneul_client_mock, _, _) = test_handler(vec![]);

        let haneul_tx_digest = TransactionDigest::random();
        let haneul_event_idx = 0;

        // ensure we get an error
        haneul_client_mock.add_events_by_tx_digest_error(haneul_tx_digest);
        handler
            .verify_haneul((haneul_tx_digest, haneul_event_idx))
            .await
            .unwrap_err();

        // Mock a cacheable error such as no bridge events in tx position (empty event list)
        haneul_client_mock.add_events_by_tx_digest(haneul_tx_digest, vec![]);
        assert!(matches!(
            handler
                .verify_haneul((haneul_tx_digest, haneul_event_idx))
                .await,
            Err(BridgeError::NoBridgeEventsInTxPosition)
        ));

        // Test `sign` caches Ok result
        let emitted_event_1 = MoveTokenDepositedEvent {
            seq_num: 1,
            source_chain: BridgeChainId::HaneulCustom as u8,
            sender_address: HaneulAddress::random_for_testing_only().to_vec(),
            target_chain: BridgeChainId::EthCustom as u8,
            target_address: EthAddress::random().to_vec(),
            token_type: TOKEN_ID_USDC,
            amount_haneul_adjusted: 12345,
        };

        init_all_struct_tags();

        let mut haneul_event_1 = HaneulEvent::random_for_testing();
        haneul_event_1.type_ = HaneulToEthTokenBridgeV1.get().unwrap().clone();
        haneul_event_1.bcs = BcsEvent::new(bcs::to_bytes(&emitted_event_1).unwrap());
        let haneul_tx_digest = haneul_event_1.id.tx_digest;

        let mut haneul_event_2 = HaneulEvent::random_for_testing();
        haneul_event_2.type_ = HaneulToEthTokenBridgeV1.get().unwrap().clone();
        haneul_event_2.bcs = BcsEvent::new(bcs::to_bytes(&emitted_event_1).unwrap());
        let haneul_event_idx_2 = 1;
        haneul_client_mock.add_events_by_tx_digest(haneul_tx_digest, vec![haneul_event_2.clone()]);

        haneul_client_mock.add_events_by_tx_digest(
            haneul_tx_digest,
            vec![haneul_event_1.clone(), haneul_event_2.clone()],
        );
        handler
            .verify_haneul((haneul_tx_digest, haneul_event_idx))
            .await
            .unwrap();
        handler
            .verify_haneul((haneul_tx_digest, haneul_event_idx_2))
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_eth_verify() {
        let (handler, _haneul_client_mock, eth_mock_service, contract_address) =
            test_handler(vec![]);

        // Test `sign` Ok result
        let eth_tx_hash = TxHash::random();
        let eth_event_idx = 0;
        let (log, _action) = get_test_log_and_action(contract_address, eth_tx_hash, eth_event_idx);
        eth_mock_service
            .add_response::<[TxHash; 1], TransactionReceipt, TransactionReceipt>(
                "eth_getTransactionReceipt",
                [log.transaction_hash.unwrap()],
                make_transaction_receipt(
                    EthAddress::default(),
                    log.block_number,
                    vec![log.clone()],
                ),
            )
            .unwrap();
        mock_last_finalized_block(&eth_mock_service, log.block_number.unwrap());
        // Mock eth_getBlockByNumber for the block timestamp fetch
        eth_mock_service
            .add_response(
                "eth_getBlockByNumber",
                (format!("0x{:x}", log.block_number.unwrap()), false),
                make_transaction_receipt(
                    EthAddress::default(),
                    log.block_number,
                    vec![log.clone()],
                ),
            )
            .unwrap();

        handler
            .verify_eth((eth_tx_hash, eth_event_idx))
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_signer_with_governance_verifier() {
        let action_1 = BridgeAction::EmergencyAction(EmergencyAction {
            chain_id: BridgeChainId::EthCustom,
            nonce: 1,
            action_type: EmergencyActionType::Pause,
        });
        let action_2 = BridgeAction::LimitUpdateAction(LimitUpdateAction {
            chain_id: BridgeChainId::EthCustom,
            sending_chain_id: BridgeChainId::HaneulCustom,
            nonce: 1,
            new_usd_limit: 10000,
        });

        let (handler, _, _, _) = test_handler(vec![action_1.clone(), action_2.clone()]);
        let verifier = handler.governance_verifier;
        assert_eq!(
            verifier.verify(action_1.clone()).await.unwrap(),
            action_1.clone()
        );
        assert_eq!(
            verifier.verify(action_2.clone()).await.unwrap(),
            action_2.clone()
        );

        // alter action_1 to action_3
        let action_3 = BridgeAction::EmergencyAction(EmergencyAction {
            chain_id: BridgeChainId::EthCustom,
            nonce: 1,
            action_type: EmergencyActionType::Unpause,
        });
        // action_3 is not signable
        assert!(matches!(
            verifier.verify(action_3.clone()).await.unwrap_err(),
            BridgeError::GovernanceActionIsNotApproved
        ));

        // Non governance action is not signable
        let action_4 =
            get_test_haneul_to_eth_bridge_action(None, None, None, None, None, None, None);
        assert!(matches!(
            verifier.verify(action_4.clone()).await.unwrap_err(),
            BridgeError::ActionIsNotGovernanceAction(..)
        ));
    }
}
