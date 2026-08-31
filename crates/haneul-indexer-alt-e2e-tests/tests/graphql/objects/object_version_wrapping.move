// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

//# init --protocol-version 108 --accounts A --addresses P=0x0 --simulator
// Track the versions of an object as it is being wrapped, unwrapped, and deleted.

//# publish
module P::M {
  use haneul::coin::Coin;
  use haneul::haneul::HANEUL;

  public struct Wrapper has key, store {
    id: UID,
    coin: Coin<HANEUL>,
  }

  public fun wrap(coin: Coin<HANEUL>, ctx: &mut TxContext): Wrapper {
    Wrapper { id: object::new(ctx), coin }
  }

  public fun unwrap(wrapper: Wrapper): Coin<HANEUL> {
    let Wrapper { id, coin } = wrapper;
    id.delete();
    coin
  }
}

//# programmable --sender A --inputs 42 @A
//> 0: SplitCoins(Gas, [Input(0)]);
//> 1: TransferObjects([Result(0)], Input(1))

//# programmable --sender A --inputs object(2,0) 1
//> 0: SplitCoins(Input(0), [Input(1)]);
//> 1: MergeCoins(Gas, [Result(0)])

//# programmable --sender A --inputs object(2,0) @A
//> 0: P::M::wrap(Input(0));
//> 1: TransferObjects([Result(0)], Input(1))

//# programmable --sender A --inputs object(4,0) @A
//> 0: P::M::unwrap(Input(0));
//> 1: TransferObjects([Result(0)], Input(1))

//# programmable --sender A --inputs object(2,0)
//> 0: MergeCoins(Gas, [Input(0)])

//# create-checkpoint

//# run-graphql
{
  objectVersions(address: "@{obj_2_0}") {
    pageInfo {
      hasNextPage
    }
    nodes {
      version
      asMoveObject {
        contents {
          json
        }
      }
    }
  }
}
