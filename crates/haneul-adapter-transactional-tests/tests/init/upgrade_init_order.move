// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

// tests the order for modules is predictable for upgrades

//# init --addresses v0=0x0 v1=0x0 --accounts A

//# publish --upgradeable --sender A
module v0::m {
}

//# stage-package
module v1::m {
}
module v1::b {
    fun init(_: &mut TxContext) {
        abort 0xb
    }
}
module v1::a {
    fun init(_: &mut TxContext) {
        abort 0xa
    }
}

//# programmable --sender A --inputs 10 @A object(1,1) 0u8 digest(v1)
// 'a' will abort first since it is earlier in the module list
//> 0: haneul::package::authorize_upgrade(Input(2), Input(3), Input(4));
//> 1: Upgrade(v1, [haneul,std], v0, Result(0));
//> haneul::package::commit_upgrade(Input(2), Result(1))
