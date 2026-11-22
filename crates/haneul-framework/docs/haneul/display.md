---
title: Module `haneul::display`
---

Defines a Display struct which defines the way an Object
should be displayed. The intention is to keep data as independent
from its display as possible, protecting the development process
and keeping it separate from the ecosystem agreements.

Each of the fields of the Display object should allow for pattern
substitution and filling-in the pieces using the data from the object T.

More entry functions might be added in the future depending on the use cases.


-  [Struct `Display`](#haneul_display_Display)
-  [Struct `DisplayCreated`](#haneul_display_DisplayCreated)
-  [Struct `VersionUpdated`](#haneul_display_VersionUpdated)
-  [Constants](#@Constants_0)
-  [Function `new`](#haneul_display_new)
-  [Function `new_with_fields`](#haneul_display_new_with_fields)
-  [Function `create_and_keep`](#haneul_display_create_and_keep)
-  [Function `update_version`](#haneul_display_update_version)
-  [Function `add`](#haneul_display_add)
-  [Function `add_multiple`](#haneul_display_add_multiple)
-  [Function `edit`](#haneul_display_edit)
-  [Function `remove`](#haneul_display_remove)
-  [Function `is_authorized`](#haneul_display_is_authorized)
-  [Function `version`](#haneul_display_version)
-  [Function `fields`](#haneul_display_fields)
-  [Function `destroy`](#haneul_display_destroy)
-  [Function `add_internal`](#haneul_display_add_internal)


<pre><code><b>use</b> <a href="../haneul/accumulator.md#haneul_accumulator">haneul::accumulator</a>;
<b>use</b> <a href="../haneul/accumulator_settlement.md#haneul_accumulator_settlement">haneul::accumulator_settlement</a>;
<b>use</b> <a href="../haneul/address.md#haneul_address">haneul::address</a>;
<b>use</b> <a href="../haneul/bcs.md#haneul_bcs">haneul::bcs</a>;
<b>use</b> <a href="../haneul/dynamic_field.md#haneul_dynamic_field">haneul::dynamic_field</a>;
<b>use</b> <a href="../haneul/event.md#haneul_event">haneul::event</a>;
<b>use</b> <a href="../haneul/hash.md#haneul_hash">haneul::hash</a>;
<b>use</b> <a href="../haneul/hex.md#haneul_hex">haneul::hex</a>;
<b>use</b> <a href="../haneul/object.md#haneul_object">haneul::object</a>;
<b>use</b> <a href="../haneul/package.md#haneul_package">haneul::package</a>;
<b>use</b> <a href="../haneul/party.md#haneul_party">haneul::party</a>;
<b>use</b> <a href="../haneul/transfer.md#haneul_transfer">haneul::transfer</a>;
<b>use</b> <a href="../haneul/tx_context.md#haneul_tx_context">haneul::tx_context</a>;
<b>use</b> <a href="../haneul/types.md#haneul_types">haneul::types</a>;
<b>use</b> <a href="../haneul/vec_map.md#haneul_vec_map">haneul::vec_map</a>;
<b>use</b> <a href="../std/address.md#std_address">std::address</a>;
<b>use</b> <a href="../std/ascii.md#std_ascii">std::ascii</a>;
<b>use</b> <a href="../std/bcs.md#std_bcs">std::bcs</a>;
<b>use</b> <a href="../std/option.md#std_option">std::option</a>;
<b>use</b> <a href="../std/string.md#std_string">std::string</a>;
<b>use</b> <a href="../std/type_name.md#std_type_name">std::type_name</a>;
<b>use</b> <a href="../std/vector.md#std_vector">std::vector</a>;
</code></pre>



<a name="haneul_display_Display"></a>

## Struct `Display`

The Display<T> object. Defines the way a T instance should be
displayed. Display object can only be created and modified with
a PublisherCap, making sure that the rules are set by the owner
of the type.

Each of the display properties should support patterns outside
of the system, making it simpler to customize Display based
on the property values of an Object.
```
// Example of a display object
Display<0x...::capy::Capy> {
 fields:
   <name, "Capy { genes }">
   <link, "https://capy.art/capy/{ id }">
   <image, "https://api.capy.art/capy/{ id }/svg">
   <description, "Lovely Capy, one of many">
}
```

Uses only String type due to external-facing nature of the object,
the property names have a priority over their types.


<pre><code><b>public</b> <b>struct</b> <a href="../haneul/display.md#haneul_display_Display">Display</a>&lt;<b>phantom</b> T: key&gt; <b>has</b> key, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>id: <a href="../haneul/object.md#haneul_object_UID">haneul::object::UID</a></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../haneul/display.md#haneul_display_fields">fields</a>: <a href="../haneul/vec_map.md#haneul_vec_map_VecMap">haneul::vec_map::VecMap</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>, <a href="../std/string.md#std_string_String">std::string::String</a>&gt;</code>
</dt>
<dd>
 Contains fields for display. Currently supported
 fields are: name, link, image and description.
</dd>
<dt>
<code><a href="../haneul/display.md#haneul_display_version">version</a>: u16</code>
</dt>
<dd>
 Version that can only be updated manually by the Publisher.
</dd>
</dl>


</details>

<a name="haneul_display_DisplayCreated"></a>

## Struct `DisplayCreated`

Event: emitted when a new Display object has been created for type T.
Type signature of the event corresponds to the type while id serves for
the discovery.

Since Haneul RPC supports querying events by type, finding a Display for the T
would be as simple as looking for the first event with <code><a href="../haneul/display.md#haneul_display_Display">Display</a>&lt;T&gt;</code>.


<pre><code><b>public</b> <b>struct</b> <a href="../haneul/display.md#haneul_display_DisplayCreated">DisplayCreated</a>&lt;<b>phantom</b> T: key&gt; <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>id: <a href="../haneul/object.md#haneul_object_ID">haneul::object::ID</a></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="haneul_display_VersionUpdated"></a>

## Struct `VersionUpdated`

Version of Display got updated -


<pre><code><b>public</b> <b>struct</b> <a href="../haneul/display.md#haneul_display_VersionUpdated">VersionUpdated</a>&lt;<b>phantom</b> T: key&gt; <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>id: <a href="../haneul/object.md#haneul_object_ID">haneul::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../haneul/display.md#haneul_display_version">version</a>: u16</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../haneul/display.md#haneul_display_fields">fields</a>: <a href="../haneul/vec_map.md#haneul_vec_map_VecMap">haneul::vec_map::VecMap</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>, <a href="../std/string.md#std_string_String">std::string::String</a>&gt;</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="haneul_display_ENotOwner"></a>

For when T does not belong to the package <code>Publisher</code>.


<pre><code><b>const</b> <a href="../haneul/display.md#haneul_display_ENotOwner">ENotOwner</a>: u64 = 0;
</code></pre>



<a name="haneul_display_EVecLengthMismatch"></a>

For when vectors passed into one of the multiple insert functions
don't match in their lengths.


<pre><code><b>const</b> <a href="../haneul/display.md#haneul_display_EVecLengthMismatch">EVecLengthMismatch</a>: u64 = 1;
</code></pre>



<a name="haneul_display_ENoLongerSupported"></a>

Creation of Display V1 is no longer available, see <code><a href="../haneul/display_registry.md#haneul_display_registry">display_registry</a></code> for
options for creating Display V2.


<pre><code><b>const</b> <a href="../haneul/display.md#haneul_display_ENoLongerSupported">ENoLongerSupported</a>: u64 = 2;
</code></pre>



<a name="haneul_display_new"></a>

## Function `new`

Create an empty Display object. It can either be shared empty or filled
with data right away via cheaper <code>set_owned</code> method.


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/display.md#haneul_display_new">new</a>&lt;T: key&gt;(pub: &<a href="../haneul/package.md#haneul_package_Publisher">haneul::package::Publisher</a>, ctx: &<b>mut</b> <a href="../haneul/tx_context.md#haneul_tx_context_TxContext">haneul::tx_context::TxContext</a>): <a href="../haneul/display.md#haneul_display_Display">haneul::display::Display</a>&lt;T&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/display.md#haneul_display_new">new</a>&lt;T: key&gt;(pub: &Publisher, ctx: &<b>mut</b> TxContext): <a href="../haneul/display.md#haneul_display_Display">Display</a>&lt;T&gt; {
    <b>abort</b> <a href="../haneul/display.md#haneul_display_ENoLongerSupported">ENoLongerSupported</a>
}
</code></pre>



</details>

<a name="haneul_display_new_with_fields"></a>

## Function `new_with_fields`

Create a new Display<T> object with a set of fields.


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/display.md#haneul_display_new_with_fields">new_with_fields</a>&lt;T: key&gt;(_pub: &<a href="../haneul/package.md#haneul_package_Publisher">haneul::package::Publisher</a>, _fields: vector&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;, _values: vector&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;, _ctx: &<b>mut</b> <a href="../haneul/tx_context.md#haneul_tx_context_TxContext">haneul::tx_context::TxContext</a>): <a href="../haneul/display.md#haneul_display_Display">haneul::display::Display</a>&lt;T&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/display.md#haneul_display_new_with_fields">new_with_fields</a>&lt;T: key&gt;(
    _pub: &Publisher,
    _fields: vector&lt;String&gt;,
    _values: vector&lt;String&gt;,
    _ctx: &<b>mut</b> TxContext,
): <a href="../haneul/display.md#haneul_display_Display">Display</a>&lt;T&gt; {
    <b>abort</b> <a href="../haneul/display.md#haneul_display_ENoLongerSupported">ENoLongerSupported</a>
}
</code></pre>



</details>

<a name="haneul_display_create_and_keep"></a>

## Function `create_and_keep`

Create a new empty Display<T> object and keep it.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../haneul/display.md#haneul_display_create_and_keep">create_and_keep</a>&lt;T: key&gt;(pub: &<a href="../haneul/package.md#haneul_package_Publisher">haneul::package::Publisher</a>, ctx: &<b>mut</b> <a href="../haneul/tx_context.md#haneul_tx_context_TxContext">haneul::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../haneul/display.md#haneul_display_create_and_keep">create_and_keep</a>&lt;T: key&gt;(pub: &Publisher, ctx: &<b>mut</b> TxContext) {
    <b>abort</b> <a href="../haneul/display.md#haneul_display_ENoLongerSupported">ENoLongerSupported</a>
}
</code></pre>



</details>

<a name="haneul_display_update_version"></a>

## Function `update_version`

Manually bump the version and emit an event with the updated version's contents.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../haneul/display.md#haneul_display_update_version">update_version</a>&lt;T: key&gt;(<a href="../haneul/display.md#haneul_display">display</a>: &<b>mut</b> <a href="../haneul/display.md#haneul_display_Display">haneul::display::Display</a>&lt;T&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../haneul/display.md#haneul_display_update_version">update_version</a>&lt;T: key&gt;(<a href="../haneul/display.md#haneul_display">display</a>: &<b>mut</b> <a href="../haneul/display.md#haneul_display_Display">Display</a>&lt;T&gt;) {
    <a href="../haneul/display.md#haneul_display">display</a>.<a href="../haneul/display.md#haneul_display_version">version</a> = <a href="../haneul/display.md#haneul_display">display</a>.<a href="../haneul/display.md#haneul_display_version">version</a> + 1;
    <a href="../haneul/event.md#haneul_event_emit">event::emit</a>(<a href="../haneul/display.md#haneul_display_VersionUpdated">VersionUpdated</a>&lt;T&gt; {
        <a href="../haneul/display.md#haneul_display_version">version</a>: <a href="../haneul/display.md#haneul_display">display</a>.<a href="../haneul/display.md#haneul_display_version">version</a>,
        <a href="../haneul/display.md#haneul_display_fields">fields</a>: *&<a href="../haneul/display.md#haneul_display">display</a>.<a href="../haneul/display.md#haneul_display_fields">fields</a>,
        id: <a href="../haneul/display.md#haneul_display">display</a>.id.to_inner(),
    })
}
</code></pre>



</details>

<a name="haneul_display_add"></a>

## Function `add`

Sets a custom <code>name</code> field with the <code>value</code>.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../haneul/display.md#haneul_display_add">add</a>&lt;T: key&gt;(self: &<b>mut</b> <a href="../haneul/display.md#haneul_display_Display">haneul::display::Display</a>&lt;T&gt;, name: <a href="../std/string.md#std_string_String">std::string::String</a>, value: <a href="../std/string.md#std_string_String">std::string::String</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../haneul/display.md#haneul_display_add">add</a>&lt;T: key&gt;(self: &<b>mut</b> <a href="../haneul/display.md#haneul_display_Display">Display</a>&lt;T&gt;, name: String, value: String) {
    self.<a href="../haneul/display.md#haneul_display_add_internal">add_internal</a>(name, value)
}
</code></pre>



</details>

<a name="haneul_display_add_multiple"></a>

## Function `add_multiple`

Sets multiple <code><a href="../haneul/display.md#haneul_display_fields">fields</a></code> with <code>values</code>.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../haneul/display.md#haneul_display_add_multiple">add_multiple</a>&lt;T: key&gt;(self: &<b>mut</b> <a href="../haneul/display.md#haneul_display_Display">haneul::display::Display</a>&lt;T&gt;, <a href="../haneul/display.md#haneul_display_fields">fields</a>: vector&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;, values: vector&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../haneul/display.md#haneul_display_add_multiple">add_multiple</a>&lt;T: key&gt;(
    self: &<b>mut</b> <a href="../haneul/display.md#haneul_display_Display">Display</a>&lt;T&gt;,
    <a href="../haneul/display.md#haneul_display_fields">fields</a>: vector&lt;String&gt;,
    values: vector&lt;String&gt;,
) {
    <b>let</b> len = <a href="../haneul/display.md#haneul_display_fields">fields</a>.length();
    <b>assert</b>!(len == values.length(), <a href="../haneul/display.md#haneul_display_EVecLengthMismatch">EVecLengthMismatch</a>);
    <a href="../haneul/display.md#haneul_display_fields">fields</a>.zip_do!(values, |field, value| self.<a href="../haneul/display.md#haneul_display_add_internal">add_internal</a>(field, value));
}
</code></pre>



</details>

<a name="haneul_display_edit"></a>

## Function `edit`

Change the value of the field.
TODO (long run): version changes;


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../haneul/display.md#haneul_display_edit">edit</a>&lt;T: key&gt;(self: &<b>mut</b> <a href="../haneul/display.md#haneul_display_Display">haneul::display::Display</a>&lt;T&gt;, name: <a href="../std/string.md#std_string_String">std::string::String</a>, value: <a href="../std/string.md#std_string_String">std::string::String</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../haneul/display.md#haneul_display_edit">edit</a>&lt;T: key&gt;(self: &<b>mut</b> <a href="../haneul/display.md#haneul_display_Display">Display</a>&lt;T&gt;, name: String, value: String) {
    <b>let</b> (_, _) = self.<a href="../haneul/display.md#haneul_display_fields">fields</a>.<a href="../haneul/display.md#haneul_display_remove">remove</a>(&name);
    self.<a href="../haneul/display.md#haneul_display_add_internal">add_internal</a>(name, value)
}
</code></pre>



</details>

<a name="haneul_display_remove"></a>

## Function `remove`

Remove the key from the Display.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../haneul/display.md#haneul_display_remove">remove</a>&lt;T: key&gt;(self: &<b>mut</b> <a href="../haneul/display.md#haneul_display_Display">haneul::display::Display</a>&lt;T&gt;, name: <a href="../std/string.md#std_string_String">std::string::String</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../haneul/display.md#haneul_display_remove">remove</a>&lt;T: key&gt;(self: &<b>mut</b> <a href="../haneul/display.md#haneul_display_Display">Display</a>&lt;T&gt;, name: String) {
    self.<a href="../haneul/display.md#haneul_display_fields">fields</a>.<a href="../haneul/display.md#haneul_display_remove">remove</a>(&name);
}
</code></pre>



</details>

<a name="haneul_display_is_authorized"></a>

## Function `is_authorized`

Authorization check; can be performed externally to implement protection rules for Display.


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/display.md#haneul_display_is_authorized">is_authorized</a>&lt;T: key&gt;(pub: &<a href="../haneul/package.md#haneul_package_Publisher">haneul::package::Publisher</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/display.md#haneul_display_is_authorized">is_authorized</a>&lt;T: key&gt;(pub: &Publisher): bool {
    pub.from_package&lt;T&gt;()
}
</code></pre>



</details>

<a name="haneul_display_version"></a>

## Function `version`

Read the <code><a href="../haneul/display.md#haneul_display_version">version</a></code> field.


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/display.md#haneul_display_version">version</a>&lt;T: key&gt;(d: &<a href="../haneul/display.md#haneul_display_Display">haneul::display::Display</a>&lt;T&gt;): u16
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/display.md#haneul_display_version">version</a>&lt;T: key&gt;(d: &<a href="../haneul/display.md#haneul_display_Display">Display</a>&lt;T&gt;): u16 {
    d.<a href="../haneul/display.md#haneul_display_version">version</a>
}
</code></pre>



</details>

<a name="haneul_display_fields"></a>

## Function `fields`

Read the <code><a href="../haneul/display.md#haneul_display_fields">fields</a></code> field.


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/display.md#haneul_display_fields">fields</a>&lt;T: key&gt;(d: &<a href="../haneul/display.md#haneul_display_Display">haneul::display::Display</a>&lt;T&gt;): &<a href="../haneul/vec_map.md#haneul_vec_map_VecMap">haneul::vec_map::VecMap</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>, <a href="../std/string.md#std_string_String">std::string::String</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/display.md#haneul_display_fields">fields</a>&lt;T: key&gt;(d: &<a href="../haneul/display.md#haneul_display_Display">Display</a>&lt;T&gt;): &VecMap&lt;String, String&gt; {
    &d.<a href="../haneul/display.md#haneul_display_fields">fields</a>
}
</code></pre>



</details>

<a name="haneul_display_destroy"></a>

## Function `destroy`

Allow destroying legacy display objects.


<pre><code><b>public</b>(<a href="../haneul/package.md#haneul_package">package</a>) <b>fun</b> <a href="../haneul/display.md#haneul_display_destroy">destroy</a>&lt;T: key&gt;(<a href="../haneul/display.md#haneul_display">display</a>: <a href="../haneul/display.md#haneul_display_Display">haneul::display::Display</a>&lt;T&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../haneul/package.md#haneul_package">package</a>) <b>fun</b> <a href="../haneul/display.md#haneul_display_destroy">destroy</a>&lt;T: key&gt;(<a href="../haneul/display.md#haneul_display">display</a>: <a href="../haneul/display.md#haneul_display_Display">Display</a>&lt;T&gt;) {
    <b>let</b> <a href="../haneul/display.md#haneul_display_Display">Display</a> { id, .. } = <a href="../haneul/display.md#haneul_display">display</a>;
    id.delete();
}
</code></pre>



</details>

<a name="haneul_display_add_internal"></a>

## Function `add_internal`

Private method for inserting fields without security checks.


<pre><code><b>fun</b> <a href="../haneul/display.md#haneul_display_add_internal">add_internal</a>&lt;T: key&gt;(<a href="../haneul/display.md#haneul_display">display</a>: &<b>mut</b> <a href="../haneul/display.md#haneul_display_Display">haneul::display::Display</a>&lt;T&gt;, name: <a href="../std/string.md#std_string_String">std::string::String</a>, value: <a href="../std/string.md#std_string_String">std::string::String</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../haneul/display.md#haneul_display_add_internal">add_internal</a>&lt;T: key&gt;(<a href="../haneul/display.md#haneul_display">display</a>: &<b>mut</b> <a href="../haneul/display.md#haneul_display_Display">Display</a>&lt;T&gt;, name: String, value: String) {
    <a href="../haneul/display.md#haneul_display">display</a>.<a href="../haneul/display.md#haneul_display_fields">fields</a>.insert(name, value)
}
</code></pre>



</details>
