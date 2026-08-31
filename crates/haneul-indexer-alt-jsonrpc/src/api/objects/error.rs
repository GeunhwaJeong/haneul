// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

#[derive(thiserror::Error, Debug)]
pub(crate) enum Error {
    #[error("Pagination issue: {0}")]
    Pagination(#[from] crate::paginate::Error),

    #[error("Requested {requested} keys, exceeding maximum {max}")]
    TooManyKeys { requested: usize, max: usize },

    #[error("Multiple exclusions unsupported")]
    MultipleExclusions,
}
