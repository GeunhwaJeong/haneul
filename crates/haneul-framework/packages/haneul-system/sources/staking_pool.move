// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

#[allow(unused_const)]
module haneul_system::staking_pool;

use haneul::bag::{Self, Bag};
use haneul::balance::{Self, Balance};
use haneul::haneul::HANEUL;
use haneul::table::{Self, Table};

/// StakedHaneul objects cannot be split to below this amount.
const MIN_STAKING_THRESHOLD: u64 = 1_000_000_000; // 1 HANEUL

const EInsufficientPoolTokenBalance: u64 = 0;
const EWrongPool: u64 = 1;
const EWithdrawAmountCannotBeZero: u64 = 2;
const EInsufficientHaneulTokenBalance: u64 = 3;
const EInsufficientRewardsPoolBalance: u64 = 4;
const EDestroyNonzeroBalance: u64 = 5;
const ETokenTimeLockIsSome: u64 = 6;
const EWrongDelegation: u64 = 7;
const EPendingDelegationDoesNotExist: u64 = 8;
const ETokenBalancesDoNotMatchExchangeRate: u64 = 9;
const EDelegationToInactivePool: u64 = 10;
const EDeactivationOfInactivePool: u64 = 11;
const EIncompatibleStakedHaneul: u64 = 12;
const EWithdrawalInSameEpoch: u64 = 13;
const EPoolAlreadyActive: u64 = 14;
const EPoolPreactiveOrInactive: u64 = 15;
const EActivationOfInactivePool: u64 = 16;
const EDelegationOfZeroHaneul: u64 = 17;
const EStakedHaneulBelowThreshold: u64 = 18;
const ECannotMintFungibleStakedHaneulYet: u64 = 19;
const EInvariantFailure: u64 = 20;

/// A staking pool embedded in each validator struct in the system state object.
public struct StakingPool has key, store {
    id: UID,
    /// The epoch at which this pool became active.
    /// The value is `None` if the pool is pre-active and `Some(<epoch_number>)` if active or inactive.
    activation_epoch: Option<u64>,
    /// The epoch at which this staking pool ceased to be active. `None` = {pre-active, active},
    /// `Some(<epoch_number>)` if in-active, and it was de-activated at epoch `<epoch_number>`.
    deactivation_epoch: Option<u64>,
    /// The total number of HANEUL tokens in this pool, including the HANEUL in the rewards_pool, as well as in all the principal
    /// in the `StakedHaneul` object, updated at epoch boundaries.
    haneul_balance: u64,
    /// The epoch stake rewards will be added here at the end of each epoch.
    rewards_pool: Balance<HANEUL>,
    /// Total number of pool tokens issued by the pool.
    pool_token_balance: u64,
    /// Exchange rate history of previous epochs. Key is the epoch number.
    /// The entries start from the `activation_epoch` of this pool and contains exchange rates at the beginning of each epoch,
    /// i.e., right after the rewards for the previous epoch have been deposited into the pool.
    exchange_rates: Table<u64, PoolTokenExchangeRate>,
    /// Pending stake amount for this epoch, emptied at epoch boundaries.
    pending_stake: u64,
    /// Pending stake withdrawn during the current epoch, emptied at epoch boundaries.
    /// This includes both the principal and rewards HANEUL withdrawn.
    pending_total_haneul_withdraw: u64,
    /// Pending pool token withdrawn during the current epoch, emptied at epoch boundaries.
    pending_pool_token_withdraw: u64,
    /// Any extra fields that's not defined statically.
    extra_fields: Bag,
}

/// Struct representing the exchange rate of the stake pool token to HANEUL.
public struct PoolTokenExchangeRate has copy, drop, store {
    haneul_amount: u64,
    pool_token_amount: u64,
}

/// A self-custodial object holding the staked HANEUL tokens.
public struct StakedHaneul has key, store {
    id: UID,
    /// ID of the staking pool we are staking with.
    pool_id: ID,
    /// The epoch at which the stake becomes active.
    stake_activation_epoch: u64,
    /// The staked HANEUL tokens.
    principal: Balance<HANEUL>,
}

/// An alternative to `StakedHaneul` that holds the pool token amount instead of the HANEUL balance.
/// StakedHaneul objects can be converted to FungibleStakedHaneuls after the initial warmup period.
/// The advantage of this is that you can now merge multiple StakedHaneul objects from different
/// activation epochs into a single FungibleStakedHaneul object.
public struct FungibleStakedHaneul has key, store {
    id: UID,
    /// ID of the staking pool we are staking with.
    pool_id: ID,
    /// The pool token amount.
    value: u64,
}

