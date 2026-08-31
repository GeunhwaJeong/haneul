// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

use anyhow::Result;
use async_trait::async_trait;
use haneul_core::authority::AuthorityState;
use haneul_macros::*;
use haneul_swarm_config::genesis_config::{AccountConfig, DEFAULT_GAS_AMOUNT};
use haneul_types::{
    HANEUL_SYSTEM_PACKAGE_ID,
    base_types::{HaneulAddress, ObjectDigest, ObjectID, ObjectRef},
    governance::StakedHaneul,
    haneul_system_state::{
        HaneulSystemStateTrait,
        haneul_system_state_summary::{HaneulSystemStateSummary, HaneulValidatorSummary},
    },
    object::{Object, Owner},
    programmable_transaction_builder::ProgrammableTransactionBuilder,
    storage::ObjectStore,
    transaction::{
        Argument, Command, ObjectArg, ProgrammableTransaction,
        TEST_ONLY_GAS_UNIT_FOR_HEAVY_COMPUTATION_STORAGE, TransactionData,
    },
};
use haneul_types::{
    base_types::SequenceNumber,
    effects::{TransactionEffects, TransactionEffectsAPI},
};
use haneullabs_common::ZipDebugEqIteratorExt;
use move_core_types::ident_str;
use rand::{Rng, SeedableRng, rngs::StdRng};
use std::{
    collections::{BTreeMap, BTreeSet},
    sync::Arc,
};
use test_cluster::{TestCluster, TestClusterBuilder};
use tracing::info;

const MAX_DELEGATION_AMOUNT: u64 = 1_000_000_000_000_000; // 1M HANEUL
const MIN_DELEGATION_AMOUNT: u64 = 500_000_000_000_000; // 0.5M HANEUL

macro_rules! move_call {
    {$builder:expr, ($addr:expr)::$module_name:ident::$func:ident($($args:expr),* $(,)?)} => {
        $builder.programmable_move_call(
            $addr,
            ident_str!(stringify!($module_name)).to_owned(),
            ident_str!(stringify!($func)).to_owned(),
            vec![],
            vec![$($args),*],
        )
    }
}

trait GenStateChange {
    type StateChange: StatePredicate;
    fn create(&self, runner: &mut StressTestRunner) -> Self::StateChange;
}

#[async_trait]
trait StatePredicate {
    async fn run(&mut self, runner: &mut StressTestRunner) -> Result<TransactionEffects>;
    async fn pre_epoch_post_condition(
        &mut self,
        runner: &mut StressTestRunner,
        effects: &TransactionEffects,
    );
    #[allow(unused)]
    async fn post_epoch_post_condition(
        &mut self,
        runner: &StressTestRunner,
        effects: &TransactionEffects,
    );
}

#[allow(dead_code)]
struct StressTestRunner {
    pub post_epoch_predicates: Vec<Box<dyn StatePredicate + Send + Sync>>,
    pub test_cluster: TestCluster,
    pub accounts: Vec<HaneulAddress>,
    pub active_validators: BTreeSet<HaneulAddress>,
    pub preactive_validators: BTreeMap<HaneulAddress, u64>,
    pub removed_validators: BTreeSet<HaneulAddress>,
    pub delegation_requests_this_epoch: BTreeMap<ObjectID, HaneulAddress>,
    pub delegation_withdraws_this_epoch: u64,
    pub delegations: BTreeMap<ObjectID, (HaneulAddress, ObjectID, ObjectDigest, SequenceNumber)>,
    pub reports: BTreeMap<HaneulAddress, BTreeSet<HaneulAddress>>,
    pub rng: StdRng,
}

impl StressTestRunner {
    pub async fn new(size: usize) -> Self {
        let test_cluster = TestClusterBuilder::new()
            .with_num_validators(size) // number of validators has to exceed 10
            .with_accounts(vec![
                AccountConfig {
                    gas_amounts: vec![DEFAULT_GAS_AMOUNT],
                    address: None,
                };
                100
            ])
            .build()
            .await;
        let accounts = test_cluster.wallet.get_addresses();
        Self {
            post_epoch_predicates: vec![],
            test_cluster,
            accounts,
            active_validators: BTreeSet::new(),
            preactive_validators: BTreeMap::new(),
            removed_validators: BTreeSet::new(),
            delegation_requests_this_epoch: BTreeMap::new(),
            delegation_withdraws_this_epoch: 0,
            delegations: BTreeMap::new(),
            reports: BTreeMap::new(),
            rng: StdRng::from_seed([0; 32]),
        }
    }

