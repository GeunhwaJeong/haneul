// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

// Test basic coin transfer

//# init --accounts A B C

//# programmable --sender B --inputs 10 @B
//> SplitCoins(Gas, [Input(0)]);
//> TransferObjects([Result(0)], Input(1))

//# view-object 1,0

//# run haneul::pay::split_and_transfer --type-args haneul::haneul::HANEUL --args object(1,0) 10 @A --sender B

//# view-object 1,0

//# view-object 3,0

//# run haneul::pay::split_and_transfer --type-args haneul::haneul::HANEUL --args object(1,0) 0 @C --sender A

//# view-object 1,0