/// Holds useful information
public struct FungibleStakedHaneulData has key, store {
    id: UID,
    /// fungible_staked_haneul supply
    total_supply: u64,
    /// principal balance. Rewards are withdrawn from the reward pool
    principal: Balance<HANEUL>,
}

// === dynamic field keys ===
public struct FungibleStakedHaneulDataKey has copy, drop, store {}

/// Holds the amount of HANEUL that was underflowed when withdrawing from the pool
/// post safe mode. Cleaned up in the same transaction.
public struct UnderflowHaneulBalance has copy, drop, store {}

// ==== initializer ====

/// Create a new, empty staking pool.
public(package) fun new(ctx: &mut TxContext): StakingPool {
    StakingPool {
        id: object::new(ctx),
        activation_epoch: option::none(),
        deactivation_epoch: option::none(),
        haneul_balance: 0,
        rewards_pool: balance::zero(),
        pool_token_balance: 0,
        exchange_rates: table::new(ctx),
        pending_stake: 0,
        pending_total_haneul_withdraw: 0,
        pending_pool_token_withdraw: 0,
        extra_fields: bag::new(ctx),
    }
}

// ==== stake requests ====

/// Request to stake to a staking pool. The stake starts counting at the beginning of the next epoch,
public(package) fun request_add_stake(
    pool: &mut StakingPool,
    stake: Balance<HANEUL>,
    stake_activation_epoch: u64,
    ctx: &mut TxContext,
): StakedHaneul {
    let haneul_amount = stake.value();
    assert!(!pool.is_inactive(), EDelegationToInactivePool);
    assert!(haneul_amount > 0, EDelegationOfZeroHaneul);

    pool.pending_stake = pool.pending_stake + haneul_amount;
    StakedHaneul {
        id: object::new(ctx),
        pool_id: object::id(pool),
        stake_activation_epoch,
        principal: stake,
    }
}

/// Request to withdraw the given stake plus rewards from a staking pool.
/// Both the principal and corresponding rewards in HANEUL are withdrawn.
/// A proportional amount of pool token withdraw is recorded and processed at epoch change time.
public(package) fun request_withdraw_stake(
    pool: &mut StakingPool,
    staked_haneul: StakedHaneul,
    ctx: &TxContext,
): Balance<HANEUL> {
    // stake is inactive and the pool is not preactive - allow direct withdraw
    // the reason why we exclude preactive pools is to avoid potential underflow
    // on subtraction, and we need to enforce `pending_stake_withdraw` call.
    if (staked_haneul.stake_activation_epoch > ctx.epoch() && !pool.is_preactive()) {
        let principal = staked_haneul.into_balance();
        pool.pending_stake = pool.pending_stake - principal.value();
        return principal
    };

    let (pool_token_withdraw_amount, mut principal_withdraw) = pool.withdraw_from_principal(
        staked_haneul,
    );
    let principal_withdraw_amount = principal_withdraw.value();

    let rewards_withdraw = pool.withdraw_rewards(
        principal_withdraw_amount,
        pool_token_withdraw_amount,
        ctx.epoch(),
    );
    let total_haneul_withdraw_amount = principal_withdraw_amount + rewards_withdraw.value();

    pool.pending_total_haneul_withdraw = pool.pending_total_haneul_withdraw + total_haneul_withdraw_amount;
    pool.pending_pool_token_withdraw =
        pool.pending_pool_token_withdraw + pool_token_withdraw_amount;

    // If the pool is inactive or preactive, we immediately process the withdrawal.
    if (pool.is_inactive() || pool.is_preactive()) pool.process_pending_stake_withdraw();

    // TODO: implement withdraw bonding period here.
    principal_withdraw.join(rewards_withdraw);
    principal_withdraw
}