    pub fn pick_random_sender(&mut self) -> HaneulAddress {
        self.accounts[self.rng.gen_range(0..self.accounts.len())]
    }

    pub fn system_state(&self) -> HaneulSystemStateSummary {
        self.state()
            .get_haneul_system_state_object_for_testing()
            .unwrap()
            .into_haneul_system_state_summary()
    }

    pub fn pick_random_active_validator(&mut self) -> HaneulValidatorSummary {
        let system_state = self.system_state();
        system_state
            .active_validators
            .get(self.rng.gen_range(0..system_state.active_validators.len()))
            .unwrap()
            .clone()
    }

    pub async fn run(
        &self,
        sender: HaneulAddress,
        pt: ProgrammableTransaction,
    ) -> TransactionEffects {
        let rgp = self.test_cluster.get_reference_gas_price().await;
        let gas_object = self
            .test_cluster
            .wallet
            .get_one_gas_object_owned_by_address(sender)
            .await
            .unwrap()
            .unwrap();
        let transaction_data = TransactionData::new_programmable(
            sender,
            vec![gas_object],
            pt,
            rgp * TEST_ONLY_GAS_UNIT_FOR_HEAVY_COMPUTATION_STORAGE,
            rgp,
        );
        let transaction = self
            .test_cluster
            .wallet
            .sign_transaction(&transaction_data)
            .await;
        let (effects, _) = self
            .test_cluster
            .execute_transaction_return_raw_effects(transaction)
            .await
            .unwrap();

        assert!(effects.status().is_ok());
        effects
    }

    // Useful for debugging and the like
    pub fn display_effects(&self, effects: &TransactionEffects) {
        println!("CREATED:");
        let state = self.state();

        let epoch_store = state.load_epoch_store_one_call_per_task();
        let backing_package_store = state.get_backing_package_store();
        let mut layout_resolver = epoch_store.executor().type_layout_resolver(
            epoch_store.protocol_config(),
            Box::new(backing_package_store.as_ref()),
        );
        for (obj_ref, _) in effects.created() {
            let object_opt = state
                .get_object_store()
                .get_object_by_key(&obj_ref.0, obj_ref.1);
            let Some(object) = object_opt else { continue };
            let struct_tag = object.struct_tag().unwrap();
            let total_haneul =
                object.get_total_haneul(layout_resolver.as_mut()).unwrap() - object.storage_rebate;
            println!(">> {struct_tag} TOTAL_HANEUL: {total_haneul}");
        }

        println!("MUTATED:");
        for (obj_ref, _) in effects.mutated() {
            let object = state
                .get_object_store()
                .get_object_by_key(&obj_ref.0, obj_ref.1)
                .unwrap();
            let struct_tag = object.struct_tag().unwrap();
            let total_haneul =
                object.get_total_haneul(layout_resolver.as_mut()).unwrap() - object.storage_rebate;
            println!(">> {struct_tag} TOTAL_HANEUL: {total_haneul}");
        }

        println!("CONSENSUS:");
        for kind in effects.input_consensus_objects() {
            let (obj_id, version) = kind.id_and_version();
            let object = state
                .get_object_store()
                .get_object_by_key(&obj_id, version)
                .unwrap();
            let struct_tag = object.struct_tag().unwrap();
            let total_haneul =
                object.get_total_haneul(layout_resolver.as_mut()).unwrap() - object.storage_rebate;
            println!(">> {struct_tag} TOTAL_HANEUL: {total_haneul}");
        }
    }

    /*
    pub fn db(&self) -> Arc<AuthorityStore> {
        self.state().db()
    }*/

    pub fn state(&self) -> Arc<AuthorityState> {
        self.test_cluster.fullnode_handle.haneul_node.state()
    }

    pub async fn change_epoch(&self) {
        let pre_state_summary = self.system_state();
        self.test_cluster.trigger_reconfiguration().await;
        let post_state_summary = self.system_state();
        info!(
            "Changing epoch form {} to {}",
            pre_state_summary.epoch, post_state_summary.epoch
        );
    }

    pub async fn get_created_object_of_type_name(
        &self,
        effects: &TransactionEffects,
        name: &str,
    ) -> Option<Object> {
        self.get_from_effects(&effects.created(), name).await
    }

    #[allow(dead_code)]
    pub async fn get_mutated_object_of_type_name(
        &self,
        effects: &TransactionEffects,
        name: &str,
    ) -> Option<Object> {
        self.get_from_effects(&effects.mutated(), name).await
    }

