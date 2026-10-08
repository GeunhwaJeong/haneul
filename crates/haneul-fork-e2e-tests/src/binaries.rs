// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

//! Locate the `haneul` and `haneul-fork` binaries by building them with cargo.
//!
//! Cargo exports `CARGO_BIN_EXE_<name>` only to integration tests of the package that owns the
//! binary, and a workspace test run never links the `haneul-fork` binary at all because that package
//! has no integration tests. Guessing `target/<profile>/<name>` instead would run whatever was
//! built there last, which silently tests stale code. Building on demand costs one cargo
//! invocation per test process, which is a fingerprint check once the binaries are current, and
//! cargo's build directory lock serialises the test processes that nextest runs in parallel.

use std::ffi::OsStr;
use std::ffi::OsString;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use std::process::Stdio;
use std::sync::OnceLock;

use anyhow::Context;
use anyhow::Result;
use anyhow::anyhow;
use anyhow::ensure;
use serde_json::Value;

/// Paths of the freshly built `haneul` and `haneul-fork` executables.
#[derive(Clone, Debug)]
pub struct Binaries {
    pub haneul: PathBuf,
    pub haneul_fork: PathBuf,
}

impl Binaries {
    /// Build both binaries in the running test's profile and cache the result for this process.
    pub fn build() -> Result<&'static Self> {
        static BINARIES: OnceLock<Result<Binaries, String>> = OnceLock::new();
        BINARIES
            .get_or_init(|| build_binaries().map_err(|error| format!("{error:#}")))
            .as_ref()
            .map_err(|error| anyhow!("{error}"))
    }

    /// Return `path` with the directories holding `haneul` and `haneul-fork` prepended.
    pub fn path_with(&self, path: &OsStr) -> Result<OsString> {
        let mut dirs = vec![
            self.haneul
                .parent()
                .context("haneul has no parent dir")?
                .to_path_buf(),
            self.haneul_fork
                .parent()
                .context("haneul-fork has no parent dir")?
                .to_path_buf(),
        ];
        dirs.dedup();
        dirs.extend(std::env::split_paths(path));
        std::env::join_paths(dirs).context("PATH entry contains a separator")
    }
}

/// Run `cargo build` for both binaries and read their paths from the JSON artifact messages.
///
/// Reading `executable` from the messages rather than composing `target/<profile>/<name>` keeps
/// the harness correct under a custom `CARGO_TARGET_DIR` or profile, because cargo reports where
/// it actually put the file. Fresh artifacts are reported too, so a no-op build still resolves
/// both paths.
fn build_binaries() -> Result<Binaries> {
    let cargo = std::env::var_os("CARGO").unwrap_or_else(|| env!("CARGO").into());
    let workspace = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let output = Command::new(cargo)
        .current_dir(&workspace)
        .args(["build", "--message-format=json-render-diagnostics"])
        .args(["--package", "haneul", "--bin", "haneul"])
        .args(["--package", "haneul-fork", "--bin", "haneul-fork"])
        .args(profile_flags()?)
        .stdin(Stdio::null())
        .stderr(Stdio::inherit())
        .output()
        .context("failed to run cargo build for the haneul and haneul-fork binaries")?;
    ensure!(
        output.status.success(),
        "cargo build of the haneul and haneul-fork binaries failed with {}",
        output.status
    );

    let mut haneul = None;
    let mut haneul_fork = None;
    for line in String::from_utf8_lossy(&output.stdout).lines() {
        let Ok(message) = serde_json::from_str::<Value>(line) else {
            continue;
        };
        if message["reason"] != "compiler-artifact" {
            continue;
        }
        let Some(executable) = message["executable"].as_str() else {
            continue;
        };
        let is_bin = message["target"]["kind"]
            .as_array()
            .is_some_and(|kinds| kinds.iter().any(|kind| kind.as_str() == Some("bin")));
        if !is_bin {
            continue;
        }
        match message["target"]["name"].as_str() {
            Some("haneul") => haneul = Some(PathBuf::from(executable)),
            Some("haneul-fork") => haneul_fork = Some(PathBuf::from(executable)),
            _ => {}
        }
    }

    Ok(Binaries {
        haneul: haneul.context("cargo build did not report the haneul executable")?,
        haneul_fork: haneul_fork
            .context("cargo build did not report the haneul-fork executable")?,
    })
}

/// Derive cargo profile flags from where the test executable lives.
///
/// Test executables sit at `<target>/<profile dir>/deps/<name>`. The profile directory is `debug`
/// for the dev and test profiles, `release` for release and bench, and the profile's own name
/// otherwise, so building with the matching flag puts the binaries next to the test and at the
/// same optimisation level.
fn profile_flags() -> Result<Vec<String>> {
    let exe = std::env::current_exe().context("cannot locate the test executable")?;
    let profile_dir = exe
        .parent()
        .and_then(Path::parent)
        .and_then(Path::file_name)
        .and_then(OsStr::to_str)
        .context("test executable is not under <target>/<profile>/deps")?;
    Ok(match profile_dir {
        "debug" => vec![],
        "release" => vec!["--release".to_owned()],
        custom => vec!["--profile".to_owned(), custom.to_owned()],
    })
}