public(package) fun redeem_fungible_staked_haneul(
    pool: &mut StakingPool,
    fungible_staked_haneul: FungibleStakedHaneul,
    ctx: &TxContext,
): Balance<HANEUL> {
    let FungibleStakedHaneul { id, pool_id, value } = fungible_staked_haneul;
    assert!(pool_id == object::id(pool), EWrongPool);

    id.delete();

    let latest_exchange_rate = pool.pool_token_exchange_rate_at_epoch(ctx.epoch());
    let fungible_staked_haneul_data: &mut FungibleStakedHaneulData =
        &mut pool.extra_fields[FungibleStakedHaneulDataKey {}];

    let (
        principal_amount,
        rewards_amount,
    ) = latest_exchange_rate.calculate_fungible_staked_haneul_withdraw_amount(
        value,
        fungible_staked_haneul_data.principal.value(),
        fungible_staked_haneul_data.total_supply,
    );

    fungible_staked_haneul_data.total_supply = fungible_staked_haneul_data.total_supply - value;

    let mut haneul_out = fungible_staked_haneul_data.principal.split(principal_amount);
    haneul_out.join(pool.rewards_pool.split(rewards_amount));

    pool.pending_total_haneul_withdraw = pool.pending_total_haneul_withdraw + haneul_out.value();
    pool.pending_pool_token_withdraw = pool.pending_pool_token_withdraw + value;

    haneul_out
}

/// written in separate function so i can test with random values
/// returns (principal_withdraw_amount, rewards_withdraw_amount)
fun calculate_fungible_staked_haneul_withdraw_amount(
    latest_exchange_rate: PoolTokenExchangeRate,
    fungible_staked_haneul_value: u64,
    fungible_staked_haneul_data_principal_amount: u64, // fungible_staked_haneul_data.principal.value()
    fungible_staked_haneul_data_total_supply: u64, // fungible_staked_haneul_data.total_supply
): (u64, u64) {
    // 1. if the entire FungibleStakedHaneulData supply is redeemed, how much haneul should we receive?
    let total_haneul_amount = latest_exchange_rate.get_haneul_amount(
        fungible_staked_haneul_data_total_supply,
    );

    // min with total_haneul_amount to prevent underflow
    let fungible_staked_haneul_data_principal_amount = fungible_staked_haneul_data_principal_amount.min(
        total_haneul_amount,
    );

    // 2. how much do we need to withdraw from the rewards pool?
    let total_rewards = total_haneul_amount - fungible_staked_haneul_data_principal_amount;

    // 3. proportionally withdraw from both wrt the fungible_staked_haneul_value.
    let principal_withdraw_amount = mul_div!(
        fungible_staked_haneul_value,
        fungible_staked_haneul_data_principal_amount,
        fungible_staked_haneul_data_total_supply,
    );

    let rewards_withdraw_amount = mul_div!(
        fungible_staked_haneul_value,
        total_rewards,
        fungible_staked_haneul_data_total_supply,
    );

    // invariant check, just in case
    let expected_haneul_amount = latest_exchange_rate.get_haneul_amount(fungible_staked_haneul_value);
    assert!(
        principal_withdraw_amount + rewards_withdraw_amount <= expected_haneul_amount,
        EInvariantFailure,
    );

    (principal_withdraw_amount, rewards_withdraw_amount)
}

/// Convert the given staked HANEUL to an FungibleStakedHaneul object
public(package) fun convert_to_fungible_staked_haneul(
    pool: &mut StakingPool,
    staked_haneul: StakedHaneul,
    ctx: &mut TxContext,
): FungibleStakedHaneul {
    let StakedHaneul { id, pool_id, stake_activation_epoch, principal } = staked_haneul;

    assert!(pool_id == object::id(pool), EWrongPool);
    assert!(ctx.epoch() >= stake_activation_epoch, ECannotMintFungibleStakedHaneulYet);
    assert!(!pool.is_preactive() && !pool.is_inactive(), EPoolPreactiveOrInactive);

    id.delete();

    let exchange_rate_at_staking_epoch = pool.pool_token_exchange_rate_at_epoch(
        stake_activation_epoch,
    );

    let pool_token_amount = exchange_rate_at_staking_epoch.get_token_amount(principal.value());
    assert!(pool_token_amount > 0, EStakedHaneulBelowThreshold);

    let key = FungibleStakedHaneulDataKey {};

    if (!pool.extra_fields.contains(key)) {
        pool
            .extra_fields
            .add(
                key,
                FungibleStakedHaneulData {
                    id: object::new(ctx),
                    total_supply: pool_token_amount,
                    principal,
                },
            );
    } else {
        let fungible_staked_haneul_data: &mut FungibleStakedHaneulData = &mut pool.extra_fields[key];
        fungible_staked_haneul_data.total_supply =
            fungible_staked_haneul_data.total_supply + pool_token_amount;
        fungible_staked_haneul_data.principal.join(principal);
    };

    FungibleStakedHaneul {
        id: object::new(ctx),
        pool_id,
        value: pool_token_amount,
    }
}

