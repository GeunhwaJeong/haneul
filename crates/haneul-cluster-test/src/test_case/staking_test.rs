// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

use crate::{TestCaseImpl, TestContext};
use async_trait::async_trait;
use haneul_test_transaction_builder::TestTransactionBuilder;
use haneul_types::base_types::HaneulAddress;
use haneul_types::effects::TransactionEffectsAPI;
use haneul_types::gas_coin::{GAS, GasCoin};
use haneul_types::governance::StakedHaneul;
use tracing::info;

pub struct StakingTest;

#[async_trait]
impl TestCaseImpl for StakingTest {
    fn name(&self) -> &'static str {
        "Staking"
    }

    fn description(&self) -> &'static str {
        "Stake HANEUL with a validator and verify the created StakedHaneul object"
    }

    async fn run(&self, ctx: &mut TestContext) -> Result<(), anyhow::Error> {
        info!("Testing staking workflow");

        let sender = ctx.get_wallet_address();
        // Fund two coins: one to stake, one to pay for gas. Both object refs are
        // supplied explicitly (from the faucet response), keeping transaction
        // construction deterministic.
        let coins = ctx.get_haneul_from_faucet(Some(2)).await;
        let stake_coin_id = *coins[0].id();
        let gas_coin_id = *coins[1].id();

        // Pick a validator from the gRPC system-state summary and remember its
        // staking pool so we can correlate the created StakedHaneul object.
        let system_state = ctx.get_latest_haneul_system_state().await;
        let validator = system_state
            .active_validators
            .first()
            .expect("Should have at least one active validator");
        let validator_addr = validator.haneul_address;
        let validator_pool_id = validator.staking_pool_id;
        info!("Staking to validator: {validator_addr} (pool {validator_pool_id})");

        let gas_price = ctx.get_reference_gas_price().await;
        let gas_ref = ctx.current_object_ref(gas_coin_id).await;
        let stake_ref = ctx.current_object_ref(stake_coin_id).await;

        // Snapshot the staker's indexed HANEUL balance and owned `Coin<HANEUL>` count
        // (StateService) before staking, so we can verify the post-tx accounting.
        let haneul_balance_before = Self::haneul_balance(ctx, sender).await;
        let haneul_coin_count_before = Self::haneul_coin_count(ctx, sender).await;

        let data = TestTransactionBuilder::new(sender, gas_ref, gas_price)
            .call_staking(stake_ref, validator_addr)
            .build();
        let response = ctx.sign_and_execute(data, "staking transaction").await;

        // The staker's HANEUL balance change from the executed transaction effects.
        let sender_sdk: haneul_sdk_types::Address = sender.into();
        let haneul_type: haneul_sdk_types::TypeTag =
            haneul_types::haneul_sdk_types_conversions::type_tag_core_to_sdk(GAS::type_tag())
                .unwrap();
        let haneul_balance_change = response
            .balance_changes
            .iter()
            .find(|b| b.address == sender_sdk && b.coin_type == haneul_type)
            .map(|b| b.amount)
            .expect("staking should produce a HANEUL balance change for the staker");

        // Identify the created StakedHaneul object from the effects, then fetch and
        // decode it natively (`LedgerService`).
        let created = response.effects.created();
        let mut staked_haneul = None;
        for (obj_ref, _owner) in &created {
            let object = ctx.get_grpc_client().get_object(obj_ref.0).await?;
            if let Ok(stake) = StakedHaneul::try_from(&object) {
                staked_haneul = Some((object, stake));
                break;
            }
        }
        let (object, stake) = staked_haneul.expect("Staking should create a StakedHaneul object");

        // Owner is the staker.
        assert_eq!(
            object.owner(),
            &haneul_types::object::Owner::AddressOwner(sender),
            "StakedHaneul should be owned by the staker",
        );
        // Principal matches the staked coin value.
        assert_eq!(
            stake.principal(),
            coins[0].value(),
            "StakedHaneul principal should equal the staked coin value",
        );
        // Staking pool matches the chosen validator.
        assert_eq!(
            stake.pool_id(),
            validator_pool_id,
            "StakedHaneul pool should match the validator's staking pool",
        );
        // A newly requested stake activates in the epoch AFTER the one the
        // transaction executed in — it must not be reported as active now.
        // Derive the execution epoch from the transaction's own checkpoint
        // (a pre-execution system-state read races epoch boundaries).
        let checkpoint_seq = response
            .checkpoint
            .expect("waited execution should carry a checkpoint");
        let execution_epoch = ctx
            .get_grpc_client()
            .get_checkpoint_summary(checkpoint_seq)
            .await?
            .data()
            .epoch;
        assert_eq!(
            stake.activation_epoch(),
            execution_epoch + 1,
            "Newly requested stake should activate in the epoch after execution",
        );
        info!(
            "Staking verified: StakedHaneul {} principal {} pool {} activates epoch {}",
            stake.id(),
            stake.principal(),
            stake.pool_id(),
            stake.activation_epoch(),
        );

        // Balance / coin-count accounting (StateService), matching the old
        // CoinIndex staking flow:
        //  - the staked `Coin<HANEUL>` object is consumed, so the owned HANEUL coin
        //    count drops by exactly one (the gas coin remains);
        //  - the indexed HANEUL balance changes by exactly the effects-reported
        //    balance change (principal staked + gas spent).
        let haneul_coin_count_after = Self::haneul_coin_count(ctx, sender).await;
        assert_eq!(
            haneul_coin_count_after,
            haneul_coin_count_before - 1,
            "staking should consume exactly one Coin<HANEUL> object",
        );
        let haneul_balance_after = Self::haneul_balance(ctx, sender).await;
        assert_eq!(
            haneul_balance_after,
            (haneul_balance_before as i128 + haneul_balance_change) as u128,
            "post-stake HANEUL balance should equal pre-stake balance plus the effects balance change",
        );

        Ok(())
    }
}

impl StakingTest {
    /// Indexed total HANEUL balance (`StateService::GetBalance`, keyed on the inner
    /// coin type `0x2::haneul::HANEUL`).
    async fn haneul_balance(ctx: &TestContext, owner: HaneulAddress) -> u128 {
        ctx.get_grpc_client()
            .get_balance(owner, &GAS::type_())
            .await
            .unwrap()
            .balance
            .unwrap_or_default() as u128
    }

    /// Count of owned `Coin<HANEUL>` objects (`StateService::ListOwnedObjects`
    /// filtered by the object type `Coin<0x2::haneul::HANEUL>`). Stream errors are
    /// propagated (not counted as objects), so an RPC or decoding failure fails
    /// the test rather than inflating the count.
    async fn haneul_coin_count(ctx: &TestContext, owner: HaneulAddress) -> usize {
        use futures::TryStreamExt;
        let client = ctx.get_grpc_client();
        let objects: Vec<_> = client
            .list_owned_objects(owner, Some(GasCoin::type_()))
            .try_collect()
            .await
            .expect("failed to enumerate owned Coin<HANEUL> objects");
        objects.len()
    }
}
