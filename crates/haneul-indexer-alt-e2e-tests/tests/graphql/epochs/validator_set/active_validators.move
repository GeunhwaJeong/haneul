// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

//# init --protocol-version 108 --accounts A --simulator

//# run-graphql
{
  epoch(epochId: 0) {
    epochId
    validatorSet {
      activeValidators {
        pageInfo {
         hasPreviousPage
         hasNextPage
         startCursor
         endCursor
        }

        nodes {
          contents {
            json

            address: extract(path: "metadata.haneul_address") {
              asAddress {
                balances {
                  nodes {
                    coinType { repr }
                    totalBalance
                  }
                }

                objects {
                  nodes {
                    contents {
                      type { repr }
                      json
                    }
                  }
                }
              }
            }
          }

          atRisk
        }
      }
    }
  }
}
