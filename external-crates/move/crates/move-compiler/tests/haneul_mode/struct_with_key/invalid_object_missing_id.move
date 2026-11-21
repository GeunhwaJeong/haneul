module a::test {
    use haneul::object::UID;

    struct S has key {
        id: UID,
    }

    fun make(): S {
        S {}
    }
}

module haneul::object {
    struct UID has store {
        id: address,
    }
}

module haneul::transfer {
    public fun transfer<T: key>(_: T, _: address) {
        abort 0
    }
}
