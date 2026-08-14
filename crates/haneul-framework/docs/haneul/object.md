---
title: Module `haneul::object`
---

Haneul object identifiers


-  [Struct `ID`](#haneul_object_ID)
-  [Struct `UID`](#haneul_object_UID)
-  [Constants](#@Constants_0)
-  [Function `id_to_bytes`](#haneul_object_id_to_bytes)
-  [Function `id_to_address`](#haneul_object_id_to_address)
-  [Function `id_from_bytes`](#haneul_object_id_from_bytes)
-  [Function `id_from_address`](#haneul_object_id_from_address)
-  [Function `haneul_system_state`](#haneul_object_haneul_system_state)
-  [Function `clock`](#haneul_object_clock)
-  [Function `authenticator_state`](#haneul_object_authenticator_state)
-  [Function `randomness_state`](#haneul_object_randomness_state)
-  [Function `haneul_deny_list_object_id`](#haneul_object_haneul_deny_list_object_id)
-  [Function `haneul_accumulator_root_object_id`](#haneul_object_haneul_accumulator_root_object_id)
-  [Function `haneul_accumulator_root_address`](#haneul_object_haneul_accumulator_root_address)
-  [Function `haneul_coin_registry_object_id`](#haneul_object_haneul_coin_registry_object_id)
-  [Function `haneul_coin_registry_address`](#haneul_object_haneul_coin_registry_address)
-  [Function `haneul_display_registry_object_id`](#haneul_object_haneul_display_registry_object_id)
-  [Function `haneul_display_registry_address`](#haneul_object_haneul_display_registry_address)
-  [Function `bridge`](#haneul_object_bridge)
-  [Function `address_alias_state`](#haneul_object_address_alias_state)
-  [Function `forwarding_address_registry`](#haneul_object_forwarding_address_registry)
-  [Function `uid_as_inner`](#haneul_object_uid_as_inner)
-  [Function `uid_to_inner`](#haneul_object_uid_to_inner)
-  [Function `uid_to_bytes`](#haneul_object_uid_to_bytes)
-  [Function `uid_to_address`](#haneul_object_uid_to_address)
-  [Function `new`](#haneul_object_new)
-  [Function `delete`](#haneul_object_delete)
-  [Function `id`](#haneul_object_id)
-  [Function `borrow_id`](#haneul_object_borrow_id)
-  [Function `id_bytes`](#haneul_object_id_bytes)
-  [Function `id_address`](#haneul_object_id_address)
-  [Function `borrow_uid`](#haneul_object_borrow_uid)
-  [Function `new_uid_from_hash`](#haneul_object_new_uid_from_hash)
-  [Function `delete_impl`](#haneul_object_delete_impl)
-  [Function `record_new_uid_from_hash`](#haneul_object_record_new_uid_from_hash)


<pre><code><b>use</b> <a href="../haneul/address.md#haneul_address">haneul::address</a>;
<b>use</b> <a href="../haneul/hex.md#haneul_hex">haneul::hex</a>;
<b>use</b> <a href="../haneul/tx_context.md#haneul_tx_context">haneul::tx_context</a>;
<b>use</b> <a href="../std/ascii.md#std_ascii">std::ascii</a>;
<b>use</b> <a href="../std/bcs.md#std_bcs">std::bcs</a>;
<b>use</b> <a href="../std/option.md#std_option">std::option</a>;
<b>use</b> <a href="../std/string.md#std_string">std::string</a>;
<b>use</b> <a href="../std/vector.md#std_vector">std::vector</a>;
</code></pre>



<a name="haneul_object_ID"></a>

## Struct `ID`

An object ID. This is used to reference Haneul Objects.
This is *not* guaranteed to be globally unique--anyone can create an <code><a href="../haneul/object.md#haneul_object_ID">ID</a></code> from a <code><a href="../haneul/object.md#haneul_object_UID">UID</a></code> or
from an object, and ID's can be freely copied and dropped.
Here, the values are not globally unique because there can be multiple values of type <code><a href="../haneul/object.md#haneul_object_ID">ID</a></code>
with the same underlying bytes. For example, <code><a href="../haneul/object.md#haneul_object_id">object::id</a>(&obj)</code> can be called as many times
as you want for a given <code>obj</code>, and each <code><a href="../haneul/object.md#haneul_object_ID">ID</a></code> value will be identical.


<pre><code><b>public</b> <b>struct</b> <a href="../haneul/object.md#haneul_object_ID">ID</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>bytes: <b>address</b></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="haneul_object_UID"></a>

## Struct `UID`

Globally unique IDs that define an object's ID in storage. Any Haneul Object, that is a struct
with the <code>key</code> ability, must have <code><a href="../haneul/object.md#haneul_object_id">id</a>: <a href="../haneul/object.md#haneul_object_UID">UID</a></code> as its first field.
These are globally unique in the sense that no two values of type <code><a href="../haneul/object.md#haneul_object_UID">UID</a></code> are ever equal, in
other words for any two values <code>id1: <a href="../haneul/object.md#haneul_object_UID">UID</a></code> and <code>id2: <a href="../haneul/object.md#haneul_object_UID">UID</a></code>, <code>id1</code> != <code>id2</code>.
This is a privileged type that can only be derived from a <code>TxContext</code>.
<code><a href="../haneul/object.md#haneul_object_UID">UID</a></code> doesn't have the <code>drop</code> ability, so deleting a <code><a href="../haneul/object.md#haneul_object_UID">UID</a></code> requires a call to <code><a href="../haneul/object.md#haneul_object_delete">delete</a></code>.


<pre><code><b>public</b> <b>struct</b> <a href="../haneul/object.md#haneul_object_UID">UID</a> <b>has</b> store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../haneul/object.md#haneul_object_id">id</a>: <a href="../haneul/object.md#haneul_object_ID">haneul::object::ID</a></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="haneul_object_HANEUL_SYSTEM_STATE_OBJECT_ID"></a>

The hardcoded ID for the singleton Haneul System State Object.


<pre><code><b>const</b> <a href="../haneul/object.md#haneul_object_HANEUL_SYSTEM_STATE_OBJECT_ID">HANEUL_SYSTEM_STATE_OBJECT_ID</a>: <b>address</b> = 0x5;
</code></pre>



<a name="haneul_object_HANEUL_CLOCK_OBJECT_ID"></a>

The hardcoded ID for the singleton Clock Object.


<pre><code><b>const</b> <a href="../haneul/object.md#haneul_object_HANEUL_CLOCK_OBJECT_ID">HANEUL_CLOCK_OBJECT_ID</a>: <b>address</b> = 0x6;
</code></pre>



<a name="haneul_object_HANEUL_AUTHENTICATOR_STATE_ID"></a>

The hardcoded ID for the singleton AuthenticatorState Object.


<pre><code><b>const</b> <a href="../haneul/object.md#haneul_object_HANEUL_AUTHENTICATOR_STATE_ID">HANEUL_AUTHENTICATOR_STATE_ID</a>: <b>address</b> = 0x7;
</code></pre>



<a name="haneul_object_HANEUL_RANDOM_ID"></a>

The hardcoded ID for the singleton Random Object.


<pre><code><b>const</b> <a href="../haneul/object.md#haneul_object_HANEUL_RANDOM_ID">HANEUL_RANDOM_ID</a>: <b>address</b> = 0x8;
</code></pre>



<a name="haneul_object_HANEUL_DENY_LIST_OBJECT_ID"></a>

The hardcoded ID for the singleton DenyList.


<pre><code><b>const</b> <a href="../haneul/object.md#haneul_object_HANEUL_DENY_LIST_OBJECT_ID">HANEUL_DENY_LIST_OBJECT_ID</a>: <b>address</b> = 0x403;
</code></pre>



<a name="haneul_object_HANEUL_ACCUMULATOR_ROOT_OBJECT_ID"></a>

The hardcoded ID for the singleton AccumulatorRoot Object.


<pre><code><b>const</b> <a href="../haneul/object.md#haneul_object_HANEUL_ACCUMULATOR_ROOT_OBJECT_ID">HANEUL_ACCUMULATOR_ROOT_OBJECT_ID</a>: <b>address</b> = 0xacc;
</code></pre>



<a name="haneul_object_HANEUL_BRIDGE_ID"></a>

The hardcoded ID for the Bridge Object.


<pre><code><b>const</b> <a href="../haneul/object.md#haneul_object_HANEUL_BRIDGE_ID">HANEUL_BRIDGE_ID</a>: <b>address</b> = 0x9;
</code></pre>



<a name="haneul_object_HANEUL_COIN_REGISTRY_OBJECT_ID"></a>

The hardcoded ID for the Coin Registry Object.


<pre><code><b>const</b> <a href="../haneul/object.md#haneul_object_HANEUL_COIN_REGISTRY_OBJECT_ID">HANEUL_COIN_REGISTRY_OBJECT_ID</a>: <b>address</b> = 0xc;
</code></pre>



<a name="haneul_object_HANEUL_DISPLAY_REGISTRY_OBJECT_ID"></a>

The hardcoded ID for the Display Registry Object.


<pre><code><b>const</b> <a href="../haneul/object.md#haneul_object_HANEUL_DISPLAY_REGISTRY_OBJECT_ID">HANEUL_DISPLAY_REGISTRY_OBJECT_ID</a>: <b>address</b> = 0xd;
</code></pre>



<a name="haneul_object_HANEUL_ADDRESS_ALIAS_STATE_ID"></a>

The hardcoded ID for the AddressAliasState Object.


<pre><code><b>const</b> <a href="../haneul/object.md#haneul_object_HANEUL_ADDRESS_ALIAS_STATE_ID">HANEUL_ADDRESS_ALIAS_STATE_ID</a>: <b>address</b> = 0xa;
</code></pre>



<a name="haneul_object_HANEUL_FORWARDING_ADDRESS_REGISTRY_OBJECT_ID"></a>

The hardcoded ID for the singleton ForwardingAddressRegistry object.


<pre><code><b>const</b> <a href="../haneul/object.md#haneul_object_HANEUL_FORWARDING_ADDRESS_REGISTRY_OBJECT_ID">HANEUL_FORWARDING_ADDRESS_REGISTRY_OBJECT_ID</a>: <b>address</b> = 0xfa;
</code></pre>



<a name="haneul_object_ENotSystemAddress"></a>

Sender is not @0x0 the system address.


<pre><code><b>const</b> <a href="../haneul/object.md#haneul_object_ENotSystemAddress">ENotSystemAddress</a>: u64 = 0;
</code></pre>



<a name="haneul_object_id_to_bytes"></a>

## Function `id_to_bytes`

Get the raw bytes of a <code><a href="../haneul/object.md#haneul_object_ID">ID</a></code>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/object.md#haneul_object_id_to_bytes">id_to_bytes</a>(<a href="../haneul/object.md#haneul_object_id">id</a>: &<a href="../haneul/object.md#haneul_object_ID">haneul::object::ID</a>): vector&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/object.md#haneul_object_id_to_bytes">id_to_bytes</a>(<a href="../haneul/object.md#haneul_object_id">id</a>: &<a href="../haneul/object.md#haneul_object_ID">ID</a>): vector&lt;u8&gt; {
    <a href="../haneul/bcs.md#haneul_bcs_to_bytes">bcs::to_bytes</a>(&<a href="../haneul/object.md#haneul_object_id">id</a>.bytes)
}
</code></pre>



</details>

<a name="haneul_object_id_to_address"></a>

## Function `id_to_address`

Get the inner bytes of <code><a href="../haneul/object.md#haneul_object_id">id</a></code> as an address.


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/object.md#haneul_object_id_to_address">id_to_address</a>(<a href="../haneul/object.md#haneul_object_id">id</a>: &<a href="../haneul/object.md#haneul_object_ID">haneul::object::ID</a>): <b>address</b>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/object.md#haneul_object_id_to_address">id_to_address</a>(<a href="../haneul/object.md#haneul_object_id">id</a>: &<a href="../haneul/object.md#haneul_object_ID">ID</a>): <b>address</b> {
    <a href="../haneul/object.md#haneul_object_id">id</a>.bytes
}
</code></pre>



</details>

<a name="haneul_object_id_from_bytes"></a>

## Function `id_from_bytes`

Make an <code><a href="../haneul/object.md#haneul_object_ID">ID</a></code> from raw bytes.


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/object.md#haneul_object_id_from_bytes">id_from_bytes</a>(bytes: vector&lt;u8&gt;): <a href="../haneul/object.md#haneul_object_ID">haneul::object::ID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/object.md#haneul_object_id_from_bytes">id_from_bytes</a>(bytes: vector&lt;u8&gt;): <a href="../haneul/object.md#haneul_object_ID">ID</a> {
    <a href="../haneul/address.md#haneul_address_from_bytes">address::from_bytes</a>(bytes).to_id()
}
</code></pre>



</details>

<a name="haneul_object_id_from_address"></a>

## Function `id_from_address`

Make an <code><a href="../haneul/object.md#haneul_object_ID">ID</a></code> from an address.


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/object.md#haneul_object_id_from_address">id_from_address</a>(bytes: <b>address</b>): <a href="../haneul/object.md#haneul_object_ID">haneul::object::ID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/object.md#haneul_object_id_from_address">id_from_address</a>(bytes: <b>address</b>): <a href="../haneul/object.md#haneul_object_ID">ID</a> {
    <a href="../haneul/object.md#haneul_object_ID">ID</a> { bytes }
}
</code></pre>



</details>

<a name="haneul_object_haneul_system_state"></a>

## Function `haneul_system_state`

Create the <code><a href="../haneul/object.md#haneul_object_UID">UID</a></code> for the singleton <code>HaneulSystemState</code> object.
This should only be called once from <code>haneul_system</code>.


<pre><code><b>fun</b> <a href="../haneul/object.md#haneul_object_haneul_system_state">haneul_system_state</a>(ctx: &<a href="../haneul/tx_context.md#haneul_tx_context_TxContext">haneul::tx_context::TxContext</a>): <a href="../haneul/object.md#haneul_object_UID">haneul::object::UID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../haneul/object.md#haneul_object_haneul_system_state">haneul_system_state</a>(ctx: &TxContext): <a href="../haneul/object.md#haneul_object_UID">UID</a> {
    <b>assert</b>!(ctx.sender() == @0x0, <a href="../haneul/object.md#haneul_object_ENotSystemAddress">ENotSystemAddress</a>);
    <a href="../haneul/object.md#haneul_object_UID">UID</a> {
        <a href="../haneul/object.md#haneul_object_id">id</a>: <a href="../haneul/object.md#haneul_object_ID">ID</a> { bytes: <a href="../haneul/object.md#haneul_object_HANEUL_SYSTEM_STATE_OBJECT_ID">HANEUL_SYSTEM_STATE_OBJECT_ID</a> },
    }
}
</code></pre>



</details>

<a name="haneul_object_clock"></a>

## Function `clock`

Create the <code><a href="../haneul/object.md#haneul_object_UID">UID</a></code> for the singleton <code>Clock</code> object.
This should only be called once from <code><a href="../haneul/clock.md#haneul_clock">clock</a></code>.


<pre><code><b>public</b>(<a href="../haneul/package.md#haneul_package">package</a>) <b>fun</b> <a href="../haneul/clock.md#haneul_clock">clock</a>(): <a href="../haneul/object.md#haneul_object_UID">haneul::object::UID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../haneul/package.md#haneul_package">package</a>) <b>fun</b> <a href="../haneul/clock.md#haneul_clock">clock</a>(): <a href="../haneul/object.md#haneul_object_UID">UID</a> {
    <a href="../haneul/object.md#haneul_object_UID">UID</a> {
        <a href="../haneul/object.md#haneul_object_id">id</a>: <a href="../haneul/object.md#haneul_object_ID">ID</a> { bytes: <a href="../haneul/object.md#haneul_object_HANEUL_CLOCK_OBJECT_ID">HANEUL_CLOCK_OBJECT_ID</a> },
    }
}
</code></pre>



</details>

<a name="haneul_object_authenticator_state"></a>

## Function `authenticator_state`

Create the <code><a href="../haneul/object.md#haneul_object_UID">UID</a></code> for the singleton <code>AuthenticatorState</code> object.
This should only be called once from <code><a href="../haneul/authenticator_state.md#haneul_authenticator_state">authenticator_state</a></code>.


<pre><code><b>public</b>(<a href="../haneul/package.md#haneul_package">package</a>) <b>fun</b> <a href="../haneul/authenticator_state.md#haneul_authenticator_state">authenticator_state</a>(): <a href="../haneul/object.md#haneul_object_UID">haneul::object::UID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../haneul/package.md#haneul_package">package</a>) <b>fun</b> <a href="../haneul/authenticator_state.md#haneul_authenticator_state">authenticator_state</a>(): <a href="../haneul/object.md#haneul_object_UID">UID</a> {
    <a href="../haneul/object.md#haneul_object_UID">UID</a> {
        <a href="../haneul/object.md#haneul_object_id">id</a>: <a href="../haneul/object.md#haneul_object_ID">ID</a> { bytes: <a href="../haneul/object.md#haneul_object_HANEUL_AUTHENTICATOR_STATE_ID">HANEUL_AUTHENTICATOR_STATE_ID</a> },
    }
}
</code></pre>



</details>

<a name="haneul_object_randomness_state"></a>

## Function `randomness_state`

Create the <code><a href="../haneul/object.md#haneul_object_UID">UID</a></code> for the singleton <code>Random</code> object.
This should only be called once from <code><a href="../haneul/random.md#haneul_random">random</a></code>.


<pre><code><b>public</b>(<a href="../haneul/package.md#haneul_package">package</a>) <b>fun</b> <a href="../haneul/object.md#haneul_object_randomness_state">randomness_state</a>(): <a href="../haneul/object.md#haneul_object_UID">haneul::object::UID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../haneul/package.md#haneul_package">package</a>) <b>fun</b> <a href="../haneul/object.md#haneul_object_randomness_state">randomness_state</a>(): <a href="../haneul/object.md#haneul_object_UID">UID</a> {
    <a href="../haneul/object.md#haneul_object_UID">UID</a> {
        <a href="../haneul/object.md#haneul_object_id">id</a>: <a href="../haneul/object.md#haneul_object_ID">ID</a> { bytes: <a href="../haneul/object.md#haneul_object_HANEUL_RANDOM_ID">HANEUL_RANDOM_ID</a> },
    }
}
</code></pre>



</details>

<a name="haneul_object_haneul_deny_list_object_id"></a>

## Function `haneul_deny_list_object_id`

Create the <code><a href="../haneul/object.md#haneul_object_UID">UID</a></code> for the singleton <code>DenyList</code> object.
This should only be called once from <code><a href="../haneul/deny_list.md#haneul_deny_list">deny_list</a></code>.


<pre><code><b>public</b>(<a href="../haneul/package.md#haneul_package">package</a>) <b>fun</b> <a href="../haneul/object.md#haneul_object_haneul_deny_list_object_id">haneul_deny_list_object_id</a>(): <a href="../haneul/object.md#haneul_object_UID">haneul::object::UID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../haneul/package.md#haneul_package">package</a>) <b>fun</b> <a href="../haneul/object.md#haneul_object_haneul_deny_list_object_id">haneul_deny_list_object_id</a>(): <a href="../haneul/object.md#haneul_object_UID">UID</a> {
    <a href="../haneul/object.md#haneul_object_UID">UID</a> {
        <a href="../haneul/object.md#haneul_object_id">id</a>: <a href="../haneul/object.md#haneul_object_ID">ID</a> { bytes: <a href="../haneul/object.md#haneul_object_HANEUL_DENY_LIST_OBJECT_ID">HANEUL_DENY_LIST_OBJECT_ID</a> },
    }
}
</code></pre>



</details>

<a name="haneul_object_haneul_accumulator_root_object_id"></a>

## Function `haneul_accumulator_root_object_id`



<pre><code><b>public</b>(<a href="../haneul/package.md#haneul_package">package</a>) <b>fun</b> <a href="../haneul/object.md#haneul_object_haneul_accumulator_root_object_id">haneul_accumulator_root_object_id</a>(): <a href="../haneul/object.md#haneul_object_UID">haneul::object::UID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../haneul/package.md#haneul_package">package</a>) <b>fun</b> <a href="../haneul/object.md#haneul_object_haneul_accumulator_root_object_id">haneul_accumulator_root_object_id</a>(): <a href="../haneul/object.md#haneul_object_UID">UID</a> {
    <a href="../haneul/object.md#haneul_object_UID">UID</a> {
        <a href="../haneul/object.md#haneul_object_id">id</a>: <a href="../haneul/object.md#haneul_object_ID">ID</a> { bytes: <a href="../haneul/object.md#haneul_object_HANEUL_ACCUMULATOR_ROOT_OBJECT_ID">HANEUL_ACCUMULATOR_ROOT_OBJECT_ID</a> },
    }
}
</code></pre>



</details>

<a name="haneul_object_haneul_accumulator_root_address"></a>

## Function `haneul_accumulator_root_address`



<pre><code><b>public</b>(<a href="../haneul/package.md#haneul_package">package</a>) <b>fun</b> <a href="../haneul/object.md#haneul_object_haneul_accumulator_root_address">haneul_accumulator_root_address</a>(): <b>address</b>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../haneul/package.md#haneul_package">package</a>) <b>fun</b> <a href="../haneul/object.md#haneul_object_haneul_accumulator_root_address">haneul_accumulator_root_address</a>(): <b>address</b> {
    <a href="../haneul/object.md#haneul_object_HANEUL_ACCUMULATOR_ROOT_OBJECT_ID">HANEUL_ACCUMULATOR_ROOT_OBJECT_ID</a>
}
</code></pre>



</details>

<a name="haneul_object_haneul_coin_registry_object_id"></a>

## Function `haneul_coin_registry_object_id`

Create the <code><a href="../haneul/object.md#haneul_object_UID">UID</a></code> for the singleton <code>CoinRegistry</code> object.
This should only be called once from <code><a href="../haneul/coin_registry.md#haneul_coin_registry">coin_registry</a></code>.


<pre><code><b>public</b>(<a href="../haneul/package.md#haneul_package">package</a>) <b>fun</b> <a href="../haneul/object.md#haneul_object_haneul_coin_registry_object_id">haneul_coin_registry_object_id</a>(): <a href="../haneul/object.md#haneul_object_UID">haneul::object::UID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../haneul/package.md#haneul_package">package</a>) <b>fun</b> <a href="../haneul/object.md#haneul_object_haneul_coin_registry_object_id">haneul_coin_registry_object_id</a>(): <a href="../haneul/object.md#haneul_object_UID">UID</a> {
    <a href="../haneul/object.md#haneul_object_UID">UID</a> {
        <a href="../haneul/object.md#haneul_object_id">id</a>: <a href="../haneul/object.md#haneul_object_ID">ID</a> { bytes: <a href="../haneul/object.md#haneul_object_HANEUL_COIN_REGISTRY_OBJECT_ID">HANEUL_COIN_REGISTRY_OBJECT_ID</a> },
    }
}
</code></pre>



</details>

<a name="haneul_object_haneul_coin_registry_address"></a>

## Function `haneul_coin_registry_address`



<pre><code><b>public</b>(<a href="../haneul/package.md#haneul_package">package</a>) <b>fun</b> <a href="../haneul/object.md#haneul_object_haneul_coin_registry_address">haneul_coin_registry_address</a>(): <b>address</b>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../haneul/package.md#haneul_package">package</a>) <b>fun</b> <a href="../haneul/object.md#haneul_object_haneul_coin_registry_address">haneul_coin_registry_address</a>(): <b>address</b> {
    <a href="../haneul/object.md#haneul_object_HANEUL_COIN_REGISTRY_OBJECT_ID">HANEUL_COIN_REGISTRY_OBJECT_ID</a>
}
</code></pre>



</details>

<a name="haneul_object_haneul_display_registry_object_id"></a>

## Function `haneul_display_registry_object_id`

Create the <code><a href="../haneul/object.md#haneul_object_UID">UID</a></code> for the singleton <code>DisplayRegistry</code> object.
This should only be called once from <code><a href="../haneul/display_registry.md#haneul_display_registry">display_registry</a></code>.


<pre><code><b>public</b>(<a href="../haneul/package.md#haneul_package">package</a>) <b>fun</b> <a href="../haneul/object.md#haneul_object_haneul_display_registry_object_id">haneul_display_registry_object_id</a>(): <a href="../haneul/object.md#haneul_object_UID">haneul::object::UID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../haneul/package.md#haneul_package">package</a>) <b>fun</b> <a href="../haneul/object.md#haneul_object_haneul_display_registry_object_id">haneul_display_registry_object_id</a>(): <a href="../haneul/object.md#haneul_object_UID">UID</a> {
    <a href="../haneul/object.md#haneul_object_UID">UID</a> {
        <a href="../haneul/object.md#haneul_object_id">id</a>: <a href="../haneul/object.md#haneul_object_ID">ID</a> { bytes: <a href="../haneul/object.md#haneul_object_HANEUL_DISPLAY_REGISTRY_OBJECT_ID">HANEUL_DISPLAY_REGISTRY_OBJECT_ID</a> },
    }
}
</code></pre>



</details>

<a name="haneul_object_haneul_display_registry_address"></a>

## Function `haneul_display_registry_address`



<pre><code><b>public</b>(<a href="../haneul/package.md#haneul_package">package</a>) <b>fun</b> <a href="../haneul/object.md#haneul_object_haneul_display_registry_address">haneul_display_registry_address</a>(): <b>address</b>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../haneul/package.md#haneul_package">package</a>) <b>fun</b> <a href="../haneul/object.md#haneul_object_haneul_display_registry_address">haneul_display_registry_address</a>(): <b>address</b> {
    <a href="../haneul/object.md#haneul_object_HANEUL_DISPLAY_REGISTRY_OBJECT_ID">HANEUL_DISPLAY_REGISTRY_OBJECT_ID</a>
}
</code></pre>



</details>

<a name="haneul_object_bridge"></a>

## Function `bridge`

Create the <code><a href="../haneul/object.md#haneul_object_UID">UID</a></code> for the singleton <code>Bridge</code> object.
This should only be called once from <code><a href="../haneul/object.md#haneul_object_bridge">bridge</a></code>.


<pre><code><b>fun</b> <a href="../haneul/object.md#haneul_object_bridge">bridge</a>(): <a href="../haneul/object.md#haneul_object_UID">haneul::object::UID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../haneul/object.md#haneul_object_bridge">bridge</a>(): <a href="../haneul/object.md#haneul_object_UID">UID</a> {
    <a href="../haneul/object.md#haneul_object_UID">UID</a> {
        <a href="../haneul/object.md#haneul_object_id">id</a>: <a href="../haneul/object.md#haneul_object_ID">ID</a> { bytes: <a href="../haneul/object.md#haneul_object_HANEUL_BRIDGE_ID">HANEUL_BRIDGE_ID</a> },
    }
}
</code></pre>



</details>

<a name="haneul_object_address_alias_state"></a>

## Function `address_alias_state`

Create the <code><a href="../haneul/object.md#haneul_object_UID">UID</a></code> for the singleton <code>AddressAliasState</code> object.
This should only be called once from <code><a href="../haneul/address_alias.md#haneul_address_alias">address_alias</a></code>.


<pre><code><b>public</b>(<a href="../haneul/package.md#haneul_package">package</a>) <b>fun</b> <a href="../haneul/object.md#haneul_object_address_alias_state">address_alias_state</a>(): <a href="../haneul/object.md#haneul_object_UID">haneul::object::UID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../haneul/package.md#haneul_package">package</a>) <b>fun</b> <a href="../haneul/object.md#haneul_object_address_alias_state">address_alias_state</a>(): <a href="../haneul/object.md#haneul_object_UID">UID</a> {
    <a href="../haneul/object.md#haneul_object_UID">UID</a> {
        <a href="../haneul/object.md#haneul_object_id">id</a>: <a href="../haneul/object.md#haneul_object_ID">ID</a> { bytes: <a href="../haneul/object.md#haneul_object_HANEUL_ADDRESS_ALIAS_STATE_ID">HANEUL_ADDRESS_ALIAS_STATE_ID</a> },
    }
}
</code></pre>



</details>

<a name="haneul_object_forwarding_address_registry"></a>

## Function `forwarding_address_registry`

Create the <code><a href="../haneul/object.md#haneul_object_UID">UID</a></code> for the singleton <code>ForwardingAddressRegistry</code> object.
This should only be called once from <code><a href="../haneul/forwarding_address.md#haneul_forwarding_address">forwarding_address</a></code>.


<pre><code><b>public</b>(<a href="../haneul/package.md#haneul_package">package</a>) <b>fun</b> <a href="../haneul/object.md#haneul_object_forwarding_address_registry">forwarding_address_registry</a>(): <a href="../haneul/object.md#haneul_object_UID">haneul::object::UID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../haneul/package.md#haneul_package">package</a>) <b>fun</b> <a href="../haneul/object.md#haneul_object_forwarding_address_registry">forwarding_address_registry</a>(): <a href="../haneul/object.md#haneul_object_UID">UID</a> {
    <a href="../haneul/object.md#haneul_object_UID">UID</a> {
        <a href="../haneul/object.md#haneul_object_id">id</a>: <a href="../haneul/object.md#haneul_object_ID">ID</a> { bytes: <a href="../haneul/object.md#haneul_object_HANEUL_FORWARDING_ADDRESS_REGISTRY_OBJECT_ID">HANEUL_FORWARDING_ADDRESS_REGISTRY_OBJECT_ID</a> },
    }
}
</code></pre>



</details>

<a name="haneul_object_uid_as_inner"></a>

## Function `uid_as_inner`

Get the inner <code><a href="../haneul/object.md#haneul_object_ID">ID</a></code> of <code>uid</code>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/object.md#haneul_object_uid_as_inner">uid_as_inner</a>(uid: &<a href="../haneul/object.md#haneul_object_UID">haneul::object::UID</a>): &<a href="../haneul/object.md#haneul_object_ID">haneul::object::ID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/object.md#haneul_object_uid_as_inner">uid_as_inner</a>(uid: &<a href="../haneul/object.md#haneul_object_UID">UID</a>): &<a href="../haneul/object.md#haneul_object_ID">ID</a> {
    &uid.<a href="../haneul/object.md#haneul_object_id">id</a>
}
</code></pre>



</details>

<a name="haneul_object_uid_to_inner"></a>

## Function `uid_to_inner`

Get the raw bytes of a <code>uid</code>'s inner <code><a href="../haneul/object.md#haneul_object_ID">ID</a></code>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/object.md#haneul_object_uid_to_inner">uid_to_inner</a>(uid: &<a href="../haneul/object.md#haneul_object_UID">haneul::object::UID</a>): <a href="../haneul/object.md#haneul_object_ID">haneul::object::ID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/object.md#haneul_object_uid_to_inner">uid_to_inner</a>(uid: &<a href="../haneul/object.md#haneul_object_UID">UID</a>): <a href="../haneul/object.md#haneul_object_ID">ID</a> {
    uid.<a href="../haneul/object.md#haneul_object_id">id</a>
}
</code></pre>



</details>

<a name="haneul_object_uid_to_bytes"></a>

## Function `uid_to_bytes`

Get the raw bytes of a <code><a href="../haneul/object.md#haneul_object_UID">UID</a></code>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/object.md#haneul_object_uid_to_bytes">uid_to_bytes</a>(uid: &<a href="../haneul/object.md#haneul_object_UID">haneul::object::UID</a>): vector&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/object.md#haneul_object_uid_to_bytes">uid_to_bytes</a>(uid: &<a href="../haneul/object.md#haneul_object_UID">UID</a>): vector&lt;u8&gt; {
    <a href="../haneul/bcs.md#haneul_bcs_to_bytes">bcs::to_bytes</a>(&uid.<a href="../haneul/object.md#haneul_object_id">id</a>.bytes)
}
</code></pre>



</details>

<a name="haneul_object_uid_to_address"></a>

## Function `uid_to_address`

Get the inner bytes of <code><a href="../haneul/object.md#haneul_object_id">id</a></code> as an address.


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/object.md#haneul_object_uid_to_address">uid_to_address</a>(uid: &<a href="../haneul/object.md#haneul_object_UID">haneul::object::UID</a>): <b>address</b>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/object.md#haneul_object_uid_to_address">uid_to_address</a>(uid: &<a href="../haneul/object.md#haneul_object_UID">UID</a>): <b>address</b> {
    uid.<a href="../haneul/object.md#haneul_object_id">id</a>.bytes
}
</code></pre>



</details>

<a name="haneul_object_new"></a>

## Function `new`

Create a new object. Returns the <code><a href="../haneul/object.md#haneul_object_UID">UID</a></code> that must be stored in a Haneul object.
This is the only way to create <code><a href="../haneul/object.md#haneul_object_UID">UID</a></code>s.


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/object.md#haneul_object_new">new</a>(ctx: &<b>mut</b> <a href="../haneul/tx_context.md#haneul_tx_context_TxContext">haneul::tx_context::TxContext</a>): <a href="../haneul/object.md#haneul_object_UID">haneul::object::UID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/object.md#haneul_object_new">new</a>(ctx: &<b>mut</b> TxContext): <a href="../haneul/object.md#haneul_object_UID">UID</a> {
    <a href="../haneul/object.md#haneul_object_UID">UID</a> {
        <a href="../haneul/object.md#haneul_object_id">id</a>: <a href="../haneul/object.md#haneul_object_ID">ID</a> { bytes: ctx.fresh_object_address() },
    }
}
</code></pre>



</details>

<a name="haneul_object_delete"></a>

## Function `delete`

Delete the object and its <code><a href="../haneul/object.md#haneul_object_UID">UID</a></code>. This is the only way to eliminate a <code><a href="../haneul/object.md#haneul_object_UID">UID</a></code>.
This exists to inform Haneul of object deletions. When an object
gets unpacked, the programmer will have to do something with its
<code><a href="../haneul/object.md#haneul_object_UID">UID</a></code>. The implementation of this function emits a deleted
system event so Haneul knows to process the object deletion


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/object.md#haneul_object_delete">delete</a>(<a href="../haneul/object.md#haneul_object_id">id</a>: <a href="../haneul/object.md#haneul_object_UID">haneul::object::UID</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/object.md#haneul_object_delete">delete</a>(<a href="../haneul/object.md#haneul_object_id">id</a>: <a href="../haneul/object.md#haneul_object_UID">UID</a>) {
    <b>let</b> <a href="../haneul/object.md#haneul_object_UID">UID</a> { <a href="../haneul/object.md#haneul_object_id">id</a>: <a href="../haneul/object.md#haneul_object_ID">ID</a> { bytes } } = <a href="../haneul/object.md#haneul_object_id">id</a>;
    <a href="../haneul/object.md#haneul_object_delete_impl">delete_impl</a>(bytes)
}
</code></pre>



</details>

<a name="haneul_object_id"></a>

## Function `id`

Get the underlying <code><a href="../haneul/object.md#haneul_object_ID">ID</a></code> of <code>obj</code>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/object.md#haneul_object_id">id</a>&lt;T: key&gt;(obj: &T): <a href="../haneul/object.md#haneul_object_ID">haneul::object::ID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/object.md#haneul_object_id">id</a>&lt;T: key&gt;(obj: &T): <a href="../haneul/object.md#haneul_object_ID">ID</a> {
    <a href="../haneul/object.md#haneul_object_borrow_uid">borrow_uid</a>(obj).<a href="../haneul/object.md#haneul_object_id">id</a>
}
</code></pre>



</details>

<a name="haneul_object_borrow_id"></a>

## Function `borrow_id`

Borrow the underlying <code><a href="../haneul/object.md#haneul_object_ID">ID</a></code> of <code>obj</code>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/object.md#haneul_object_borrow_id">borrow_id</a>&lt;T: key&gt;(obj: &T): &<a href="../haneul/object.md#haneul_object_ID">haneul::object::ID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/object.md#haneul_object_borrow_id">borrow_id</a>&lt;T: key&gt;(obj: &T): &<a href="../haneul/object.md#haneul_object_ID">ID</a> {
    &<a href="../haneul/object.md#haneul_object_borrow_uid">borrow_uid</a>(obj).<a href="../haneul/object.md#haneul_object_id">id</a>
}
</code></pre>



</details>

<a name="haneul_object_id_bytes"></a>

## Function `id_bytes`

Get the raw bytes for the underlying <code><a href="../haneul/object.md#haneul_object_ID">ID</a></code> of <code>obj</code>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/object.md#haneul_object_id_bytes">id_bytes</a>&lt;T: key&gt;(obj: &T): vector&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/object.md#haneul_object_id_bytes">id_bytes</a>&lt;T: key&gt;(obj: &T): vector&lt;u8&gt; {
    <a href="../haneul/bcs.md#haneul_bcs_to_bytes">bcs::to_bytes</a>(&<a href="../haneul/object.md#haneul_object_borrow_uid">borrow_uid</a>(obj).<a href="../haneul/object.md#haneul_object_id">id</a>)
}
</code></pre>



</details>

<a name="haneul_object_id_address"></a>

## Function `id_address`

Get the inner bytes for the underlying <code><a href="../haneul/object.md#haneul_object_ID">ID</a></code> of <code>obj</code>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/object.md#haneul_object_id_address">id_address</a>&lt;T: key&gt;(obj: &T): <b>address</b>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/object.md#haneul_object_id_address">id_address</a>&lt;T: key&gt;(obj: &T): <b>address</b> {
    <a href="../haneul/object.md#haneul_object_borrow_uid">borrow_uid</a>(obj).<a href="../haneul/object.md#haneul_object_id">id</a>.bytes
}
</code></pre>



</details>

<a name="haneul_object_borrow_uid"></a>

## Function `borrow_uid`

Get the <code><a href="../haneul/object.md#haneul_object_UID">UID</a></code> for <code>obj</code>.
Safe because Haneul has an extra bytecode verifier pass that forces every struct with
the <code>key</code> ability to have a distinguished <code><a href="../haneul/object.md#haneul_object_UID">UID</a></code> field.
Cannot be made public as the access to <code><a href="../haneul/object.md#haneul_object_UID">UID</a></code> for a given object must be privileged, and
restrictable in the object's module.


<pre><code><b>fun</b> <a href="../haneul/object.md#haneul_object_borrow_uid">borrow_uid</a>&lt;T: key&gt;(obj: &T): &<a href="../haneul/object.md#haneul_object_UID">haneul::object::UID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>native</b> <b>fun</b> <a href="../haneul/object.md#haneul_object_borrow_uid">borrow_uid</a>&lt;T: key&gt;(obj: &T): &<a href="../haneul/object.md#haneul_object_UID">UID</a>;
</code></pre>



</details>

<a name="haneul_object_new_uid_from_hash"></a>

## Function `new_uid_from_hash`

Generate a new UID specifically used for creating a UID from a hash


<pre><code><b>public</b>(<a href="../haneul/package.md#haneul_package">package</a>) <b>fun</b> <a href="../haneul/object.md#haneul_object_new_uid_from_hash">new_uid_from_hash</a>(parent: <b>address</b>, bytes: <b>address</b>): <a href="../haneul/object.md#haneul_object_UID">haneul::object::UID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../haneul/package.md#haneul_package">package</a>) <b>fun</b> <a href="../haneul/object.md#haneul_object_new_uid_from_hash">new_uid_from_hash</a>(parent: <b>address</b>, bytes: <b>address</b>): <a href="../haneul/object.md#haneul_object_UID">UID</a> {
    <a href="../haneul/object.md#haneul_object_record_new_uid_from_hash">record_new_uid_from_hash</a>(parent, bytes);
    <a href="../haneul/object.md#haneul_object_UID">UID</a> { <a href="../haneul/object.md#haneul_object_id">id</a>: <a href="../haneul/object.md#haneul_object_ID">ID</a> { bytes } }
}
</code></pre>



</details>

<a name="haneul_object_delete_impl"></a>

## Function `delete_impl`



<pre><code><b>fun</b> <a href="../haneul/object.md#haneul_object_delete_impl">delete_impl</a>(<a href="../haneul/object.md#haneul_object_id">id</a>: <b>address</b>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>native</b> <b>fun</b> <a href="../haneul/object.md#haneul_object_delete_impl">delete_impl</a>(<a href="../haneul/object.md#haneul_object_id">id</a>: <b>address</b>);
</code></pre>



</details>

<a name="haneul_object_record_new_uid_from_hash"></a>

## Function `record_new_uid_from_hash`



<pre><code><b>fun</b> <a href="../haneul/object.md#haneul_object_record_new_uid_from_hash">record_new_uid_from_hash</a>(parent: <b>address</b>, bytes: <b>address</b>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>native</b> <b>fun</b> <a href="../haneul/object.md#haneul_object_record_new_uid_from_hash">record_new_uid_from_hash</a>(parent: <b>address</b>, bytes: <b>address</b>);
</code></pre>



</details>