/// Withdraw the principal HANEUL stored in the StakedHaneul object, and calculate the corresponding amount of pool
/// tokens using exchange rate at staking epoch.
/// Returns values are amount of pool tokens withdrawn and withdrawn principal portion of HANEUL.
public(package) fun withdraw_from_principal(
    pool: &StakingPool,
    staked_haneul: StakedHaneul,
): (u64, Balance<HANEUL>) {
    // Check that the stake information matches the pool.
    assert!(staked_haneul.pool_id == object::id(pool), EWrongPool);

    let exchange_rate_at_staking_epoch = pool.pool_token_exchange_rate_at_epoch(staked_haneul.stake_activation_epoch);
    let principal_withdraw = staked_haneul.into_balance();
    let pool_token_withdraw_amount = exchange_rate_at_staking_epoch.get_token_amount(principal_withdraw.value());

    (pool_token_withdraw_amount, principal_withdraw)
}

/// Allows calling `.into_balance()` on `StakedHaneul` to invoke `unwrap_staked_haneul`
use fun unwrap_staked_haneul as StakedHaneul.into_balance;

fun unwrap_staked_haneul(staked_haneul: StakedHaneul): Balance<HANEUL> {
    let StakedHaneul { id, principal, .. } = staked_haneul;
    id.delete();
    principal
}

// ==== functions called at epoch boundaries ===

/// Called at epoch advancement times to add rewards (in HANEUL) to the staking pool.
public(package) fun deposit_rewards(pool: &mut StakingPool, rewards: Balance<HANEUL>) {
    pool.haneul_balance = pool.haneul_balance + rewards.value();
    pool.rewards_pool.join(rewards);
}

public(package) fun process_pending_stakes_and_withdraws(pool: &mut StakingPool, ctx: &TxContext) {
    let new_epoch = ctx.epoch() + 1;
    pool.process_pending_stake_withdraw();
    pool.process_pending_stake();
    pool
        .exchange_rates
        .add(
            new_epoch,
            PoolTokenExchangeRate {
                haneul_amount: pool.haneul_balance,
                pool_token_amount: pool.pool_token_balance,
            },
        );

    pool.check_balance_invariants(new_epoch);
}

/// Called at epoch boundaries to process pending stake withdraws requested during the epoch.
/// Also called immediately upon withdrawal if the pool is inactive.
fun process_pending_stake_withdraw(pool: &mut StakingPool) {
    pool.haneul_balance = if (pool.haneul_balance >= pool.pending_total_haneul_withdraw) {
        pool.haneul_balance - pool.pending_total_haneul_withdraw
    } else {
        let diff = pool.pending_total_haneul_withdraw - pool.haneul_balance;
        // While this key is expected to be removed in the next call to `process_pending_stake`,
        // we do not call `process_pending_stake` for inactive pools — skip the bookkeeping.
        if (!pool.is_inactive()) {
            pool.extra_fields.add(UnderflowHaneulBalance {}, diff);
        };
        0
    };

    pool.pool_token_balance = if (pool.pool_token_balance >= pool.pending_pool_token_withdraw) {
        pool.pool_token_balance - pool.pending_pool_token_withdraw
    } else {
        0
    };

    pool.pending_total_haneul_withdraw = 0;
    pool.pending_pool_token_withdraw = 0;
}

/// Called at epoch boundaries to process the pending stake.
public(package) fun process_pending_stake(pool: &mut StakingPool) {
    // Use the most up to date exchange rate with the rewards deposited and withdraws effectuated.
    let latest_exchange_rate = PoolTokenExchangeRate {
        haneul_amount: pool.haneul_balance,
        pool_token_amount: pool.pool_token_balance,
    };

    // This key is only present if the `haneul_balance` underflowed, hence, the current value of `haneul_balance`
    // is `0`. Pool token balance will be recalculated automatically for `0` value.
    let haneul_diff = {
        let key = UnderflowHaneulBalance {};
        if (pool.extra_fields.contains(key)) pool.extra_fields.remove(key) else 0
    };

    pool.haneul_balance = pool.haneul_balance + pool.pending_stake - haneul_diff;
    pool.pool_token_balance = latest_exchange_rate.get_token_amount(pool.haneul_balance);
    pool.pending_stake = 0;
}

