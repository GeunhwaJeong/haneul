// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

use haneul_config::NodeConfig;
use tokio::runtime::Runtime;

pub struct HaneulRuntimes {
    // Order in this struct is the order in which runtimes are stopped
    pub haneul_node: Runtime,
    pub metrics: Runtime,
}

impl HaneulRuntimes {
    pub fn new(_confg: &NodeConfig) -> Self {
        let haneul_node = tokio::runtime::Builder::new_multi_thread()
            .thread_name("haneul-node-runtime")
            .enable_all()
            .build()
            .unwrap();
        let metrics = tokio::runtime::Builder::new_multi_thread()
            .thread_name("metrics-runtime")
            .worker_threads(2)
            .enable_all()
            .build()
            .unwrap();

        Self {
            haneul_node,
            metrics,
        }
    }
}
