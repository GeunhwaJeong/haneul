// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

use crate::displays::Pretty;
use haneul_json_rpc_types::HaneulExecutionStatus::{self, Failure, Success};
use haneul_types::execution_status::{ExecutionFailure, ExecutionStatus};
use std::fmt::{Display, Formatter};

impl Display for Pretty<'_, HaneulExecutionStatus> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let Pretty(status) = self;

        let output = match status {
            Success => "success".to_string(),
            Failure { error } => format!("failed due to {error}"),
        };

        write!(f, "{}", output)
    }
}

impl Display for Pretty<'_, ExecutionStatus> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let Pretty(status) = self;

        let output = match status {
            ExecutionStatus::Success => "success".to_string(),
            ExecutionStatus::Failure(ExecutionFailure { error, command }) => {
                if let Some(command) = command {
                    format!("failed due to {error:?} in command {command}")
                } else {
                    format!("failed due to {error:?}")
                }
            }
        };

        write!(f, "{}", output)
    }
}
