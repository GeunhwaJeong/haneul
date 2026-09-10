// invalid cases. Clock and Random must be passed by immutable reference.
// TxContext can appear multiple times but uniquely if mutable. And cannot be owned.

module a::m {
    const ERR: u64 = 0;
    fun no_clock_mut(_: &mut haneul::clock::Clock) {
        abort ERR
    }
    fun no_clock_val(_: haneul::clock::Clock) {
        abort ERR
    }
    fun no_random_mut(_: &mut haneul::random::Random) {
        abort ERR
    }
    fun no_random_val(_: haneul::random::Random) {
        abort ERR
    }

    use haneul::tx_context::TxContext;
    fun two_mut_ctx(_: &mut TxContext, _: &mut TxContext) {
        abort ERR
    }
    fun mut_and_imm_ctx(_: &mut TxContext, _: &TxContext) {
        abort ERR
    }
    fun owned_ctx(_: TxContext, _: &mut TxContext, _: &mut TxContext) {
        abort ERR
    }

    // TxContext cannot appear in return position of a public or entry function
    public fun ret_ctx_val(): TxContext {
        abort ERR
    }
    #[allow(lint(prefer_mut_tx_context))]
    public fun ret_ctx_imm(ctx: &TxContext): &TxContext {
        ctx
    }
    public fun ret_ctx_mut(ctx: &mut TxContext): &mut TxContext {
        ctx
    }
    public fun ret_ctx_tuple(ctx: &mut TxContext): (u64, &mut TxContext) {
        (0, ctx)
    }
    entry fun ret_ctx_entry(): TxContext {
        abort ERR
    }

}

module haneul::clock {
    const ERR: u64 = 0;

    struct Clock has key {
        id: haneul::object::UID,
    }

    // no warning
    fun test_clock_mut(_: &mut Clock) {}
    // no warning
    fun test_clock_val(_: Clock) { abort ERR }
}

module haneul::random {
    const ERR: u64 = 0;

    struct Random has key {
        id: haneul::object::UID,
    }

    // no warning
    fun test_random_mut(_: &mut Random) {}
    // no warning
    fun test_random_val(_: Random) { abort ERR }
}


module haneul::object {
    struct UID has store {
        id: address,
    }
}

module haneul::tx_context {
    struct TxContext has drop {}
}
