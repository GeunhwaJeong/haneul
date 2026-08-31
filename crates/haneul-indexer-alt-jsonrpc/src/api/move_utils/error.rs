// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

#[derive(thiserror::Error, Debug)]
pub(super) enum Error {
    #[error("Invalid Move identifier: {0:?}")]
    BadIdentifier(String),

    #[error("{0}")]
    NotFound(haneul_package_resolver::error::Error),

    #[error("Type resolution limit reached: {0}")]
    ResolutionLimit(haneul_package_resolver::error::Error),
}
