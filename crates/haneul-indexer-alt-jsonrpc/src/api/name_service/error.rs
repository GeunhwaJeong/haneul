// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

#[derive(thiserror::Error, Debug)]
pub(super) enum Error {
    #[error("Domain not found: {0}")]
    NotFound(String),

    #[error(transparent)]
    NameService(haneul_name_service::NameServiceError),
}
