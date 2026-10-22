// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    BcsSerialisationError(#[from] bcs::Error),
    #[error("Data error: {0}")]
    DataError(String),
    #[error("Invalid signature")]
    InvalidSignature,
}
