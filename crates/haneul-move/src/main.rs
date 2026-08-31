// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

use std::path::PathBuf;

use clap::*;
use colored::Colorize;
use haneul_config::{HANEUL_CLIENT_CONFIG, haneul_config_dir};
use haneul_move::execute_move_command;
use haneul_sdk::wallet_context::WalletContext;
use haneul_types::exit_main;
use move_package_alt_compilation::build_config::BuildConfig as MoveBuildConfig;
use tracing::debug;

// Define the `GIT_REVISION` and `VERSION` consts
bin_version::bin_version!();

#[derive(Parser)]
#[clap(
    name = env!("CARGO_BIN_NAME"),
    about = "Haneul-Move CLI",
    rename_all = "kebab-case",
    author,
    version = VERSION,
)]
struct Args {
    /// Path to a package which the command should be run with respect to.
    #[clap(long = "path", short = 'p', global = true)]
    pub package_path: Option<PathBuf>,
    /// If true, run the Move bytecode verifier on the bytecode from a successful build
    #[clap(long, global = true)]
    pub run_bytecode_verifier: bool,
    /// If true, print build diagnostics to stderr--no printing if false
    #[clap(long, global = true)]
    pub print_diags_to_stderr: bool,
    /// Package build options
    #[clap(flatten)]
    pub build_config: MoveBuildConfig,
    /// Subcommands.
    #[clap(subcommand)]
    pub cmd: haneul_move::Command,
}

#[tokio::main]
async fn main() {
    #[cfg(windows)]
    colored::control::set_virtual_terminal(true).unwrap();

    let bin_name = env!("CARGO_BIN_NAME");
    let args = Args::parse();
    // let _guard = match args.command {
    //     HaneulCommand::Console { .. } | HaneulCommand::Client { .. } => {
    //         telemetry_subscribers::TelemetryConfig::new()
    //             .with_log_file(&format!("{bin_name}.log"))
    //             .with_env()
    //             .init()
    //     }
    //     _ => telemetry_subscribers::TelemetryConfig::new()
    //         .with_env()
    //         .init(),
    // };

    let _guard = telemetry_subscribers::TelemetryConfig::new()
        .with_log_file(&format!("{bin_name}.log"))
        .with_env()
        .init();
    debug!("Haneul-Move CLI version: {VERSION}");

    exit_main!(execute(args).await);
}

async fn execute(args: Args) -> anyhow::Result<()> {
    let config = haneul_config_dir()?.join(HANEUL_CLIENT_CONFIG);
    let wallet = WalletContext::new(&config)?;
    execute_move_command(
        args.package_path.as_deref(),
        args.build_config,
        args.cmd,
        None,
        &wallet,
    )
    .await
}
