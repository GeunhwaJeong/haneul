// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use anyhow::bail;
use haneul_rpc_api::Client as GrpcClient;
use haneul_sdk_types::BalanceChange;
use haneul_types::base_types::HaneulAddress;
use haneul_types::gas_coin::GasCoin;
use haneul_types::haneul_sdk_types_conversions::type_tag_core_to_sdk;
use haneul_types::object::Object;
use haneul_types::{base_types::ObjectID, object::Owner, parse_haneul_type_tag};
use tracing::{debug, trace};

/// A util struct that helps verify a Haneul object over gRPC (`LedgerService`).
/// Use builder style to construct the conditions. When optional fields are not
/// set, related checks are omitted. Consuming functions such as `check` perform
/// the check and panic if verification results are unexpected. `check_into_object`
/// and `check_into_gas_coin` return the native `Object` and `GasCoin`
/// respectively.
///
/// Deleted/wrapped/unwrapped dispositions are verified through transaction
/// effects at the call sites, not by observing a `get_object` failure, so this
/// checker only handles the "object still exists" reads.
#[derive(Debug)]
pub struct ObjectChecker {
    object_id: ObjectID,
    owner: Option<Owner>,
    is_haneul_coin: Option<bool>,
}

impl ObjectChecker {
    pub fn new(object_id: ObjectID) -> ObjectChecker {
        Self {
            object_id,
            owner: None,
            is_haneul_coin: None,
        }
    }

    pub fn owner(mut self, owner: Owner) -> Self {
        self.owner = Some(owner);
        self
    }

    pub fn is_haneul_coin(mut self, is_haneul_coin: bool) -> Self {
        self.is_haneul_coin = Some(is_haneul_coin);
        self
    }

    pub async fn check_into_gas_coin(self, client: &GrpcClient) -> GasCoin {
        if self.is_haneul_coin == Some(false) {
            panic!("'check_into_gas_coin' shouldn't be called with 'is_haneul_coin' set as false");
        }
        self.is_haneul_coin(true)
            .check(client)
            .await
            .unwrap()
            .into_gas_coin()
    }

    pub async fn check_into_object(self, client: &GrpcClient) -> Object {
        self.check(client).await.unwrap().into_object()
    }

    pub async fn check(self, client: &GrpcClient) -> Result<CheckerResultObject, anyhow::Error> {
        debug!(?self);

        let object_id = self.object_id;
        let mut client = client.clone();
        let object = match client.get_object(object_id).await {
            Ok(object) => object,
            Err(err) => bail!("Failed to get object info (id: {}), err: {err}", object_id),
        };

        trace!("getting object {object_id}, info :: {object:?}");

        if let Some(owner) = self.owner {
            let object_owner = object.owner().clone();
            assert_eq!(
                object_owner, owner,
                "Object {} does not belong to {}, but {}",
                object_id, owner, object_owner
            );
        }

        if self.is_haneul_coin == Some(true) {
            let gas_coin = GasCoin::try_from(&object).map_err(|e| {
                anyhow::anyhow!("Object {} is not a HANEUL gas coin: {e}", object_id)
            })?;
            return Ok(CheckerResultObject::new(Some(gas_coin), Some(object)));
        }

        Ok(CheckerResultObject::new(None, Some(object)))
    }
}

pub struct CheckerResultObject {
    gas_coin: Option<GasCoin>,
    object: Option<Object>,
}

impl CheckerResultObject {
    pub fn new(gas_coin: Option<GasCoin>, object: Option<Object>) -> Self {
        Self { gas_coin, object }
    }
    pub fn into_gas_coin(self) -> GasCoin {
        self.gas_coin.unwrap()
    }
    pub fn into_object(self) -> Object {
        self.object.unwrap()
    }
}

#[macro_export]
macro_rules! assert_eq_if_present {
    ($left:expr, $right:expr, $($arg:tt)+) => {
        match (&$left, &$right) {
            (Some(left_val), right_val) if !(&left_val == right_val) => {
                panic!("{} does not match, left: {:?}, right: {:?}", $($arg)+, left_val, right_val);
            }
            _ => ()
        }
    };
}

/// Verifies a native SDK balance change (`haneul_sdk_types::BalanceChange`) returned
/// by the gRPC execution result. Coin types are compared exactly (as canonical
/// `TypeTag`s), never by substring matching.
#[derive(Default, Debug)]
pub struct BalanceChangeChecker {
    address: Option<HaneulAddress>,
    coin_type: Option<haneul_sdk_types::TypeTag>,
    amount: Option<i128>,
}

impl BalanceChangeChecker {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn address(mut self, address: HaneulAddress) -> Self {
        self.address = Some(address);
        self
    }

    pub fn coin_type(mut self, coin_type: &str) -> Self {
        // Parse into a native `TypeTag` then into the SDK type so we do an exact,
        // canonical comparison rather than a string/substring match.
        let type_tag = parse_haneul_type_tag(coin_type).unwrap();
        let sdk_type_tag =
            type_tag_core_to_sdk(type_tag).expect("coin type should convert into an SDK TypeTag");
        self.coin_type = Some(sdk_type_tag);
        self
    }

    pub fn amount(mut self, amount: i128) -> Self {
        self.amount = Some(amount);
        self
    }

    pub fn check(self, change: &BalanceChange) {
        let BalanceChange {
            address,
            coin_type,
            amount,
        } = change;

        if let Some(expected) = self.address {
            let expected_sdk: haneul_sdk_types::Address = expected.into();
            assert_eq!(
                &expected_sdk, address,
                "balance change address does not match"
            );
        }
        assert_eq_if_present!(self.coin_type, coin_type, "coin_type");
        assert_eq_if_present!(self.amount, amount, "amount");
    }
}
