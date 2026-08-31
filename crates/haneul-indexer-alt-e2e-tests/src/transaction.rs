// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

//! Transaction-building helpers shared across the integration tests.

use haneul_types::base_types::HaneulAddress;
use haneul_types::base_types::ObjectRef;
use haneul_types::crypto::AccountKeyPair;
use haneul_types::digests::TransactionDigest;
use haneul_types::effects::TransactionEffectsAPI;
use haneul_types::programmable_transaction_builder::ProgrammableTransactionBuilder;
use haneul_types::transaction::Transaction;
use haneul_types::transaction::TransactionData;

use crate::FullCluster;

/// 5 HANEUL — a default gas budget generous enough for the simple transactions these tests build.
pub const DEFAULT_GAS_BUDGET: u64 = 5_000_000_000;

/// Execute a transfer of `amount` GEUNHWA from `sender` to itself, paid for by and gas-threaded through
/// `gas`, signed by `kp`. Returns the new gas object reference (to thread into the next transaction)
/// and the transaction's digest.
pub fn send_haneul(
    cluster: &mut FullCluster,
    sender: HaneulAddress,
    kp: &AccountKeyPair,
    gas: ObjectRef,
    amount: u64,
) -> (ObjectRef, TransactionDigest) {
    let rgp = cluster.reference_gas_price();

    let mut builder = ProgrammableTransactionBuilder::new();
    builder.transfer_haneul(sender, Some(amount));

    let data = TransactionData::new_programmable(
        sender,
        vec![gas],
        builder.finish(),
        DEFAULT_GAS_BUDGET,
        rgp,
    );

    let (fx, _) = cluster
        .execute_transaction(Transaction::from_data_and_signer(data, vec![kp]))
        .expect("Failed to execute transaction");
    assert!(fx.status().is_ok(), "transaction failed: {:?}", fx.status());

    (fx.gas_object().unwrap().0, *fx.transaction_digest())
}