    fn split_off(builder: &mut ProgrammableTransactionBuilder, amount: u64) -> Argument {
        let amt_arg = builder.pure(amount).unwrap();
        builder.command(Command::SplitCoins(Argument::GasCoin, vec![amt_arg]))
    }

    async fn get_from_effects(&self, effects: &[(ObjectRef, Owner)], name: &str) -> Option<Object> {
        let db = self.state().get_object_store().clone();
        let found: Vec<_> = effects
            .iter()
            .filter_map(|(obj_ref, _)| {
                let object = db.get_object_by_key(&obj_ref.0, obj_ref.1).unwrap();
                let struct_tag = object.struct_tag().unwrap();
                if struct_tag.name.to_string() == name {
                    Some(object)
                } else {
                    None
                }
            })
            .collect();
        assert!(found.len() <= 1, "Multiple objects of type {name} found");
        found.first().cloned()
    }
}

mod add_stake {
    use super::*;
    use haneul_types::effects::TransactionEffects;

    pub struct RequestAddStakeGen;

    pub struct RequestAddStake {
        sender: HaneulAddress,
        stake_amount: u64,
        staked_with: HaneulAddress,
    }

    impl GenStateChange for RequestAddStakeGen {
        type StateChange = RequestAddStake;

        fn create(&self, runner: &mut StressTestRunner) -> Self::StateChange {
            let stake_amount = runner
                .rng
                .gen_range(MIN_DELEGATION_AMOUNT..=MAX_DELEGATION_AMOUNT);
            let staked_with = runner.pick_random_active_validator().haneul_address;
            let sender = runner.pick_random_sender();
            RequestAddStake {
                sender,
                stake_amount,
                staked_with,
            }
        }
    }

    #[async_trait]
    impl StatePredicate for RequestAddStake {
        async fn run(&mut self, runner: &mut StressTestRunner) -> Result<TransactionEffects> {
            let pt = {
                let mut builder = ProgrammableTransactionBuilder::new();
                builder.obj(ObjectArg::HANEUL_SYSTEM_MUT).unwrap();
                builder.pure(self.staked_with).unwrap();
                let coin = StressTestRunner::split_off(&mut builder, self.stake_amount);
                move_call! {
                    builder,
                    (HANEUL_SYSTEM_PACKAGE_ID)::haneul_system::request_add_stake(Argument::Input(0), coin, Argument::Input(1))
                };
                builder.finish()
            };
            let effects = runner.run(self.sender, pt).await;

            Ok(effects)
        }

        async fn pre_epoch_post_condition(
            &mut self,
            runner: &mut StressTestRunner,
            effects: &TransactionEffects,
        ) {
            // Assert that a `StakedHaneul` object matching the amount delegated is created.
            // Assert that this staked haneul
            let object = runner
                .get_created_object_of_type_name(effects, "StakedHaneul")
                .await
                .unwrap();

            // Testing via effects
            {
                let state = runner.state();
                let cache = state.get_backing_package_store();
                let epoch_store = state.load_epoch_store_one_call_per_task();
                let mut layout_resolver = epoch_store
                    .executor()
                    .type_layout_resolver(epoch_store.protocol_config(), Box::new(cache.as_ref()));
                let staked_amount = object.get_total_haneul(layout_resolver.as_mut()).unwrap()
                    - object.storage_rebate;
                assert_eq!(staked_amount, self.stake_amount);
            };

            // Get object contents and make sure that the values in it are correct.
            let staked_haneul: StakedHaneul =
                bcs::from_bytes(object.data.try_as_move().unwrap().contents()).unwrap();

            assert_eq!(staked_haneul.principal(), self.stake_amount);
            assert_eq!(object.owner.get_owner_address().unwrap(), self.sender);

            // Keep track of all delegations, we will need it in stake withdrawals.
            runner.delegations.insert(
                object.id(),
                (self.sender, object.id(), object.digest(), object.version()),
            );
            runner.display_effects(effects);
        }

        async fn post_epoch_post_condition(
            &mut self,
            _runner: &StressTestRunner,
            _effects: &TransactionEffects,
        ) {
            todo!()
        }
    }
}

mod remove_stake {
    use super::*;

    pub struct RequestWithdrawStakeGen;

    pub struct RequestWithdrawStake {
        object_id: ObjectID,
        digest: ObjectDigest,
        version: SequenceNumber,
        owner: HaneulAddress,
    }

    impl GenStateChange for RequestWithdrawStakeGen {
        type StateChange = RequestWithdrawStake;