/// This function does the following:
///     1. Calculates the total amount of HANEUL (including principal and rewards) that the provided pool tokens represent
///        at the current exchange rate.
///     2. Using the above number and the given `principal_withdraw_amount`, calculates the rewards portion of the
///        stake we should withdraw.
///     3. Withdraws the rewards portion from the rewards pool at the current exchange rate. We only withdraw the rewards
///        portion because the principal portion was already taken out of the staker's self custodied StakedHaneul.
fun withdraw_rewards(
    pool: &mut StakingPool,
    principal_withdraw_amount: u64,
    pool_token_withdraw_amount: u64,
    epoch: u64,
): Balance<HANEUL> {
    let exchange_rate = pool.pool_token_exchange_rate_at_epoch(epoch);
    let total_haneul_withdraw_amount = exchange_rate.get_haneul_amount(pool_token_withdraw_amount);
    let mut reward_withdraw_amount = if (total_haneul_withdraw_amount >= principal_withdraw_amount) {
        total_haneul_withdraw_amount - principal_withdraw_amount
    } else 0;

    // This may happen when we are withdrawing everything from the pool and
    // the rewards pool balance may be less than reward_withdraw_amount.
    // TODO: FIGURE OUT EXACTLY WHY THIS CAN HAPPEN.
    reward_withdraw_amount = reward_withdraw_amount.min(pool.rewards_pool.value());
    pool.rewards_pool.split(reward_withdraw_amount)
}

// ==== preactive pool related ====

/// Called by `validator` module to activate a staking pool.
public(package) fun activate_staking_pool(pool: &mut StakingPool, activation_epoch: u64) {
    // Add the initial exchange rate to the table.
    pool.exchange_rates.add(activation_epoch, initial_exchange_rate());
    // Check that the pool is preactive and not inactive.
    assert!(pool.is_preactive(), EPoolAlreadyActive);
    assert!(!pool.is_inactive(), EActivationOfInactivePool);
    // Fill in the active epoch.
    pool.activation_epoch.fill(activation_epoch);
}

// ==== inactive pool related ====

/// Deactivate a staking pool by setting the `deactivation_epoch`. After
/// this pool deactivation, the pool stops earning rewards. Only stake
/// withdraws can be made to the pool.
public(package) fun deactivate_staking_pool(pool: &mut StakingPool, deactivation_epoch: u64) {
    // We can't deactivate an already deactivated pool.
    assert!(!pool.is_inactive(), EDeactivationOfInactivePool);
    pool.deactivation_epoch = option::some(deactivation_epoch);
}

// ==== getters and misc utility functions ====

public fun haneul_balance(pool: &StakingPool): u64 { pool.haneul_balance }

public fun pool_id(staked_haneul: &StakedHaneul): ID { staked_haneul.pool_id }

public use fun fungible_staked_haneul_pool_id as FungibleStakedHaneul.pool_id;

public fun fungible_staked_haneul_pool_id(fungible_staked_haneul: &FungibleStakedHaneul): ID {
    fungible_staked_haneul.pool_id
}

/// Allows calling `.amount()` on `StakedHaneul` to invoke `staked_haneul_amount`
public use fun staked_haneul_amount as StakedHaneul.amount;

/// Returns the principal amount of `StakedHaneul`.
public fun staked_haneul_amount(staked_haneul: &StakedHaneul): u64 { staked_haneul.principal.value() }

public use fun stake_activation_epoch as StakedHaneul.activation_epoch;

/// Returns the activation epoch of `StakedHaneul`.
public fun stake_activation_epoch(staked_haneul: &StakedHaneul): u64 {
    staked_haneul.stake_activation_epoch
}

/// Returns true if the input staking pool is preactive.
public fun is_preactive(pool: &StakingPool): bool {
    pool.activation_epoch.is_none()
}

/// Returns the activation epoch of the `StakingPool`. For validator candidates,
/// or pending validators, the value returned is `None`. For active validators,
/// the value is the epoch before the validator was activated.
public(package) fun activation_epoch(pool: &StakingPool): Option<u64> {
    pool.activation_epoch
}

/// Returns true if the input staking pool is inactive.
public fun is_inactive(pool: &StakingPool): bool {
    pool.deactivation_epoch.is_some()
}

public use fun fungible_staked_haneul_value as FungibleStakedHaneul.value;

