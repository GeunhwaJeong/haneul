// not allowed since C is not packed with a fresh UID
module a::a {
    use haneul::object::UID;

    struct A has key {
        id: UID,
    }
}

module b::b {
    use a::a::A;
    use haneul::object::UID;

    struct B has key {
        id: UID,
    }

    public fun no(b: B): A {
        let B { id } = b;
        A { id }
    }
}

module haneul::object {
    struct UID has store {
        id: address,
    }
}