        fn create(&self, runner: &mut StressTestRunner) -> Self::StateChange {
            // pick next delegation object
            let delegation_object_id = *runner.delegations.keys().next().unwrap();
            let (owner, object_id, digest, version) =
                runner.delegations.remove(&delegation_object_id).unwrap();

            RequestWithdrawStake {
                object_id,
                digest,
                owner,
                version,
            }
        }
    }

    #[async_trait]
    impl StatePredicate for RequestWithdrawStake {
        async fn run(&mut self, runner: &mut StressTestRunner) -> Result<TransactionEffects> {
            let pt = {
                let mut builder = ProgrammableTransactionBuilder::new();
                builder.obj(ObjectArg::HANEUL_SYSTEM_MUT).unwrap();
                let staked_haneul = builder
                    .obj(ObjectArg::ImmOrOwnedObject((
                        self.object_id,
                        self.version,
                        self.digest,
                    )))
                    .unwrap();

                move_call! {
                    builder,
                    (HANEUL_SYSTEM_PACKAGE_ID)::haneul_system::request_withdraw_stake(Argument::Input(0), staked_haneul)
                };
                builder.finish()
            };
            let effects = runner.run(self.owner, pt).await;
            Ok(effects)
        }

        async fn pre_epoch_post_condition(
            &mut self,
            _runner: &mut StressTestRunner,
            _effects: &TransactionEffects,
        ) {
            // keeping the body empty, nothing will really change on that
            // operation except consuming the StakedWal object; actual withdrawal
            // will happen in the next epoch.
        }

        async fn post_epoch_post_condition(
            &mut self,
            _runner: &StressTestRunner,
            _effects: &TransactionEffects,
        ) {
            todo!()
        }
    }
}

#[sim_test]
async fn fuzz_dynamic_committee() {
    let num_operations = 20;
    let committee_size = 12;

    // Add more actions here as we create them
    let mut runner = StressTestRunner::new(committee_size).await;
    let actions = [Box::new(add_stake::RequestAddStakeGen)];

    for _ in 0..num_operations {
        let index = runner.rng.gen_range(0..actions.len());
        let mut task = actions[index].create(&mut runner);
        let effects = task.run(&mut runner).await.unwrap();
        task.pre_epoch_post_condition(&mut runner, &effects).await;
    }

    let mut initial_committee = runner
        .system_state()
        .active_validators
        .iter()
        .map(|v| (v.haneul_address, v.voting_power))
        .collect::<Vec<_>>();

    // Sorted by address.
    initial_committee.sort_by_key(|a| a.0);

    // Advance epoch to see the resulting state.
    runner.change_epoch().await;

    // Collect information about total stake of validators, and then check if each validator's
    // voting power is the right % of the total stake.
    let active_validators = runner.system_state().active_validators;
    let total_stake = active_validators
        .iter()
        .fold(0, |acc, v| acc + v.staking_pool_haneul_balance);

    // Use the formula for voting_power from Haneul System to check if the voting power is correctly
    // set. See `crates/haneul-framework/packages/haneul-system/sources/voting_power.move`.
    // Validator voting power in a larger setup cannot exceed 1000.
    // The remaining voting power is redistributed to the remaining validators.
    //
    // Note: this is a simplified condition with the assumption that no node can have more than
    //  1000 voting power due to the number of validators being > 10. If this was not the case, we'd
    //  have to calculate remainder voting power and redistribute it to the remaining validators.
    active_validators.iter().for_each(|v| {
        assert!(v.voting_power <= 1_000); // limitation
        let calculated_power = ((v.staking_pool_haneul_balance as u128 * 10_000)
            / total_stake as u128)
            .min(1_000) as u64;
        assert!(v.voting_power.abs_diff(calculated_power) < 2); // rounding error correction
    });

    // Unstake all randomly assigned stakes.
    for _ in 0..num_operations {
        let mut task = remove_stake::RequestWithdrawStakeGen.create(&mut runner);
        let effects = task.run(&mut runner).await.unwrap();
        task.pre_epoch_post_condition(&mut runner, &effects).await;
    }

    // Advance epoch, so requests are processed.
    runner.change_epoch().await;

    // Expect the active set to return to initial state.
    let mut post_epoch_committee = runner
        .system_state()
        .active_validators
        .iter()
        .map(|v| (v.haneul_address, v.voting_power))
        .collect::<Vec<_>>();

    post_epoch_committee.sort_by_key(|a| a.0);
    post_epoch_committee
        .iter()
        .zip_debug_eq(initial_committee.iter())
        .for_each(|(a, b)| {
            assert_eq!(a.0, b.0); // same address
            assert!(a.1.abs_diff(b.1) < 2); // rounding error correction
        });
}
