---
title: Module `haneul::scratch`
---

<code><a href="../haneul/scratch.md#haneul_scratch">haneul::scratch</a></code> is an ephemeral, per-transaction key-value store. Unlike <code><a href="../haneul/dynamic_field.md#haneul_dynamic_field">haneul::dynamic_field</a></code>,
scratch entries are not attached to any object, and are instead dropped at the end of the
transaction.

Each entry is identified by the pair of its key type and key value, hashed together in the same
way as a dynamic field name (see <code><a href="../haneul/dynamic_field.md#haneul_dynamic_field_hash_type_and_key">haneul::dynamic_field::hash_type_and_key</a></code>).

All access (mutable and immutable) is controlled through the module that defines the key type
<code>K</code>. The functions are gated by a <code><a href="../haneul/scratch.md#haneul_scratch_Permit">Permit</a>&lt;K&gt;</code>, which can be granted via an
<code>internal::Permit&lt;K&gt;</code>.


-  [Struct `Permit`](#haneul_scratch_Permit)
-  [Constants](#@Constants_0)
-  [Function `permit`](#haneul_scratch_permit)
-  [Function `add`](#haneul_scratch_add)
-  [Function `read`](#haneul_scratch_read)
-  [Function `remove`](#haneul_scratch_remove)
-  [Function `exists`](#haneul_scratch_exists)
-  [Function `exists_with_type`](#haneul_scratch_exists_with_type)
-  [Function `read_opt`](#haneul_scratch_read_opt)
-  [Function `remove_opt`](#haneul_scratch_remove_opt)
-  [Function `replace`](#haneul_scratch_replace)
-  [Macro function `internal_add`](#haneul_scratch_internal_add)
-  [Macro function `internal_read`](#haneul_scratch_internal_read)
-  [Macro function `internal_remove`](#haneul_scratch_internal_remove)
-  [Macro function `internal_exists`](#haneul_scratch_internal_exists)
-  [Macro function `internal_exists_with_type`](#haneul_scratch_internal_exists_with_type)
-  [Macro function `internal_read_opt`](#haneul_scratch_internal_read_opt)
-  [Macro function `internal_remove_opt`](#haneul_scratch_internal_remove_opt)
-  [Macro function `internal_replace`](#haneul_scratch_internal_replace)
-  [Function `hash_type_and_key`](#haneul_scratch_hash_type_and_key)
-  [Function `add_impl`](#haneul_scratch_add_impl)
-  [Function `read_impl`](#haneul_scratch_read_impl)
-  [Function `remove_impl`](#haneul_scratch_remove_impl)
-  [Function `exists_impl`](#haneul_scratch_exists_impl)
-  [Function `exists_with_type_impl`](#haneul_scratch_exists_with_type_impl)


<pre><code><b>use</b> <a href="../haneul/address.md#haneul_address">haneul::address</a>;
<b>use</b> <a href="../haneul/dynamic_field.md#haneul_dynamic_field">haneul::dynamic_field</a>;
<b>use</b> <a href="../haneul/hex.md#haneul_hex">haneul::hex</a>;
<b>use</b> <a href="../haneul/object.md#haneul_object">haneul::object</a>;
<b>use</b> <a href="../haneul/tx_context.md#haneul_tx_context">haneul::tx_context</a>;
<b>use</b> <a href="../std/ascii.md#std_ascii">std::ascii</a>;
<b>use</b> <a href="../std/bcs.md#std_bcs">std::bcs</a>;
<b>use</b> <a href="../std/internal.md#std_internal">std::internal</a>;
<b>use</b> <a href="../std/option.md#std_option">std::option</a>;
<b>use</b> <a href="../std/string.md#std_string">std::string</a>;
<b>use</b> <a href="../std/vector.md#std_vector">std::vector</a>;
</code></pre>



<a name="haneul_scratch_Permit"></a>

## Struct `Permit`

A <code><a href="../haneul/scratch.md#haneul_scratch_Permit">Permit</a>&lt;K&gt;</code> gates access to all entries keyed by values of type <code>K</code>.
It is issued from an <code>internal::Permit&lt;K&gt;</code>, allowing the module that defines <code>K</code> to control
all access to scratch entries.


<pre><code><b>public</b> <b>struct</b> <a href="../haneul/scratch.md#haneul_scratch_Permit">Permit</a>&lt;<b>phantom</b> K: <b>copy</b>, drop&gt; <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="haneul_scratch_DUMMY_ROOT"></a>

Stand-in parent address used when hashing scratch keys.


<pre><code><b>const</b> <a href="../haneul/scratch.md#haneul_scratch_DUMMY_ROOT">DUMMY_ROOT</a>: <b>address</b> = 0x0;
</code></pre>



<a name="haneul_scratch_EEntryAlreadyExists"></a>

The scratch store already has an entry for this key.


<pre><code><b>const</b> <a href="../haneul/scratch.md#haneul_scratch_EEntryAlreadyExists">EEntryAlreadyExists</a>: u64 = 0;
</code></pre>



<a name="haneul_scratch_EEntryDoesNotExist"></a>

The scratch store does not have an entry for this key.


<pre><code><b>const</b> <a href="../haneul/scratch.md#haneul_scratch_EEntryDoesNotExist">EEntryDoesNotExist</a>: u64 = 1;
</code></pre>



<a name="haneul_scratch_EEntryTypeMismatch"></a>

The scratch store has an entry for this key, but the value type does not match.


<pre><code><b>const</b> <a href="../haneul/scratch.md#haneul_scratch_EEntryTypeMismatch">EEntryTypeMismatch</a>: u64 = 2;
</code></pre>



<a name="haneul_scratch_permit"></a>

## Function `permit`

Issues a <code><a href="../haneul/scratch.md#haneul_scratch_Permit">Permit</a>&lt;K&gt;</code> from the privileged <code>internal::Permit&lt;K&gt;</code>, granting access to the
scratch entries keyed by values of type <code>K</code>.


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/scratch.md#haneul_scratch_permit">permit</a>&lt;K: <b>copy</b>, drop&gt;(_: <a href="../std/internal.md#std_internal_Permit">std::internal::Permit</a>&lt;K&gt;): <a href="../haneul/scratch.md#haneul_scratch_Permit">haneul::scratch::Permit</a>&lt;K&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/scratch.md#haneul_scratch_permit">permit</a>&lt;K: <b>copy</b> + drop&gt;(_: internal::Permit&lt;K&gt;): <a href="../haneul/scratch.md#haneul_scratch_Permit">Permit</a>&lt;K&gt; {
    <a href="../haneul/scratch.md#haneul_scratch_Permit">Permit</a>()
}
</code></pre>



</details>

<a name="haneul_scratch_add"></a>

## Function `add`

Adds the <code>key</code>-<code>value</code> pair to the scratch store. Requires a <code><a href="../haneul/scratch.md#haneul_scratch_Permit">Permit</a>&lt;K&gt;</code> for the key type.
Aborts with <code><a href="../haneul/scratch.md#haneul_scratch_EEntryAlreadyExists">EEntryAlreadyExists</a></code> if there is already an entry for <code>key</code>, regardless of its
value type.


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/scratch.md#haneul_scratch_add">add</a>&lt;K: <b>copy</b>, drop, V: drop&gt;(_: &<b>mut</b> <a href="../haneul/tx_context.md#haneul_tx_context_TxContext">haneul::tx_context::TxContext</a>, _: <a href="../haneul/scratch.md#haneul_scratch_Permit">haneul::scratch::Permit</a>&lt;K&gt;, key: K, value: V)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/scratch.md#haneul_scratch_add">add</a>&lt;K: <b>copy</b> + drop, V: drop&gt;(_: &<b>mut</b> TxContext, _: <a href="../haneul/scratch.md#haneul_scratch_Permit">Permit</a>&lt;K&gt;, key: K, value: V) {
    <a href="../haneul/scratch.md#haneul_scratch_add_impl">add_impl</a>(<a href="../haneul/scratch.md#haneul_scratch_hash_type_and_key">hash_type_and_key</a>(key), value)
}
</code></pre>



</details>

<a name="haneul_scratch_read"></a>

## Function `read`

Returns a copy of the value bound to <code>key</code>. Requires a <code><a href="../haneul/scratch.md#haneul_scratch_Permit">Permit</a>&lt;K&gt;</code> for the key type.
Aborts with <code><a href="../haneul/scratch.md#haneul_scratch_EEntryDoesNotExist">EEntryDoesNotExist</a></code> if there is no entry for <code>key</code>.
Aborts with <code><a href="../haneul/scratch.md#haneul_scratch_EEntryTypeMismatch">EEntryTypeMismatch</a></code> if the entry exists, but its value is not of type <code>V</code>.


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/scratch.md#haneul_scratch_read">read</a>&lt;K: <b>copy</b>, drop, V: <b>copy</b>, drop&gt;(_: &<a href="../haneul/tx_context.md#haneul_tx_context_TxContext">haneul::tx_context::TxContext</a>, _: <a href="../haneul/scratch.md#haneul_scratch_Permit">haneul::scratch::Permit</a>&lt;K&gt;, key: K): V
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/scratch.md#haneul_scratch_read">read</a>&lt;K: <b>copy</b> + drop, V: <b>copy</b> + drop&gt;(_: &TxContext, _: <a href="../haneul/scratch.md#haneul_scratch_Permit">Permit</a>&lt;K&gt;, key: K): V {
    <a href="../haneul/scratch.md#haneul_scratch_read_impl">read_impl</a>(<a href="../haneul/scratch.md#haneul_scratch_hash_type_and_key">hash_type_and_key</a>(key))
}
</code></pre>



</details>

<a name="haneul_scratch_remove"></a>

## Function `remove`

Removes the entry bound to <code>key</code> and returns its value. Requires a <code><a href="../haneul/scratch.md#haneul_scratch_Permit">Permit</a>&lt;K&gt;</code> for the key type.
Aborts with <code><a href="../haneul/scratch.md#haneul_scratch_EEntryDoesNotExist">EEntryDoesNotExist</a></code> if there is no entry for <code>key</code>.
Aborts with <code><a href="../haneul/scratch.md#haneul_scratch_EEntryTypeMismatch">EEntryTypeMismatch</a></code> if the entry exists, but its value is not of type <code>V</code>.


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/scratch.md#haneul_scratch_remove">remove</a>&lt;K: <b>copy</b>, drop, V: drop&gt;(_: &<b>mut</b> <a href="../haneul/tx_context.md#haneul_tx_context_TxContext">haneul::tx_context::TxContext</a>, _: <a href="../haneul/scratch.md#haneul_scratch_Permit">haneul::scratch::Permit</a>&lt;K&gt;, key: K): V
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/scratch.md#haneul_scratch_remove">remove</a>&lt;K: <b>copy</b> + drop, V: drop&gt;(_: &<b>mut</b> TxContext, _: <a href="../haneul/scratch.md#haneul_scratch_Permit">Permit</a>&lt;K&gt;, key: K): V {
    <a href="../haneul/scratch.md#haneul_scratch_remove_impl">remove_impl</a>(<a href="../haneul/scratch.md#haneul_scratch_hash_type_and_key">hash_type_and_key</a>(key))
}
</code></pre>



</details>

<a name="haneul_scratch_exists"></a>

## Function `exists`

Returns true if and only if the scratch store has an entry for <code>key</code>, without regard to the
value type. Requires a <code><a href="../haneul/scratch.md#haneul_scratch_Permit">Permit</a>&lt;K&gt;</code> for the key type.


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/scratch.md#haneul_scratch_exists">exists</a>&lt;K: <b>copy</b>, drop&gt;(_: &<a href="../haneul/tx_context.md#haneul_tx_context_TxContext">haneul::tx_context::TxContext</a>, _: <a href="../haneul/scratch.md#haneul_scratch_Permit">haneul::scratch::Permit</a>&lt;K&gt;, key: K): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/scratch.md#haneul_scratch_exists">exists</a>&lt;K: <b>copy</b> + drop&gt;(_: &TxContext, _: <a href="../haneul/scratch.md#haneul_scratch_Permit">Permit</a>&lt;K&gt;, key: K): bool {
    <a href="../haneul/scratch.md#haneul_scratch_exists_impl">exists_impl</a>(<a href="../haneul/scratch.md#haneul_scratch_hash_type_and_key">hash_type_and_key</a>(key))
}
</code></pre>



</details>

<a name="haneul_scratch_exists_with_type"></a>

## Function `exists_with_type`

Returns true if and only if the scratch store has an entry for <code>key</code> whose value is of type <code>V</code>.
Requires a <code><a href="../haneul/scratch.md#haneul_scratch_Permit">Permit</a>&lt;K&gt;</code> for the key type.


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/scratch.md#haneul_scratch_exists_with_type">exists_with_type</a>&lt;K: <b>copy</b>, drop, V: drop&gt;(_: &<a href="../haneul/tx_context.md#haneul_tx_context_TxContext">haneul::tx_context::TxContext</a>, _: <a href="../haneul/scratch.md#haneul_scratch_Permit">haneul::scratch::Permit</a>&lt;K&gt;, key: K): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/scratch.md#haneul_scratch_exists_with_type">exists_with_type</a>&lt;K: <b>copy</b> + drop, V: drop&gt;(_: &TxContext, _: <a href="../haneul/scratch.md#haneul_scratch_Permit">Permit</a>&lt;K&gt;, key: K): bool {
    <a href="../haneul/scratch.md#haneul_scratch_exists_with_type_impl">exists_with_type_impl</a>&lt;V&gt;(<a href="../haneul/scratch.md#haneul_scratch_hash_type_and_key">hash_type_and_key</a>(key))
}
</code></pre>



</details>

<a name="haneul_scratch_read_opt"></a>

## Function `read_opt`

Returns a copy of the value bound to <code>key</code> as <code>some(value)</code> if it exists, or <code>none</code> otherwise.
Aborts with <code><a href="../haneul/scratch.md#haneul_scratch_EEntryTypeMismatch">EEntryTypeMismatch</a></code> if the entry exists, but its value is not of type <code>V</code>.


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/scratch.md#haneul_scratch_read_opt">read_opt</a>&lt;K: <b>copy</b>, drop, V: <b>copy</b>, drop&gt;(ctx: &<a href="../haneul/tx_context.md#haneul_tx_context_TxContext">haneul::tx_context::TxContext</a>, <a href="../haneul/scratch.md#haneul_scratch_permit">permit</a>: <a href="../haneul/scratch.md#haneul_scratch_Permit">haneul::scratch::Permit</a>&lt;K&gt;, key: K): <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;V&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/scratch.md#haneul_scratch_read_opt">read_opt</a>&lt;K: <b>copy</b> + drop, V: <b>copy</b> + drop&gt;(
    ctx: &TxContext,
    <a href="../haneul/scratch.md#haneul_scratch_permit">permit</a>: <a href="../haneul/scratch.md#haneul_scratch_Permit">Permit</a>&lt;K&gt;,
    key: K,
): Option&lt;V&gt; {
    <b>if</b> (<a href="../haneul/scratch.md#haneul_scratch_exists">exists</a>(ctx, <a href="../haneul/scratch.md#haneul_scratch_permit">permit</a>, key)) option::some(<a href="../haneul/scratch.md#haneul_scratch_read">read</a>(ctx, <a href="../haneul/scratch.md#haneul_scratch_permit">permit</a>, key)) <b>else</b> option::none()
}
</code></pre>



</details>

<a name="haneul_scratch_remove_opt"></a>

## Function `remove_opt`

Removes the entry bound to <code>key</code> if it exists, returning <code>some(value)</code>, or <code>none</code> otherwise.
Aborts with <code><a href="../haneul/scratch.md#haneul_scratch_EEntryTypeMismatch">EEntryTypeMismatch</a></code> if the entry exists, but its value is not of type <code>V</code>.


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/scratch.md#haneul_scratch_remove_opt">remove_opt</a>&lt;K: <b>copy</b>, drop, V: drop&gt;(ctx: &<b>mut</b> <a href="../haneul/tx_context.md#haneul_tx_context_TxContext">haneul::tx_context::TxContext</a>, <a href="../haneul/scratch.md#haneul_scratch_permit">permit</a>: <a href="../haneul/scratch.md#haneul_scratch_Permit">haneul::scratch::Permit</a>&lt;K&gt;, key: K): <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;V&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/scratch.md#haneul_scratch_remove_opt">remove_opt</a>&lt;K: <b>copy</b> + drop, V: drop&gt;(
    ctx: &<b>mut</b> TxContext,
    <a href="../haneul/scratch.md#haneul_scratch_permit">permit</a>: <a href="../haneul/scratch.md#haneul_scratch_Permit">Permit</a>&lt;K&gt;,
    key: K,
): Option&lt;V&gt; {
    <b>if</b> (<a href="../haneul/scratch.md#haneul_scratch_exists">exists</a>(ctx, <a href="../haneul/scratch.md#haneul_scratch_permit">permit</a>, key)) option::some(<a href="../haneul/scratch.md#haneul_scratch_remove">remove</a>(ctx, <a href="../haneul/scratch.md#haneul_scratch_permit">permit</a>, key)) <b>else</b> option::none()
}
</code></pre>



</details>

<a name="haneul_scratch_replace"></a>

## Function `replace`

Removes the existing value at <code>key</code> (if any) and adds <code>value</code> in its place.
Returns the old value if it existed, or <code>none</code> otherwise.
Note: the old and new value types may differ.
Aborts with <code><a href="../haneul/scratch.md#haneul_scratch_EEntryTypeMismatch">EEntryTypeMismatch</a></code> if the entry exists, but its value is not of type <code>VOld</code>.


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/scratch.md#haneul_scratch_replace">replace</a>&lt;K: <b>copy</b>, drop, VNew: drop, VOld: drop&gt;(ctx: &<b>mut</b> <a href="../haneul/tx_context.md#haneul_tx_context_TxContext">haneul::tx_context::TxContext</a>, <a href="../haneul/scratch.md#haneul_scratch_permit">permit</a>: <a href="../haneul/scratch.md#haneul_scratch_Permit">haneul::scratch::Permit</a>&lt;K&gt;, key: K, value: VNew): <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;VOld&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/scratch.md#haneul_scratch_replace">replace</a>&lt;K: <b>copy</b> + drop, VNew: drop, VOld: drop&gt;(
    ctx: &<b>mut</b> TxContext,
    <a href="../haneul/scratch.md#haneul_scratch_permit">permit</a>: <a href="../haneul/scratch.md#haneul_scratch_Permit">Permit</a>&lt;K&gt;,
    key: K,
    value: VNew,
): Option&lt;VOld&gt; {
    <b>let</b> old = <a href="../haneul/scratch.md#haneul_scratch_remove_opt">remove_opt</a>&lt;K, VOld&gt;(ctx, <a href="../haneul/scratch.md#haneul_scratch_permit">permit</a>, key);
    <a href="../haneul/scratch.md#haneul_scratch_add">add</a>(ctx, <a href="../haneul/scratch.md#haneul_scratch_permit">permit</a>, key, value);
    old
}
</code></pre>



</details>

<a name="haneul_scratch_internal_add"></a>

## Macro function `internal_add`

A wrapper for <code><a href="../haneul/scratch.md#haneul_scratch_add">add</a></code> that constructs the <code><a href="../haneul/scratch.md#haneul_scratch_Permit">Permit</a>&lt;$K&gt;</code> directly.
Aborts with <code><a href="../haneul/scratch.md#haneul_scratch_EEntryAlreadyExists">EEntryAlreadyExists</a></code> if there is already an entry for <code>$key</code>, regardless of its
value type.


<pre><code><b>public</b> <b>macro</b> <b>fun</b> <a href="../haneul/scratch.md#haneul_scratch_internal_add">internal_add</a>&lt;$K: <b>copy</b>, drop, $V: drop&gt;($ctx: &<b>mut</b> <a href="../haneul/tx_context.md#haneul_tx_context_TxContext">haneul::tx_context::TxContext</a>, $key: $K, $value: $V)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>macro</b> <b>fun</b> <a href="../haneul/scratch.md#haneul_scratch_internal_add">internal_add</a>&lt;$K: <b>copy</b> + drop, $V: drop&gt;(
    $ctx: &<b>mut</b> TxContext,
    $key: $K,
    $value: $V,
) {
    <a href="../haneul/scratch.md#haneul_scratch_add">add</a>($ctx, <a href="../haneul/scratch.md#haneul_scratch_permit">permit</a>(internal::permit&lt;$K&gt;()), $key, $value)
}
</code></pre>



</details>

<a name="haneul_scratch_internal_read"></a>

## Macro function `internal_read`

A wrapper for <code><a href="../haneul/scratch.md#haneul_scratch_read">read</a></code> that constructs the <code><a href="../haneul/scratch.md#haneul_scratch_Permit">Permit</a>&lt;$K&gt;</code> directly.
Aborts with <code><a href="../haneul/scratch.md#haneul_scratch_EEntryDoesNotExist">EEntryDoesNotExist</a></code> if there is no entry for <code>$key</code>.
Aborts with <code><a href="../haneul/scratch.md#haneul_scratch_EEntryTypeMismatch">EEntryTypeMismatch</a></code> if the entry exists, but its value is not of type <code>$V</code>.


<pre><code><b>public</b> <b>macro</b> <b>fun</b> <a href="../haneul/scratch.md#haneul_scratch_internal_read">internal_read</a>&lt;$K: <b>copy</b>, drop, $V: <b>copy</b>, drop&gt;($ctx: &<a href="../haneul/tx_context.md#haneul_tx_context_TxContext">haneul::tx_context::TxContext</a>, $key: $K): $V
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>macro</b> <b>fun</b> <a href="../haneul/scratch.md#haneul_scratch_internal_read">internal_read</a>&lt;$K: <b>copy</b> + drop, $V: <b>copy</b> + drop&gt;($ctx: &TxContext, $key: $K): $V {
    <a href="../haneul/scratch.md#haneul_scratch_read">read</a>&lt;$K, $V&gt;($ctx, <a href="../haneul/scratch.md#haneul_scratch_permit">permit</a>(internal::permit&lt;$K&gt;()), $key)
}
</code></pre>



</details>

<a name="haneul_scratch_internal_remove"></a>

## Macro function `internal_remove`

A wrapper for <code><a href="../haneul/scratch.md#haneul_scratch_remove">remove</a></code> that constructs the <code><a href="../haneul/scratch.md#haneul_scratch_Permit">Permit</a>&lt;$K&gt;</code> directly.
Aborts with <code><a href="../haneul/scratch.md#haneul_scratch_EEntryDoesNotExist">EEntryDoesNotExist</a></code> if there is no entry for <code>$key</code>.
Aborts with <code><a href="../haneul/scratch.md#haneul_scratch_EEntryTypeMismatch">EEntryTypeMismatch</a></code> if the entry exists, but its value is not of type <code>$V</code>.


<pre><code><b>public</b> <b>macro</b> <b>fun</b> <a href="../haneul/scratch.md#haneul_scratch_internal_remove">internal_remove</a>&lt;$K: <b>copy</b>, drop, $V: drop&gt;($ctx: &<b>mut</b> <a href="../haneul/tx_context.md#haneul_tx_context_TxContext">haneul::tx_context::TxContext</a>, $key: $K): $V
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>macro</b> <b>fun</b> <a href="../haneul/scratch.md#haneul_scratch_internal_remove">internal_remove</a>&lt;$K: <b>copy</b> + drop, $V: drop&gt;($ctx: &<b>mut</b> TxContext, $key: $K): $V {
    <a href="../haneul/scratch.md#haneul_scratch_remove">remove</a>&lt;$K, $V&gt;($ctx, <a href="../haneul/scratch.md#haneul_scratch_permit">permit</a>(internal::permit&lt;$K&gt;()), $key)
}
</code></pre>



</details>

<a name="haneul_scratch_internal_exists"></a>

## Macro function `internal_exists`

A wrapper for <code><a href="../haneul/scratch.md#haneul_scratch_exists">exists</a></code> that constructs the <code><a href="../haneul/scratch.md#haneul_scratch_Permit">Permit</a>&lt;$K&gt;</code> directly.


<pre><code><b>public</b> <b>macro</b> <b>fun</b> <a href="../haneul/scratch.md#haneul_scratch_internal_exists">internal_exists</a>&lt;$K: <b>copy</b>, drop&gt;($ctx: &<a href="../haneul/tx_context.md#haneul_tx_context_TxContext">haneul::tx_context::TxContext</a>, $key: $K): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>macro</b> <b>fun</b> <a href="../haneul/scratch.md#haneul_scratch_internal_exists">internal_exists</a>&lt;$K: <b>copy</b> + drop&gt;($ctx: &TxContext, $key: $K): bool {
    <a href="../haneul/scratch.md#haneul_scratch_exists">exists</a>($ctx, <a href="../haneul/scratch.md#haneul_scratch_permit">permit</a>(internal::permit&lt;$K&gt;()), $key)
}
</code></pre>



</details>

<a name="haneul_scratch_internal_exists_with_type"></a>

## Macro function `internal_exists_with_type`

A wrapper for <code><a href="../haneul/scratch.md#haneul_scratch_exists_with_type">exists_with_type</a></code> that constructs the <code><a href="../haneul/scratch.md#haneul_scratch_Permit">Permit</a>&lt;$K&gt;</code> directly.


<pre><code><b>public</b> <b>macro</b> <b>fun</b> <a href="../haneul/scratch.md#haneul_scratch_internal_exists_with_type">internal_exists_with_type</a>&lt;$K: <b>copy</b>, drop, $V: drop&gt;($ctx: &<a href="../haneul/tx_context.md#haneul_tx_context_TxContext">haneul::tx_context::TxContext</a>, $key: $K): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>macro</b> <b>fun</b> <a href="../haneul/scratch.md#haneul_scratch_internal_exists_with_type">internal_exists_with_type</a>&lt;$K: <b>copy</b> + drop, $V: drop&gt;(
    $ctx: &TxContext,
    $key: $K,
): bool {
    <a href="../haneul/scratch.md#haneul_scratch_exists_with_type">exists_with_type</a>&lt;$K, $V&gt;($ctx, <a href="../haneul/scratch.md#haneul_scratch_permit">permit</a>(internal::permit&lt;$K&gt;()), $key)
}
</code></pre>



</details>

<a name="haneul_scratch_internal_read_opt"></a>

## Macro function `internal_read_opt`

A wrapper for <code><a href="../haneul/scratch.md#haneul_scratch_read_opt">read_opt</a></code> that constructs the <code><a href="../haneul/scratch.md#haneul_scratch_Permit">Permit</a>&lt;$K&gt;</code> directly.
Aborts with <code><a href="../haneul/scratch.md#haneul_scratch_EEntryTypeMismatch">EEntryTypeMismatch</a></code> if the entry exists, but its value is not of type <code>$V</code>.


<pre><code><b>public</b> <b>macro</b> <b>fun</b> <a href="../haneul/scratch.md#haneul_scratch_internal_read_opt">internal_read_opt</a>&lt;$K: <b>copy</b>, drop, $V: <b>copy</b>, drop&gt;($ctx: &<a href="../haneul/tx_context.md#haneul_tx_context_TxContext">haneul::tx_context::TxContext</a>, $key: $K): <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;$V&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>macro</b> <b>fun</b> <a href="../haneul/scratch.md#haneul_scratch_internal_read_opt">internal_read_opt</a>&lt;$K: <b>copy</b> + drop, $V: <b>copy</b> + drop&gt;(
    $ctx: &TxContext,
    $key: $K,
): Option&lt;$V&gt; {
    <a href="../haneul/scratch.md#haneul_scratch_read_opt">read_opt</a>&lt;$K, $V&gt;($ctx, <a href="../haneul/scratch.md#haneul_scratch_permit">permit</a>(internal::permit&lt;$K&gt;()), $key)
}
</code></pre>



</details>

<a name="haneul_scratch_internal_remove_opt"></a>

## Macro function `internal_remove_opt`

A wrapper for <code><a href="../haneul/scratch.md#haneul_scratch_remove_opt">remove_opt</a></code> that constructs the <code><a href="../haneul/scratch.md#haneul_scratch_Permit">Permit</a>&lt;$K&gt;</code> directly.
Aborts with <code><a href="../haneul/scratch.md#haneul_scratch_EEntryTypeMismatch">EEntryTypeMismatch</a></code> if the entry exists, but its value is not of type <code>$V</code>.


<pre><code><b>public</b> <b>macro</b> <b>fun</b> <a href="../haneul/scratch.md#haneul_scratch_internal_remove_opt">internal_remove_opt</a>&lt;$K: <b>copy</b>, drop, $V: drop&gt;($ctx: &<b>mut</b> <a href="../haneul/tx_context.md#haneul_tx_context_TxContext">haneul::tx_context::TxContext</a>, $key: $K): <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;$V&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>macro</b> <b>fun</b> <a href="../haneul/scratch.md#haneul_scratch_internal_remove_opt">internal_remove_opt</a>&lt;$K: <b>copy</b> + drop, $V: drop&gt;(
    $ctx: &<b>mut</b> TxContext,
    $key: $K,
): Option&lt;$V&gt; {
    <a href="../haneul/scratch.md#haneul_scratch_remove_opt">remove_opt</a>&lt;$K, $V&gt;($ctx, <a href="../haneul/scratch.md#haneul_scratch_permit">permit</a>(internal::permit&lt;$K&gt;()), $key)
}
</code></pre>



</details>

<a name="haneul_scratch_internal_replace"></a>

## Macro function `internal_replace`

A wrapper for <code><a href="../haneul/scratch.md#haneul_scratch_replace">replace</a></code> that constructs the <code><a href="../haneul/scratch.md#haneul_scratch_Permit">Permit</a>&lt;$K&gt;</code> directly.
Aborts with <code><a href="../haneul/scratch.md#haneul_scratch_EEntryTypeMismatch">EEntryTypeMismatch</a></code> if the entry exists, but its value is not of type <code>$VOld</code>.


<pre><code><b>public</b> <b>macro</b> <b>fun</b> <a href="../haneul/scratch.md#haneul_scratch_internal_replace">internal_replace</a>&lt;$K: <b>copy</b>, drop, $VNew: drop, $VOld: drop&gt;($ctx: &<b>mut</b> <a href="../haneul/tx_context.md#haneul_tx_context_TxContext">haneul::tx_context::TxContext</a>, $key: $K, $value: $VNew): <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;$VOld&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>macro</b> <b>fun</b> <a href="../haneul/scratch.md#haneul_scratch_internal_replace">internal_replace</a>&lt;$K: <b>copy</b> + drop, $VNew: drop, $VOld: drop&gt;(
    $ctx: &<b>mut</b> TxContext,
    $key: $K,
    $value: $VNew,
): Option&lt;$VOld&gt; {
    <a href="../haneul/scratch.md#haneul_scratch_replace">replace</a>&lt;$K, $VNew, $VOld&gt;($ctx, <a href="../haneul/scratch.md#haneul_scratch_permit">permit</a>(internal::permit&lt;$K&gt;()), $key, $value)
}
</code></pre>



</details>

<a name="haneul_scratch_hash_type_and_key"></a>

## Function `hash_type_and_key`

Hashes the type and value of <code>k</code> against <code><a href="../haneul/scratch.md#haneul_scratch_DUMMY_ROOT">DUMMY_ROOT</a></code> to produce the address identifying its
scratch entry.


<pre><code><b>fun</b> <a href="../haneul/scratch.md#haneul_scratch_hash_type_and_key">hash_type_and_key</a>&lt;K: <b>copy</b>, drop&gt;(k: K): <b>address</b>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../haneul/scratch.md#haneul_scratch_hash_type_and_key">hash_type_and_key</a>&lt;K: <b>copy</b> + drop&gt;(k: K): <b>address</b> {
    <a href="../haneul/dynamic_field.md#haneul_dynamic_field_hash_type_and_key">haneul::dynamic_field::hash_type_and_key</a>(<a href="../haneul/scratch.md#haneul_scratch_DUMMY_ROOT">DUMMY_ROOT</a>, k)
}
</code></pre>



</details>

<a name="haneul_scratch_add_impl"></a>

## Function `add_impl`

Aborts with <code><a href="../haneul/scratch.md#haneul_scratch_EEntryAlreadyExists">EEntryAlreadyExists</a></code> if there is an entry already for <code>key</code>, regardless of the
type of <code>V</code>


<pre><code><b>fun</b> <a href="../haneul/scratch.md#haneul_scratch_add_impl">add_impl</a>&lt;V: drop&gt;(key: <b>address</b>, value: V)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>native</b> <b>fun</b> <a href="../haneul/scratch.md#haneul_scratch_add_impl">add_impl</a>&lt;V: drop&gt;(key: <b>address</b>, value: V);
</code></pre>



</details>

<a name="haneul_scratch_read_impl"></a>

## Function `read_impl`

Aborts with <code><a href="../haneul/scratch.md#haneul_scratch_EEntryDoesNotExist">EEntryDoesNotExist</a></code> if there is no entry for <code>key</code>.
Aborts with <code><a href="../haneul/scratch.md#haneul_scratch_EEntryTypeMismatch">EEntryTypeMismatch</a></code> if there is an entry for <code>key</code> but it is not of type <code>V</code>.


<pre><code><b>fun</b> <a href="../haneul/scratch.md#haneul_scratch_read_impl">read_impl</a>&lt;V: <b>copy</b>, drop&gt;(key: <b>address</b>): V
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>native</b> <b>fun</b> <a href="../haneul/scratch.md#haneul_scratch_read_impl">read_impl</a>&lt;V: <b>copy</b> + drop&gt;(key: <b>address</b>): V;
</code></pre>



</details>

<a name="haneul_scratch_remove_impl"></a>

## Function `remove_impl`

Aborts with <code><a href="../haneul/scratch.md#haneul_scratch_EEntryDoesNotExist">EEntryDoesNotExist</a></code> if there is no entry for <code>key</code>.
Aborts with <code><a href="../haneul/scratch.md#haneul_scratch_EEntryTypeMismatch">EEntryTypeMismatch</a></code> if there is an entry for <code>key</code> but it is not of type <code>V</code>.


<pre><code><b>fun</b> <a href="../haneul/scratch.md#haneul_scratch_remove_impl">remove_impl</a>&lt;V: drop&gt;(key: <b>address</b>): V
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>native</b> <b>fun</b> <a href="../haneul/scratch.md#haneul_scratch_remove_impl">remove_impl</a>&lt;V: drop&gt;(key: <b>address</b>): V;
</code></pre>



</details>

<a name="haneul_scratch_exists_impl"></a>

## Function `exists_impl`



<pre><code><b>fun</b> <a href="../haneul/scratch.md#haneul_scratch_exists_impl">exists_impl</a>(key: <b>address</b>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>native</b> <b>fun</b> <a href="../haneul/scratch.md#haneul_scratch_exists_impl">exists_impl</a>(key: <b>address</b>): bool;
</code></pre>



</details>

<a name="haneul_scratch_exists_with_type_impl"></a>

## Function `exists_with_type_impl`



<pre><code><b>fun</b> <a href="../haneul/scratch.md#haneul_scratch_exists_with_type_impl">exists_with_type_impl</a>&lt;V: drop&gt;(key: <b>address</b>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>native</b> <b>fun</b> <a href="../haneul/scratch.md#haneul_scratch_exists_with_type_impl">exists_with_type_impl</a>&lt;V: drop&gt;(key: <b>address</b>): bool;
</code></pre>



</details>