public fun fungible_staked_haneul_value(fungible_staked_haneul: &FungibleStakedHaneul): u64 {
    fungible_staked_haneul.value
}

public use fun split_fungible_staked_haneul as FungibleStakedHaneul.split;

public fun split_fungible_staked_haneul(
    fungible_staked_haneul: &mut FungibleStakedHaneul,
    split_amount: u64,
    ctx: &mut TxContext,
): FungibleStakedHaneul {
    assert!(split_amount <= fungible_staked_haneul.value, EInsufficientPoolTokenBalance);

    fungible_staked_haneul.value = fungible_staked_haneul.value - split_amount;

    FungibleStakedHaneul {
        id: object::new(ctx),
        pool_id: fungible_staked_haneul.pool_id,
        value: split_amount,
    }
}

public use fun join_fungible_staked_haneul as FungibleStakedHaneul.join;

public fun join_fungible_staked_haneul(self: &mut FungibleStakedHaneul, other: FungibleStakedHaneul) {
    let FungibleStakedHaneul { id, pool_id, value } = other;
    assert!(self.pool_id == pool_id, EWrongPool);

    id.delete();

    self.value = self.value + value;
}

/// Split StakedHaneul `self` to two parts, one with principal `split_amount`,
/// and the remaining principal is left in `self`.
/// All the other parameters of the StakedHaneul like `stake_activation_epoch` or `pool_id` remain the same.
public fun split(self: &mut StakedHaneul, split_amount: u64, ctx: &mut TxContext): StakedHaneul {
    let original_amount = self.principal.value();
    assert!(split_amount <= original_amount, EInsufficientHaneulTokenBalance);
    let remaining_amount = original_amount - split_amount;
    // Both resulting parts should have at least MIN_STAKING_THRESHOLD.
    assert!(remaining_amount >= MIN_STAKING_THRESHOLD, EStakedHaneulBelowThreshold);
    assert!(split_amount >= MIN_STAKING_THRESHOLD, EStakedHaneulBelowThreshold);
    StakedHaneul {
        id: object::new(ctx),
        pool_id: self.pool_id,
        stake_activation_epoch: self.stake_activation_epoch,
        principal: self.principal.split(split_amount),
    }
}

/// Allows calling `.split_to_sender()` on `StakedHaneul` to invoke `split_staked_haneul`
public use fun split_staked_haneul as StakedHaneul.split_to_sender;

#[allow(lint(public_entry))]
/// Split the given StakedHaneul to the two parts, one with principal `split_amount`,
/// transfer the newly split part to the sender address.
public entry fun split_staked_haneul(stake: &mut StakedHaneul, split_amount: u64, ctx: &mut TxContext) {
    transfer::transfer(stake.split(split_amount, ctx), ctx.sender());
}

/// Allows calling `.join()` on `StakedHaneul` to invoke `join_staked_haneul`
public use fun join_staked_haneul as StakedHaneul.join;

#[allow(lint(public_entry))]
/// Consume the staked haneul `other` and add its value to `self`.
/// Aborts if some of the staking parameters are incompatible (pool id, stake activation epoch, etc.)
public entry fun join_staked_haneul(self: &mut StakedHaneul, other: StakedHaneul) {
    assert!(is_equal_staking_metadata(self, &other), EIncompatibleStakedHaneul);
    let StakedHaneul { id, principal, .. } = other;

    id.delete();
    self.principal.join(principal);
}

/// Returns true if all the staking parameters of the staked haneul except the principal are identical
public fun is_equal_staking_metadata(self: &StakedHaneul, other: &StakedHaneul): bool {
    (self.pool_id == other.pool_id) &&
    (self.stake_activation_epoch == other.stake_activation_epoch)
}

public fun pool_token_exchange_rate_at_epoch(
    pool: &StakingPool,
    epoch: u64,
): PoolTokenExchangeRate {
    // If the pool is preactive then the exchange rate is always 1:1.
    if (pool.is_preactive_at_epoch(epoch)) {
        return initial_exchange_rate()
    };
    let clamped_epoch = pool.deactivation_epoch.get_with_default(epoch);
    let mut epoch = clamped_epoch.min(epoch);
    let activation_epoch = *pool.activation_epoch.borrow();

    // Find the latest epoch that's earlier than the given epoch with an entry in the table
    while (epoch >= activation_epoch) {
        if (pool.exchange_rates.contains(epoch)) {
            return pool.exchange_rates[epoch]
        };
        epoch = epoch - 1;
    };
    // This line really should be unreachable. Do we want an assert false here?
    initial_exchange_rate()
}

