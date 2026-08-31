// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

//# init --protocol-version 108 --accounts A --simulator

//# advance-epoch

//# run-graphql
{
  checkpoint(sequenceNumber: 1) {
    query {
      epoch(epochId: 0) {
        startTimestamp
        endTimestamp
      }
    }
  }
}
