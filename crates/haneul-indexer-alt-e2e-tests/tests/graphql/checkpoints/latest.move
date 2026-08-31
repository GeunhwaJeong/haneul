// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

//# init --protocol-version 108 --accounts A --simulator

//# run-graphql
{
  checkpoint { sequenceNumber }
}

//# create-checkpoint

//# run-graphql
{
  checkpoint { sequenceNumber }
}

//# create-checkpoint

//# run-graphql
{
  checkpoint { sequenceNumber }
}

//# run-graphql
{
  # Test that explicit null defaults to latest checkpoint
  checkpoint(sequenceNumber: null) { sequenceNumber }
}