/// Returns the total value of the pending staking requests for this staking pool.
public fun pending_stake_amount(staking_pool: &StakingPool): u64 {
    staking_pool.pending_stake
}

/// Returns the total withdrawal from the staking pool this epoch.
public fun pending_stake_withdraw_amount(staking_pool: &StakingPool): u64 {
    staking_pool.pending_total_haneul_withdraw
}

public(package) fun exchange_rates(pool: &StakingPool): &Table<u64, PoolTokenExchangeRate> {
    &pool.exchange_rates
}

public fun haneul_amount(exchange_rate: &PoolTokenExchangeRate): u64 {
    exchange_rate.haneul_amount
}

public fun pool_token_amount(exchange_rate: &PoolTokenExchangeRate): u64 {
    exchange_rate.pool_token_amount
}

/// Returns true if the provided staking pool is preactive at the provided epoch.
fun is_preactive_at_epoch(pool: &StakingPool, epoch: u64): bool {
    // Either the pool is currently preactive or the pool's starting epoch is later than the provided epoch.
    pool.is_preactive() || (*pool.activation_epoch.borrow() > epoch)
}

fun get_haneul_amount(exchange_rate: &PoolTokenExchangeRate, token_amount: u64): u64 {
    // When either amount is 0, that means we have no stakes with this pool.
    // The other amount might be non-zero when there's dust left in the pool.
    if (exchange_rate.haneul_amount == 0 || exchange_rate.pool_token_amount == 0) {
        return token_amount
    };

    mul_div!(exchange_rate.haneul_amount, token_amount, exchange_rate.pool_token_amount)
}

fun get_token_amount(exchange_rate: &PoolTokenExchangeRate, haneul_amount: u64): u64 {
    // When either amount is 0, that means we have no stakes with this pool.
    // The other amount might be non-zero when there's dust left in the pool.
    if (exchange_rate.haneul_amount == 0 || exchange_rate.pool_token_amount == 0) {
        return haneul_amount
    };

    mul_div!(exchange_rate.pool_token_amount, haneul_amount, exchange_rate.haneul_amount)
}

fun initial_exchange_rate(): PoolTokenExchangeRate {
    PoolTokenExchangeRate { haneul_amount: 0, pool_token_amount: 0 }
}

fun check_balance_invariants(pool: &StakingPool, epoch: u64) {
    let exchange_rate = pool.pool_token_exchange_rate_at_epoch(epoch);
    // check that the pool token balance and haneul balance ratio matches the exchange rate stored.
    let expected = exchange_rate.get_token_amount(pool.haneul_balance);
    let actual = pool.pool_token_balance;
    assert!(expected == actual, ETokenBalancesDoNotMatchExchangeRate)
}

macro fun mul_div($a: u64, $b: u64, $c: u64): u64 {
    (($a as u128) * ($b as u128) / ($c as u128)) as u64
}

// Given the `staked_haneul` receipt calculate the current rewards (in terms of HANEUL) for it.
public(package) fun calculate_rewards(
    pool: &StakingPool,
    staked_haneul: &StakedHaneul,
    current_epoch: u64,
): u64 {
    let staked_amount = staked_haneul.amount();
    let pool_token_withdraw_amount = {
        let exchange_rate_at_staking_epoch = pool.pool_token_exchange_rate_at_epoch(staked_haneul.stake_activation_epoch);
        exchange_rate_at_staking_epoch.get_token_amount(staked_amount)
    };

    let new_epoch_exchange_rate = pool.pool_token_exchange_rate_at_epoch(current_epoch);
    let total_haneul_withdraw_amount = new_epoch_exchange_rate.get_haneul_amount(
        pool_token_withdraw_amount,
    );

    let mut reward_withdraw_amount = if (total_haneul_withdraw_amount >= staked_amount) {
        total_haneul_withdraw_amount - staked_amount
    } else 0;
    reward_withdraw_amount = reward_withdraw_amount.min(pool.rewards_pool.value());

    reward_withdraw_amount
}

// ==== test-related functions ====

#[test_only]
public(package) fun fungible_staked_haneul_data(pool: &StakingPool): &FungibleStakedHaneulData {
    bag::borrow(&pool.extra_fields, FungibleStakedHaneulDataKey {})
}

