// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

//! Helpers for interacting with the `emit_test_event` Move package in tests.

use std::path::PathBuf;

use haneul_indexer_alt_e2e_tests::move_helpers::execute_ptb;
use haneul_indexer_alt_e2e_tests::move_helpers::publish_package;
use haneul_types::HANEUL_CLOCK_OBJECT_ID;
use haneul_types::HANEUL_CLOCK_OBJECT_SHARED_VERSION;
use haneul_types::base_types::ObjectID;
use haneul_types::base_types::ObjectRef;
use haneul_types::effects::TransactionEffectsAPI;
use haneul_types::programmable_transaction_builder::ProgrammableTransactionBuilder;
use haneul_types::transaction::ObjectArg;
use haneul_types::transaction::SharedObjectMutability;
use move_core_types::ident_str;

pub async fn publish(cluster: &mut test_cluster::TestCluster) -> ObjectID {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.extend(["packages", "event", "emit_test_event"]);
    publish_package(cluster, &path).await
}

/// Append a call to `emit_test_event::emit_test_event()` onto an existing PTB. Use this
/// to compose with other move calls in the same transaction.
pub fn add_emit_call(ptb: &mut ProgrammableTransactionBuilder, package_id: ObjectID) {
    ptb.programmable_move_call(
        package_id,
        ident_str!("emit_test_event").to_owned(),
        ident_str!("emit_test_event").to_owned(),
        vec![],
        vec![],
    );
}

/// Append a call to `emit_test_event::emit_with_value(value)` onto an existing PTB.
/// Each call with a distinct `value` produces a distinct `TestEvent`, useful for tests
/// that need to identify events individually.
pub fn add_emit_with_value_call(
    ptb: &mut ProgrammableTransactionBuilder,
    package_id: ObjectID,
    value: u64,
) {
    let value_arg = ptb.pure(value).unwrap();
    ptb.programmable_move_call(
        package_id,
        ident_str!("emit_test_event").to_owned(),
        ident_str!("emit_with_value").to_owned(),
        vec![],
        vec![value_arg],
    );
}

pub async fn emit(cluster: &mut test_cluster::TestCluster, package_id: ObjectID) -> String {
    let mut ptb = ProgrammableTransactionBuilder::new();
    add_emit_call(&mut ptb, package_id);
    execute_ptb(cluster, ptb).await.0
}

pub async fn emit_with_value(
    cluster: &mut test_cluster::TestCluster,
    package_id: ObjectID,
    value: u64,
) -> String {
    let mut ptb = ProgrammableTransactionBuilder::new();
    add_emit_with_value_call(&mut ptb, package_id, value);
    execute_ptb(cluster, ptb).await.0
}

/// Emit a `TestEvent` and create a `TestObject` in the same transaction. Used by tests
/// that need an event subscription yield to also reference object changes via
/// `event { transaction { effects { objectChanges } } }`.
pub async fn emit_and_create(
    cluster: &mut test_cluster::TestCluster,
    package_id: ObjectID,
    value: u64,
) -> String {
    let sender = cluster.wallet.active_address().unwrap();
    let mut ptb = ProgrammableTransactionBuilder::new();
    let value_arg = ptb.pure(value).unwrap();
    let test_object = ptb.programmable_move_call(
        package_id,
        ident_str!("emit_test_event").to_owned(),
        ident_str!("emit_and_create").to_owned(),
        vec![],
        vec![value_arg],
    );
    ptb.transfer_arg(sender, test_object);
    execute_ptb(cluster, ptb).await.0
}

/// Create a `TestObject` (no event emitted), transferred to the sender. Returns the
/// transaction digest and the new object's ref so a subsequent `mutate_and_emit` call
/// can reference it.
pub async fn create_object(
    cluster: &mut test_cluster::TestCluster,
    package_id: ObjectID,
    value: u64,
) -> (String, ObjectRef) {
    let sender = cluster.wallet.active_address().unwrap();
    let mut ptb = ProgrammableTransactionBuilder::new();
    let value_arg = ptb.pure(value).unwrap();
    let test_object = ptb.programmable_move_call(
        package_id,
        ident_str!("emit_test_event").to_owned(),
        ident_str!("create_object").to_owned(),
        vec![],
        vec![value_arg],
    );
    ptb.transfer_arg(sender, test_object);
    let (digest, effects) = execute_ptb(cluster, ptb).await;
    let created = effects
        .created()
        .into_iter()
        .find(|(_, owner)| owner.get_address_owner_address().is_ok())
        .expect("Should have created an owned object")
        .0;
    (digest, created)
}

/// Mutate an existing `TestObject` and emit a `TestAddressEvent` carrying its address.
/// Pairs with `create_object` for tests that want to observe the object's
/// `inputState`/`outputState` transition through `asTransactionObject`.
pub async fn mutate_and_emit(
    cluster: &mut test_cluster::TestCluster,
    package_id: ObjectID,
    object_ref: ObjectRef,
    new_value: u64,
) -> String {
    let mut ptb = ProgrammableTransactionBuilder::new();
    let object_arg = ptb.obj(ObjectArg::ImmOrOwnedObject(object_ref)).unwrap();
    let value_arg = ptb.pure(new_value).unwrap();
    ptb.programmable_move_call(
        package_id,
        ident_str!("emit_test_event").to_owned(),
        ident_str!("mutate_and_emit").to_owned(),
        vec![],
        vec![object_arg, value_arg],
    );
    execute_ptb(cluster, ptb).await.0
}

/// Emit a `TestAddressEvent` whose payload address points at the (read-only) shared clock
/// object (`0x6`). Used by `asTransactionObject` tests for the `ConsensusObjectRead`
/// variant: the clock is referenced as a read-only consensus input, not a change.
pub async fn emit_with_clock(
    cluster: &mut test_cluster::TestCluster,
    package_id: ObjectID,
) -> String {
    let mut ptb = ProgrammableTransactionBuilder::new();
    let clock_arg = ptb
        .obj(ObjectArg::SharedObject {
            id: HANEUL_CLOCK_OBJECT_ID,
            initial_shared_version: HANEUL_CLOCK_OBJECT_SHARED_VERSION,
            mutability: SharedObjectMutability::Immutable,
        })
        .unwrap();
    ptb.programmable_move_call(
        package_id,
        ident_str!("emit_test_event").to_owned(),
        ident_str!("emit_with_clock").to_owned(),
        vec![],
        vec![clock_arg],
    );
    execute_ptb(cluster, ptb).await.0
}
