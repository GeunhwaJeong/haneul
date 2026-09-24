---
title: Module `haneul::balance`
---

A storable handler for Balances in general. Is used in the <code>Coin</code>
module to allow balance operations and can be used to implement
custom coins with <code><a href="../haneul/balance.md#haneul_balance_Supply">Supply</a></code> and <code><a href="../haneul/balance.md#haneul_balance_Balance">Balance</a></code>s.


-  [Struct `Supply`](#haneul_balance_Supply)
-  [Struct `Balance`](#haneul_balance_Balance)
-  [Constants](#@Constants_0)
-  [Function `value`](#haneul_balance_value)
-  [Function `supply_value`](#haneul_balance_supply_value)
-  [Function `create_supply`](#haneul_balance_create_supply)
-  [Function `increase_supply`](#haneul_balance_increase_supply)
-  [Function `decrease_supply`](#haneul_balance_decrease_supply)
-  [Function `zero`](#haneul_balance_zero)
-  [Function `join`](#haneul_balance_join)
-  [Function `split`](#haneul_balance_split)
-  [Function `withdraw_all`](#haneul_balance_withdraw_all)
-  [Function `destroy_zero`](#haneul_balance_destroy_zero)
-  [Function `send_funds`](#haneul_balance_send_funds)
-  [Function `redeem_funds`](#haneul_balance_redeem_funds)
-  [Function `withdraw_funds_from_object`](#haneul_balance_withdraw_funds_from_object)
-  [Function `settled_funds_value`](#haneul_balance_settled_funds_value)
-  [Function `create_supply_internal`](#haneul_balance_create_supply_internal)
-  [Function `create_staking_rewards`](#haneul_balance_create_staking_rewards)
-  [Function `destroy_storage_rebates`](#haneul_balance_destroy_storage_rebates)
-  [Function `destroy_supply`](#haneul_balance_destroy_supply)


<pre><code><b>use</b> <a href="../haneul/accumulator.md#haneul_accumulator">haneul::accumulator</a>;
<b>use</b> <a href="../haneul/address.md#haneul_address">haneul::address</a>;
<b>use</b> <a href="../haneul/dynamic_field.md#haneul_dynamic_field">haneul::dynamic_field</a>;
<b>use</b> <a href="../haneul/funds_accumulator.md#haneul_funds_accumulator">haneul::funds_accumulator</a>;
<b>use</b> <a href="../haneul/hex.md#haneul_hex">haneul::hex</a>;
<b>use</b> <a href="../haneul/object.md#haneul_object">haneul::object</a>;
<b>use</b> <a href="../haneul/party.md#haneul_party">haneul::party</a>;
<b>use</b> <a href="../haneul/protocol_config.md#haneul_protocol_config">haneul::protocol_config</a>;
<b>use</b> <a href="../haneul/transfer.md#haneul_transfer">haneul::transfer</a>;
<b>use</b> <a href="../haneul/tx_context.md#haneul_tx_context">haneul::tx_context</a>;
<b>use</b> <a href="../haneul/vec_map.md#haneul_vec_map">haneul::vec_map</a>;
<b>use</b> <a href="../std/address.md#std_address">std::address</a>;
<b>use</b> <a href="../std/ascii.md#std_ascii">std::ascii</a>;
<b>use</b> <a href="../std/bcs.md#std_bcs">std::bcs</a>;
<b>use</b> <a href="../std/internal.md#std_internal">std::internal</a>;
<b>use</b> <a href="../std/option.md#std_option">std::option</a>;
<b>use</b> <a href="../std/string.md#std_string">std::string</a>;
<b>use</b> <a href="../std/type_name.md#std_type_name">std::type_name</a>;
<b>use</b> <a href="../std/u128.md#std_u128">std::u128</a>;
<b>use</b> <a href="../std/vector.md#std_vector">std::vector</a>;
</code></pre>



<a name="haneul_balance_Supply"></a>

## Struct `Supply`

A Supply of T. Used for minting and burning.
Wrapped into a <code>TreasuryCap</code> in the <code>Coin</code> module.


<pre><code><b>public</b> <b>struct</b> <a href="../haneul/balance.md#haneul_balance_Supply">Supply</a>&lt;<b>phantom</b> T&gt; <b>has</b> store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../haneul/balance.md#haneul_balance_value">value</a>: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="haneul_balance_Balance"></a>

## Struct `Balance`

Storable balance - an inner struct of a Coin type.
Can be used to store coins which don't need the key ability.


<pre><code><b>public</b> <b>struct</b> <a href="../haneul/balance.md#haneul_balance_Balance">Balance</a>&lt;<b>phantom</b> T&gt; <b>has</b> store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../haneul/balance.md#haneul_balance_value">value</a>: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="haneul_balance_ENonZero"></a>

For when trying to destroy a non-zero balance.


<pre><code><b>const</b> <a href="../haneul/balance.md#haneul_balance_ENonZero">ENonZero</a>: u64 = 0;
</code></pre>



<a name="haneul_balance_EOverflow"></a>

For when an overflow is happening on Supply operations.


<pre><code><b>const</b> <a href="../haneul/balance.md#haneul_balance_EOverflow">EOverflow</a>: u64 = 1;
</code></pre>



<a name="haneul_balance_ENotEnough"></a>

For when trying to withdraw more than there is.


<pre><code><b>const</b> <a href="../haneul/balance.md#haneul_balance_ENotEnough">ENotEnough</a>: u64 = 2;
</code></pre>



<a name="haneul_balance_ENotSystemAddress"></a>

Sender is not @0x0 the system address.


<pre><code><b>const</b> <a href="../haneul/balance.md#haneul_balance_ENotSystemAddress">ENotSystemAddress</a>: u64 = 3;
</code></pre>



<a name="haneul_balance_ENotHANEUL"></a>

System operation performed for a coin other than HANEUL


<pre><code><b>const</b> <a href="../haneul/balance.md#haneul_balance_ENotHANEUL">ENotHANEUL</a>: u64 = 4;
</code></pre>



<a name="haneul_balance_HANEUL_TYPE_NAME"></a>



<pre><code><b>const</b> <a href="../haneul/balance.md#haneul_balance_HANEUL_TYPE_NAME">HANEUL_TYPE_NAME</a>: vector&lt;u8&gt; = vector[48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 50, 58, 58, 104, 97, 110, 101, 117, 108, 58, 58, 72, 65, 78, 69, 85, 76];
</code></pre>



<a name="haneul_balance_value"></a>

## Function `value`

Get the amount stored in a <code><a href="../haneul/balance.md#haneul_balance_Balance">Balance</a></code>.


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/balance.md#haneul_balance_value">value</a>&lt;T&gt;(self: &<a href="../haneul/balance.md#haneul_balance_Balance">haneul::balance::Balance</a>&lt;T&gt;): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/balance.md#haneul_balance_value">value</a>&lt;T&gt;(self: &<a href="../haneul/balance.md#haneul_balance_Balance">Balance</a>&lt;T&gt;): u64 {
    self.<a href="../haneul/balance.md#haneul_balance_value">value</a>
}
</code></pre>



</details>

<a name="haneul_balance_supply_value"></a>

## Function `supply_value`

Get the <code><a href="../haneul/balance.md#haneul_balance_Supply">Supply</a></code> value.


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/balance.md#haneul_balance_supply_value">supply_value</a>&lt;T&gt;(supply: &<a href="../haneul/balance.md#haneul_balance_Supply">haneul::balance::Supply</a>&lt;T&gt;): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/balance.md#haneul_balance_supply_value">supply_value</a>&lt;T&gt;(supply: &<a href="../haneul/balance.md#haneul_balance_Supply">Supply</a>&lt;T&gt;): u64 {
    supply.<a href="../haneul/balance.md#haneul_balance_value">value</a>
}
</code></pre>



</details>

<a name="haneul_balance_create_supply"></a>

## Function `create_supply`

Create a new supply for type T.


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/balance.md#haneul_balance_create_supply">create_supply</a>&lt;T: drop&gt;(_: T): <a href="../haneul/balance.md#haneul_balance_Supply">haneul::balance::Supply</a>&lt;T&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/balance.md#haneul_balance_create_supply">create_supply</a>&lt;T: drop&gt;(_: T): <a href="../haneul/balance.md#haneul_balance_Supply">Supply</a>&lt;T&gt; {
    <a href="../haneul/balance.md#haneul_balance_Supply">Supply</a> { <a href="../haneul/balance.md#haneul_balance_value">value</a>: 0 }
}
</code></pre>



</details>

<a name="haneul_balance_increase_supply"></a>

## Function `increase_supply`

Increase supply by <code><a href="../haneul/balance.md#haneul_balance_value">value</a></code> and create a new <code><a href="../haneul/balance.md#haneul_balance_Balance">Balance</a>&lt;T&gt;</code> with this value.


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/balance.md#haneul_balance_increase_supply">increase_supply</a>&lt;T&gt;(self: &<b>mut</b> <a href="../haneul/balance.md#haneul_balance_Supply">haneul::balance::Supply</a>&lt;T&gt;, <a href="../haneul/balance.md#haneul_balance_value">value</a>: u64): <a href="../haneul/balance.md#haneul_balance_Balance">haneul::balance::Balance</a>&lt;T&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/balance.md#haneul_balance_increase_supply">increase_supply</a>&lt;T&gt;(self: &<b>mut</b> <a href="../haneul/balance.md#haneul_balance_Supply">Supply</a>&lt;T&gt;, <a href="../haneul/balance.md#haneul_balance_value">value</a>: u64): <a href="../haneul/balance.md#haneul_balance_Balance">Balance</a>&lt;T&gt; {
    <b>assert</b>!(<a href="../haneul/balance.md#haneul_balance_value">value</a> &lt;= (<a href="../std/u64.md#std_u64_max_value">std::u64::max_value</a>!() - self.<a href="../haneul/balance.md#haneul_balance_value">value</a>), <a href="../haneul/balance.md#haneul_balance_EOverflow">EOverflow</a>);
    self.<a href="../haneul/balance.md#haneul_balance_value">value</a> = self.<a href="../haneul/balance.md#haneul_balance_value">value</a> + <a href="../haneul/balance.md#haneul_balance_value">value</a>;
    <a href="../haneul/balance.md#haneul_balance_Balance">Balance</a> { <a href="../haneul/balance.md#haneul_balance_value">value</a> }
}
</code></pre>



</details>

<a name="haneul_balance_decrease_supply"></a>

## Function `decrease_supply`

Burn a Balance<T> and decrease Supply<T>.


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/balance.md#haneul_balance_decrease_supply">decrease_supply</a>&lt;T&gt;(self: &<b>mut</b> <a href="../haneul/balance.md#haneul_balance_Supply">haneul::balance::Supply</a>&lt;T&gt;, <a href="../haneul/balance.md#haneul_balance">balance</a>: <a href="../haneul/balance.md#haneul_balance_Balance">haneul::balance::Balance</a>&lt;T&gt;): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/balance.md#haneul_balance_decrease_supply">decrease_supply</a>&lt;T&gt;(self: &<b>mut</b> <a href="../haneul/balance.md#haneul_balance_Supply">Supply</a>&lt;T&gt;, <a href="../haneul/balance.md#haneul_balance">balance</a>: <a href="../haneul/balance.md#haneul_balance_Balance">Balance</a>&lt;T&gt;): u64 {
    <b>let</b> <a href="../haneul/balance.md#haneul_balance_Balance">Balance</a> { <a href="../haneul/balance.md#haneul_balance_value">value</a> } = <a href="../haneul/balance.md#haneul_balance">balance</a>;
    <b>assert</b>!(self.<a href="../haneul/balance.md#haneul_balance_value">value</a> &gt;= <a href="../haneul/balance.md#haneul_balance_value">value</a>, <a href="../haneul/balance.md#haneul_balance_EOverflow">EOverflow</a>);
    self.<a href="../haneul/balance.md#haneul_balance_value">value</a> = self.<a href="../haneul/balance.md#haneul_balance_value">value</a> - <a href="../haneul/balance.md#haneul_balance_value">value</a>;
    <a href="../haneul/balance.md#haneul_balance_value">value</a>
}
</code></pre>



</details>

<a name="haneul_balance_zero"></a>

## Function `zero`

Create a zero <code><a href="../haneul/balance.md#haneul_balance_Balance">Balance</a></code> for type <code>T</code>.


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/balance.md#haneul_balance_zero">zero</a>&lt;T&gt;(): <a href="../haneul/balance.md#haneul_balance_Balance">haneul::balance::Balance</a>&lt;T&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/balance.md#haneul_balance_zero">zero</a>&lt;T&gt;(): <a href="../haneul/balance.md#haneul_balance_Balance">Balance</a>&lt;T&gt; {
    <a href="../haneul/balance.md#haneul_balance_Balance">Balance</a> { <a href="../haneul/balance.md#haneul_balance_value">value</a>: 0 }
}
</code></pre>



</details>

<a name="haneul_balance_join"></a>

## Function `join`

Join two balances together.


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/balance.md#haneul_balance_join">join</a>&lt;T&gt;(self: &<b>mut</b> <a href="../haneul/balance.md#haneul_balance_Balance">haneul::balance::Balance</a>&lt;T&gt;, <a href="../haneul/balance.md#haneul_balance">balance</a>: <a href="../haneul/balance.md#haneul_balance_Balance">haneul::balance::Balance</a>&lt;T&gt;): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/balance.md#haneul_balance_join">join</a>&lt;T&gt;(self: &<b>mut</b> <a href="../haneul/balance.md#haneul_balance_Balance">Balance</a>&lt;T&gt;, <a href="../haneul/balance.md#haneul_balance">balance</a>: <a href="../haneul/balance.md#haneul_balance_Balance">Balance</a>&lt;T&gt;): u64 {
    <b>let</b> <a href="../haneul/balance.md#haneul_balance_Balance">Balance</a> { <a href="../haneul/balance.md#haneul_balance_value">value</a> } = <a href="../haneul/balance.md#haneul_balance">balance</a>;
    self.<a href="../haneul/balance.md#haneul_balance_value">value</a> = self.<a href="../haneul/balance.md#haneul_balance_value">value</a> + <a href="../haneul/balance.md#haneul_balance_value">value</a>;
    self.<a href="../haneul/balance.md#haneul_balance_value">value</a>
}
</code></pre>



</details>

<a name="haneul_balance_split"></a>

## Function `split`

Split a <code><a href="../haneul/balance.md#haneul_balance_Balance">Balance</a></code> and take a sub balance from it.


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/balance.md#haneul_balance_split">split</a>&lt;T&gt;(self: &<b>mut</b> <a href="../haneul/balance.md#haneul_balance_Balance">haneul::balance::Balance</a>&lt;T&gt;, <a href="../haneul/balance.md#haneul_balance_value">value</a>: u64): <a href="../haneul/balance.md#haneul_balance_Balance">haneul::balance::Balance</a>&lt;T&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/balance.md#haneul_balance_split">split</a>&lt;T&gt;(self: &<b>mut</b> <a href="../haneul/balance.md#haneul_balance_Balance">Balance</a>&lt;T&gt;, <a href="../haneul/balance.md#haneul_balance_value">value</a>: u64): <a href="../haneul/balance.md#haneul_balance_Balance">Balance</a>&lt;T&gt; {
    <b>assert</b>!(self.<a href="../haneul/balance.md#haneul_balance_value">value</a> &gt;= <a href="../haneul/balance.md#haneul_balance_value">value</a>, <a href="../haneul/balance.md#haneul_balance_ENotEnough">ENotEnough</a>);
    self.<a href="../haneul/balance.md#haneul_balance_value">value</a> = self.<a href="../haneul/balance.md#haneul_balance_value">value</a> - <a href="../haneul/balance.md#haneul_balance_value">value</a>;
    <a href="../haneul/balance.md#haneul_balance_Balance">Balance</a> { <a href="../haneul/balance.md#haneul_balance_value">value</a> }
}
</code></pre>



</details>

<a name="haneul_balance_withdraw_all"></a>

## Function `withdraw_all`

Withdraw all balance. After this the remaining balance must be 0.


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/balance.md#haneul_balance_withdraw_all">withdraw_all</a>&lt;T&gt;(self: &<b>mut</b> <a href="../haneul/balance.md#haneul_balance_Balance">haneul::balance::Balance</a>&lt;T&gt;): <a href="../haneul/balance.md#haneul_balance_Balance">haneul::balance::Balance</a>&lt;T&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/balance.md#haneul_balance_withdraw_all">withdraw_all</a>&lt;T&gt;(self: &<b>mut</b> <a href="../haneul/balance.md#haneul_balance_Balance">Balance</a>&lt;T&gt;): <a href="../haneul/balance.md#haneul_balance_Balance">Balance</a>&lt;T&gt; {
    <b>let</b> <a href="../haneul/balance.md#haneul_balance_value">value</a> = self.<a href="../haneul/balance.md#haneul_balance_value">value</a>;
    <a href="../haneul/balance.md#haneul_balance_split">split</a>(self, <a href="../haneul/balance.md#haneul_balance_value">value</a>)
}
</code></pre>



</details>

<a name="haneul_balance_destroy_zero"></a>

## Function `destroy_zero`

Destroy a zero <code><a href="../haneul/balance.md#haneul_balance_Balance">Balance</a></code>.


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/balance.md#haneul_balance_destroy_zero">destroy_zero</a>&lt;T&gt;(<a href="../haneul/balance.md#haneul_balance">balance</a>: <a href="../haneul/balance.md#haneul_balance_Balance">haneul::balance::Balance</a>&lt;T&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/balance.md#haneul_balance_destroy_zero">destroy_zero</a>&lt;T&gt;(<a href="../haneul/balance.md#haneul_balance">balance</a>: <a href="../haneul/balance.md#haneul_balance_Balance">Balance</a>&lt;T&gt;) {
    <b>assert</b>!(<a href="../haneul/balance.md#haneul_balance">balance</a>.<a href="../haneul/balance.md#haneul_balance_value">value</a> == 0, <a href="../haneul/balance.md#haneul_balance_ENonZero">ENonZero</a>);
    <b>let</b> <a href="../haneul/balance.md#haneul_balance_Balance">Balance</a> { <a href="../haneul/balance.md#haneul_balance_value">value</a>: _ } = <a href="../haneul/balance.md#haneul_balance">balance</a>;
}
</code></pre>



</details>

<a name="haneul_balance_send_funds"></a>

## Function `send_funds`

Send a <code><a href="../haneul/balance.md#haneul_balance_Balance">Balance</a></code> to an address's funds accumulator.


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/balance.md#haneul_balance_send_funds">send_funds</a>&lt;T&gt;(<a href="../haneul/balance.md#haneul_balance">balance</a>: <a href="../haneul/balance.md#haneul_balance_Balance">haneul::balance::Balance</a>&lt;T&gt;, recipient: <b>address</b>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/balance.md#haneul_balance_send_funds">send_funds</a>&lt;T&gt;(<a href="../haneul/balance.md#haneul_balance">balance</a>: <a href="../haneul/balance.md#haneul_balance_Balance">Balance</a>&lt;T&gt;, recipient: <b>address</b>) {
    <a href="../haneul/funds_accumulator.md#haneul_funds_accumulator_add_impl">haneul::funds_accumulator::add_impl</a>(<a href="../haneul/balance.md#haneul_balance">balance</a>, recipient);
}
</code></pre>



</details>

<a name="haneul_balance_redeem_funds"></a>

## Function `redeem_funds`

Redeem a <code>Withdrawal&lt;<a href="../haneul/balance.md#haneul_balance_Balance">Balance</a>&lt;T&gt;&gt;</code> to get the underlying <code><a href="../haneul/balance.md#haneul_balance_Balance">Balance</a>&lt;T&gt;</code> from an address's funds
accumulator.


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/balance.md#haneul_balance_redeem_funds">redeem_funds</a>&lt;T&gt;(withdrawal: <a href="../haneul/funds_accumulator.md#haneul_funds_accumulator_Withdrawal">haneul::funds_accumulator::Withdrawal</a>&lt;<a href="../haneul/balance.md#haneul_balance_Balance">haneul::balance::Balance</a>&lt;T&gt;&gt;): <a href="../haneul/balance.md#haneul_balance_Balance">haneul::balance::Balance</a>&lt;T&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/balance.md#haneul_balance_redeem_funds">redeem_funds</a>&lt;T&gt;(withdrawal: <a href="../haneul/funds_accumulator.md#haneul_funds_accumulator_Withdrawal">haneul::funds_accumulator::Withdrawal</a>&lt;<a href="../haneul/balance.md#haneul_balance_Balance">Balance</a>&lt;T&gt;&gt;): <a href="../haneul/balance.md#haneul_balance_Balance">Balance</a>&lt;T&gt; {
    withdrawal.redeem(internal::permit())
}
</code></pre>



</details>

<a name="haneul_balance_withdraw_funds_from_object"></a>

## Function `withdraw_funds_from_object`

Create a <code>Withdrawal&lt;<a href="../haneul/balance.md#haneul_balance_Balance">Balance</a>&lt;T&gt;&gt;</code> from an object to withdraw funds from it.
Aborts if <code><a href="../haneul/balance.md#haneul_balance_value">value</a></code> exceeds the funds currently available to <code>obj</code>.


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/balance.md#haneul_balance_withdraw_funds_from_object">withdraw_funds_from_object</a>&lt;T&gt;(obj: &<b>mut</b> <a href="../haneul/object.md#haneul_object_UID">haneul::object::UID</a>, <a href="../haneul/balance.md#haneul_balance_value">value</a>: u64): <a href="../haneul/funds_accumulator.md#haneul_funds_accumulator_Withdrawal">haneul::funds_accumulator::Withdrawal</a>&lt;<a href="../haneul/balance.md#haneul_balance_Balance">haneul::balance::Balance</a>&lt;T&gt;&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/balance.md#haneul_balance_withdraw_funds_from_object">withdraw_funds_from_object</a>&lt;T&gt;(obj: &<b>mut</b> UID, <a href="../haneul/balance.md#haneul_balance_value">value</a>: u64): Withdrawal&lt;<a href="../haneul/balance.md#haneul_balance_Balance">Balance</a>&lt;T&gt;&gt; {
    <a href="../haneul/funds_accumulator.md#haneul_funds_accumulator_withdraw_from_object">haneul::funds_accumulator::withdraw_from_object</a>(obj, <a href="../haneul/balance.md#haneul_balance_value">value</a> <b>as</b> u256)
}
</code></pre>



</details>

<a name="haneul_balance_settled_funds_value"></a>

## Function `settled_funds_value`

Read the value of the funds of type T owned by <code><b>address</b></code> as of the beginning of
the current consensus commit. Can read either address-owned or object-owned balances.


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/balance.md#haneul_balance_settled_funds_value">settled_funds_value</a>&lt;T&gt;(root: &<a href="../haneul/accumulator.md#haneul_accumulator_AccumulatorRoot">haneul::accumulator::AccumulatorRoot</a>, <b>address</b>: <b>address</b>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/balance.md#haneul_balance_settled_funds_value">settled_funds_value</a>&lt;T&gt;(root: &<a href="../haneul/accumulator.md#haneul_accumulator_AccumulatorRoot">haneul::accumulator::AccumulatorRoot</a>, <b>address</b>: <b>address</b>): u64 {
    <b>if</b> (!root.u128_exists&lt;<a href="../haneul/balance.md#haneul_balance_Balance">Balance</a>&lt;T&gt;&gt;(<b>address</b>)) {
        <b>return</b> 0
    };
    <b>let</b> val: u128 = root.u128_read&lt;<a href="../haneul/balance.md#haneul_balance_Balance">Balance</a>&lt;T&gt;&gt;(<b>address</b>);
    <b>let</b> val = <a href="../std/u128.md#std_u128_min">std::u128::min</a>(<a href="../std/u64.md#std_u64_max_value">std::u64::max_value</a>!() <b>as</b> u128, val);
    val <b>as</b> u64
}
</code></pre>



</details>

<a name="haneul_balance_create_supply_internal"></a>

## Function `create_supply_internal`



<pre><code><b>public</b>(<a href="../haneul/package.md#haneul_package">package</a>) <b>fun</b> <a href="../haneul/balance.md#haneul_balance_create_supply_internal">create_supply_internal</a>&lt;T&gt;(): <a href="../haneul/balance.md#haneul_balance_Supply">haneul::balance::Supply</a>&lt;T&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../haneul/package.md#haneul_package">package</a>) <b>fun</b> <a href="../haneul/balance.md#haneul_balance_create_supply_internal">create_supply_internal</a>&lt;T&gt;(): <a href="../haneul/balance.md#haneul_balance_Supply">Supply</a>&lt;T&gt; {
    <a href="../haneul/balance.md#haneul_balance_Supply">Supply</a> { <a href="../haneul/balance.md#haneul_balance_value">value</a>: 0 }
}
</code></pre>



</details>

<a name="haneul_balance_create_staking_rewards"></a>

## Function `create_staking_rewards`

CAUTION: this function creates a <code><a href="../haneul/balance.md#haneul_balance_Balance">Balance</a></code> without increasing the supply.
It should only be called by the epoch change system txn to create staking rewards,
and nowhere else.


<pre><code><b>fun</b> <a href="../haneul/balance.md#haneul_balance_create_staking_rewards">create_staking_rewards</a>&lt;T&gt;(<a href="../haneul/balance.md#haneul_balance_value">value</a>: u64, ctx: &<a href="../haneul/tx_context.md#haneul_tx_context_TxContext">haneul::tx_context::TxContext</a>): <a href="../haneul/balance.md#haneul_balance_Balance">haneul::balance::Balance</a>&lt;T&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../haneul/balance.md#haneul_balance_create_staking_rewards">create_staking_rewards</a>&lt;T&gt;(<a href="../haneul/balance.md#haneul_balance_value">value</a>: u64, ctx: &TxContext): <a href="../haneul/balance.md#haneul_balance_Balance">Balance</a>&lt;T&gt; {
    <b>assert</b>!(ctx.sender() == @0x0, <a href="../haneul/balance.md#haneul_balance_ENotSystemAddress">ENotSystemAddress</a>);
    <b>assert</b>!(
        <a href="../std/type_name.md#std_type_name_with_defining_ids">std::type_name::with_defining_ids</a>&lt;T&gt;().into_string().into_bytes() == <a href="../haneul/balance.md#haneul_balance_HANEUL_TYPE_NAME">HANEUL_TYPE_NAME</a>,
        <a href="../haneul/balance.md#haneul_balance_ENotHANEUL">ENotHANEUL</a>,
    );
    <a href="../haneul/balance.md#haneul_balance_Balance">Balance</a> { <a href="../haneul/balance.md#haneul_balance_value">value</a> }
}
</code></pre>



</details>

<a name="haneul_balance_destroy_storage_rebates"></a>

## Function `destroy_storage_rebates`

CAUTION: this function destroys a <code><a href="../haneul/balance.md#haneul_balance_Balance">Balance</a></code> without decreasing the supply.
It should only be called by the epoch change system txn to destroy storage rebates,
and nowhere else.


<pre><code><b>fun</b> <a href="../haneul/balance.md#haneul_balance_destroy_storage_rebates">destroy_storage_rebates</a>&lt;T&gt;(self: <a href="../haneul/balance.md#haneul_balance_Balance">haneul::balance::Balance</a>&lt;T&gt;, ctx: &<a href="../haneul/tx_context.md#haneul_tx_context_TxContext">haneul::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../haneul/balance.md#haneul_balance_destroy_storage_rebates">destroy_storage_rebates</a>&lt;T&gt;(self: <a href="../haneul/balance.md#haneul_balance_Balance">Balance</a>&lt;T&gt;, ctx: &TxContext) {
    <b>assert</b>!(ctx.sender() == @0x0, <a href="../haneul/balance.md#haneul_balance_ENotSystemAddress">ENotSystemAddress</a>);
    <b>assert</b>!(
        <a href="../std/type_name.md#std_type_name_with_defining_ids">std::type_name::with_defining_ids</a>&lt;T&gt;().into_string().into_bytes() == <a href="../haneul/balance.md#haneul_balance_HANEUL_TYPE_NAME">HANEUL_TYPE_NAME</a>,
        <a href="../haneul/balance.md#haneul_balance_ENotHANEUL">ENotHANEUL</a>,
    );
    <b>let</b> <a href="../haneul/balance.md#haneul_balance_Balance">Balance</a> { <a href="../haneul/balance.md#haneul_balance_value">value</a>: _ } = self;
}
</code></pre>



</details>

<a name="haneul_balance_destroy_supply"></a>

## Function `destroy_supply`

Destroy a <code><a href="../haneul/balance.md#haneul_balance_Supply">Supply</a></code> preventing any further minting and burning.


<pre><code><b>public</b>(<a href="../haneul/package.md#haneul_package">package</a>) <b>fun</b> <a href="../haneul/balance.md#haneul_balance_destroy_supply">destroy_supply</a>&lt;T&gt;(self: <a href="../haneul/balance.md#haneul_balance_Supply">haneul::balance::Supply</a>&lt;T&gt;): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../haneul/package.md#haneul_package">package</a>) <b>fun</b> <a href="../haneul/balance.md#haneul_balance_destroy_supply">destroy_supply</a>&lt;T&gt;(self: <a href="../haneul/balance.md#haneul_balance_Supply">Supply</a>&lt;T&gt;): u64 {
    <b>let</b> <a href="../haneul/balance.md#haneul_balance_Supply">Supply</a> { <a href="../haneul/balance.md#haneul_balance_value">value</a> } = self;
    <a href="../haneul/balance.md#haneul_balance_value">value</a>
}
</code></pre>



</details>