#[test_only]
public use fun fungible_staked_haneul_data_total_supply as FungibleStakedHaneulData.total_supply;

#[test_only]
public(package) fun fungible_staked_haneul_data_total_supply(
    fungible_staked_haneul_data: &FungibleStakedHaneulData,
): u64 {
    fungible_staked_haneul_data.total_supply
}

#[test_only]
public use fun fungible_staked_haneul_data_principal_value as FungibleStakedHaneulData.principal_value;

#[test_only]
public(package) fun fungible_staked_haneul_data_principal_value(
    fungible_staked_haneul_data: &FungibleStakedHaneulData,
): u64 {
    fungible_staked_haneul_data.principal.value()
}

#[test_only]
public(package) fun pending_pool_token_withdraw_amount(pool: &StakingPool): u64 {
    pool.pending_pool_token_withdraw
}

#[test_only]
public(package) fun create_fungible_staked_haneul_for_testing(
    self: &StakingPool,
    value: u64,
    ctx: &mut TxContext,
): FungibleStakedHaneul {
    FungibleStakedHaneul {
        id: object::new(ctx),
        pool_id: object::id(self),
        value,
    }
}

#[test_only]
public(package) fun process_pending_stake_withdraw_for_testing(pool: &mut StakingPool) {
    pool.process_pending_stake_withdraw()
}

#[test_only]
public(package) fun increase_pending_pool_token_withdraw_for_testing(
    pool: &mut StakingPool,
    delta: u64,
) {
    pool.pending_pool_token_withdraw = pool.pending_pool_token_withdraw + delta
}

#[test_only]
public(package) fun increase_pending_total_haneul_withdraw_for_testing(
    pool: &mut StakingPool,
    delta: u64,
) {
    pool.pending_total_haneul_withdraw = pool.pending_total_haneul_withdraw + delta
}

#[test_only]
public(package) fun pool_token_balance(pool: &StakingPool): u64 {
    pool.pool_token_balance
}

// ==== tests ====

#[random_test]
fun test_calculate_fungible_staked_haneul_withdraw_amount(
    mut total_haneul_amount: u64,
    // these are all in basis points
    mut pool_token_frac: u16,
    mut fungible_staked_haneul_data_total_supply_frac: u16,
    mut fungible_staked_haneul_data_principal_frac: u16,
    mut fungible_staked_haneul_value_bps: u16,
) {
    total_haneul_amount = total_haneul_amount.max(1);

    pool_token_frac = pool_token_frac % 10_000;
    fungible_staked_haneul_data_total_supply_frac =
        fungible_staked_haneul_data_total_supply_frac % 10_000;
    fungible_staked_haneul_data_principal_frac = fungible_staked_haneul_data_principal_frac % 10_000;
    fungible_staked_haneul_value_bps = fungible_staked_haneul_value_bps % 10_000;

    let total_pool_token_amount = mul_div!(total_haneul_amount, pool_token_frac as u64, 10_000).max(1);

    let exchange_rate = PoolTokenExchangeRate {
        haneul_amount: total_haneul_amount,
        pool_token_amount: total_pool_token_amount,
    };

    let fungible_staked_haneul_data_total_supply = mul_div!(
        total_pool_token_amount,
        fungible_staked_haneul_data_total_supply_frac as u64,
        10_000,
    ).max(1);
    let fungible_staked_haneul_value = mul_div!(
        fungible_staked_haneul_data_total_supply,
        fungible_staked_haneul_value_bps as u64,
        10_000,
    );

    let max_principal = exchange_rate.get_haneul_amount(fungible_staked_haneul_data_total_supply);
    let fungible_staked_haneul_data_principal_amount = mul_div!(
        max_principal,
        fungible_staked_haneul_data_principal_frac as u64,
        10_000,
    ).max(1);

    let (principal_amount, rewards_amount) = calculate_fungible_staked_haneul_withdraw_amount(
        exchange_rate,
        fungible_staked_haneul_value,
        fungible_staked_haneul_data_principal_amount,
        fungible_staked_haneul_data_total_supply,
    );

    let expected_out = exchange_rate.get_haneul_amount(fungible_staked_haneul_value);

    assert!(principal_amount + rewards_amount <= expected_out, 0);

    let min_out = if (expected_out > 2) expected_out - 2 else 0;
    assert!(principal_amount + rewards_amount >= min_out, 0);
}
