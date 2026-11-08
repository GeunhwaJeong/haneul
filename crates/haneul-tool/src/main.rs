// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

use clap::*;
use colored::Colorize;
use haneul_tool::commands::ToolCommand;
use haneul_types::exit_main;

// Define the `GIT_REVISION` and `VERSION` consts
bin_version::bin_version!();

// Same allocator as haneul-node. Formal snapshot restore runs many writer threads that
// allocate short-lived batch buffers next to values the tidehunter value cache keeps
// for millions of inserts; glibc malloc fragments badly on that pattern.
#[cfg(all(not(target_env = "msvc"), feature = "jemalloc"))]
#[global_allocator]
static JEMALLOC: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;

#[derive(Parser)]
#[command(
    name = "haneul-tool",
    about = "Debugging utilities for haneul",
    rename_all = "kebab-case",
    author,
    version = VERSION,
)]
struct App {
    #[command(subcommand)]
    command: ToolCommand,
}

#[tokio::main]
async fn main() {
    #[cfg(windows)]
    colored::control::set_virtual_terminal(true).unwrap();

    let app = App::parse();
    let (_guards, handle) = telemetry_subscribers::TelemetryConfig::new()
        .with_env()
        .init();

    exit_main!(app.command.execute(handle).await);
}
