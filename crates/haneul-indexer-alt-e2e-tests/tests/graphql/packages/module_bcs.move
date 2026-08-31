// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

//# init --protocol-version 108 --accounts A --addresses P1=0x0 --simulator

//# publish --upgradeable --sender A
module P1::M { }

//# create-checkpoint

//# run-graphql
{
  package(address: "@{P1}") {
    moduleBcs
  }
}