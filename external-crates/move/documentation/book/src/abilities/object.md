# Haneul Objects

For Haneul, `key` is used to signify an _object_. Objects the only way to store data in Haneul--allowing
the data to persist between transactions.

For more details, see the Haneul documentation on

- [The Object Model](https://docs.haneul.io/concepts/object-model)
- [Move Rules for Objects](https://docs.haneul.io/concepts/haneul-move-concepts#global-unique)
- [Transferring Objects](https://docs.haneul.io/concepts/transfers)

## Object Rules

An object is a [`struct`](../structs.md) with the [`key`](../abilities.md#key) ability. The first
field of the struct must be `id: haneul::object::UID`. This 32-byte field (a strongly typed wrapper
around an [`address`](../primitive-types/address.md)) is then used to uniquely identify the object.

Note that since `haneul::object::UID` has only the `store` ability (it does not have `copy` or `drop`),
no object has `copy` or `drop`.

## Transfer Rules

Objects can be have their ownership changed and transferred in the `haneul::transfer` module. Many
functions in the module have "public" and "private" variant, where the "private" variant can only be
called inside of the module that defines the object's type. The "public" variants can be called only
if the object has `store`.

For example if we had two objects `A` and `B` defined in the module `my_module`:

```
module a::my_module {
    public struct A has key {
        id: haneul::object::UID,
    }
    public struct B has key, store {
        id: haneul::object::UID,
    }
}
```

`A` can only be transferred using the `haneul::transfer::transfer` inside of `a::my_module`, while `B`
can be transferred anywhere using `haneul::transfer::public_transfer`. These rules are enforced by a
custom type system (bytecode verifier) rule in Haneul.
