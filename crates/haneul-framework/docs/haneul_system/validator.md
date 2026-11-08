---
title: Module `haneul_system::validator`
---



-  [Struct `ValidatorMetadata`](#haneul_system_validator_ValidatorMetadata)
-  [Struct `Validator`](#haneul_system_validator_Validator)
-  [Struct `StakingRequestEvent`](#haneul_system_validator_StakingRequestEvent)
-  [Struct `UnstakingRequestEvent`](#haneul_system_validator_UnstakingRequestEvent)
-  [Struct `ConvertingToFungibleStakedHaneulEvent`](#haneul_system_validator_ConvertingToFungibleStakedHaneulEvent)
-  [Struct `RedeemingFungibleStakedHaneulEvent`](#haneul_system_validator_RedeemingFungibleStakedHaneulEvent)
-  [Constants](#@Constants_0)
-  [Function `new_metadata`](#haneul_system_validator_new_metadata)
-  [Function `new`](#haneul_system_validator_new)
-  [Function `deactivate`](#haneul_system_validator_deactivate)
-  [Function `activate`](#haneul_system_validator_activate)
-  [Function `adjust_stake_and_gas_price`](#haneul_system_validator_adjust_stake_and_gas_price)
-  [Function `request_add_stake`](#haneul_system_validator_request_add_stake)
-  [Function `convert_to_fungible_staked_haneul`](#haneul_system_validator_convert_to_fungible_staked_haneul)
-  [Function `redeem_fungible_staked_haneul`](#haneul_system_validator_redeem_fungible_staked_haneul)
-  [Function `request_add_stake_at_genesis`](#haneul_system_validator_request_add_stake_at_genesis)
-  [Function `request_withdraw_stake`](#haneul_system_validator_request_withdraw_stake)
-  [Function `request_set_gas_price`](#haneul_system_validator_request_set_gas_price)
-  [Function `set_candidate_gas_price`](#haneul_system_validator_set_candidate_gas_price)
-  [Function `request_set_commission_rate`](#haneul_system_validator_request_set_commission_rate)
-  [Function `set_candidate_commission_rate`](#haneul_system_validator_set_candidate_commission_rate)
-  [Function `deposit_stake_rewards`](#haneul_system_validator_deposit_stake_rewards)
-  [Function `process_pending_stakes_and_withdraws`](#haneul_system_validator_process_pending_stakes_and_withdraws)
-  [Function `is_preactive`](#haneul_system_validator_is_preactive)
-  [Function `metadata`](#haneul_system_validator_metadata)
-  [Function `haneul_address`](#haneul_system_validator_haneul_address)
-  [Function `name`](#haneul_system_validator_name)
-  [Function `description`](#haneul_system_validator_description)
-  [Function `image_url`](#haneul_system_validator_image_url)
-  [Function `project_url`](#haneul_system_validator_project_url)
-  [Function `network_address`](#haneul_system_validator_network_address)
-  [Function `p2p_address`](#haneul_system_validator_p2p_address)
-  [Function `primary_address`](#haneul_system_validator_primary_address)
-  [Function `worker_address`](#haneul_system_validator_worker_address)
-  [Function `protocol_pubkey_bytes`](#haneul_system_validator_protocol_pubkey_bytes)
-  [Function `proof_of_possession`](#haneul_system_validator_proof_of_possession)
-  [Function `network_pubkey_bytes`](#haneul_system_validator_network_pubkey_bytes)
-  [Function `worker_pubkey_bytes`](#haneul_system_validator_worker_pubkey_bytes)
-  [Function `next_epoch_network_address`](#haneul_system_validator_next_epoch_network_address)
-  [Function `next_epoch_p2p_address`](#haneul_system_validator_next_epoch_p2p_address)
-  [Function `next_epoch_primary_address`](#haneul_system_validator_next_epoch_primary_address)
-  [Function `next_epoch_worker_address`](#haneul_system_validator_next_epoch_worker_address)
-  [Function `next_epoch_protocol_pubkey_bytes`](#haneul_system_validator_next_epoch_protocol_pubkey_bytes)
-  [Function `next_epoch_proof_of_possession`](#haneul_system_validator_next_epoch_proof_of_possession)
-  [Function `next_epoch_network_pubkey_bytes`](#haneul_system_validator_next_epoch_network_pubkey_bytes)
-  [Function `next_epoch_worker_pubkey_bytes`](#haneul_system_validator_next_epoch_worker_pubkey_bytes)
-  [Function `operation_cap_id`](#haneul_system_validator_operation_cap_id)
-  [Function `next_epoch_gas_price`](#haneul_system_validator_next_epoch_gas_price)
-  [Function `total_stake_amount`](#haneul_system_validator_total_stake_amount)
-  [Function `stake_amount`](#haneul_system_validator_stake_amount)
-  [Function `total_stake`](#haneul_system_validator_total_stake)
-  [Function `voting_power`](#haneul_system_validator_voting_power)
-  [Function `set_voting_power`](#haneul_system_validator_set_voting_power)
-  [Function `pending_stake_amount`](#haneul_system_validator_pending_stake_amount)
-  [Function `pending_stake_withdraw_amount`](#haneul_system_validator_pending_stake_withdraw_amount)
-  [Function `gas_price`](#haneul_system_validator_gas_price)
-  [Function `commission_rate`](#haneul_system_validator_commission_rate)
-  [Function `pool_token_exchange_rate_at_epoch`](#haneul_system_validator_pool_token_exchange_rate_at_epoch)
-  [Function `staking_pool_id`](#haneul_system_validator_staking_pool_id)
-  [Function `is_duplicate`](#haneul_system_validator_is_duplicate)
-  [Macro function `both_some_and_equal`](#haneul_system_validator_both_some_and_equal)
-  [Function `new_unverified_validator_operation_cap_and_transfer`](#haneul_system_validator_new_unverified_validator_operation_cap_and_transfer)
-  [Function `update_name`](#haneul_system_validator_update_name)
-  [Function `update_description`](#haneul_system_validator_update_description)
-  [Function `update_image_url`](#haneul_system_validator_update_image_url)
-  [Function `update_project_url`](#haneul_system_validator_update_project_url)
-  [Function `update_next_epoch_network_address`](#haneul_system_validator_update_next_epoch_network_address)
-  [Function `update_candidate_network_address`](#haneul_system_validator_update_candidate_network_address)
-  [Function `update_next_epoch_p2p_address`](#haneul_system_validator_update_next_epoch_p2p_address)
-  [Function `update_candidate_p2p_address`](#haneul_system_validator_update_candidate_p2p_address)
-  [Function `update_next_epoch_primary_address`](#haneul_system_validator_update_next_epoch_primary_address)
-  [Function `update_candidate_primary_address`](#haneul_system_validator_update_candidate_primary_address)
-  [Function `update_next_epoch_worker_address`](#haneul_system_validator_update_next_epoch_worker_address)
-  [Function `update_candidate_worker_address`](#haneul_system_validator_update_candidate_worker_address)
-  [Function `update_next_epoch_protocol_pubkey`](#haneul_system_validator_update_next_epoch_protocol_pubkey)
-  [Function `update_candidate_protocol_pubkey`](#haneul_system_validator_update_candidate_protocol_pubkey)
-  [Function `update_next_epoch_network_pubkey`](#haneul_system_validator_update_next_epoch_network_pubkey)
-  [Function `update_candidate_network_pubkey`](#haneul_system_validator_update_candidate_network_pubkey)
-  [Function `update_next_epoch_worker_pubkey`](#haneul_system_validator_update_next_epoch_worker_pubkey)
-  [Function `update_candidate_worker_pubkey`](#haneul_system_validator_update_candidate_worker_pubkey)
-  [Function `effectuate_staged_metadata`](#haneul_system_validator_effectuate_staged_metadata)
-  [Macro function `do_extract`](#haneul_system_validator_do_extract)
-  [Function `validate_metadata`](#haneul_system_validator_validate_metadata)
-  [Function `validate_metadata_bcs`](#haneul_system_validator_validate_metadata_bcs)
-  [Function `get_staking_pool_ref`](#haneul_system_validator_get_staking_pool_ref)
-  [Function `new_from_metadata`](#haneul_system_validator_new_from_metadata)


<pre><code><b>use</b> <a href="../haneul/accumulator.md#haneul_accumulator">haneul::accumulator</a>;
<b>use</b> <a href="../haneul/accumulator_settlement.md#haneul_accumulator_settlement">haneul::accumulator_settlement</a>;
<b>use</b> <a href="../haneul/address.md#haneul_address">haneul::address</a>;
<b>use</b> <a href="../haneul/bag.md#haneul_bag">haneul::bag</a>;
<b>use</b> <a href="../haneul/balance.md#haneul_balance">haneul::balance</a>;
<b>use</b> <a href="../haneul/bcs.md#haneul_bcs">haneul::bcs</a>;
<b>use</b> <a href="../haneul/coin.md#haneul_coin">haneul::coin</a>;
<b>use</b> <a href="../haneul/config.md#haneul_config">haneul::config</a>;
<b>use</b> <a href="../haneul/deny_list.md#haneul_deny_list">haneul::deny_list</a>;
<b>use</b> <a href="../haneul/dynamic_field.md#haneul_dynamic_field">haneul::dynamic_field</a>;
<b>use</b> <a href="../haneul/dynamic_object_field.md#haneul_dynamic_object_field">haneul::dynamic_object_field</a>;
<b>use</b> <a href="../haneul/event.md#haneul_event">haneul::event</a>;
<b>use</b> <a href="../haneul/funds_accumulator.md#haneul_funds_accumulator">haneul::funds_accumulator</a>;
<b>use</b> <a href="../haneul/haneul.md#haneul_haneul">haneul::haneul</a>;
<b>use</b> <a href="../haneul/hash.md#haneul_hash">haneul::hash</a>;
<b>use</b> <a href="../haneul/hex.md#haneul_hex">haneul::hex</a>;
<b>use</b> <a href="../haneul/object.md#haneul_object">haneul::object</a>;
<b>use</b> <a href="../haneul/party.md#haneul_party">haneul::party</a>;
<b>use</b> <a href="../haneul/protocol_config.md#haneul_protocol_config">haneul::protocol_config</a>;
<b>use</b> <a href="../haneul/table.md#haneul_table">haneul::table</a>;
<b>use</b> <a href="../haneul/transfer.md#haneul_transfer">haneul::transfer</a>;
<b>use</b> <a href="../haneul/tx_context.md#haneul_tx_context">haneul::tx_context</a>;
<b>use</b> <a href="../haneul/types.md#haneul_types">haneul::types</a>;
<b>use</b> <a href="../haneul/url.md#haneul_url">haneul::url</a>;
<b>use</b> <a href="../haneul/vec_map.md#haneul_vec_map">haneul::vec_map</a>;
<b>use</b> <a href="../haneul/vec_set.md#haneul_vec_set">haneul::vec_set</a>;
<b>use</b> <a href="../haneul_system/staking_pool.md#haneul_system_staking_pool">haneul_system::staking_pool</a>;
<b>use</b> <a href="../haneul_system/validator_cap.md#haneul_system_validator_cap">haneul_system::validator_cap</a>;
<b>use</b> <a href="../std/address.md#std_address">std::address</a>;
<b>use</b> <a href="../std/ascii.md#std_ascii">std::ascii</a>;
<b>use</b> <a href="../std/bcs.md#std_bcs">std::bcs</a>;
<b>use</b> <a href="../std/internal.md#std_internal">std::internal</a>;
<b>use</b> <a href="../std/option.md#std_option">std::option</a>;
<b>use</b> <a href="../std/string.md#std_string">std::string</a>;
<b>use</b> <a href="../std/type_name.md#std_type_name">std::type_name</a>;
<b>use</b> <a href="../std/u128.md#std_u128">std::u128</a>;
<b>use</b> <a href="../std/u64.md#std_u64">std::u64</a>;
<b>use</b> <a href="../std/vector.md#std_vector">std::vector</a>;
</code></pre>



<a name="haneul_system_validator_ValidatorMetadata"></a>

## Struct `ValidatorMetadata`



<pre><code><b>public</b> <b>struct</b> <a href="../haneul_system/validator.md#haneul_system_validator_ValidatorMetadata">ValidatorMetadata</a> <b>has</b> store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../haneul_system/validator.md#haneul_system_validator_haneul_address">haneul_address</a>: <b>address</b></code>
</dt>
<dd>
 The Haneul Address of the validator. This is the sender that created the Validator object,
 and also the address to send validator/coins to during withdraws.
</dd>
<dt>
<code><a href="../haneul_system/validator.md#haneul_system_validator_protocol_pubkey_bytes">protocol_pubkey_bytes</a>: vector&lt;u8&gt;</code>
</dt>
<dd>
 The public key bytes corresponding to the private key that the validator
 holds to sign transactions. For now, this is the same as AuthorityName.
</dd>
<dt>
<code><a href="../haneul_system/validator.md#haneul_system_validator_network_pubkey_bytes">network_pubkey_bytes</a>: vector&lt;u8&gt;</code>
</dt>
<dd>
 The public key bytes corresponding to the private key that the validator
 uses to establish TLS connections
</dd>
<dt>
<code><a href="../haneul_system/validator.md#haneul_system_validator_worker_pubkey_bytes">worker_pubkey_bytes</a>: vector&lt;u8&gt;</code>
</dt>
<dd>
 The public key bytes corresponding to the Narwhal Worker
</dd>
<dt>
<code><a href="../haneul_system/validator.md#haneul_system_validator_proof_of_possession">proof_of_possession</a>: vector&lt;u8&gt;</code>
</dt>
<dd>
 This is a proof that the validator has ownership of the private key
</dd>
<dt>
<code><a href="../haneul_system/validator.md#haneul_system_validator_name">name</a>: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
 A unique human-readable name of this validator.
</dd>
<dt>
<code><a href="../haneul_system/validator.md#haneul_system_validator_description">description</a>: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../haneul_system/validator.md#haneul_system_validator_image_url">image_url</a>: <a href="../haneul/url.md#haneul_url_Url">haneul::url::Url</a></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../haneul_system/validator.md#haneul_system_validator_project_url">project_url</a>: <a href="../haneul/url.md#haneul_url_Url">haneul::url::Url</a></code>
</dt>
<dd>
</dd>
<dt>
<code>net_address: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
 The network address of the validator (could also contain extra info such as port, DNS and etc.).
</dd>
<dt>
<code><a href="../haneul_system/validator.md#haneul_system_validator_p2p_address">p2p_address</a>: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
 The address of the validator used for p2p activities such as state sync (could also contain extra info such as port, DNS and etc.).
</dd>
<dt>
<code><a href="../haneul_system/validator.md#haneul_system_validator_primary_address">primary_address</a>: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
 The address of the narwhal primary
</dd>
<dt>
<code><a href="../haneul_system/validator.md#haneul_system_validator_worker_address">worker_address</a>: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
 The address of the narwhal worker
</dd>
<dt>
<code><a href="../haneul_system/validator.md#haneul_system_validator_next_epoch_protocol_pubkey_bytes">next_epoch_protocol_pubkey_bytes</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;vector&lt;u8&gt;&gt;</code>
</dt>
<dd>
 "next_epoch" metadata only takes effects in the next epoch.
 If none, current value will stay unchanged.
</dd>
<dt>
<code><a href="../haneul_system/validator.md#haneul_system_validator_next_epoch_proof_of_possession">next_epoch_proof_of_possession</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;vector&lt;u8&gt;&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../haneul_system/validator.md#haneul_system_validator_next_epoch_network_pubkey_bytes">next_epoch_network_pubkey_bytes</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;vector&lt;u8&gt;&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../haneul_system/validator.md#haneul_system_validator_next_epoch_worker_pubkey_bytes">next_epoch_worker_pubkey_bytes</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;vector&lt;u8&gt;&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>next_epoch_net_address: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../haneul_system/validator.md#haneul_system_validator_next_epoch_p2p_address">next_epoch_p2p_address</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../haneul_system/validator.md#haneul_system_validator_next_epoch_primary_address">next_epoch_primary_address</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../haneul_system/validator.md#haneul_system_validator_next_epoch_worker_address">next_epoch_worker_address</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>extra_fields: <a href="../haneul/bag.md#haneul_bag_Bag">haneul::bag::Bag</a></code>
</dt>
<dd>
 Any extra fields that's not defined statically.
</dd>
</dl>


</details>

<a name="haneul_system_validator_Validator"></a>

## Struct `Validator`



<pre><code><b>public</b> <b>struct</b> <a href="../haneul_system/validator.md#haneul_system_validator_Validator">Validator</a> <b>has</b> store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../haneul_system/validator.md#haneul_system_validator_metadata">metadata</a>: <a href="../haneul_system/validator.md#haneul_system_validator_ValidatorMetadata">haneul_system::validator::ValidatorMetadata</a></code>
</dt>
<dd>
 Summary of the validator.
</dd>
<dt>
<code><a href="../haneul_system/voting_power.md#haneul_system_voting_power">voting_power</a>: u64</code>
</dt>
<dd>
 The voting power of this validator, which might be different from its
 stake amount.
</dd>
<dt>
<code><a href="../haneul_system/validator.md#haneul_system_validator_operation_cap_id">operation_cap_id</a>: <a href="../haneul/object.md#haneul_object_ID">haneul::object::ID</a></code>
</dt>
<dd>
 The ID of this validator's current valid <code>UnverifiedValidatorOperationCap</code>
</dd>
<dt>
<code><a href="../haneul_system/validator.md#haneul_system_validator_gas_price">gas_price</a>: u64</code>
</dt>
<dd>
 Gas price quote, updated only at end of epoch.
</dd>
<dt>
<code><a href="../haneul_system/staking_pool.md#haneul_system_staking_pool">staking_pool</a>: <a href="../haneul_system/staking_pool.md#haneul_system_staking_pool_StakingPool">haneul_system::staking_pool::StakingPool</a></code>
</dt>
<dd>
 Staking pool for this validator.
</dd>
<dt>
<code><a href="../haneul_system/validator.md#haneul_system_validator_commission_rate">commission_rate</a>: u64</code>
</dt>
<dd>
 Commission rate of the validator, in basis point.
</dd>
<dt>
<code>next_epoch_stake: u64</code>
</dt>
<dd>
 Total amount of stake that would be active in the next epoch.
</dd>
<dt>
<code><a href="../haneul_system/validator.md#haneul_system_validator_next_epoch_gas_price">next_epoch_gas_price</a>: u64</code>
</dt>
<dd>
 This validator's gas price quote for the next epoch.
</dd>
<dt>
<code>next_epoch_commission_rate: u64</code>
</dt>
<dd>
 The commission rate of the validator starting the next epoch, in basis point.
</dd>
<dt>
<code>extra_fields: <a href="../haneul/bag.md#haneul_bag_Bag">haneul::bag::Bag</a></code>
</dt>
<dd>
 Any extra fields that's not defined statically.
</dd>
</dl>


</details>

<a name="haneul_system_validator_StakingRequestEvent"></a>

## Struct `StakingRequestEvent`

Event emitted when a new stake request is received.


<pre><code><b>public</b> <b>struct</b> <a href="../haneul_system/validator.md#haneul_system_validator_StakingRequestEvent">StakingRequestEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>pool_id: <a href="../haneul/object.md#haneul_object_ID">haneul::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>validator_address: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>staker_address: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>epoch: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>amount: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="haneul_system_validator_UnstakingRequestEvent"></a>

## Struct `UnstakingRequestEvent`

Event emitted when a new unstake request is received.


<pre><code><b>public</b> <b>struct</b> <a href="../haneul_system/validator.md#haneul_system_validator_UnstakingRequestEvent">UnstakingRequestEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>pool_id: <a href="../haneul/object.md#haneul_object_ID">haneul::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>validator_address: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>staker_address: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>stake_activation_epoch: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>unstaking_epoch: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>principal_amount: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>reward_amount: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="haneul_system_validator_ConvertingToFungibleStakedHaneulEvent"></a>

## Struct `ConvertingToFungibleStakedHaneulEvent`

Event emitted when a staked HANEUL is converted to a fungible staked HANEUL.


<pre><code><b>public</b> <b>struct</b> <a href="../haneul_system/validator.md#haneul_system_validator_ConvertingToFungibleStakedHaneulEvent">ConvertingToFungibleStakedHaneulEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>pool_id: <a href="../haneul/object.md#haneul_object_ID">haneul::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>stake_activation_epoch: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>staked_haneul_principal_amount: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>fungible_staked_haneul_amount: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="haneul_system_validator_RedeemingFungibleStakedHaneulEvent"></a>

## Struct `RedeemingFungibleStakedHaneulEvent`

Event emitted when a fungible staked HANEUL is redeemed.


<pre><code><b>public</b> <b>struct</b> <a href="../haneul_system/validator.md#haneul_system_validator_RedeemingFungibleStakedHaneulEvent">RedeemingFungibleStakedHaneulEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>pool_id: <a href="../haneul/object.md#haneul_object_ID">haneul::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>fungible_staked_haneul_amount: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>haneul_amount: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="haneul_system_validator_EInvalidProofOfPossession"></a>

Invalid proof_of_possession field in ValidatorMetadata


<pre><code><b>const</b> <a href="../haneul_system/validator.md#haneul_system_validator_EInvalidProofOfPossession">EInvalidProofOfPossession</a>: u64 = 0;
</code></pre>



<a name="haneul_system_validator_EMetadataInvalidPubkey"></a>

Invalid pubkey_bytes field in ValidatorMetadata


<pre><code><b>const</b> <a href="../haneul_system/validator.md#haneul_system_validator_EMetadataInvalidPubkey">EMetadataInvalidPubkey</a>: u64 = 1;
</code></pre>



<a name="haneul_system_validator_EMetadataInvalidNetPubkey"></a>

Invalid network_pubkey_bytes field in ValidatorMetadata


<pre><code><b>const</b> <a href="../haneul_system/validator.md#haneul_system_validator_EMetadataInvalidNetPubkey">EMetadataInvalidNetPubkey</a>: u64 = 2;
</code></pre>



<a name="haneul_system_validator_EMetadataInvalidWorkerPubkey"></a>

Invalid worker_pubkey_bytes field in ValidatorMetadata


<pre><code><b>const</b> <a href="../haneul_system/validator.md#haneul_system_validator_EMetadataInvalidWorkerPubkey">EMetadataInvalidWorkerPubkey</a>: u64 = 3;
</code></pre>



<a name="haneul_system_validator_EMetadataInvalidNetAddr"></a>

Invalid net_address field in ValidatorMetadata


<pre><code><b>const</b> <a href="../haneul_system/validator.md#haneul_system_validator_EMetadataInvalidNetAddr">EMetadataInvalidNetAddr</a>: u64 = 4;
</code></pre>



<a name="haneul_system_validator_EMetadataInvalidP2pAddr"></a>

Invalid p2p_address field in ValidatorMetadata


<pre><code><b>const</b> <a href="../haneul_system/validator.md#haneul_system_validator_EMetadataInvalidP2pAddr">EMetadataInvalidP2pAddr</a>: u64 = 5;
</code></pre>



<a name="haneul_system_validator_EMetadataInvalidPrimaryAddr"></a>

Invalid primary_address field in ValidatorMetadata


<pre><code><b>const</b> <a href="../haneul_system/validator.md#haneul_system_validator_EMetadataInvalidPrimaryAddr">EMetadataInvalidPrimaryAddr</a>: u64 = 6;
</code></pre>



<a name="haneul_system_validator_EMetadataInvalidWorkerAddr"></a>

Invalid worker_address field in ValidatorMetadata


<pre><code><b>const</b> <a href="../haneul_system/validator.md#haneul_system_validator_EMetadataInvalidWorkerAddr">EMetadataInvalidWorkerAddr</a>: u64 = 7;
</code></pre>



<a name="haneul_system_validator_ECommissionRateTooHigh"></a>

Commission rate set by the validator is higher than the threshold


<pre><code><b>const</b> <a href="../haneul_system/validator.md#haneul_system_validator_ECommissionRateTooHigh">ECommissionRateTooHigh</a>: u64 = 8;
</code></pre>



<a name="haneul_system_validator_EValidatorMetadataExceedingLengthLimit"></a>

Validator Metadata is too long


<pre><code><b>const</b> <a href="../haneul_system/validator.md#haneul_system_validator_EValidatorMetadataExceedingLengthLimit">EValidatorMetadataExceedingLengthLimit</a>: u64 = 9;
</code></pre>



<a name="haneul_system_validator_ENotValidatorCandidate"></a>

Intended validator is not a candidate one.


<pre><code><b>const</b> <a href="../haneul_system/validator.md#haneul_system_validator_ENotValidatorCandidate">ENotValidatorCandidate</a>: u64 = 10;
</code></pre>



<a name="haneul_system_validator_EInvalidStakeAmount"></a>

Stake amount is invalid or wrong.


<pre><code><b>const</b> <a href="../haneul_system/validator.md#haneul_system_validator_EInvalidStakeAmount">EInvalidStakeAmount</a>: u64 = 11;
</code></pre>



<a name="haneul_system_validator_ECalledDuringNonGenesis"></a>

Function called during non-genesis times.


<pre><code><b>const</b> <a href="../haneul_system/validator.md#haneul_system_validator_ECalledDuringNonGenesis">ECalledDuringNonGenesis</a>: u64 = 12;
</code></pre>



<a name="haneul_system_validator_ENewCapNotCreatedByValidatorItself"></a>

New Capability is not created by the validator itself


<pre><code><b>const</b> <a href="../haneul_system/validator.md#haneul_system_validator_ENewCapNotCreatedByValidatorItself">ENewCapNotCreatedByValidatorItself</a>: u64 = 100;
</code></pre>



<a name="haneul_system_validator_EInvalidCap"></a>

Capability code is not valid


<pre><code><b>const</b> <a href="../haneul_system/validator.md#haneul_system_validator_EInvalidCap">EInvalidCap</a>: u64 = 101;
</code></pre>



<a name="haneul_system_validator_EGasPriceHigherThanThreshold"></a>

Validator trying to set gas price higher than threshold.


<pre><code><b>const</b> <a href="../haneul_system/validator.md#haneul_system_validator_EGasPriceHigherThanThreshold">EGasPriceHigherThanThreshold</a>: u64 = 102;
</code></pre>



<a name="haneul_system_validator_EInvalidProtocolPubKeyLength"></a>

Invalid protocol public key length.


<pre><code><b>const</b> <a href="../haneul_system/validator.md#haneul_system_validator_EInvalidProtocolPubKeyLength">EInvalidProtocolPubKeyLength</a>: u64 = 16;
</code></pre>



<a name="haneul_system_validator_MAX_COMMISSION_RATE"></a>



<pre><code><b>const</b> <a href="../haneul_system/validator.md#haneul_system_validator_MAX_COMMISSION_RATE">MAX_COMMISSION_RATE</a>: u64 = 2000;
</code></pre>



<a name="haneul_system_validator_MAX_VALIDATOR_METADATA_LENGTH"></a>



<pre><code><b>const</b> <a href="../haneul_system/validator.md#haneul_system_validator_MAX_VALIDATOR_METADATA_LENGTH">MAX_VALIDATOR_METADATA_LENGTH</a>: u64 = 256;
</code></pre>



<a name="haneul_system_validator_MAX_VALIDATOR_GAS_PRICE"></a>

Max gas price a validator can set is 100K GEUNHWA.


<pre><code><b>const</b> <a href="../haneul_system/validator.md#haneul_system_validator_MAX_VALIDATOR_GAS_PRICE">MAX_VALIDATOR_GAS_PRICE</a>: u64 = 100000;
</code></pre>



<a name="haneul_system_validator_new_metadata"></a>

## Function `new_metadata`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../haneul_system/validator.md#haneul_system_validator_new_metadata">new_metadata</a>(<a href="../haneul_system/validator.md#haneul_system_validator_haneul_address">haneul_address</a>: <b>address</b>, <a href="../haneul_system/validator.md#haneul_system_validator_protocol_pubkey_bytes">protocol_pubkey_bytes</a>: vector&lt;u8&gt;, <a href="../haneul_system/validator.md#haneul_system_validator_network_pubkey_bytes">network_pubkey_bytes</a>: vector&lt;u8&gt;, <a href="../haneul_system/validator.md#haneul_system_validator_worker_pubkey_bytes">worker_pubkey_bytes</a>: vector&lt;u8&gt;, <a href="../haneul_system/validator.md#haneul_system_validator_proof_of_possession">proof_of_possession</a>: vector&lt;u8&gt;, <a href="../haneul_system/validator.md#haneul_system_validator_name">name</a>: <a href="../std/string.md#std_string_String">std::string::String</a>, <a href="../haneul_system/validator.md#haneul_system_validator_description">description</a>: <a href="../std/string.md#std_string_String">std::string::String</a>, <a href="../haneul_system/validator.md#haneul_system_validator_image_url">image_url</a>: <a href="../haneul/url.md#haneul_url_Url">haneul::url::Url</a>, <a href="../haneul_system/validator.md#haneul_system_validator_project_url">project_url</a>: <a href="../haneul/url.md#haneul_url_Url">haneul::url::Url</a>, net_address: <a href="../std/string.md#std_string_String">std::string::String</a>, <a href="../haneul_system/validator.md#haneul_system_validator_p2p_address">p2p_address</a>: <a href="../std/string.md#std_string_String">std::string::String</a>, <a href="../haneul_system/validator.md#haneul_system_validator_primary_address">primary_address</a>: <a href="../std/string.md#std_string_String">std::string::String</a>, <a href="../haneul_system/validator.md#haneul_system_validator_worker_address">worker_address</a>: <a href="../std/string.md#std_string_String">std::string::String</a>, extra_fields: <a href="../haneul/bag.md#haneul_bag_Bag">haneul::bag::Bag</a>): <a href="../haneul_system/validator.md#haneul_system_validator_ValidatorMetadata">haneul_system::validator::ValidatorMetadata</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../haneul_system/validator.md#haneul_system_validator_new_metadata">new_metadata</a>(
    <a href="../haneul_system/validator.md#haneul_system_validator_haneul_address">haneul_address</a>: <b>address</b>,
    <a href="../haneul_system/validator.md#haneul_system_validator_protocol_pubkey_bytes">protocol_pubkey_bytes</a>: vector&lt;u8&gt;,
    <a href="../haneul_system/validator.md#haneul_system_validator_network_pubkey_bytes">network_pubkey_bytes</a>: vector&lt;u8&gt;,
    <a href="../haneul_system/validator.md#haneul_system_validator_worker_pubkey_bytes">worker_pubkey_bytes</a>: vector&lt;u8&gt;,
    <a href="../haneul_system/validator.md#haneul_system_validator_proof_of_possession">proof_of_possession</a>: vector&lt;u8&gt;,
    <a href="../haneul_system/validator.md#haneul_system_validator_name">name</a>: String,
    <a href="../haneul_system/validator.md#haneul_system_validator_description">description</a>: String,
    <a href="../haneul_system/validator.md#haneul_system_validator_image_url">image_url</a>: Url,
    <a href="../haneul_system/validator.md#haneul_system_validator_project_url">project_url</a>: Url,
    net_address: String,
    <a href="../haneul_system/validator.md#haneul_system_validator_p2p_address">p2p_address</a>: String,
    <a href="../haneul_system/validator.md#haneul_system_validator_primary_address">primary_address</a>: String,
    <a href="../haneul_system/validator.md#haneul_system_validator_worker_address">worker_address</a>: String,
    extra_fields: Bag,
): <a href="../haneul_system/validator.md#haneul_system_validator_ValidatorMetadata">ValidatorMetadata</a> {
    <a href="../haneul_system/validator.md#haneul_system_validator_ValidatorMetadata">ValidatorMetadata</a> {
        <a href="../haneul_system/validator.md#haneul_system_validator_haneul_address">haneul_address</a>,
        <a href="../haneul_system/validator.md#haneul_system_validator_protocol_pubkey_bytes">protocol_pubkey_bytes</a>,
        <a href="../haneul_system/validator.md#haneul_system_validator_network_pubkey_bytes">network_pubkey_bytes</a>,
        <a href="../haneul_system/validator.md#haneul_system_validator_worker_pubkey_bytes">worker_pubkey_bytes</a>,
        <a href="../haneul_system/validator.md#haneul_system_validator_proof_of_possession">proof_of_possession</a>,
        <a href="../haneul_system/validator.md#haneul_system_validator_name">name</a>,
        <a href="../haneul_system/validator.md#haneul_system_validator_description">description</a>,
        <a href="../haneul_system/validator.md#haneul_system_validator_image_url">image_url</a>,
        <a href="../haneul_system/validator.md#haneul_system_validator_project_url">project_url</a>,
        net_address,
        <a href="../haneul_system/validator.md#haneul_system_validator_p2p_address">p2p_address</a>,
        <a href="../haneul_system/validator.md#haneul_system_validator_primary_address">primary_address</a>,
        <a href="../haneul_system/validator.md#haneul_system_validator_worker_address">worker_address</a>,
        <a href="../haneul_system/validator.md#haneul_system_validator_next_epoch_protocol_pubkey_bytes">next_epoch_protocol_pubkey_bytes</a>: option::none(),
        <a href="../haneul_system/validator.md#haneul_system_validator_next_epoch_network_pubkey_bytes">next_epoch_network_pubkey_bytes</a>: option::none(),
        <a href="../haneul_system/validator.md#haneul_system_validator_next_epoch_worker_pubkey_bytes">next_epoch_worker_pubkey_bytes</a>: option::none(),
        <a href="../haneul_system/validator.md#haneul_system_validator_next_epoch_proof_of_possession">next_epoch_proof_of_possession</a>: option::none(),
        next_epoch_net_address: option::none(),
        <a href="../haneul_system/validator.md#haneul_system_validator_next_epoch_p2p_address">next_epoch_p2p_address</a>: option::none(),
        <a href="../haneul_system/validator.md#haneul_system_validator_next_epoch_primary_address">next_epoch_primary_address</a>: option::none(),
        <a href="../haneul_system/validator.md#haneul_system_validator_next_epoch_worker_address">next_epoch_worker_address</a>: option::none(),
        extra_fields,
    }
}
</code></pre>



</details>

<a name="haneul_system_validator_new"></a>

## Function `new`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../haneul_system/validator.md#haneul_system_validator_new">new</a>(<a href="../haneul_system/validator.md#haneul_system_validator_haneul_address">haneul_address</a>: <b>address</b>, <a href="../haneul_system/validator.md#haneul_system_validator_protocol_pubkey_bytes">protocol_pubkey_bytes</a>: vector&lt;u8&gt;, <a href="../haneul_system/validator.md#haneul_system_validator_network_pubkey_bytes">network_pubkey_bytes</a>: vector&lt;u8&gt;, <a href="../haneul_system/validator.md#haneul_system_validator_worker_pubkey_bytes">worker_pubkey_bytes</a>: vector&lt;u8&gt;, <a href="../haneul_system/validator.md#haneul_system_validator_proof_of_possession">proof_of_possession</a>: vector&lt;u8&gt;, <a href="../haneul_system/validator.md#haneul_system_validator_name">name</a>: vector&lt;u8&gt;, <a href="../haneul_system/validator.md#haneul_system_validator_description">description</a>: vector&lt;u8&gt;, <a href="../haneul_system/validator.md#haneul_system_validator_image_url">image_url</a>: vector&lt;u8&gt;, <a href="../haneul_system/validator.md#haneul_system_validator_project_url">project_url</a>: vector&lt;u8&gt;, net_address: vector&lt;u8&gt;, <a href="../haneul_system/validator.md#haneul_system_validator_p2p_address">p2p_address</a>: vector&lt;u8&gt;, <a href="../haneul_system/validator.md#haneul_system_validator_primary_address">primary_address</a>: vector&lt;u8&gt;, <a href="../haneul_system/validator.md#haneul_system_validator_worker_address">worker_address</a>: vector&lt;u8&gt;, <a href="../haneul_system/validator.md#haneul_system_validator_gas_price">gas_price</a>: u64, <a href="../haneul_system/validator.md#haneul_system_validator_commission_rate">commission_rate</a>: u64, ctx: &<b>mut</b> <a href="../haneul/tx_context.md#haneul_tx_context_TxContext">haneul::tx_context::TxContext</a>): <a href="../haneul_system/validator.md#haneul_system_validator_Validator">haneul_system::validator::Validator</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../haneul_system/validator.md#haneul_system_validator_new">new</a>(
    <a href="../haneul_system/validator.md#haneul_system_validator_haneul_address">haneul_address</a>: <b>address</b>,
    <a href="../haneul_system/validator.md#haneul_system_validator_protocol_pubkey_bytes">protocol_pubkey_bytes</a>: vector&lt;u8&gt;,
    <a href="../haneul_system/validator.md#haneul_system_validator_network_pubkey_bytes">network_pubkey_bytes</a>: vector&lt;u8&gt;,
    <a href="../haneul_system/validator.md#haneul_system_validator_worker_pubkey_bytes">worker_pubkey_bytes</a>: vector&lt;u8&gt;,
    <a href="../haneul_system/validator.md#haneul_system_validator_proof_of_possession">proof_of_possession</a>: vector&lt;u8&gt;,
    <a href="../haneul_system/validator.md#haneul_system_validator_name">name</a>: vector&lt;u8&gt;,
    <a href="../haneul_system/validator.md#haneul_system_validator_description">description</a>: vector&lt;u8&gt;,
    <a href="../haneul_system/validator.md#haneul_system_validator_image_url">image_url</a>: vector&lt;u8&gt;,
    <a href="../haneul_system/validator.md#haneul_system_validator_project_url">project_url</a>: vector&lt;u8&gt;,
    net_address: vector&lt;u8&gt;,
    <a href="../haneul_system/validator.md#haneul_system_validator_p2p_address">p2p_address</a>: vector&lt;u8&gt;,
    <a href="../haneul_system/validator.md#haneul_system_validator_primary_address">primary_address</a>: vector&lt;u8&gt;,
    <a href="../haneul_system/validator.md#haneul_system_validator_worker_address">worker_address</a>: vector&lt;u8&gt;,
    <a href="../haneul_system/validator.md#haneul_system_validator_gas_price">gas_price</a>: u64,
    <a href="../haneul_system/validator.md#haneul_system_validator_commission_rate">commission_rate</a>: u64,
    ctx: &<b>mut</b> TxContext,
): <a href="../haneul_system/validator.md#haneul_system_validator_Validator">Validator</a> {
    <b>assert</b>!(
        net_address.length() &lt;= <a href="../haneul_system/validator.md#haneul_system_validator_MAX_VALIDATOR_METADATA_LENGTH">MAX_VALIDATOR_METADATA_LENGTH</a>
            && <a href="../haneul_system/validator.md#haneul_system_validator_p2p_address">p2p_address</a>.length() &lt;= <a href="../haneul_system/validator.md#haneul_system_validator_MAX_VALIDATOR_METADATA_LENGTH">MAX_VALIDATOR_METADATA_LENGTH</a>
            && <a href="../haneul_system/validator.md#haneul_system_validator_primary_address">primary_address</a>.length() &lt;= <a href="../haneul_system/validator.md#haneul_system_validator_MAX_VALIDATOR_METADATA_LENGTH">MAX_VALIDATOR_METADATA_LENGTH</a>
            && <a href="../haneul_system/validator.md#haneul_system_validator_worker_address">worker_address</a>.length() &lt;= <a href="../haneul_system/validator.md#haneul_system_validator_MAX_VALIDATOR_METADATA_LENGTH">MAX_VALIDATOR_METADATA_LENGTH</a>
            && <a href="../haneul_system/validator.md#haneul_system_validator_name">name</a>.length() &lt;= <a href="../haneul_system/validator.md#haneul_system_validator_MAX_VALIDATOR_METADATA_LENGTH">MAX_VALIDATOR_METADATA_LENGTH</a>
            && <a href="../haneul_system/validator.md#haneul_system_validator_description">description</a>.length() &lt;= <a href="../haneul_system/validator.md#haneul_system_validator_MAX_VALIDATOR_METADATA_LENGTH">MAX_VALIDATOR_METADATA_LENGTH</a>
            && <a href="../haneul_system/validator.md#haneul_system_validator_image_url">image_url</a>.length() &lt;= <a href="../haneul_system/validator.md#haneul_system_validator_MAX_VALIDATOR_METADATA_LENGTH">MAX_VALIDATOR_METADATA_LENGTH</a>
            && <a href="../haneul_system/validator.md#haneul_system_validator_project_url">project_url</a>.length() &lt;= <a href="../haneul_system/validator.md#haneul_system_validator_MAX_VALIDATOR_METADATA_LENGTH">MAX_VALIDATOR_METADATA_LENGTH</a>,
        <a href="../haneul_system/validator.md#haneul_system_validator_EValidatorMetadataExceedingLengthLimit">EValidatorMetadataExceedingLengthLimit</a>,
    );
    <b>assert</b>!(<a href="../haneul_system/validator.md#haneul_system_validator_commission_rate">commission_rate</a> &lt;= <a href="../haneul_system/validator.md#haneul_system_validator_MAX_COMMISSION_RATE">MAX_COMMISSION_RATE</a>, <a href="../haneul_system/validator.md#haneul_system_validator_ECommissionRateTooHigh">ECommissionRateTooHigh</a>);
    <b>assert</b>!(<a href="../haneul_system/validator.md#haneul_system_validator_gas_price">gas_price</a> &lt; <a href="../haneul_system/validator.md#haneul_system_validator_MAX_VALIDATOR_GAS_PRICE">MAX_VALIDATOR_GAS_PRICE</a>, <a href="../haneul_system/validator.md#haneul_system_validator_EGasPriceHigherThanThreshold">EGasPriceHigherThanThreshold</a>);
    <b>assert</b>!(<a href="../haneul_system/validator.md#haneul_system_validator_protocol_pubkey_bytes">protocol_pubkey_bytes</a>.length() == 96, <a href="../haneul_system/validator.md#haneul_system_validator_EInvalidProtocolPubKeyLength">EInvalidProtocolPubKeyLength</a>);
    <b>let</b> <a href="../haneul_system/validator.md#haneul_system_validator_metadata">metadata</a> = <a href="../haneul_system/validator.md#haneul_system_validator_new_metadata">new_metadata</a>(
        <a href="../haneul_system/validator.md#haneul_system_validator_haneul_address">haneul_address</a>,
        <a href="../haneul_system/validator.md#haneul_system_validator_protocol_pubkey_bytes">protocol_pubkey_bytes</a>,
        <a href="../haneul_system/validator.md#haneul_system_validator_network_pubkey_bytes">network_pubkey_bytes</a>,
        <a href="../haneul_system/validator.md#haneul_system_validator_worker_pubkey_bytes">worker_pubkey_bytes</a>,
        <a href="../haneul_system/validator.md#haneul_system_validator_proof_of_possession">proof_of_possession</a>,
        <a href="../haneul_system/validator.md#haneul_system_validator_name">name</a>.to_ascii_string().to_string(),
        <a href="../haneul_system/validator.md#haneul_system_validator_description">description</a>.to_ascii_string().to_string(),
        url::new_unsafe_from_bytes(<a href="../haneul_system/validator.md#haneul_system_validator_image_url">image_url</a>),
        url::new_unsafe_from_bytes(<a href="../haneul_system/validator.md#haneul_system_validator_project_url">project_url</a>),
        net_address.to_ascii_string().to_string(),
        <a href="../haneul_system/validator.md#haneul_system_validator_p2p_address">p2p_address</a>.to_ascii_string().to_string(),
        <a href="../haneul_system/validator.md#haneul_system_validator_primary_address">primary_address</a>.to_ascii_string().to_string(),
        <a href="../haneul_system/validator.md#haneul_system_validator_worker_address">worker_address</a>.to_ascii_string().to_string(),
        bag::new(ctx),
    );
    // Checks that the keys & addresses & PoP are valid.
    <a href="../haneul_system/validator.md#haneul_system_validator_metadata">metadata</a>.validate();
    <a href="../haneul_system/validator.md#haneul_system_validator_metadata">metadata</a>.<a href="../haneul_system/validator.md#haneul_system_validator_new_from_metadata">new_from_metadata</a>(<a href="../haneul_system/validator.md#haneul_system_validator_gas_price">gas_price</a>, <a href="../haneul_system/validator.md#haneul_system_validator_commission_rate">commission_rate</a>, ctx)
}
</code></pre>



</details>

<a name="haneul_system_validator_deactivate"></a>

## Function `deactivate`

Mark Validator's <code>StakingPool</code> as inactive by setting the <code>deactivation_epoch</code>.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../haneul_system/validator.md#haneul_system_validator_deactivate">deactivate</a>(self: &<b>mut</b> <a href="../haneul_system/validator.md#haneul_system_validator_Validator">haneul_system::validator::Validator</a>, deactivation_epoch: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../haneul_system/validator.md#haneul_system_validator_deactivate">deactivate</a>(self: &<b>mut</b> <a href="../haneul_system/validator.md#haneul_system_validator_Validator">Validator</a>, deactivation_epoch: u64) {
    self.<a href="../haneul_system/staking_pool.md#haneul_system_staking_pool">staking_pool</a>.deactivate_staking_pool(deactivation_epoch)
}
</code></pre>



</details>

<a name="haneul_system_validator_activate"></a>

## Function `activate`

Activate Validator's <code>StakingPool</code> by setting the <code>activation_epoch</code>.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../haneul_system/validator.md#haneul_system_validator_activate">activate</a>(self: &<b>mut</b> <a href="../haneul_system/validator.md#haneul_system_validator_Validator">haneul_system::validator::Validator</a>, activation_epoch: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../haneul_system/validator.md#haneul_system_validator_activate">activate</a>(self: &<b>mut</b> <a href="../haneul_system/validator.md#haneul_system_validator_Validator">Validator</a>, activation_epoch: u64) {
    self.<a href="../haneul_system/staking_pool.md#haneul_system_staking_pool">staking_pool</a>.activate_staking_pool(activation_epoch);
}
</code></pre>



</details>

<a name="haneul_system_validator_adjust_stake_and_gas_price"></a>

## Function `adjust_stake_and_gas_price`

Process pending stake and pending withdraws, and update the gas price.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../haneul_system/validator.md#haneul_system_validator_adjust_stake_and_gas_price">adjust_stake_and_gas_price</a>(self: &<b>mut</b> <a href="../haneul_system/validator.md#haneul_system_validator_Validator">haneul_system::validator::Validator</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../haneul_system/validator.md#haneul_system_validator_adjust_stake_and_gas_price">adjust_stake_and_gas_price</a>(self: &<b>mut</b> <a href="../haneul_system/validator.md#haneul_system_validator_Validator">Validator</a>) {
    self.<a href="../haneul_system/validator.md#haneul_system_validator_gas_price">gas_price</a> = self.<a href="../haneul_system/validator.md#haneul_system_validator_next_epoch_gas_price">next_epoch_gas_price</a>;
    self.<a href="../haneul_system/validator.md#haneul_system_validator_commission_rate">commission_rate</a> = self.next_epoch_commission_rate;
}
</code></pre>



</details>

<a name="haneul_system_validator_request_add_stake"></a>

## Function `request_add_stake`

Request to add stake to the validator's staking pool, processed at the end of the epoch.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../haneul_system/validator.md#haneul_system_validator_request_add_stake">request_add_stake</a>(self: &<b>mut</b> <a href="../haneul_system/validator.md#haneul_system_validator_Validator">haneul_system::validator::Validator</a>, stake: <a href="../haneul/balance.md#haneul_balance_Balance">haneul::balance::Balance</a>&lt;<a href="../haneul/haneul.md#haneul_haneul_HANEUL">haneul::haneul::HANEUL</a>&gt;, staker_address: <b>address</b>, ctx: &<b>mut</b> <a href="../haneul/tx_context.md#haneul_tx_context_TxContext">haneul::tx_context::TxContext</a>): <a href="../haneul_system/staking_pool.md#haneul_system_staking_pool_StakedHaneul">haneul_system::staking_pool::StakedHaneul</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../haneul_system/validator.md#haneul_system_validator_request_add_stake">request_add_stake</a>(
    self: &<b>mut</b> <a href="../haneul_system/validator.md#haneul_system_validator_Validator">Validator</a>,
    stake: Balance&lt;HANEUL&gt;,
    staker_address: <b>address</b>,
    ctx: &<b>mut</b> TxContext,
): StakedHaneul {
    <b>let</b> <a href="../haneul_system/validator.md#haneul_system_validator_stake_amount">stake_amount</a> = stake.value();
    <b>assert</b>!(<a href="../haneul_system/validator.md#haneul_system_validator_stake_amount">stake_amount</a> &gt; 0, <a href="../haneul_system/validator.md#haneul_system_validator_EInvalidStakeAmount">EInvalidStakeAmount</a>);
    <b>let</b> stake_epoch = ctx.epoch() + 1;
    <b>let</b> staked_haneul = self.<a href="../haneul_system/staking_pool.md#haneul_system_staking_pool">staking_pool</a>.<a href="../haneul_system/validator.md#haneul_system_validator_request_add_stake">request_add_stake</a>(stake, stake_epoch, ctx);
    // Process stake right away <b>if</b> staking pool is preactive.
    <b>if</b> (self.<a href="../haneul_system/staking_pool.md#haneul_system_staking_pool">staking_pool</a>.<a href="../haneul_system/validator.md#haneul_system_validator_is_preactive">is_preactive</a>()) {
        self.<a href="../haneul_system/staking_pool.md#haneul_system_staking_pool">staking_pool</a>.process_pending_stake();
    };
    self.next_epoch_stake = self.next_epoch_stake + <a href="../haneul_system/validator.md#haneul_system_validator_stake_amount">stake_amount</a>;
    event::emit(<a href="../haneul_system/validator.md#haneul_system_validator_StakingRequestEvent">StakingRequestEvent</a> {
        pool_id: self.<a href="../haneul_system/validator.md#haneul_system_validator_staking_pool_id">staking_pool_id</a>(),
        validator_address: self.<a href="../haneul_system/validator.md#haneul_system_validator_metadata">metadata</a>.<a href="../haneul_system/validator.md#haneul_system_validator_haneul_address">haneul_address</a>,
        staker_address,
        epoch: ctx.epoch(),
        amount: <a href="../haneul_system/validator.md#haneul_system_validator_stake_amount">stake_amount</a>,
    });
    staked_haneul
}
</code></pre>



</details>

<a name="haneul_system_validator_convert_to_fungible_staked_haneul"></a>

## Function `convert_to_fungible_staked_haneul`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../haneul_system/validator.md#haneul_system_validator_convert_to_fungible_staked_haneul">convert_to_fungible_staked_haneul</a>(self: &<b>mut</b> <a href="../haneul_system/validator.md#haneul_system_validator_Validator">haneul_system::validator::Validator</a>, staked_haneul: <a href="../haneul_system/staking_pool.md#haneul_system_staking_pool_StakedHaneul">haneul_system::staking_pool::StakedHaneul</a>, ctx: &<b>mut</b> <a href="../haneul/tx_context.md#haneul_tx_context_TxContext">haneul::tx_context::TxContext</a>): <a href="../haneul_system/staking_pool.md#haneul_system_staking_pool_FungibleStakedHaneul">haneul_system::staking_pool::FungibleStakedHaneul</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../haneul_system/validator.md#haneul_system_validator_convert_to_fungible_staked_haneul">convert_to_fungible_staked_haneul</a>(
    self: &<b>mut</b> <a href="../haneul_system/validator.md#haneul_system_validator_Validator">Validator</a>,
    staked_haneul: StakedHaneul,
    ctx: &<b>mut</b> TxContext,
): FungibleStakedHaneul {
    <b>let</b> stake_activation_epoch = staked_haneul.activation_epoch();
    <b>let</b> staked_haneul_principal_amount = staked_haneul.amount();
    <b>let</b> fungible_staked_haneul = self.<a href="../haneul_system/staking_pool.md#haneul_system_staking_pool">staking_pool</a>.<a href="../haneul_system/validator.md#haneul_system_validator_convert_to_fungible_staked_haneul">convert_to_fungible_staked_haneul</a>(staked_haneul, ctx);
    event::emit(<a href="../haneul_system/validator.md#haneul_system_validator_ConvertingToFungibleStakedHaneulEvent">ConvertingToFungibleStakedHaneulEvent</a> {
        pool_id: self.<a href="../haneul_system/validator.md#haneul_system_validator_staking_pool_id">staking_pool_id</a>(),
        stake_activation_epoch,
        staked_haneul_principal_amount,
        fungible_staked_haneul_amount: fungible_staked_haneul.value(),
    });
    fungible_staked_haneul
}
</code></pre>



</details>

<a name="haneul_system_validator_redeem_fungible_staked_haneul"></a>

## Function `redeem_fungible_staked_haneul`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../haneul_system/validator.md#haneul_system_validator_redeem_fungible_staked_haneul">redeem_fungible_staked_haneul</a>(self: &<b>mut</b> <a href="../haneul_system/validator.md#haneul_system_validator_Validator">haneul_system::validator::Validator</a>, fungible_staked_haneul: <a href="../haneul_system/staking_pool.md#haneul_system_staking_pool_FungibleStakedHaneul">haneul_system::staking_pool::FungibleStakedHaneul</a>, ctx: &<a href="../haneul/tx_context.md#haneul_tx_context_TxContext">haneul::tx_context::TxContext</a>): <a href="../haneul/balance.md#haneul_balance_Balance">haneul::balance::Balance</a>&lt;<a href="../haneul/haneul.md#haneul_haneul_HANEUL">haneul::haneul::HANEUL</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../haneul_system/validator.md#haneul_system_validator_redeem_fungible_staked_haneul">redeem_fungible_staked_haneul</a>(
    self: &<b>mut</b> <a href="../haneul_system/validator.md#haneul_system_validator_Validator">Validator</a>,
    fungible_staked_haneul: FungibleStakedHaneul,
    ctx: &TxContext,
): Balance&lt;HANEUL&gt; {
    <b>let</b> fungible_staked_haneul_amount = fungible_staked_haneul.value();
    <b>let</b> haneul = self.<a href="../haneul_system/staking_pool.md#haneul_system_staking_pool">staking_pool</a>.<a href="../haneul_system/validator.md#haneul_system_validator_redeem_fungible_staked_haneul">redeem_fungible_staked_haneul</a>(fungible_staked_haneul, ctx);
    self.next_epoch_stake = self.next_epoch_stake - haneul.value();
    event::emit(<a href="../haneul_system/validator.md#haneul_system_validator_RedeemingFungibleStakedHaneulEvent">RedeemingFungibleStakedHaneulEvent</a> {
        pool_id: self.<a href="../haneul_system/validator.md#haneul_system_validator_staking_pool_id">staking_pool_id</a>(),
        fungible_staked_haneul_amount,
        haneul_amount: haneul.value(),
    });
    haneul
}
</code></pre>



</details>

<a name="haneul_system_validator_request_add_stake_at_genesis"></a>

## Function `request_add_stake_at_genesis`

Request to add stake to the validator's staking pool at genesis


<pre><code><b>public</b>(package) <b>fun</b> <a href="../haneul_system/validator.md#haneul_system_validator_request_add_stake_at_genesis">request_add_stake_at_genesis</a>(self: &<b>mut</b> <a href="../haneul_system/validator.md#haneul_system_validator_Validator">haneul_system::validator::Validator</a>, stake: <a href="../haneul/balance.md#haneul_balance_Balance">haneul::balance::Balance</a>&lt;<a href="../haneul/haneul.md#haneul_haneul_HANEUL">haneul::haneul::HANEUL</a>&gt;, staker_address: <b>address</b>, ctx: &<b>mut</b> <a href="../haneul/tx_context.md#haneul_tx_context_TxContext">haneul::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../haneul_system/validator.md#haneul_system_validator_request_add_stake_at_genesis">request_add_stake_at_genesis</a>(
    self: &<b>mut</b> <a href="../haneul_system/validator.md#haneul_system_validator_Validator">Validator</a>,
    stake: Balance&lt;HANEUL&gt;,
    staker_address: <b>address</b>,
    ctx: &<b>mut</b> TxContext,
) {
    <b>assert</b>!(ctx.epoch() == 0, <a href="../haneul_system/validator.md#haneul_system_validator_ECalledDuringNonGenesis">ECalledDuringNonGenesis</a>);
    <b>let</b> <a href="../haneul_system/validator.md#haneul_system_validator_stake_amount">stake_amount</a> = stake.value();
    <b>assert</b>!(<a href="../haneul_system/validator.md#haneul_system_validator_stake_amount">stake_amount</a> &gt; 0, <a href="../haneul_system/validator.md#haneul_system_validator_EInvalidStakeAmount">EInvalidStakeAmount</a>);
    // 0 = <a href="../haneul_system/genesis.md#haneul_system_genesis">genesis</a> epoch
    <b>let</b> staked_haneul = self.<a href="../haneul_system/staking_pool.md#haneul_system_staking_pool">staking_pool</a>.<a href="../haneul_system/validator.md#haneul_system_validator_request_add_stake">request_add_stake</a>(stake, 0, ctx);
    transfer::public_transfer(staked_haneul, staker_address);
    // Process stake right away
    self.<a href="../haneul_system/staking_pool.md#haneul_system_staking_pool">staking_pool</a>.process_pending_stake();
    self.next_epoch_stake = self.next_epoch_stake + <a href="../haneul_system/validator.md#haneul_system_validator_stake_amount">stake_amount</a>;
}
</code></pre>



</details>

<a name="haneul_system_validator_request_withdraw_stake"></a>

## Function `request_withdraw_stake`

Request to withdraw stake from the validator's staking pool, processed at the end of the epoch.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../haneul_system/validator.md#haneul_system_validator_request_withdraw_stake">request_withdraw_stake</a>(self: &<b>mut</b> <a href="../haneul_system/validator.md#haneul_system_validator_Validator">haneul_system::validator::Validator</a>, staked_haneul: <a href="../haneul_system/staking_pool.md#haneul_system_staking_pool_StakedHaneul">haneul_system::staking_pool::StakedHaneul</a>, ctx: &<a href="../haneul/tx_context.md#haneul_tx_context_TxContext">haneul::tx_context::TxContext</a>): <a href="../haneul/balance.md#haneul_balance_Balance">haneul::balance::Balance</a>&lt;<a href="../haneul/haneul.md#haneul_haneul_HANEUL">haneul::haneul::HANEUL</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../haneul_system/validator.md#haneul_system_validator_request_withdraw_stake">request_withdraw_stake</a>(
    self: &<b>mut</b> <a href="../haneul_system/validator.md#haneul_system_validator_Validator">Validator</a>,
    staked_haneul: StakedHaneul,
    ctx: &TxContext,
): Balance&lt;HANEUL&gt; {
    <b>let</b> principal_amount = staked_haneul.amount();
    <b>let</b> stake_activation_epoch = staked_haneul.activation_epoch();
    <b>let</b> withdrawn_stake = self.<a href="../haneul_system/staking_pool.md#haneul_system_staking_pool">staking_pool</a>.<a href="../haneul_system/validator.md#haneul_system_validator_request_withdraw_stake">request_withdraw_stake</a>(staked_haneul, ctx);
    <b>let</b> withdraw_amount = withdrawn_stake.value();
    <b>let</b> reward_amount = withdraw_amount - principal_amount;
    self.next_epoch_stake = self.next_epoch_stake - withdraw_amount;
    event::emit(<a href="../haneul_system/validator.md#haneul_system_validator_UnstakingRequestEvent">UnstakingRequestEvent</a> {
        pool_id: self.<a href="../haneul_system/validator.md#haneul_system_validator_staking_pool_id">staking_pool_id</a>(),
        validator_address: self.<a href="../haneul_system/validator.md#haneul_system_validator_metadata">metadata</a>.<a href="../haneul_system/validator.md#haneul_system_validator_haneul_address">haneul_address</a>,
        staker_address: ctx.sender(),
        stake_activation_epoch,
        unstaking_epoch: ctx.epoch(),
        principal_amount,
        reward_amount,
    });
    withdrawn_stake
}
</code></pre>



</details>

<a name="haneul_system_validator_request_set_gas_price"></a>

## Function `request_set_gas_price`

Request to set new gas price for the next epoch.
Need to present a <code>ValidatorOperationCap</code>.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../haneul_system/validator.md#haneul_system_validator_request_set_gas_price">request_set_gas_price</a>(self: &<b>mut</b> <a href="../haneul_system/validator.md#haneul_system_validator_Validator">haneul_system::validator::Validator</a>, verified_cap: <a href="../haneul_system/validator_cap.md#haneul_system_validator_cap_ValidatorOperationCap">haneul_system::validator_cap::ValidatorOperationCap</a>, new_price: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../haneul_system/validator.md#haneul_system_validator_request_set_gas_price">request_set_gas_price</a>(
    self: &<b>mut</b> <a href="../haneul_system/validator.md#haneul_system_validator_Validator">Validator</a>,
    verified_cap: ValidatorOperationCap,
    new_price: u64,
) {
    <b>assert</b>!(new_price &lt; <a href="../haneul_system/validator.md#haneul_system_validator_MAX_VALIDATOR_GAS_PRICE">MAX_VALIDATOR_GAS_PRICE</a>, <a href="../haneul_system/validator.md#haneul_system_validator_EGasPriceHigherThanThreshold">EGasPriceHigherThanThreshold</a>);
    <b>let</b> validator_address = verified_cap.verified_operation_cap_address();
    <b>assert</b>!(validator_address == self.<a href="../haneul_system/validator.md#haneul_system_validator_metadata">metadata</a>.<a href="../haneul_system/validator.md#haneul_system_validator_haneul_address">haneul_address</a>, <a href="../haneul_system/validator.md#haneul_system_validator_EInvalidCap">EInvalidCap</a>);
    self.<a href="../haneul_system/validator.md#haneul_system_validator_next_epoch_gas_price">next_epoch_gas_price</a> = new_price;
}
</code></pre>



</details>

<a name="haneul_system_validator_set_candidate_gas_price"></a>

## Function `set_candidate_gas_price`

Set new gas price for the candidate validator.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../haneul_system/validator.md#haneul_system_validator_set_candidate_gas_price">set_candidate_gas_price</a>(self: &<b>mut</b> <a href="../haneul_system/validator.md#haneul_system_validator_Validator">haneul_system::validator::Validator</a>, verified_cap: <a href="../haneul_system/validator_cap.md#haneul_system_validator_cap_ValidatorOperationCap">haneul_system::validator_cap::ValidatorOperationCap</a>, new_price: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../haneul_system/validator.md#haneul_system_validator_set_candidate_gas_price">set_candidate_gas_price</a>(
    self: &<b>mut</b> <a href="../haneul_system/validator.md#haneul_system_validator_Validator">Validator</a>,
    verified_cap: ValidatorOperationCap,
    new_price: u64,
) {
    <b>assert</b>!(self.<a href="../haneul_system/validator.md#haneul_system_validator_is_preactive">is_preactive</a>(), <a href="../haneul_system/validator.md#haneul_system_validator_ENotValidatorCandidate">ENotValidatorCandidate</a>);
    <b>assert</b>!(new_price &lt; <a href="../haneul_system/validator.md#haneul_system_validator_MAX_VALIDATOR_GAS_PRICE">MAX_VALIDATOR_GAS_PRICE</a>, <a href="../haneul_system/validator.md#haneul_system_validator_EGasPriceHigherThanThreshold">EGasPriceHigherThanThreshold</a>);
    <b>let</b> validator_address = verified_cap.verified_operation_cap_address();
    <b>assert</b>!(validator_address == self.<a href="../haneul_system/validator.md#haneul_system_validator_metadata">metadata</a>.<a href="../haneul_system/validator.md#haneul_system_validator_haneul_address">haneul_address</a>, <a href="../haneul_system/validator.md#haneul_system_validator_EInvalidCap">EInvalidCap</a>);
    self.<a href="../haneul_system/validator.md#haneul_system_validator_next_epoch_gas_price">next_epoch_gas_price</a> = new_price;
    self.<a href="../haneul_system/validator.md#haneul_system_validator_gas_price">gas_price</a> = new_price;
}
</code></pre>



</details>

<a name="haneul_system_validator_request_set_commission_rate"></a>

## Function `request_set_commission_rate`

Request to set new commission rate for the next epoch.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../haneul_system/validator.md#haneul_system_validator_request_set_commission_rate">request_set_commission_rate</a>(self: &<b>mut</b> <a href="../haneul_system/validator.md#haneul_system_validator_Validator">haneul_system::validator::Validator</a>, new_commission_rate: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../haneul_system/validator.md#haneul_system_validator_request_set_commission_rate">request_set_commission_rate</a>(self: &<b>mut</b> <a href="../haneul_system/validator.md#haneul_system_validator_Validator">Validator</a>, new_commission_rate: u64) {
    <b>assert</b>!(new_commission_rate &lt;= <a href="../haneul_system/validator.md#haneul_system_validator_MAX_COMMISSION_RATE">MAX_COMMISSION_RATE</a>, <a href="../haneul_system/validator.md#haneul_system_validator_ECommissionRateTooHigh">ECommissionRateTooHigh</a>);
    self.next_epoch_commission_rate = new_commission_rate;
}
</code></pre>



</details>

<a name="haneul_system_validator_set_candidate_commission_rate"></a>

## Function `set_candidate_commission_rate`

Set new commission rate for the candidate validator.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../haneul_system/validator.md#haneul_system_validator_set_candidate_commission_rate">set_candidate_commission_rate</a>(self: &<b>mut</b> <a href="../haneul_system/validator.md#haneul_system_validator_Validator">haneul_system::validator::Validator</a>, new_commission_rate: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../haneul_system/validator.md#haneul_system_validator_set_candidate_commission_rate">set_candidate_commission_rate</a>(self: &<b>mut</b> <a href="../haneul_system/validator.md#haneul_system_validator_Validator">Validator</a>, new_commission_rate: u64) {
    <b>assert</b>!(self.<a href="../haneul_system/validator.md#haneul_system_validator_is_preactive">is_preactive</a>(), <a href="../haneul_system/validator.md#haneul_system_validator_ENotValidatorCandidate">ENotValidatorCandidate</a>);
    <b>assert</b>!(new_commission_rate &lt;= <a href="../haneul_system/validator.md#haneul_system_validator_MAX_COMMISSION_RATE">MAX_COMMISSION_RATE</a>, <a href="../haneul_system/validator.md#haneul_system_validator_ECommissionRateTooHigh">ECommissionRateTooHigh</a>);
    self.<a href="../haneul_system/validator.md#haneul_system_validator_commission_rate">commission_rate</a> = new_commission_rate;
    self.next_epoch_commission_rate = new_commission_rate;
}
</code></pre>



</details>

<a name="haneul_system_validator_deposit_stake_rewards"></a>

## Function `deposit_stake_rewards`

Deposit stakes rewards into the validator's staking pool, called at the end of the epoch.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../haneul_system/validator.md#haneul_system_validator_deposit_stake_rewards">deposit_stake_rewards</a>(self: &<b>mut</b> <a href="../haneul_system/validator.md#haneul_system_validator_Validator">haneul_system::validator::Validator</a>, reward: <a href="../haneul/balance.md#haneul_balance_Balance">haneul::balance::Balance</a>&lt;<a href="../haneul/haneul.md#haneul_haneul_HANEUL">haneul::haneul::HANEUL</a>&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../haneul_system/validator.md#haneul_system_validator_deposit_stake_rewards">deposit_stake_rewards</a>(self: &<b>mut</b> <a href="../haneul_system/validator.md#haneul_system_validator_Validator">Validator</a>, reward: Balance&lt;HANEUL&gt;) {
    self.next_epoch_stake = self.next_epoch_stake + reward.value();
    self.<a href="../haneul_system/staking_pool.md#haneul_system_staking_pool">staking_pool</a>.deposit_rewards(reward);
}
</code></pre>



</details>

<a name="haneul_system_validator_process_pending_stakes_and_withdraws"></a>

## Function `process_pending_stakes_and_withdraws`

Process pending stakes and withdraws, called at the end of the epoch.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../haneul_system/validator.md#haneul_system_validator_process_pending_stakes_and_withdraws">process_pending_stakes_and_withdraws</a>(self: &<b>mut</b> <a href="../haneul_system/validator.md#haneul_system_validator_Validator">haneul_system::validator::Validator</a>, ctx: &<a href="../haneul/tx_context.md#haneul_tx_context_TxContext">haneul::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../haneul_system/validator.md#haneul_system_validator_process_pending_stakes_and_withdraws">process_pending_stakes_and_withdraws</a>(self: &<b>mut</b> <a href="../haneul_system/validator.md#haneul_system_validator_Validator">Validator</a>, ctx: &TxContext) {
    self.<a href="../haneul_system/staking_pool.md#haneul_system_staking_pool">staking_pool</a>.<a href="../haneul_system/validator.md#haneul_system_validator_process_pending_stakes_and_withdraws">process_pending_stakes_and_withdraws</a>(ctx);
    // TODO: bring this assertion back when we are ready.
    // <b>assert</b>!(<a href="../haneul_system/validator.md#haneul_system_validator_stake_amount">stake_amount</a>(self) == self.next_epoch_stake, <a href="../haneul_system/validator.md#haneul_system_validator_EInvalidStakeAmount">EInvalidStakeAmount</a>);
}
</code></pre>



</details>

<a name="haneul_system_validator_is_preactive"></a>

## Function `is_preactive`

Returns true if the validator is preactive.


<pre><code><b>public</b> <b>fun</b> <a href="../haneul_system/validator.md#haneul_system_validator_is_preactive">is_preactive</a>(self: &<a href="../haneul_system/validator.md#haneul_system_validator_Validator">haneul_system::validator::Validator</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul_system/validator.md#haneul_system_validator_is_preactive">is_preactive</a>(self: &<a href="../haneul_system/validator.md#haneul_system_validator_Validator">Validator</a>): bool {
    self.<a href="../haneul_system/staking_pool.md#haneul_system_staking_pool">staking_pool</a>.<a href="../haneul_system/validator.md#haneul_system_validator_is_preactive">is_preactive</a>()
}
</code></pre>



</details>

<a name="haneul_system_validator_metadata"></a>

## Function `metadata`



<pre><code><b>public</b> <b>fun</b> <a href="../haneul_system/validator.md#haneul_system_validator_metadata">metadata</a>(self: &<a href="../haneul_system/validator.md#haneul_system_validator_Validator">haneul_system::validator::Validator</a>): &<a href="../haneul_system/validator.md#haneul_system_validator_ValidatorMetadata">haneul_system::validator::ValidatorMetadata</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul_system/validator.md#haneul_system_validator_metadata">metadata</a>(self: &<a href="../haneul_system/validator.md#haneul_system_validator_Validator">Validator</a>): &<a href="../haneul_system/validator.md#haneul_system_validator_ValidatorMetadata">ValidatorMetadata</a> {
    &self.<a href="../haneul_system/validator.md#haneul_system_validator_metadata">metadata</a>
}
</code></pre>



</details>

<a name="haneul_system_validator_haneul_address"></a>

## Function `haneul_address`



<pre><code><b>public</b> <b>fun</b> <a href="../haneul_system/validator.md#haneul_system_validator_haneul_address">haneul_address</a>(self: &<a href="../haneul_system/validator.md#haneul_system_validator_Validator">haneul_system::validator::Validator</a>): <b>address</b>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul_system/validator.md#haneul_system_validator_haneul_address">haneul_address</a>(self: &<a href="../haneul_system/validator.md#haneul_system_validator_Validator">Validator</a>): <b>address</b> {
    self.<a href="../haneul_system/validator.md#haneul_system_validator_metadata">metadata</a>.<a href="../haneul_system/validator.md#haneul_system_validator_haneul_address">haneul_address</a>
}
</code></pre>



</details>

<a name="haneul_system_validator_name"></a>

## Function `name`



<pre><code><b>public</b> <b>fun</b> <a href="../haneul_system/validator.md#haneul_system_validator_name">name</a>(self: &<a href="../haneul_system/validator.md#haneul_system_validator_Validator">haneul_system::validator::Validator</a>): &<a href="../std/string.md#std_string_String">std::string::String</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul_system/validator.md#haneul_system_validator_name">name</a>(self: &<a href="../haneul_system/validator.md#haneul_system_validator_Validator">Validator</a>): &String {
    &self.<a href="../haneul_system/validator.md#haneul_system_validator_metadata">metadata</a>.<a href="../haneul_system/validator.md#haneul_system_validator_name">name</a>
}
</code></pre>



</details>

<a name="haneul_system_validator_description"></a>

## Function `description`



<pre><code><b>public</b> <b>fun</b> <a href="../haneul_system/validator.md#haneul_system_validator_description">description</a>(self: &<a href="../haneul_system/validator.md#haneul_system_validator_Validator">haneul_system::validator::Validator</a>): &<a href="../std/string.md#std_string_String">std::string::String</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul_system/validator.md#haneul_system_validator_description">description</a>(self: &<a href="../haneul_system/validator.md#haneul_system_validator_Validator">Validator</a>): &String {
    &self.<a href="../haneul_system/validator.md#haneul_system_validator_metadata">metadata</a>.<a href="../haneul_system/validator.md#haneul_system_validator_description">description</a>
}
</code></pre>



</details>

<a name="haneul_system_validator_image_url"></a>

## Function `image_url`



<pre><code><b>public</b> <b>fun</b> <a href="../haneul_system/validator.md#haneul_system_validator_image_url">image_url</a>(self: &<a href="../haneul_system/validator.md#haneul_system_validator_Validator">haneul_system::validator::Validator</a>): &<a href="../haneul/url.md#haneul_url_Url">haneul::url::Url</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul_system/validator.md#haneul_system_validator_image_url">image_url</a>(self: &<a href="../haneul_system/validator.md#haneul_system_validator_Validator">Validator</a>): &Url {
    &self.<a href="../haneul_system/validator.md#haneul_system_validator_metadata">metadata</a>.<a href="../haneul_system/validator.md#haneul_system_validator_image_url">image_url</a>
}
</code></pre>



</details>

<a name="haneul_system_validator_project_url"></a>

## Function `project_url`



<pre><code><b>public</b> <b>fun</b> <a href="../haneul_system/validator.md#haneul_system_validator_project_url">project_url</a>(self: &<a href="../haneul_system/validator.md#haneul_system_validator_Validator">haneul_system::validator::Validator</a>): &<a href="../haneul/url.md#haneul_url_Url">haneul::url::Url</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul_system/validator.md#haneul_system_validator_project_url">project_url</a>(self: &<a href="../haneul_system/validator.md#haneul_system_validator_Validator">Validator</a>): &Url {
    &self.<a href="../haneul_system/validator.md#haneul_system_validator_metadata">metadata</a>.<a href="../haneul_system/validator.md#haneul_system_validator_project_url">project_url</a>
}
</code></pre>



</details>

<a name="haneul_system_validator_network_address"></a>

## Function `network_address`



<pre><code><b>public</b> <b>fun</b> <a href="../haneul_system/validator.md#haneul_system_validator_network_address">network_address</a>(self: &<a href="../haneul_system/validator.md#haneul_system_validator_Validator">haneul_system::validator::Validator</a>): &<a href="../std/string.md#std_string_String">std::string::String</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul_system/validator.md#haneul_system_validator_network_address">network_address</a>(self: &<a href="../haneul_system/validator.md#haneul_system_validator_Validator">Validator</a>): &String {
    &self.<a href="../haneul_system/validator.md#haneul_system_validator_metadata">metadata</a>.net_address
}
</code></pre>



</details>

<a name="haneul_system_validator_p2p_address"></a>

## Function `p2p_address`



<pre><code><b>public</b> <b>fun</b> <a href="../haneul_system/validator.md#haneul_system_validator_p2p_address">p2p_address</a>(self: &<a href="../haneul_system/validator.md#haneul_system_validator_Validator">haneul_system::validator::Validator</a>): &<a href="../std/string.md#std_string_String">std::string::String</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul_system/validator.md#haneul_system_validator_p2p_address">p2p_address</a>(self: &<a href="../haneul_system/validator.md#haneul_system_validator_Validator">Validator</a>): &String {
    &self.<a href="../haneul_system/validator.md#haneul_system_validator_metadata">metadata</a>.<a href="../haneul_system/validator.md#haneul_system_validator_p2p_address">p2p_address</a>
}
</code></pre>



</details>

<a name="haneul_system_validator_primary_address"></a>

## Function `primary_address`



<pre><code><b>public</b> <b>fun</b> <a href="../haneul_system/validator.md#haneul_system_validator_primary_address">primary_address</a>(self: &<a href="../haneul_system/validator.md#haneul_system_validator_Validator">haneul_system::validator::Validator</a>): &<a href="../std/string.md#std_string_String">std::string::String</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul_system/validator.md#haneul_system_validator_primary_address">primary_address</a>(self: &<a href="../haneul_system/validator.md#haneul_system_validator_Validator">Validator</a>): &String {
    &self.<a href="../haneul_system/validator.md#haneul_system_validator_metadata">metadata</a>.<a href="../haneul_system/validator.md#haneul_system_validator_primary_address">primary_address</a>
}
</code></pre>



</details>

<a name="haneul_system_validator_worker_address"></a>

## Function `worker_address`



<pre><code><b>public</b> <b>fun</b> <a href="../haneul_system/validator.md#haneul_system_validator_worker_address">worker_address</a>(self: &<a href="../haneul_system/validator.md#haneul_system_validator_Validator">haneul_system::validator::Validator</a>): &<a href="../std/string.md#std_string_String">std::string::String</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul_system/validator.md#haneul_system_validator_worker_address">worker_address</a>(self: &<a href="../haneul_system/validator.md#haneul_system_validator_Validator">Validator</a>): &String {
    &self.<a href="../haneul_system/validator.md#haneul_system_validator_metadata">metadata</a>.<a href="../haneul_system/validator.md#haneul_system_validator_worker_address">worker_address</a>
}
</code></pre>



</details>

<a name="haneul_system_validator_protocol_pubkey_bytes"></a>

## Function `protocol_pubkey_bytes`



<pre><code><b>public</b> <b>fun</b> <a href="../haneul_system/validator.md#haneul_system_validator_protocol_pubkey_bytes">protocol_pubkey_bytes</a>(self: &<a href="../haneul_system/validator.md#haneul_system_validator_Validator">haneul_system::validator::Validator</a>): &vector&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul_system/validator.md#haneul_system_validator_protocol_pubkey_bytes">protocol_pubkey_bytes</a>(self: &<a href="../haneul_system/validator.md#haneul_system_validator_Validator">Validator</a>): &vector&lt;u8&gt; {
    &self.<a href="../haneul_system/validator.md#haneul_system_validator_metadata">metadata</a>.<a href="../haneul_system/validator.md#haneul_system_validator_protocol_pubkey_bytes">protocol_pubkey_bytes</a>
}
</code></pre>



</details>

<a name="haneul_system_validator_proof_of_possession"></a>

## Function `proof_of_possession`



<pre><code><b>public</b> <b>fun</b> <a href="../haneul_system/validator.md#haneul_system_validator_proof_of_possession">proof_of_possession</a>(self: &<a href="../haneul_system/validator.md#haneul_system_validator_Validator">haneul_system::validator::Validator</a>): &vector&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul_system/validator.md#haneul_system_validator_proof_of_possession">proof_of_possession</a>(self: &<a href="../haneul_system/validator.md#haneul_system_validator_Validator">Validator</a>): &vector&lt;u8&gt; {
    &self.<a href="../haneul_system/validator.md#haneul_system_validator_metadata">metadata</a>.<a href="../haneul_system/validator.md#haneul_system_validator_proof_of_possession">proof_of_possession</a>
}
</code></pre>



</details>

<a name="haneul_system_validator_network_pubkey_bytes"></a>

## Function `network_pubkey_bytes`



<pre><code><b>public</b> <b>fun</b> <a href="../haneul_system/validator.md#haneul_system_validator_network_pubkey_bytes">network_pubkey_bytes</a>(self: &<a href="../haneul_system/validator.md#haneul_system_validator_Validator">haneul_system::validator::Validator</a>): &vector&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul_system/validator.md#haneul_system_validator_network_pubkey_bytes">network_pubkey_bytes</a>(self: &<a href="../haneul_system/validator.md#haneul_system_validator_Validator">Validator</a>): &vector&lt;u8&gt; {
    &self.<a href="../haneul_system/validator.md#haneul_system_validator_metadata">metadata</a>.<a href="../haneul_system/validator.md#haneul_system_validator_network_pubkey_bytes">network_pubkey_bytes</a>
}
</code></pre>



</details>

<a name="haneul_system_validator_worker_pubkey_bytes"></a>

## Function `worker_pubkey_bytes`



<pre><code><b>public</b> <b>fun</b> <a href="../haneul_system/validator.md#haneul_system_validator_worker_pubkey_bytes">worker_pubkey_bytes</a>(self: &<a href="../haneul_system/validator.md#haneul_system_validator_Validator">haneul_system::validator::Validator</a>): &vector&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul_system/validator.md#haneul_system_validator_worker_pubkey_bytes">worker_pubkey_bytes</a>(self: &<a href="../haneul_system/validator.md#haneul_system_validator_Validator">Validator</a>): &vector&lt;u8&gt; {
    &self.<a href="../haneul_system/validator.md#haneul_system_validator_metadata">metadata</a>.<a href="../haneul_system/validator.md#haneul_system_validator_worker_pubkey_bytes">worker_pubkey_bytes</a>
}
</code></pre>



</details>

<a name="haneul_system_validator_next_epoch_network_address"></a>

## Function `next_epoch_network_address`



<pre><code><b>public</b> <b>fun</b> <a href="../haneul_system/validator.md#haneul_system_validator_next_epoch_network_address">next_epoch_network_address</a>(self: &<a href="../haneul_system/validator.md#haneul_system_validator_Validator">haneul_system::validator::Validator</a>): &<a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul_system/validator.md#haneul_system_validator_next_epoch_network_address">next_epoch_network_address</a>(self: &<a href="../haneul_system/validator.md#haneul_system_validator_Validator">Validator</a>): &Option&lt;String&gt; {
    &self.<a href="../haneul_system/validator.md#haneul_system_validator_metadata">metadata</a>.next_epoch_net_address
}
</code></pre>



</details>

<a name="haneul_system_validator_next_epoch_p2p_address"></a>

## Function `next_epoch_p2p_address`



<pre><code><b>public</b> <b>fun</b> <a href="../haneul_system/validator.md#haneul_system_validator_next_epoch_p2p_address">next_epoch_p2p_address</a>(self: &<a href="../haneul_system/validator.md#haneul_system_validator_Validator">haneul_system::validator::Validator</a>): &<a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul_system/validator.md#haneul_system_validator_next_epoch_p2p_address">next_epoch_p2p_address</a>(self: &<a href="../haneul_system/validator.md#haneul_system_validator_Validator">Validator</a>): &Option&lt;String&gt; {
    &self.<a href="../haneul_system/validator.md#haneul_system_validator_metadata">metadata</a>.<a href="../haneul_system/validator.md#haneul_system_validator_next_epoch_p2p_address">next_epoch_p2p_address</a>
}
</code></pre>



</details>

<a name="haneul_system_validator_next_epoch_primary_address"></a>

## Function `next_epoch_primary_address`



<pre><code><b>public</b> <b>fun</b> <a href="../haneul_system/validator.md#haneul_system_validator_next_epoch_primary_address">next_epoch_primary_address</a>(self: &<a href="../haneul_system/validator.md#haneul_system_validator_Validator">haneul_system::validator::Validator</a>): &<a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul_system/validator.md#haneul_system_validator_next_epoch_primary_address">next_epoch_primary_address</a>(self: &<a href="../haneul_system/validator.md#haneul_system_validator_Validator">Validator</a>): &Option&lt;String&gt; {
    &self.<a href="../haneul_system/validator.md#haneul_system_validator_metadata">metadata</a>.<a href="../haneul_system/validator.md#haneul_system_validator_next_epoch_primary_address">next_epoch_primary_address</a>
}
</code></pre>



</details>

<a name="haneul_system_validator_next_epoch_worker_address"></a>

## Function `next_epoch_worker_address`



<pre><code><b>public</b> <b>fun</b> <a href="../haneul_system/validator.md#haneul_system_validator_next_epoch_worker_address">next_epoch_worker_address</a>(self: &<a href="../haneul_system/validator.md#haneul_system_validator_Validator">haneul_system::validator::Validator</a>): &<a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul_system/validator.md#haneul_system_validator_next_epoch_worker_address">next_epoch_worker_address</a>(self: &<a href="../haneul_system/validator.md#haneul_system_validator_Validator">Validator</a>): &Option&lt;String&gt; {
    &self.<a href="../haneul_system/validator.md#haneul_system_validator_metadata">metadata</a>.<a href="../haneul_system/validator.md#haneul_system_validator_next_epoch_worker_address">next_epoch_worker_address</a>
}
</code></pre>



</details>

<a name="haneul_system_validator_next_epoch_protocol_pubkey_bytes"></a>

## Function `next_epoch_protocol_pubkey_bytes`



<pre><code><b>public</b> <b>fun</b> <a href="../haneul_system/validator.md#haneul_system_validator_next_epoch_protocol_pubkey_bytes">next_epoch_protocol_pubkey_bytes</a>(self: &<a href="../haneul_system/validator.md#haneul_system_validator_Validator">haneul_system::validator::Validator</a>): &<a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;vector&lt;u8&gt;&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul_system/validator.md#haneul_system_validator_next_epoch_protocol_pubkey_bytes">next_epoch_protocol_pubkey_bytes</a>(self: &<a href="../haneul_system/validator.md#haneul_system_validator_Validator">Validator</a>): &Option&lt;vector&lt;u8&gt;&gt; {
    &self.<a href="../haneul_system/validator.md#haneul_system_validator_metadata">metadata</a>.<a href="../haneul_system/validator.md#haneul_system_validator_next_epoch_protocol_pubkey_bytes">next_epoch_protocol_pubkey_bytes</a>
}
</code></pre>



</details>

<a name="haneul_system_validator_next_epoch_proof_of_possession"></a>

## Function `next_epoch_proof_of_possession`



<pre><code><b>public</b> <b>fun</b> <a href="../haneul_system/validator.md#haneul_system_validator_next_epoch_proof_of_possession">next_epoch_proof_of_possession</a>(self: &<a href="../haneul_system/validator.md#haneul_system_validator_Validator">haneul_system::validator::Validator</a>): &<a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;vector&lt;u8&gt;&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul_system/validator.md#haneul_system_validator_next_epoch_proof_of_possession">next_epoch_proof_of_possession</a>(self: &<a href="../haneul_system/validator.md#haneul_system_validator_Validator">Validator</a>): &Option&lt;vector&lt;u8&gt;&gt; {
    &self.<a href="../haneul_system/validator.md#haneul_system_validator_metadata">metadata</a>.<a href="../haneul_system/validator.md#haneul_system_validator_next_epoch_proof_of_possession">next_epoch_proof_of_possession</a>
}
</code></pre>



</details>

<a name="haneul_system_validator_next_epoch_network_pubkey_bytes"></a>

## Function `next_epoch_network_pubkey_bytes`



<pre><code><b>public</b> <b>fun</b> <a href="../haneul_system/validator.md#haneul_system_validator_next_epoch_network_pubkey_bytes">next_epoch_network_pubkey_bytes</a>(self: &<a href="../haneul_system/validator.md#haneul_system_validator_Validator">haneul_system::validator::Validator</a>): &<a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;vector&lt;u8&gt;&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul_system/validator.md#haneul_system_validator_next_epoch_network_pubkey_bytes">next_epoch_network_pubkey_bytes</a>(self: &<a href="../haneul_system/validator.md#haneul_system_validator_Validator">Validator</a>): &Option&lt;vector&lt;u8&gt;&gt; {
    &self.<a href="../haneul_system/validator.md#haneul_system_validator_metadata">metadata</a>.<a href="../haneul_system/validator.md#haneul_system_validator_next_epoch_network_pubkey_bytes">next_epoch_network_pubkey_bytes</a>
}
</code></pre>



</details>

<a name="haneul_system_validator_next_epoch_worker_pubkey_bytes"></a>

## Function `next_epoch_worker_pubkey_bytes`



<pre><code><b>public</b> <b>fun</b> <a href="../haneul_system/validator.md#haneul_system_validator_next_epoch_worker_pubkey_bytes">next_epoch_worker_pubkey_bytes</a>(self: &<a href="../haneul_system/validator.md#haneul_system_validator_Validator">haneul_system::validator::Validator</a>): &<a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;vector&lt;u8&gt;&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul_system/validator.md#haneul_system_validator_next_epoch_worker_pubkey_bytes">next_epoch_worker_pubkey_bytes</a>(self: &<a href="../haneul_system/validator.md#haneul_system_validator_Validator">Validator</a>): &Option&lt;vector&lt;u8&gt;&gt; {
    &self.<a href="../haneul_system/validator.md#haneul_system_validator_metadata">metadata</a>.<a href="../haneul_system/validator.md#haneul_system_validator_next_epoch_worker_pubkey_bytes">next_epoch_worker_pubkey_bytes</a>
}
</code></pre>



</details>

<a name="haneul_system_validator_operation_cap_id"></a>

## Function `operation_cap_id`



<pre><code><b>public</b> <b>fun</b> <a href="../haneul_system/validator.md#haneul_system_validator_operation_cap_id">operation_cap_id</a>(self: &<a href="../haneul_system/validator.md#haneul_system_validator_Validator">haneul_system::validator::Validator</a>): &<a href="../haneul/object.md#haneul_object_ID">haneul::object::ID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul_system/validator.md#haneul_system_validator_operation_cap_id">operation_cap_id</a>(self: &<a href="../haneul_system/validator.md#haneul_system_validator_Validator">Validator</a>): &ID {
    &self.<a href="../haneul_system/validator.md#haneul_system_validator_operation_cap_id">operation_cap_id</a>
}
</code></pre>



</details>

<a name="haneul_system_validator_next_epoch_gas_price"></a>

## Function `next_epoch_gas_price`



<pre><code><b>public</b> <b>fun</b> <a href="../haneul_system/validator.md#haneul_system_validator_next_epoch_gas_price">next_epoch_gas_price</a>(self: &<a href="../haneul_system/validator.md#haneul_system_validator_Validator">haneul_system::validator::Validator</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul_system/validator.md#haneul_system_validator_next_epoch_gas_price">next_epoch_gas_price</a>(self: &<a href="../haneul_system/validator.md#haneul_system_validator_Validator">Validator</a>): u64 {
    self.<a href="../haneul_system/validator.md#haneul_system_validator_next_epoch_gas_price">next_epoch_gas_price</a>
}
</code></pre>



</details>

<a name="haneul_system_validator_total_stake_amount"></a>

## Function `total_stake_amount`



<pre><code><b>public</b> <b>fun</b> <a href="../haneul_system/validator.md#haneul_system_validator_total_stake_amount">total_stake_amount</a>(self: &<a href="../haneul_system/validator.md#haneul_system_validator_Validator">haneul_system::validator::Validator</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul_system/validator.md#haneul_system_validator_total_stake_amount">total_stake_amount</a>(self: &<a href="../haneul_system/validator.md#haneul_system_validator_Validator">Validator</a>): u64 {
    self.<a href="../haneul_system/staking_pool.md#haneul_system_staking_pool">staking_pool</a>.haneul_balance()
}
</code></pre>



</details>

<a name="haneul_system_validator_stake_amount"></a>

## Function `stake_amount`



<pre><code><b>public</b> <b>fun</b> <a href="../haneul_system/validator.md#haneul_system_validator_stake_amount">stake_amount</a>(self: &<a href="../haneul_system/validator.md#haneul_system_validator_Validator">haneul_system::validator::Validator</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul_system/validator.md#haneul_system_validator_stake_amount">stake_amount</a>(self: &<a href="../haneul_system/validator.md#haneul_system_validator_Validator">Validator</a>): u64 {
    self.<a href="../haneul_system/staking_pool.md#haneul_system_staking_pool">staking_pool</a>.haneul_balance()
}
</code></pre>



</details>

<a name="haneul_system_validator_total_stake"></a>

## Function `total_stake`

Return the total amount staked with this validator


<pre><code><b>public</b> <b>fun</b> <a href="../haneul_system/validator.md#haneul_system_validator_total_stake">total_stake</a>(self: &<a href="../haneul_system/validator.md#haneul_system_validator_Validator">haneul_system::validator::Validator</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul_system/validator.md#haneul_system_validator_total_stake">total_stake</a>(self: &<a href="../haneul_system/validator.md#haneul_system_validator_Validator">Validator</a>): u64 {
    self.<a href="../haneul_system/staking_pool.md#haneul_system_staking_pool">staking_pool</a>.haneul_balance()
}
</code></pre>



</details>

<a name="haneul_system_validator_voting_power"></a>

## Function `voting_power`

Return the voting power of this validator.


<pre><code><b>public</b> <b>fun</b> <a href="../haneul_system/voting_power.md#haneul_system_voting_power">voting_power</a>(self: &<a href="../haneul_system/validator.md#haneul_system_validator_Validator">haneul_system::validator::Validator</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul_system/voting_power.md#haneul_system_voting_power">voting_power</a>(self: &<a href="../haneul_system/validator.md#haneul_system_validator_Validator">Validator</a>): u64 {
    self.<a href="../haneul_system/voting_power.md#haneul_system_voting_power">voting_power</a>
}
</code></pre>



</details>

<a name="haneul_system_validator_set_voting_power"></a>

## Function `set_voting_power`

Set the voting power of this validator, called only from validator_set.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../haneul_system/validator.md#haneul_system_validator_set_voting_power">set_voting_power</a>(self: &<b>mut</b> <a href="../haneul_system/validator.md#haneul_system_validator_Validator">haneul_system::validator::Validator</a>, new_voting_power: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../haneul_system/validator.md#haneul_system_validator_set_voting_power">set_voting_power</a>(self: &<b>mut</b> <a href="../haneul_system/validator.md#haneul_system_validator_Validator">Validator</a>, new_voting_power: u64) {
    self.<a href="../haneul_system/voting_power.md#haneul_system_voting_power">voting_power</a> = new_voting_power;
}
</code></pre>



</details>

<a name="haneul_system_validator_pending_stake_amount"></a>

## Function `pending_stake_amount`



<pre><code><b>public</b> <b>fun</b> <a href="../haneul_system/validator.md#haneul_system_validator_pending_stake_amount">pending_stake_amount</a>(self: &<a href="../haneul_system/validator.md#haneul_system_validator_Validator">haneul_system::validator::Validator</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul_system/validator.md#haneul_system_validator_pending_stake_amount">pending_stake_amount</a>(self: &<a href="../haneul_system/validator.md#haneul_system_validator_Validator">Validator</a>): u64 {
    self.<a href="../haneul_system/staking_pool.md#haneul_system_staking_pool">staking_pool</a>.<a href="../haneul_system/validator.md#haneul_system_validator_pending_stake_amount">pending_stake_amount</a>()
}
</code></pre>



</details>

<a name="haneul_system_validator_pending_stake_withdraw_amount"></a>

## Function `pending_stake_withdraw_amount`



<pre><code><b>public</b> <b>fun</b> <a href="../haneul_system/validator.md#haneul_system_validator_pending_stake_withdraw_amount">pending_stake_withdraw_amount</a>(self: &<a href="../haneul_system/validator.md#haneul_system_validator_Validator">haneul_system::validator::Validator</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul_system/validator.md#haneul_system_validator_pending_stake_withdraw_amount">pending_stake_withdraw_amount</a>(self: &<a href="../haneul_system/validator.md#haneul_system_validator_Validator">Validator</a>): u64 {
    self.<a href="../haneul_system/staking_pool.md#haneul_system_staking_pool">staking_pool</a>.<a href="../haneul_system/validator.md#haneul_system_validator_pending_stake_withdraw_amount">pending_stake_withdraw_amount</a>()
}
</code></pre>



</details>

<a name="haneul_system_validator_gas_price"></a>

## Function `gas_price`



<pre><code><b>public</b> <b>fun</b> <a href="../haneul_system/validator.md#haneul_system_validator_gas_price">gas_price</a>(self: &<a href="../haneul_system/validator.md#haneul_system_validator_Validator">haneul_system::validator::Validator</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul_system/validator.md#haneul_system_validator_gas_price">gas_price</a>(self: &<a href="../haneul_system/validator.md#haneul_system_validator_Validator">Validator</a>): u64 {
    self.<a href="../haneul_system/validator.md#haneul_system_validator_gas_price">gas_price</a>
}
</code></pre>



</details>

<a name="haneul_system_validator_commission_rate"></a>

## Function `commission_rate`



<pre><code><b>public</b> <b>fun</b> <a href="../haneul_system/validator.md#haneul_system_validator_commission_rate">commission_rate</a>(self: &<a href="../haneul_system/validator.md#haneul_system_validator_Validator">haneul_system::validator::Validator</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul_system/validator.md#haneul_system_validator_commission_rate">commission_rate</a>(self: &<a href="../haneul_system/validator.md#haneul_system_validator_Validator">Validator</a>): u64 {
    self.<a href="../haneul_system/validator.md#haneul_system_validator_commission_rate">commission_rate</a>
}
</code></pre>



</details>

<a name="haneul_system_validator_pool_token_exchange_rate_at_epoch"></a>

## Function `pool_token_exchange_rate_at_epoch`



<pre><code><b>public</b> <b>fun</b> <a href="../haneul_system/validator.md#haneul_system_validator_pool_token_exchange_rate_at_epoch">pool_token_exchange_rate_at_epoch</a>(self: &<a href="../haneul_system/validator.md#haneul_system_validator_Validator">haneul_system::validator::Validator</a>, epoch: u64): <a href="../haneul_system/staking_pool.md#haneul_system_staking_pool_PoolTokenExchangeRate">haneul_system::staking_pool::PoolTokenExchangeRate</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul_system/validator.md#haneul_system_validator_pool_token_exchange_rate_at_epoch">pool_token_exchange_rate_at_epoch</a>(self: &<a href="../haneul_system/validator.md#haneul_system_validator_Validator">Validator</a>, epoch: u64): PoolTokenExchangeRate {
    self.<a href="../haneul_system/staking_pool.md#haneul_system_staking_pool">staking_pool</a>.<a href="../haneul_system/validator.md#haneul_system_validator_pool_token_exchange_rate_at_epoch">pool_token_exchange_rate_at_epoch</a>(epoch)
}
</code></pre>



</details>

<a name="haneul_system_validator_staking_pool_id"></a>

## Function `staking_pool_id`



<pre><code><b>public</b> <b>fun</b> <a href="../haneul_system/validator.md#haneul_system_validator_staking_pool_id">staking_pool_id</a>(self: &<a href="../haneul_system/validator.md#haneul_system_validator_Validator">haneul_system::validator::Validator</a>): <a href="../haneul/object.md#haneul_object_ID">haneul::object::ID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul_system/validator.md#haneul_system_validator_staking_pool_id">staking_pool_id</a>(self: &<a href="../haneul_system/validator.md#haneul_system_validator_Validator">Validator</a>): ID {
    object::id(&self.<a href="../haneul_system/staking_pool.md#haneul_system_staking_pool">staking_pool</a>)
}
</code></pre>



</details>

<a name="haneul_system_validator_is_duplicate"></a>

## Function `is_duplicate`



<pre><code><b>public</b> <b>fun</b> <a href="../haneul_system/validator.md#haneul_system_validator_is_duplicate">is_duplicate</a>(self: &<a href="../haneul_system/validator.md#haneul_system_validator_Validator">haneul_system::validator::Validator</a>, other: &<a href="../haneul_system/validator.md#haneul_system_validator_Validator">haneul_system::validator::Validator</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul_system/validator.md#haneul_system_validator_is_duplicate">is_duplicate</a>(self: &<a href="../haneul_system/validator.md#haneul_system_validator_Validator">Validator</a>, other: &<a href="../haneul_system/validator.md#haneul_system_validator_Validator">Validator</a>): bool {
    <b>let</b> self = &self.<a href="../haneul_system/validator.md#haneul_system_validator_metadata">metadata</a>;
    <b>let</b> other = &other.<a href="../haneul_system/validator.md#haneul_system_validator_metadata">metadata</a>;
    self.<a href="../haneul_system/validator.md#haneul_system_validator_haneul_address">haneul_address</a> == other.<a href="../haneul_system/validator.md#haneul_system_validator_haneul_address">haneul_address</a>
        || self.<a href="../haneul_system/validator.md#haneul_system_validator_name">name</a> == other.<a href="../haneul_system/validator.md#haneul_system_validator_name">name</a>
        || self.net_address == other.net_address
        || self.<a href="../haneul_system/validator.md#haneul_system_validator_p2p_address">p2p_address</a> == other.<a href="../haneul_system/validator.md#haneul_system_validator_p2p_address">p2p_address</a>
        || self.<a href="../haneul_system/validator.md#haneul_system_validator_primary_address">primary_address</a> == other.<a href="../haneul_system/validator.md#haneul_system_validator_primary_address">primary_address</a>
        || self.<a href="../haneul_system/validator.md#haneul_system_validator_worker_address">worker_address</a> == other.<a href="../haneul_system/validator.md#haneul_system_validator_worker_address">worker_address</a>
        || self.<a href="../haneul_system/validator.md#haneul_system_validator_protocol_pubkey_bytes">protocol_pubkey_bytes</a> == other.<a href="../haneul_system/validator.md#haneul_system_validator_protocol_pubkey_bytes">protocol_pubkey_bytes</a>
        || self.<a href="../haneul_system/validator.md#haneul_system_validator_network_pubkey_bytes">network_pubkey_bytes</a> == other.<a href="../haneul_system/validator.md#haneul_system_validator_network_pubkey_bytes">network_pubkey_bytes</a>
        || self.<a href="../haneul_system/validator.md#haneul_system_validator_network_pubkey_bytes">network_pubkey_bytes</a> == other.<a href="../haneul_system/validator.md#haneul_system_validator_worker_pubkey_bytes">worker_pubkey_bytes</a>
        || self.<a href="../haneul_system/validator.md#haneul_system_validator_worker_pubkey_bytes">worker_pubkey_bytes</a> == other.<a href="../haneul_system/validator.md#haneul_system_validator_worker_pubkey_bytes">worker_pubkey_bytes</a>
        || self.<a href="../haneul_system/validator.md#haneul_system_validator_worker_pubkey_bytes">worker_pubkey_bytes</a> == other.<a href="../haneul_system/validator.md#haneul_system_validator_network_pubkey_bytes">network_pubkey_bytes</a>
        // All next epoch parameters.
        || <a href="../haneul_system/validator.md#haneul_system_validator_both_some_and_equal">both_some_and_equal</a>!(self.next_epoch_net_address, other.next_epoch_net_address)
        || <a href="../haneul_system/validator.md#haneul_system_validator_both_some_and_equal">both_some_and_equal</a>!(self.<a href="../haneul_system/validator.md#haneul_system_validator_next_epoch_p2p_address">next_epoch_p2p_address</a>, other.<a href="../haneul_system/validator.md#haneul_system_validator_next_epoch_p2p_address">next_epoch_p2p_address</a>)
        || <a href="../haneul_system/validator.md#haneul_system_validator_both_some_and_equal">both_some_and_equal</a>!(self.<a href="../haneul_system/validator.md#haneul_system_validator_next_epoch_primary_address">next_epoch_primary_address</a>, other.<a href="../haneul_system/validator.md#haneul_system_validator_next_epoch_primary_address">next_epoch_primary_address</a>)
        || <a href="../haneul_system/validator.md#haneul_system_validator_both_some_and_equal">both_some_and_equal</a>!(self.<a href="../haneul_system/validator.md#haneul_system_validator_next_epoch_worker_address">next_epoch_worker_address</a>, other.<a href="../haneul_system/validator.md#haneul_system_validator_next_epoch_worker_address">next_epoch_worker_address</a>)
        || <a href="../haneul_system/validator.md#haneul_system_validator_both_some_and_equal">both_some_and_equal</a>!(self.<a href="../haneul_system/validator.md#haneul_system_validator_next_epoch_protocol_pubkey_bytes">next_epoch_protocol_pubkey_bytes</a>, other.<a href="../haneul_system/validator.md#haneul_system_validator_next_epoch_protocol_pubkey_bytes">next_epoch_protocol_pubkey_bytes</a>)
        || <a href="../haneul_system/validator.md#haneul_system_validator_both_some_and_equal">both_some_and_equal</a>!(self.<a href="../haneul_system/validator.md#haneul_system_validator_next_epoch_network_pubkey_bytes">next_epoch_network_pubkey_bytes</a>, other.<a href="../haneul_system/validator.md#haneul_system_validator_next_epoch_network_pubkey_bytes">next_epoch_network_pubkey_bytes</a>)
        || <a href="../haneul_system/validator.md#haneul_system_validator_both_some_and_equal">both_some_and_equal</a>!(self.<a href="../haneul_system/validator.md#haneul_system_validator_next_epoch_network_pubkey_bytes">next_epoch_network_pubkey_bytes</a>, other.<a href="../haneul_system/validator.md#haneul_system_validator_next_epoch_worker_pubkey_bytes">next_epoch_worker_pubkey_bytes</a>)
        || <a href="../haneul_system/validator.md#haneul_system_validator_both_some_and_equal">both_some_and_equal</a>!(self.<a href="../haneul_system/validator.md#haneul_system_validator_next_epoch_worker_pubkey_bytes">next_epoch_worker_pubkey_bytes</a>, other.<a href="../haneul_system/validator.md#haneul_system_validator_next_epoch_worker_pubkey_bytes">next_epoch_worker_pubkey_bytes</a>)
        || <a href="../haneul_system/validator.md#haneul_system_validator_both_some_and_equal">both_some_and_equal</a>!(self.<a href="../haneul_system/validator.md#haneul_system_validator_next_epoch_worker_pubkey_bytes">next_epoch_worker_pubkey_bytes</a>, other.<a href="../haneul_system/validator.md#haneul_system_validator_next_epoch_network_pubkey_bytes">next_epoch_network_pubkey_bytes</a>)
        // My next epoch parameters with other current epoch parameters.
        || self.next_epoch_net_address.is_some_and!(|v| v == other.net_address)
        || self.<a href="../haneul_system/validator.md#haneul_system_validator_next_epoch_p2p_address">next_epoch_p2p_address</a>.is_some_and!(|v| v == other.<a href="../haneul_system/validator.md#haneul_system_validator_p2p_address">p2p_address</a>)
        || self.<a href="../haneul_system/validator.md#haneul_system_validator_next_epoch_primary_address">next_epoch_primary_address</a>.is_some_and!(|v| v == other.<a href="../haneul_system/validator.md#haneul_system_validator_primary_address">primary_address</a>)
        || self.<a href="../haneul_system/validator.md#haneul_system_validator_next_epoch_worker_address">next_epoch_worker_address</a>.is_some_and!(|v| v == other.<a href="../haneul_system/validator.md#haneul_system_validator_worker_address">worker_address</a>)
        || self.<a href="../haneul_system/validator.md#haneul_system_validator_next_epoch_protocol_pubkey_bytes">next_epoch_protocol_pubkey_bytes</a>.is_some_and!(|v| v == other.<a href="../haneul_system/validator.md#haneul_system_validator_protocol_pubkey_bytes">protocol_pubkey_bytes</a>)
        || self.<a href="../haneul_system/validator.md#haneul_system_validator_next_epoch_network_pubkey_bytes">next_epoch_network_pubkey_bytes</a>.is_some_and!(|v| v == other.<a href="../haneul_system/validator.md#haneul_system_validator_network_pubkey_bytes">network_pubkey_bytes</a>)
        || self.<a href="../haneul_system/validator.md#haneul_system_validator_next_epoch_network_pubkey_bytes">next_epoch_network_pubkey_bytes</a>.is_some_and!(|v| v == other.<a href="../haneul_system/validator.md#haneul_system_validator_worker_pubkey_bytes">worker_pubkey_bytes</a>)
        || self.<a href="../haneul_system/validator.md#haneul_system_validator_next_epoch_worker_pubkey_bytes">next_epoch_worker_pubkey_bytes</a>.is_some_and!(|v| v == other.<a href="../haneul_system/validator.md#haneul_system_validator_worker_pubkey_bytes">worker_pubkey_bytes</a>)
        || self.<a href="../haneul_system/validator.md#haneul_system_validator_next_epoch_worker_pubkey_bytes">next_epoch_worker_pubkey_bytes</a>.is_some_and!(|v| v == other.<a href="../haneul_system/validator.md#haneul_system_validator_network_pubkey_bytes">network_pubkey_bytes</a>)
        // Other next epoch parameters with my current epoch parameters.
        || other.next_epoch_net_address.is_some_and!(|v| v == self.net_address)
        || other.<a href="../haneul_system/validator.md#haneul_system_validator_next_epoch_p2p_address">next_epoch_p2p_address</a>.is_some_and!(|v| v == self.<a href="../haneul_system/validator.md#haneul_system_validator_p2p_address">p2p_address</a>)
        || other.<a href="../haneul_system/validator.md#haneul_system_validator_next_epoch_primary_address">next_epoch_primary_address</a>.is_some_and!(|v| v == self.<a href="../haneul_system/validator.md#haneul_system_validator_primary_address">primary_address</a>)
        || other.<a href="../haneul_system/validator.md#haneul_system_validator_next_epoch_worker_address">next_epoch_worker_address</a>.is_some_and!(|v| v == self.<a href="../haneul_system/validator.md#haneul_system_validator_worker_address">worker_address</a>)
        || other.<a href="../haneul_system/validator.md#haneul_system_validator_next_epoch_protocol_pubkey_bytes">next_epoch_protocol_pubkey_bytes</a>.is_some_and!(|v| v == self.<a href="../haneul_system/validator.md#haneul_system_validator_protocol_pubkey_bytes">protocol_pubkey_bytes</a>)
        || other.<a href="../haneul_system/validator.md#haneul_system_validator_next_epoch_network_pubkey_bytes">next_epoch_network_pubkey_bytes</a>.is_some_and!(|v| v == self.<a href="../haneul_system/validator.md#haneul_system_validator_network_pubkey_bytes">network_pubkey_bytes</a>)
        || other.<a href="../haneul_system/validator.md#haneul_system_validator_next_epoch_network_pubkey_bytes">next_epoch_network_pubkey_bytes</a>.is_some_and!(|v| v == self.<a href="../haneul_system/validator.md#haneul_system_validator_worker_pubkey_bytes">worker_pubkey_bytes</a>)
        || other.<a href="../haneul_system/validator.md#haneul_system_validator_next_epoch_worker_pubkey_bytes">next_epoch_worker_pubkey_bytes</a>.is_some_and!(|v| v == self.<a href="../haneul_system/validator.md#haneul_system_validator_worker_pubkey_bytes">worker_pubkey_bytes</a>)
        || other.<a href="../haneul_system/validator.md#haneul_system_validator_next_epoch_worker_pubkey_bytes">next_epoch_worker_pubkey_bytes</a>.is_some_and!(|v| v == self.<a href="../haneul_system/validator.md#haneul_system_validator_network_pubkey_bytes">network_pubkey_bytes</a>)
}
</code></pre>



</details>

<a name="haneul_system_validator_both_some_and_equal"></a>

## Macro function `both_some_and_equal`



<pre><code><b>macro</b> <b>fun</b> <a href="../haneul_system/validator.md#haneul_system_validator_both_some_and_equal">both_some_and_equal</a>&lt;$T&gt;($a: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;$T&gt;, $b: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;$T&gt;): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>macro</b> <b>fun</b> <a href="../haneul_system/validator.md#haneul_system_validator_both_some_and_equal">both_some_and_equal</a>&lt;$T&gt;($a: Option&lt;$T&gt;, $b: Option&lt;$T&gt;): bool {
    <b>let</b> (a, b) = ($a, $b);
    a.is_some_and!(|a| b.is_some_and!(|b| a == b))
}
</code></pre>



</details>

<a name="haneul_system_validator_new_unverified_validator_operation_cap_and_transfer"></a>

## Function `new_unverified_validator_operation_cap_and_transfer`

Create a new <code>UnverifiedValidatorOperationCap</code>, transfer to the validator,
and registers it, thus revoking the previous cap's permission.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../haneul_system/validator.md#haneul_system_validator_new_unverified_validator_operation_cap_and_transfer">new_unverified_validator_operation_cap_and_transfer</a>(self: &<b>mut</b> <a href="../haneul_system/validator.md#haneul_system_validator_Validator">haneul_system::validator::Validator</a>, ctx: &<b>mut</b> <a href="../haneul/tx_context.md#haneul_tx_context_TxContext">haneul::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../haneul_system/validator.md#haneul_system_validator_new_unverified_validator_operation_cap_and_transfer">new_unverified_validator_operation_cap_and_transfer</a>(
    self: &<b>mut</b> <a href="../haneul_system/validator.md#haneul_system_validator_Validator">Validator</a>,
    ctx: &<b>mut</b> TxContext,
) {
    <b>let</b> sender = ctx.sender();
    <b>assert</b>!(sender == self.<a href="../haneul_system/validator.md#haneul_system_validator_metadata">metadata</a>.<a href="../haneul_system/validator.md#haneul_system_validator_haneul_address">haneul_address</a>, <a href="../haneul_system/validator.md#haneul_system_validator_ENewCapNotCreatedByValidatorItself">ENewCapNotCreatedByValidatorItself</a>);
    <b>let</b> new_id = <a href="../haneul_system/validator_cap.md#haneul_system_validator_cap_new_unverified_validator_operation_cap_and_transfer">validator_cap::new_unverified_validator_operation_cap_and_transfer</a>(sender, ctx);
    self.<a href="../haneul_system/validator.md#haneul_system_validator_operation_cap_id">operation_cap_id</a> = new_id;
}
</code></pre>



</details>

<a name="haneul_system_validator_update_name"></a>

## Function `update_name`

Update name of the validator.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../haneul_system/validator.md#haneul_system_validator_update_name">update_name</a>(self: &<b>mut</b> <a href="../haneul_system/validator.md#haneul_system_validator_Validator">haneul_system::validator::Validator</a>, <a href="../haneul_system/validator.md#haneul_system_validator_name">name</a>: vector&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../haneul_system/validator.md#haneul_system_validator_update_name">update_name</a>(self: &<b>mut</b> <a href="../haneul_system/validator.md#haneul_system_validator_Validator">Validator</a>, <a href="../haneul_system/validator.md#haneul_system_validator_name">name</a>: vector&lt;u8&gt;) {
    <b>assert</b>!(<a href="../haneul_system/validator.md#haneul_system_validator_name">name</a>.length() &lt;= <a href="../haneul_system/validator.md#haneul_system_validator_MAX_VALIDATOR_METADATA_LENGTH">MAX_VALIDATOR_METADATA_LENGTH</a>, <a href="../haneul_system/validator.md#haneul_system_validator_EValidatorMetadataExceedingLengthLimit">EValidatorMetadataExceedingLengthLimit</a>);
    self.<a href="../haneul_system/validator.md#haneul_system_validator_metadata">metadata</a>.<a href="../haneul_system/validator.md#haneul_system_validator_name">name</a> = <a href="../haneul_system/validator.md#haneul_system_validator_name">name</a>.to_ascii_string().to_string();
}
</code></pre>



</details>

<a name="haneul_system_validator_update_description"></a>

## Function `update_description`

Update description of the validator.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../haneul_system/validator.md#haneul_system_validator_update_description">update_description</a>(self: &<b>mut</b> <a href="../haneul_system/validator.md#haneul_system_validator_Validator">haneul_system::validator::Validator</a>, <a href="../haneul_system/validator.md#haneul_system_validator_description">description</a>: vector&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../haneul_system/validator.md#haneul_system_validator_update_description">update_description</a>(self: &<b>mut</b> <a href="../haneul_system/validator.md#haneul_system_validator_Validator">Validator</a>, <a href="../haneul_system/validator.md#haneul_system_validator_description">description</a>: vector&lt;u8&gt;) {
    <b>assert</b>!(
        <a href="../haneul_system/validator.md#haneul_system_validator_description">description</a>.length() &lt;= <a href="../haneul_system/validator.md#haneul_system_validator_MAX_VALIDATOR_METADATA_LENGTH">MAX_VALIDATOR_METADATA_LENGTH</a>,
        <a href="../haneul_system/validator.md#haneul_system_validator_EValidatorMetadataExceedingLengthLimit">EValidatorMetadataExceedingLengthLimit</a>,
    );
    self.<a href="../haneul_system/validator.md#haneul_system_validator_metadata">metadata</a>.<a href="../haneul_system/validator.md#haneul_system_validator_description">description</a> = <a href="../haneul_system/validator.md#haneul_system_validator_description">description</a>.to_ascii_string().to_string();
}
</code></pre>



</details>

<a name="haneul_system_validator_update_image_url"></a>

## Function `update_image_url`

Update image url of the validator.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../haneul_system/validator.md#haneul_system_validator_update_image_url">update_image_url</a>(self: &<b>mut</b> <a href="../haneul_system/validator.md#haneul_system_validator_Validator">haneul_system::validator::Validator</a>, <a href="../haneul_system/validator.md#haneul_system_validator_image_url">image_url</a>: vector&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../haneul_system/validator.md#haneul_system_validator_update_image_url">update_image_url</a>(self: &<b>mut</b> <a href="../haneul_system/validator.md#haneul_system_validator_Validator">Validator</a>, <a href="../haneul_system/validator.md#haneul_system_validator_image_url">image_url</a>: vector&lt;u8&gt;) {
    <b>assert</b>!(
        <a href="../haneul_system/validator.md#haneul_system_validator_image_url">image_url</a>.length() &lt;= <a href="../haneul_system/validator.md#haneul_system_validator_MAX_VALIDATOR_METADATA_LENGTH">MAX_VALIDATOR_METADATA_LENGTH</a>,
        <a href="../haneul_system/validator.md#haneul_system_validator_EValidatorMetadataExceedingLengthLimit">EValidatorMetadataExceedingLengthLimit</a>,
    );
    self.<a href="../haneul_system/validator.md#haneul_system_validator_metadata">metadata</a>.<a href="../haneul_system/validator.md#haneul_system_validator_image_url">image_url</a> = url::new_unsafe_from_bytes(<a href="../haneul_system/validator.md#haneul_system_validator_image_url">image_url</a>);
}
</code></pre>



</details>

<a name="haneul_system_validator_update_project_url"></a>

## Function `update_project_url`

Update project url of the validator.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../haneul_system/validator.md#haneul_system_validator_update_project_url">update_project_url</a>(self: &<b>mut</b> <a href="../haneul_system/validator.md#haneul_system_validator_Validator">haneul_system::validator::Validator</a>, <a href="../haneul_system/validator.md#haneul_system_validator_project_url">project_url</a>: vector&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../haneul_system/validator.md#haneul_system_validator_update_project_url">update_project_url</a>(self: &<b>mut</b> <a href="../haneul_system/validator.md#haneul_system_validator_Validator">Validator</a>, <a href="../haneul_system/validator.md#haneul_system_validator_project_url">project_url</a>: vector&lt;u8&gt;) {
    <b>assert</b>!(
        <a href="../haneul_system/validator.md#haneul_system_validator_project_url">project_url</a>.length() &lt;= <a href="../haneul_system/validator.md#haneul_system_validator_MAX_VALIDATOR_METADATA_LENGTH">MAX_VALIDATOR_METADATA_LENGTH</a>,
        <a href="../haneul_system/validator.md#haneul_system_validator_EValidatorMetadataExceedingLengthLimit">EValidatorMetadataExceedingLengthLimit</a>,
    );
    self.<a href="../haneul_system/validator.md#haneul_system_validator_metadata">metadata</a>.<a href="../haneul_system/validator.md#haneul_system_validator_project_url">project_url</a> = url::new_unsafe_from_bytes(<a href="../haneul_system/validator.md#haneul_system_validator_project_url">project_url</a>);
}
</code></pre>



</details>

<a name="haneul_system_validator_update_next_epoch_network_address"></a>

## Function `update_next_epoch_network_address`

Update network address of this validator, taking effects from next epoch


<pre><code><b>public</b>(package) <b>fun</b> <a href="../haneul_system/validator.md#haneul_system_validator_update_next_epoch_network_address">update_next_epoch_network_address</a>(self: &<b>mut</b> <a href="../haneul_system/validator.md#haneul_system_validator_Validator">haneul_system::validator::Validator</a>, net_address: vector&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../haneul_system/validator.md#haneul_system_validator_update_next_epoch_network_address">update_next_epoch_network_address</a>(
    self: &<b>mut</b> <a href="../haneul_system/validator.md#haneul_system_validator_Validator">Validator</a>,
    net_address: vector&lt;u8&gt;,
) {
    <b>assert</b>!(
        net_address.length() &lt;= <a href="../haneul_system/validator.md#haneul_system_validator_MAX_VALIDATOR_METADATA_LENGTH">MAX_VALIDATOR_METADATA_LENGTH</a>,
        <a href="../haneul_system/validator.md#haneul_system_validator_EValidatorMetadataExceedingLengthLimit">EValidatorMetadataExceedingLengthLimit</a>,
    );
    <b>let</b> net_address = net_address.to_ascii_string().to_string();
    self.<a href="../haneul_system/validator.md#haneul_system_validator_metadata">metadata</a>.next_epoch_net_address = option::some(net_address);
    self.<a href="../haneul_system/validator.md#haneul_system_validator_metadata">metadata</a>.validate();
}
</code></pre>



</details>

<a name="haneul_system_validator_update_candidate_network_address"></a>

## Function `update_candidate_network_address`

Update network address of this candidate validator


<pre><code><b>public</b>(package) <b>fun</b> <a href="../haneul_system/validator.md#haneul_system_validator_update_candidate_network_address">update_candidate_network_address</a>(self: &<b>mut</b> <a href="../haneul_system/validator.md#haneul_system_validator_Validator">haneul_system::validator::Validator</a>, net_address: vector&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../haneul_system/validator.md#haneul_system_validator_update_candidate_network_address">update_candidate_network_address</a>(
    self: &<b>mut</b> <a href="../haneul_system/validator.md#haneul_system_validator_Validator">Validator</a>,
    net_address: vector&lt;u8&gt;,
) {
    <b>assert</b>!(self.<a href="../haneul_system/validator.md#haneul_system_validator_is_preactive">is_preactive</a>(), <a href="../haneul_system/validator.md#haneul_system_validator_ENotValidatorCandidate">ENotValidatorCandidate</a>);
    <b>assert</b>!(
        net_address.length() &lt;= <a href="../haneul_system/validator.md#haneul_system_validator_MAX_VALIDATOR_METADATA_LENGTH">MAX_VALIDATOR_METADATA_LENGTH</a>,
        <a href="../haneul_system/validator.md#haneul_system_validator_EValidatorMetadataExceedingLengthLimit">EValidatorMetadataExceedingLengthLimit</a>,
    );
    <b>let</b> net_address = net_address.to_ascii_string().to_string();
    self.<a href="../haneul_system/validator.md#haneul_system_validator_metadata">metadata</a>.net_address = net_address;
    self.<a href="../haneul_system/validator.md#haneul_system_validator_metadata">metadata</a>.validate();
}
</code></pre>



</details>

<a name="haneul_system_validator_update_next_epoch_p2p_address"></a>

## Function `update_next_epoch_p2p_address`

Update p2p address of this validator, taking effects from next epoch


<pre><code><b>public</b>(package) <b>fun</b> <a href="../haneul_system/validator.md#haneul_system_validator_update_next_epoch_p2p_address">update_next_epoch_p2p_address</a>(self: &<b>mut</b> <a href="../haneul_system/validator.md#haneul_system_validator_Validator">haneul_system::validator::Validator</a>, <a href="../haneul_system/validator.md#haneul_system_validator_p2p_address">p2p_address</a>: vector&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../haneul_system/validator.md#haneul_system_validator_update_next_epoch_p2p_address">update_next_epoch_p2p_address</a>(self: &<b>mut</b> <a href="../haneul_system/validator.md#haneul_system_validator_Validator">Validator</a>, <a href="../haneul_system/validator.md#haneul_system_validator_p2p_address">p2p_address</a>: vector&lt;u8&gt;) {
    <b>assert</b>!(
        <a href="../haneul_system/validator.md#haneul_system_validator_p2p_address">p2p_address</a>.length() &lt;= <a href="../haneul_system/validator.md#haneul_system_validator_MAX_VALIDATOR_METADATA_LENGTH">MAX_VALIDATOR_METADATA_LENGTH</a>,
        <a href="../haneul_system/validator.md#haneul_system_validator_EValidatorMetadataExceedingLengthLimit">EValidatorMetadataExceedingLengthLimit</a>,
    );
    <b>let</b> <a href="../haneul_system/validator.md#haneul_system_validator_p2p_address">p2p_address</a> = <a href="../haneul_system/validator.md#haneul_system_validator_p2p_address">p2p_address</a>.to_ascii_string().to_string();
    self.<a href="../haneul_system/validator.md#haneul_system_validator_metadata">metadata</a>.<a href="../haneul_system/validator.md#haneul_system_validator_next_epoch_p2p_address">next_epoch_p2p_address</a> = option::some(<a href="../haneul_system/validator.md#haneul_system_validator_p2p_address">p2p_address</a>);
    self.<a href="../haneul_system/validator.md#haneul_system_validator_metadata">metadata</a>.validate();
}
</code></pre>



</details>

<a name="haneul_system_validator_update_candidate_p2p_address"></a>

## Function `update_candidate_p2p_address`

Update p2p address of this candidate validator


<pre><code><b>public</b>(package) <b>fun</b> <a href="../haneul_system/validator.md#haneul_system_validator_update_candidate_p2p_address">update_candidate_p2p_address</a>(self: &<b>mut</b> <a href="../haneul_system/validator.md#haneul_system_validator_Validator">haneul_system::validator::Validator</a>, <a href="../haneul_system/validator.md#haneul_system_validator_p2p_address">p2p_address</a>: vector&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../haneul_system/validator.md#haneul_system_validator_update_candidate_p2p_address">update_candidate_p2p_address</a>(self: &<b>mut</b> <a href="../haneul_system/validator.md#haneul_system_validator_Validator">Validator</a>, <a href="../haneul_system/validator.md#haneul_system_validator_p2p_address">p2p_address</a>: vector&lt;u8&gt;) {
    <b>assert</b>!(self.<a href="../haneul_system/validator.md#haneul_system_validator_is_preactive">is_preactive</a>(), <a href="../haneul_system/validator.md#haneul_system_validator_ENotValidatorCandidate">ENotValidatorCandidate</a>);
    <b>assert</b>!(
        <a href="../haneul_system/validator.md#haneul_system_validator_p2p_address">p2p_address</a>.length() &lt;= <a href="../haneul_system/validator.md#haneul_system_validator_MAX_VALIDATOR_METADATA_LENGTH">MAX_VALIDATOR_METADATA_LENGTH</a>,
        <a href="../haneul_system/validator.md#haneul_system_validator_EValidatorMetadataExceedingLengthLimit">EValidatorMetadataExceedingLengthLimit</a>,
    );
    <b>let</b> <a href="../haneul_system/validator.md#haneul_system_validator_p2p_address">p2p_address</a> = <a href="../haneul_system/validator.md#haneul_system_validator_p2p_address">p2p_address</a>.to_ascii_string().to_string();
    self.<a href="../haneul_system/validator.md#haneul_system_validator_metadata">metadata</a>.<a href="../haneul_system/validator.md#haneul_system_validator_p2p_address">p2p_address</a> = <a href="../haneul_system/validator.md#haneul_system_validator_p2p_address">p2p_address</a>;
    self.<a href="../haneul_system/validator.md#haneul_system_validator_metadata">metadata</a>.validate();
}
</code></pre>



</details>

<a name="haneul_system_validator_update_next_epoch_primary_address"></a>

## Function `update_next_epoch_primary_address`

Update primary address of this validator, taking effects from next epoch


<pre><code><b>public</b>(package) <b>fun</b> <a href="../haneul_system/validator.md#haneul_system_validator_update_next_epoch_primary_address">update_next_epoch_primary_address</a>(self: &<b>mut</b> <a href="../haneul_system/validator.md#haneul_system_validator_Validator">haneul_system::validator::Validator</a>, <a href="../haneul_system/validator.md#haneul_system_validator_primary_address">primary_address</a>: vector&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../haneul_system/validator.md#haneul_system_validator_update_next_epoch_primary_address">update_next_epoch_primary_address</a>(
    self: &<b>mut</b> <a href="../haneul_system/validator.md#haneul_system_validator_Validator">Validator</a>,
    <a href="../haneul_system/validator.md#haneul_system_validator_primary_address">primary_address</a>: vector&lt;u8&gt;,
) {
    <b>assert</b>!(
        <a href="../haneul_system/validator.md#haneul_system_validator_primary_address">primary_address</a>.length() &lt;= <a href="../haneul_system/validator.md#haneul_system_validator_MAX_VALIDATOR_METADATA_LENGTH">MAX_VALIDATOR_METADATA_LENGTH</a>,
        <a href="../haneul_system/validator.md#haneul_system_validator_EValidatorMetadataExceedingLengthLimit">EValidatorMetadataExceedingLengthLimit</a>,
    );
    <b>let</b> <a href="../haneul_system/validator.md#haneul_system_validator_primary_address">primary_address</a> = <a href="../haneul_system/validator.md#haneul_system_validator_primary_address">primary_address</a>.to_ascii_string().to_string();
    self.<a href="../haneul_system/validator.md#haneul_system_validator_metadata">metadata</a>.<a href="../haneul_system/validator.md#haneul_system_validator_next_epoch_primary_address">next_epoch_primary_address</a> = option::some(<a href="../haneul_system/validator.md#haneul_system_validator_primary_address">primary_address</a>);
    self.<a href="../haneul_system/validator.md#haneul_system_validator_metadata">metadata</a>.validate();
}
</code></pre>



</details>

<a name="haneul_system_validator_update_candidate_primary_address"></a>

## Function `update_candidate_primary_address`

Update primary address of this candidate validator


<pre><code><b>public</b>(package) <b>fun</b> <a href="../haneul_system/validator.md#haneul_system_validator_update_candidate_primary_address">update_candidate_primary_address</a>(self: &<b>mut</b> <a href="../haneul_system/validator.md#haneul_system_validator_Validator">haneul_system::validator::Validator</a>, <a href="../haneul_system/validator.md#haneul_system_validator_primary_address">primary_address</a>: vector&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../haneul_system/validator.md#haneul_system_validator_update_candidate_primary_address">update_candidate_primary_address</a>(
    self: &<b>mut</b> <a href="../haneul_system/validator.md#haneul_system_validator_Validator">Validator</a>,
    <a href="../haneul_system/validator.md#haneul_system_validator_primary_address">primary_address</a>: vector&lt;u8&gt;,
) {
    <b>assert</b>!(self.<a href="../haneul_system/validator.md#haneul_system_validator_is_preactive">is_preactive</a>(), <a href="../haneul_system/validator.md#haneul_system_validator_ENotValidatorCandidate">ENotValidatorCandidate</a>);
    <b>assert</b>!(
        <a href="../haneul_system/validator.md#haneul_system_validator_primary_address">primary_address</a>.length() &lt;= <a href="../haneul_system/validator.md#haneul_system_validator_MAX_VALIDATOR_METADATA_LENGTH">MAX_VALIDATOR_METADATA_LENGTH</a>,
        <a href="../haneul_system/validator.md#haneul_system_validator_EValidatorMetadataExceedingLengthLimit">EValidatorMetadataExceedingLengthLimit</a>,
    );
    <b>let</b> <a href="../haneul_system/validator.md#haneul_system_validator_primary_address">primary_address</a> = <a href="../haneul_system/validator.md#haneul_system_validator_primary_address">primary_address</a>.to_ascii_string().to_string();
    self.<a href="../haneul_system/validator.md#haneul_system_validator_metadata">metadata</a>.<a href="../haneul_system/validator.md#haneul_system_validator_primary_address">primary_address</a> = <a href="../haneul_system/validator.md#haneul_system_validator_primary_address">primary_address</a>;
    self.<a href="../haneul_system/validator.md#haneul_system_validator_metadata">metadata</a>.validate();
}
</code></pre>



</details>

<a name="haneul_system_validator_update_next_epoch_worker_address"></a>

## Function `update_next_epoch_worker_address`

Update worker address of this validator, taking effects from next epoch


<pre><code><b>public</b>(package) <b>fun</b> <a href="../haneul_system/validator.md#haneul_system_validator_update_next_epoch_worker_address">update_next_epoch_worker_address</a>(self: &<b>mut</b> <a href="../haneul_system/validator.md#haneul_system_validator_Validator">haneul_system::validator::Validator</a>, <a href="../haneul_system/validator.md#haneul_system_validator_worker_address">worker_address</a>: vector&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../haneul_system/validator.md#haneul_system_validator_update_next_epoch_worker_address">update_next_epoch_worker_address</a>(
    self: &<b>mut</b> <a href="../haneul_system/validator.md#haneul_system_validator_Validator">Validator</a>,
    <a href="../haneul_system/validator.md#haneul_system_validator_worker_address">worker_address</a>: vector&lt;u8&gt;,
) {
    <b>assert</b>!(
        <a href="../haneul_system/validator.md#haneul_system_validator_worker_address">worker_address</a>.length() &lt;= <a href="../haneul_system/validator.md#haneul_system_validator_MAX_VALIDATOR_METADATA_LENGTH">MAX_VALIDATOR_METADATA_LENGTH</a>,
        <a href="../haneul_system/validator.md#haneul_system_validator_EValidatorMetadataExceedingLengthLimit">EValidatorMetadataExceedingLengthLimit</a>,
    );
    <b>let</b> <a href="../haneul_system/validator.md#haneul_system_validator_worker_address">worker_address</a> = <a href="../haneul_system/validator.md#haneul_system_validator_worker_address">worker_address</a>.to_ascii_string().to_string();
    self.<a href="../haneul_system/validator.md#haneul_system_validator_metadata">metadata</a>.<a href="../haneul_system/validator.md#haneul_system_validator_next_epoch_worker_address">next_epoch_worker_address</a> = option::some(<a href="../haneul_system/validator.md#haneul_system_validator_worker_address">worker_address</a>);
    self.<a href="../haneul_system/validator.md#haneul_system_validator_metadata">metadata</a>.validate();
}
</code></pre>



</details>

<a name="haneul_system_validator_update_candidate_worker_address"></a>

## Function `update_candidate_worker_address`

Update worker address of this candidate validator


<pre><code><b>public</b>(package) <b>fun</b> <a href="../haneul_system/validator.md#haneul_system_validator_update_candidate_worker_address">update_candidate_worker_address</a>(self: &<b>mut</b> <a href="../haneul_system/validator.md#haneul_system_validator_Validator">haneul_system::validator::Validator</a>, <a href="../haneul_system/validator.md#haneul_system_validator_worker_address">worker_address</a>: vector&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../haneul_system/validator.md#haneul_system_validator_update_candidate_worker_address">update_candidate_worker_address</a>(
    self: &<b>mut</b> <a href="../haneul_system/validator.md#haneul_system_validator_Validator">Validator</a>,
    <a href="../haneul_system/validator.md#haneul_system_validator_worker_address">worker_address</a>: vector&lt;u8&gt;,
) {
    <b>assert</b>!(self.<a href="../haneul_system/validator.md#haneul_system_validator_is_preactive">is_preactive</a>(), <a href="../haneul_system/validator.md#haneul_system_validator_ENotValidatorCandidate">ENotValidatorCandidate</a>);
    <b>assert</b>!(
        <a href="../haneul_system/validator.md#haneul_system_validator_worker_address">worker_address</a>.length() &lt;= <a href="../haneul_system/validator.md#haneul_system_validator_MAX_VALIDATOR_METADATA_LENGTH">MAX_VALIDATOR_METADATA_LENGTH</a>,
        <a href="../haneul_system/validator.md#haneul_system_validator_EValidatorMetadataExceedingLengthLimit">EValidatorMetadataExceedingLengthLimit</a>,
    );
    <b>let</b> <a href="../haneul_system/validator.md#haneul_system_validator_worker_address">worker_address</a> = <a href="../haneul_system/validator.md#haneul_system_validator_worker_address">worker_address</a>.to_ascii_string().to_string();
    self.<a href="../haneul_system/validator.md#haneul_system_validator_metadata">metadata</a>.<a href="../haneul_system/validator.md#haneul_system_validator_worker_address">worker_address</a> = <a href="../haneul_system/validator.md#haneul_system_validator_worker_address">worker_address</a>;
    self.<a href="../haneul_system/validator.md#haneul_system_validator_metadata">metadata</a>.validate();
}
</code></pre>



</details>

<a name="haneul_system_validator_update_next_epoch_protocol_pubkey"></a>

## Function `update_next_epoch_protocol_pubkey`

Update protocol public key of this validator, taking effects from next epoch


<pre><code><b>public</b>(package) <b>fun</b> <a href="../haneul_system/validator.md#haneul_system_validator_update_next_epoch_protocol_pubkey">update_next_epoch_protocol_pubkey</a>(self: &<b>mut</b> <a href="../haneul_system/validator.md#haneul_system_validator_Validator">haneul_system::validator::Validator</a>, protocol_pubkey: vector&lt;u8&gt;, <a href="../haneul_system/validator.md#haneul_system_validator_proof_of_possession">proof_of_possession</a>: vector&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../haneul_system/validator.md#haneul_system_validator_update_next_epoch_protocol_pubkey">update_next_epoch_protocol_pubkey</a>(
    self: &<b>mut</b> <a href="../haneul_system/validator.md#haneul_system_validator_Validator">Validator</a>,
    protocol_pubkey: vector&lt;u8&gt;,
    <a href="../haneul_system/validator.md#haneul_system_validator_proof_of_possession">proof_of_possession</a>: vector&lt;u8&gt;,
) {
    <b>assert</b>!(protocol_pubkey.length() == 96, <a href="../haneul_system/validator.md#haneul_system_validator_EInvalidProtocolPubKeyLength">EInvalidProtocolPubKeyLength</a>);
    self.<a href="../haneul_system/validator.md#haneul_system_validator_metadata">metadata</a>.<a href="../haneul_system/validator.md#haneul_system_validator_next_epoch_protocol_pubkey_bytes">next_epoch_protocol_pubkey_bytes</a> = option::some(protocol_pubkey);
    self.<a href="../haneul_system/validator.md#haneul_system_validator_metadata">metadata</a>.<a href="../haneul_system/validator.md#haneul_system_validator_next_epoch_proof_of_possession">next_epoch_proof_of_possession</a> = option::some(<a href="../haneul_system/validator.md#haneul_system_validator_proof_of_possession">proof_of_possession</a>);
    self.<a href="../haneul_system/validator.md#haneul_system_validator_metadata">metadata</a>.validate();
}
</code></pre>



</details>

<a name="haneul_system_validator_update_candidate_protocol_pubkey"></a>

## Function `update_candidate_protocol_pubkey`

Update protocol public key of this candidate validator


<pre><code><b>public</b>(package) <b>fun</b> <a href="../haneul_system/validator.md#haneul_system_validator_update_candidate_protocol_pubkey">update_candidate_protocol_pubkey</a>(self: &<b>mut</b> <a href="../haneul_system/validator.md#haneul_system_validator_Validator">haneul_system::validator::Validator</a>, protocol_pubkey: vector&lt;u8&gt;, <a href="../haneul_system/validator.md#haneul_system_validator_proof_of_possession">proof_of_possession</a>: vector&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../haneul_system/validator.md#haneul_system_validator_update_candidate_protocol_pubkey">update_candidate_protocol_pubkey</a>(
    self: &<b>mut</b> <a href="../haneul_system/validator.md#haneul_system_validator_Validator">Validator</a>,
    protocol_pubkey: vector&lt;u8&gt;,
    <a href="../haneul_system/validator.md#haneul_system_validator_proof_of_possession">proof_of_possession</a>: vector&lt;u8&gt;,
) {
    <b>assert</b>!(self.<a href="../haneul_system/validator.md#haneul_system_validator_is_preactive">is_preactive</a>(), <a href="../haneul_system/validator.md#haneul_system_validator_ENotValidatorCandidate">ENotValidatorCandidate</a>);
    <b>assert</b>!(protocol_pubkey.length() == 96, <a href="../haneul_system/validator.md#haneul_system_validator_EInvalidProtocolPubKeyLength">EInvalidProtocolPubKeyLength</a>);
    self.<a href="../haneul_system/validator.md#haneul_system_validator_metadata">metadata</a>.<a href="../haneul_system/validator.md#haneul_system_validator_protocol_pubkey_bytes">protocol_pubkey_bytes</a> = protocol_pubkey;
    self.<a href="../haneul_system/validator.md#haneul_system_validator_metadata">metadata</a>.<a href="../haneul_system/validator.md#haneul_system_validator_proof_of_possession">proof_of_possession</a> = <a href="../haneul_system/validator.md#haneul_system_validator_proof_of_possession">proof_of_possession</a>;
    self.<a href="../haneul_system/validator.md#haneul_system_validator_metadata">metadata</a>.validate();
}
</code></pre>



</details>

<a name="haneul_system_validator_update_next_epoch_network_pubkey"></a>

## Function `update_next_epoch_network_pubkey`

Update network public key of this validator, taking effects from next epoch


<pre><code><b>public</b>(package) <b>fun</b> <a href="../haneul_system/validator.md#haneul_system_validator_update_next_epoch_network_pubkey">update_next_epoch_network_pubkey</a>(self: &<b>mut</b> <a href="../haneul_system/validator.md#haneul_system_validator_Validator">haneul_system::validator::Validator</a>, network_pubkey: vector&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../haneul_system/validator.md#haneul_system_validator_update_next_epoch_network_pubkey">update_next_epoch_network_pubkey</a>(
    self: &<b>mut</b> <a href="../haneul_system/validator.md#haneul_system_validator_Validator">Validator</a>,
    network_pubkey: vector&lt;u8&gt;,
) {
    self.<a href="../haneul_system/validator.md#haneul_system_validator_metadata">metadata</a>.<a href="../haneul_system/validator.md#haneul_system_validator_next_epoch_network_pubkey_bytes">next_epoch_network_pubkey_bytes</a> = option::some(network_pubkey);
    self.<a href="../haneul_system/validator.md#haneul_system_validator_metadata">metadata</a>.validate();
}
</code></pre>



</details>

<a name="haneul_system_validator_update_candidate_network_pubkey"></a>

## Function `update_candidate_network_pubkey`

Update network public key of this candidate validator


<pre><code><b>public</b>(package) <b>fun</b> <a href="../haneul_system/validator.md#haneul_system_validator_update_candidate_network_pubkey">update_candidate_network_pubkey</a>(self: &<b>mut</b> <a href="../haneul_system/validator.md#haneul_system_validator_Validator">haneul_system::validator::Validator</a>, network_pubkey: vector&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../haneul_system/validator.md#haneul_system_validator_update_candidate_network_pubkey">update_candidate_network_pubkey</a>(
    self: &<b>mut</b> <a href="../haneul_system/validator.md#haneul_system_validator_Validator">Validator</a>,
    network_pubkey: vector&lt;u8&gt;,
) {
    <b>assert</b>!(self.<a href="../haneul_system/validator.md#haneul_system_validator_is_preactive">is_preactive</a>(), <a href="../haneul_system/validator.md#haneul_system_validator_ENotValidatorCandidate">ENotValidatorCandidate</a>);
    self.<a href="../haneul_system/validator.md#haneul_system_validator_metadata">metadata</a>.<a href="../haneul_system/validator.md#haneul_system_validator_network_pubkey_bytes">network_pubkey_bytes</a> = network_pubkey;
    self.<a href="../haneul_system/validator.md#haneul_system_validator_metadata">metadata</a>.validate();
}
</code></pre>



</details>

<a name="haneul_system_validator_update_next_epoch_worker_pubkey"></a>

## Function `update_next_epoch_worker_pubkey`

Update Narwhal worker public key of this validator, taking effects from next epoch


<pre><code><b>public</b>(package) <b>fun</b> <a href="../haneul_system/validator.md#haneul_system_validator_update_next_epoch_worker_pubkey">update_next_epoch_worker_pubkey</a>(self: &<b>mut</b> <a href="../haneul_system/validator.md#haneul_system_validator_Validator">haneul_system::validator::Validator</a>, worker_pubkey: vector&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../haneul_system/validator.md#haneul_system_validator_update_next_epoch_worker_pubkey">update_next_epoch_worker_pubkey</a>(
    self: &<b>mut</b> <a href="../haneul_system/validator.md#haneul_system_validator_Validator">Validator</a>,
    worker_pubkey: vector&lt;u8&gt;,
) {
    self.<a href="../haneul_system/validator.md#haneul_system_validator_metadata">metadata</a>.<a href="../haneul_system/validator.md#haneul_system_validator_next_epoch_worker_pubkey_bytes">next_epoch_worker_pubkey_bytes</a> = option::some(worker_pubkey);
    self.<a href="../haneul_system/validator.md#haneul_system_validator_metadata">metadata</a>.validate();
}
</code></pre>



</details>

<a name="haneul_system_validator_update_candidate_worker_pubkey"></a>

## Function `update_candidate_worker_pubkey`

Update Narwhal worker public key of this candidate validator


<pre><code><b>public</b>(package) <b>fun</b> <a href="../haneul_system/validator.md#haneul_system_validator_update_candidate_worker_pubkey">update_candidate_worker_pubkey</a>(self: &<b>mut</b> <a href="../haneul_system/validator.md#haneul_system_validator_Validator">haneul_system::validator::Validator</a>, worker_pubkey: vector&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../haneul_system/validator.md#haneul_system_validator_update_candidate_worker_pubkey">update_candidate_worker_pubkey</a>(
    self: &<b>mut</b> <a href="../haneul_system/validator.md#haneul_system_validator_Validator">Validator</a>,
    worker_pubkey: vector&lt;u8&gt;,
) {
    <b>assert</b>!(self.<a href="../haneul_system/validator.md#haneul_system_validator_is_preactive">is_preactive</a>(), <a href="../haneul_system/validator.md#haneul_system_validator_ENotValidatorCandidate">ENotValidatorCandidate</a>);
    self.<a href="../haneul_system/validator.md#haneul_system_validator_metadata">metadata</a>.<a href="../haneul_system/validator.md#haneul_system_validator_worker_pubkey_bytes">worker_pubkey_bytes</a> = worker_pubkey;
    self.<a href="../haneul_system/validator.md#haneul_system_validator_metadata">metadata</a>.validate();
}
</code></pre>



</details>

<a name="haneul_system_validator_effectuate_staged_metadata"></a>

## Function `effectuate_staged_metadata`

Effectuate all staged next epoch metadata for this validator.
NOTE: this function SHOULD ONLY be called by validator_set when
advancing an epoch.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../haneul_system/validator.md#haneul_system_validator_effectuate_staged_metadata">effectuate_staged_metadata</a>(self: &<b>mut</b> <a href="../haneul_system/validator.md#haneul_system_validator_Validator">haneul_system::validator::Validator</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../haneul_system/validator.md#haneul_system_validator_effectuate_staged_metadata">effectuate_staged_metadata</a>(self: &<b>mut</b> <a href="../haneul_system/validator.md#haneul_system_validator_Validator">Validator</a>) {
    <a href="../haneul_system/validator.md#haneul_system_validator_do_extract">do_extract</a>!(&<b>mut</b> self.<a href="../haneul_system/validator.md#haneul_system_validator_metadata">metadata</a>.next_epoch_net_address, |v| {
        self.<a href="../haneul_system/validator.md#haneul_system_validator_metadata">metadata</a>.net_address = v
    });
    <a href="../haneul_system/validator.md#haneul_system_validator_do_extract">do_extract</a>!(&<b>mut</b> self.<a href="../haneul_system/validator.md#haneul_system_validator_metadata">metadata</a>.<a href="../haneul_system/validator.md#haneul_system_validator_next_epoch_p2p_address">next_epoch_p2p_address</a>, |v| {
        self.<a href="../haneul_system/validator.md#haneul_system_validator_metadata">metadata</a>.<a href="../haneul_system/validator.md#haneul_system_validator_p2p_address">p2p_address</a> = v
    });
    <a href="../haneul_system/validator.md#haneul_system_validator_do_extract">do_extract</a>!(&<b>mut</b> self.<a href="../haneul_system/validator.md#haneul_system_validator_metadata">metadata</a>.<a href="../haneul_system/validator.md#haneul_system_validator_next_epoch_primary_address">next_epoch_primary_address</a>, |v| {
        self.<a href="../haneul_system/validator.md#haneul_system_validator_metadata">metadata</a>.<a href="../haneul_system/validator.md#haneul_system_validator_primary_address">primary_address</a> = v
    });
    <a href="../haneul_system/validator.md#haneul_system_validator_do_extract">do_extract</a>!(&<b>mut</b> self.<a href="../haneul_system/validator.md#haneul_system_validator_metadata">metadata</a>.<a href="../haneul_system/validator.md#haneul_system_validator_next_epoch_worker_address">next_epoch_worker_address</a>, |v| {
        self.<a href="../haneul_system/validator.md#haneul_system_validator_metadata">metadata</a>.<a href="../haneul_system/validator.md#haneul_system_validator_worker_address">worker_address</a> = v
    });
    <a href="../haneul_system/validator.md#haneul_system_validator_do_extract">do_extract</a>!(&<b>mut</b> self.<a href="../haneul_system/validator.md#haneul_system_validator_metadata">metadata</a>.<a href="../haneul_system/validator.md#haneul_system_validator_next_epoch_protocol_pubkey_bytes">next_epoch_protocol_pubkey_bytes</a>, |v| {
        self.<a href="../haneul_system/validator.md#haneul_system_validator_metadata">metadata</a>.<a href="../haneul_system/validator.md#haneul_system_validator_protocol_pubkey_bytes">protocol_pubkey_bytes</a> = v;
        self.<a href="../haneul_system/validator.md#haneul_system_validator_metadata">metadata</a>.<a href="../haneul_system/validator.md#haneul_system_validator_proof_of_possession">proof_of_possession</a> = self.<a href="../haneul_system/validator.md#haneul_system_validator_metadata">metadata</a>.<a href="../haneul_system/validator.md#haneul_system_validator_next_epoch_proof_of_possession">next_epoch_proof_of_possession</a>.extract();
    });
    <a href="../haneul_system/validator.md#haneul_system_validator_do_extract">do_extract</a>!(&<b>mut</b> self.<a href="../haneul_system/validator.md#haneul_system_validator_metadata">metadata</a>.<a href="../haneul_system/validator.md#haneul_system_validator_next_epoch_network_pubkey_bytes">next_epoch_network_pubkey_bytes</a>, |v| {
        self.<a href="../haneul_system/validator.md#haneul_system_validator_metadata">metadata</a>.<a href="../haneul_system/validator.md#haneul_system_validator_network_pubkey_bytes">network_pubkey_bytes</a> = v
    });
    <a href="../haneul_system/validator.md#haneul_system_validator_do_extract">do_extract</a>!(&<b>mut</b> self.<a href="../haneul_system/validator.md#haneul_system_validator_metadata">metadata</a>.<a href="../haneul_system/validator.md#haneul_system_validator_next_epoch_worker_pubkey_bytes">next_epoch_worker_pubkey_bytes</a>, |v| {
        self.<a href="../haneul_system/validator.md#haneul_system_validator_metadata">metadata</a>.<a href="../haneul_system/validator.md#haneul_system_validator_worker_pubkey_bytes">worker_pubkey_bytes</a> = v
    });
}
</code></pre>



</details>

<a name="haneul_system_validator_do_extract"></a>

## Macro function `do_extract`

Helper macro which extracts the value from <code>Some</code> and applies <code>$f</code> to it.


<pre><code><b>macro</b> <b>fun</b> <a href="../haneul_system/validator.md#haneul_system_validator_do_extract">do_extract</a>&lt;$T&gt;($o: &<b>mut</b> <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;$T&gt;, $f: |$T| -&gt; ())
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>macro</b> <b>fun</b> <a href="../haneul_system/validator.md#haneul_system_validator_do_extract">do_extract</a>&lt;$T&gt;($o: &<b>mut</b> Option&lt;$T&gt;, $f: |$T|) {
    <b>let</b> o = $o;
    <b>if</b> (o.is_some()) {
        $f(o.extract());
    }
}
</code></pre>



</details>

<a name="haneul_system_validator_validate_metadata"></a>

## Function `validate_metadata`

Aborts if validator metadata is valid


<pre><code><b>public</b> <b>fun</b> <a href="../haneul_system/validator.md#haneul_system_validator_validate_metadata">validate_metadata</a>(<a href="../haneul_system/validator.md#haneul_system_validator_metadata">metadata</a>: &<a href="../haneul_system/validator.md#haneul_system_validator_ValidatorMetadata">haneul_system::validator::ValidatorMetadata</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul_system/validator.md#haneul_system_validator_validate_metadata">validate_metadata</a>(<a href="../haneul_system/validator.md#haneul_system_validator_metadata">metadata</a>: &<a href="../haneul_system/validator.md#haneul_system_validator_ValidatorMetadata">ValidatorMetadata</a>) {
    <a href="../haneul_system/validator.md#haneul_system_validator_validate_metadata_bcs">validate_metadata_bcs</a>(bcs::to_bytes(<a href="../haneul_system/validator.md#haneul_system_validator_metadata">metadata</a>));
}
</code></pre>



</details>

<a name="haneul_system_validator_validate_metadata_bcs"></a>

## Function `validate_metadata_bcs`



<pre><code><b>public</b> <b>fun</b> <a href="../haneul_system/validator.md#haneul_system_validator_validate_metadata_bcs">validate_metadata_bcs</a>(<a href="../haneul_system/validator.md#haneul_system_validator_metadata">metadata</a>: vector&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>native</b> <b>fun</b> <a href="../haneul_system/validator.md#haneul_system_validator_validate_metadata_bcs">validate_metadata_bcs</a>(<a href="../haneul_system/validator.md#haneul_system_validator_metadata">metadata</a>: vector&lt;u8&gt;);
</code></pre>



</details>

<a name="haneul_system_validator_get_staking_pool_ref"></a>

## Function `get_staking_pool_ref`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../haneul_system/validator.md#haneul_system_validator_get_staking_pool_ref">get_staking_pool_ref</a>(self: &<a href="../haneul_system/validator.md#haneul_system_validator_Validator">haneul_system::validator::Validator</a>): &<a href="../haneul_system/staking_pool.md#haneul_system_staking_pool_StakingPool">haneul_system::staking_pool::StakingPool</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../haneul_system/validator.md#haneul_system_validator_get_staking_pool_ref">get_staking_pool_ref</a>(self: &<a href="../haneul_system/validator.md#haneul_system_validator_Validator">Validator</a>): &StakingPool {
    &self.<a href="../haneul_system/staking_pool.md#haneul_system_staking_pool">staking_pool</a>
}
</code></pre>



</details>

<a name="haneul_system_validator_new_from_metadata"></a>

## Function `new_from_metadata`

Create a new validator from the given <code><a href="../haneul_system/validator.md#haneul_system_validator_ValidatorMetadata">ValidatorMetadata</a></code>, called by both <code><a href="../haneul_system/validator.md#haneul_system_validator_new">new</a></code> and <code>new_for_testing</code>.


<pre><code><b>fun</b> <a href="../haneul_system/validator.md#haneul_system_validator_new_from_metadata">new_from_metadata</a>(<a href="../haneul_system/validator.md#haneul_system_validator_metadata">metadata</a>: <a href="../haneul_system/validator.md#haneul_system_validator_ValidatorMetadata">haneul_system::validator::ValidatorMetadata</a>, <a href="../haneul_system/validator.md#haneul_system_validator_gas_price">gas_price</a>: u64, <a href="../haneul_system/validator.md#haneul_system_validator_commission_rate">commission_rate</a>: u64, ctx: &<b>mut</b> <a href="../haneul/tx_context.md#haneul_tx_context_TxContext">haneul::tx_context::TxContext</a>): <a href="../haneul_system/validator.md#haneul_system_validator_Validator">haneul_system::validator::Validator</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../haneul_system/validator.md#haneul_system_validator_new_from_metadata">new_from_metadata</a>(
    <a href="../haneul_system/validator.md#haneul_system_validator_metadata">metadata</a>: <a href="../haneul_system/validator.md#haneul_system_validator_ValidatorMetadata">ValidatorMetadata</a>,
    <a href="../haneul_system/validator.md#haneul_system_validator_gas_price">gas_price</a>: u64,
    <a href="../haneul_system/validator.md#haneul_system_validator_commission_rate">commission_rate</a>: u64,
    ctx: &<b>mut</b> TxContext,
): <a href="../haneul_system/validator.md#haneul_system_validator_Validator">Validator</a> {
    <b>let</b> <a href="../haneul_system/validator.md#haneul_system_validator_haneul_address">haneul_address</a> = <a href="../haneul_system/validator.md#haneul_system_validator_metadata">metadata</a>.<a href="../haneul_system/validator.md#haneul_system_validator_haneul_address">haneul_address</a>;
    <b>let</b> <a href="../haneul_system/staking_pool.md#haneul_system_staking_pool">staking_pool</a> = <a href="../haneul_system/staking_pool.md#haneul_system_staking_pool_new">staking_pool::new</a>(ctx);
    <b>let</b> <a href="../haneul_system/validator.md#haneul_system_validator_operation_cap_id">operation_cap_id</a> = <a href="../haneul_system/validator_cap.md#haneul_system_validator_cap_new_unverified_validator_operation_cap_and_transfer">validator_cap::new_unverified_validator_operation_cap_and_transfer</a>(
        <a href="../haneul_system/validator.md#haneul_system_validator_haneul_address">haneul_address</a>,
        ctx,
    );
    <a href="../haneul_system/validator.md#haneul_system_validator_Validator">Validator</a> {
        <a href="../haneul_system/validator.md#haneul_system_validator_metadata">metadata</a>,
        // Initialize the voting power to be 0.
        // At the epoch change where this <a href="../haneul_system/validator.md#haneul_system_validator">validator</a> is actually added to the
        // active <a href="../haneul_system/validator.md#haneul_system_validator">validator</a> set, the voting power will be updated accordingly.
        <a href="../haneul_system/voting_power.md#haneul_system_voting_power">voting_power</a>: 0,
        <a href="../haneul_system/validator.md#haneul_system_validator_operation_cap_id">operation_cap_id</a>,
        <a href="../haneul_system/validator.md#haneul_system_validator_gas_price">gas_price</a>,
        <a href="../haneul_system/staking_pool.md#haneul_system_staking_pool">staking_pool</a>,
        <a href="../haneul_system/validator.md#haneul_system_validator_commission_rate">commission_rate</a>,
        next_epoch_stake: 0,
        <a href="../haneul_system/validator.md#haneul_system_validator_next_epoch_gas_price">next_epoch_gas_price</a>: <a href="../haneul_system/validator.md#haneul_system_validator_gas_price">gas_price</a>,
        next_epoch_commission_rate: <a href="../haneul_system/validator.md#haneul_system_validator_commission_rate">commission_rate</a>,
        extra_fields: bag::new(ctx),
    }
}
</code></pre>



</details>
