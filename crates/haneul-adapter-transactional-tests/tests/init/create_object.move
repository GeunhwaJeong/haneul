// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

//# init --addresses Test=0x0

//# publish
module Test::M1 {
   public struct X has key {
       id: UID,
   }

   fun init(ctx: &mut TxContext) { 
       haneul::transfer::transfer(X { id: object::new(ctx) }, ctx.sender());
   }
}

//# view-object 1,1
