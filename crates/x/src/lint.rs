// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

use anyhow::anyhow;
use camino::Utf8Path;
use clap::Parser;
use nexlint::{NexLintContext, prelude::*};
use nexlint_lints::{
    content::*,
    package::*,
    project::{
        BannedDepConfig, BannedDepType, BannedDeps, BannedDepsConfig, DirectDepDups,
        DirectDepDupsConfig, DirectDuplicateGitDependencies,
    },
};
static EXTERNAL_CRATE_DIR: &str = "external-crates/";
// System package sources must stay byte-identical to the latest on-chain bytecode snapshot
// between protocol upgrades: clever-error abort codes embed source line numbers, so even a
// comment line changes the compiled bytecode. License header updates for these files land
// together with the next protocol upgrade, when a new snapshot is cut anyway.
static FRAMEWORK_PACKAGES_DIR: &str = "crates/haneul-framework/packages/";
static CREATE_DAPP_TEMPLATE_DIR: &str = "sdk/create-dapp/templates";
#[derive(Debug, Parser)]
pub struct Args {
    #[clap(long)]
    fail_fast: bool,
}

pub fn run(args: Args) -> crate::Result<()> {
    let banned_deps_config = BannedDepsConfig(
        vec![
            (
                "lazy_static".to_owned(),
                BannedDepConfig {
                    message: "use once_cell::sync::Lazy instead".to_owned(),
                    type_: BannedDepType::Direct,
                },
            ),
            (
                "tracing-test".to_owned(),
                BannedDepConfig {
                    message: "you should not be testing against log lines".to_owned(),
                    type_: BannedDepType::Always,
                },
            ),
            (
                "openssl-sys".to_owned(),
                BannedDepConfig {
                    message: "use rustls for TLS".to_owned(),
                    type_: BannedDepType::Always,
                },
            ),
            (
                "actix-web".to_owned(),
                BannedDepConfig {
                    message: "use axum for a webframework instead".to_owned(),
                    type_: BannedDepType::Always,
                },
            ),
            (
                "warp".to_owned(),
                BannedDepConfig {
                    message: "use axum for a webframework instead".to_owned(),
                    type_: BannedDepType::Always,
                },
            ),
            (
                "pq-sys".to_owned(),
                BannedDepConfig {
                    message: "diesel_async asynchronous database connections instead".to_owned(),
                    type_: BannedDepType::Always,
                },
            ),
        ]
        .into_iter()
        .collect(),
    );

    let direct_dep_dups_config = DirectDepDupsConfig {
        allow: vec![
            // TODO spend the time to de-dup these direct dependencies
            "serde_yaml".to_owned(),
            "syn".to_owned(),
            // Our opentelemetry integration requires that we use the same version of these packages
            // as the opentelemetry crates.
            "prost".to_owned(),
            "tonic".to_owned(),
            // jsonrpsee uses an older version of http-body
            "http-body".to_owned(),
            // jsonrpsee uses an older version of tower
            "tower".to_owned(),
            // async-graphql uses an older version of axum, axum-extra
            "axum".to_owned(),
            "axum-extra".to_owned(),
            // consistent-store uses a newer version of bincode with breaking interface changes
            "bincode".to_owned(),
            // TODO: remove once we've migrated ethers to alloy: https://linear.app/haneullabs-labs/issue/BR-191
            "reqwest".to_owned(),
        ],
    };

    let project_linters: &[&dyn ProjectLinter] = &[
        &BannedDeps::new(&banned_deps_config),
        &DirectDepDups::new(&direct_dep_dups_config),
        &DirectDuplicateGitDependencies,
    ];

    let package_linters: &[&dyn PackageLinter] = &[
        &CrateNamesPaths,
        &IrrelevantBuildDeps,
        &WorkspaceLintsOptIn,
        // This one seems to be broken
        // &UnpublishedPackagesOnlyUsePathDependencies::new(),
        &PublishedPackagesDontDependOnUnpublishedPackages,
        &OnlyPublishToCratesIo,
        &CratesInCratesDirectory,
        // There are crates under consensus/, external-crates/.
        // &CratesOnlyInCratesDirectory,
    ];

    let file_path_linters: &[&dyn FilePathLinter] = &[
        // &AllowedPaths::new(DEFAULT_ALLOWED_PATHS_REGEX)?
        ];

    // allow whitespace exceptions for markdown files
    // let whitespace_exceptions = build_exceptions(&["*.md".to_owned()])?;
    let content_linters: &[&dyn ContentLinter] = &[
        &HaneulLicenseHeader,
        &RootToml,
        // &EofNewline::new(&whitespace_exceptions),
        // &TrailingWhitespace::new(&whitespace_exceptions),
    ];

    let nexlint_context = NexLintContext::from_current_dir()?;
    let engine = LintEngineConfig::new(&nexlint_context)
        .with_project_linters(project_linters)
        .with_package_linters(package_linters)
        .with_file_path_linters(file_path_linters)
        .with_content_linters(content_linters)
        .fail_fast(args.fail_fast)
        .build();

    let results = engine.run()?;

    handle_lint_results_exclude_external_crate_checks(results)
}

/// Enforces that every workspace member inherits `[workspace.lints]` from the root
/// Cargo.toml via `[lints] workspace = true`. `cargo clippy` / `cargo xclippy` rely on
/// this table for the project-wide lint set, so a member that doesn't opt in would
/// silently be linted with no lints at all.
#[derive(Debug)]
struct WorkspaceLintsOptIn;

impl Linter for WorkspaceLintsOptIn {
    fn name(&self) -> &'static str {
        "workspace-lints-opt-in"
    }
}

impl PackageLinter for WorkspaceLintsOptIn {
    fn run<'l>(
        &self,
        ctx: &PackageContext<'l>,
        out: &mut LintFormatter<'l, '_>,
    ) -> Result<RunStatus<'l>, SystemError> {
        let manifest_path = ctx.metadata().manifest_path();
        let contents = std::fs::read_to_string(manifest_path)
            .map_err(|err| SystemError::io("reading manifest", err))?;
        let manifest: toml::Value = toml::from_str(&contents)
            .map_err(|err| SystemError::de("deserializing manifest", err))?;
        let opted_in = manifest
            .get("lints")
            .and_then(|lints| lints.get("workspace"))
            .and_then(|workspace| workspace.as_bool())
            == Some(true);
        if !opted_in {
            out.write(
                LintLevel::Error,
                "missing `[lints] workspace = true` in Cargo.toml: all workspace members \
                 must inherit `[workspace.lints]` so clippy applies the project lint set",
            );
        }
        Ok(RunStatus::Executed)
    }
}

/// Define custom handler so we can skip certain lints on certain files. This is a temporary till we upstream this logic
pub fn handle_lint_results_exclude_external_crate_checks(
    results: LintResults,
) -> crate::Result<()> {
    // ignore_funcs is a slice of funcs to execute against lint sources and their path
    // if a func returns true, it means it will be ignored and not throw a lint error
    let ignore_funcs = [
        // legacy ignore checks
        |source: &LintSource, path: &Utf8Path| -> bool {
            (path.starts_with(EXTERNAL_CRATE_DIR)
                || path.starts_with(FRAMEWORK_PACKAGES_DIR)
                || path.starts_with(CREATE_DAPP_TEMPLATE_DIR)
                || path.to_string().contains("/generated/")
                || path.to_string().contains("/proto/")
                || path.file_name() == Some("codegen.rs"))
                && source.name() == "license-header"
        },
        // ignore check to skip buck related code paths, meta (fb) derived starlark, etc.
        |_source: &LintSource, path: &Utf8Path| -> bool {
            path.starts_with("buck/") || path.starts_with("third-party/")
        },
    ];

    // TODO: handle skipped results
    let mut errs = false;
    for (source, message) in &results.messages {
        if let LintKind::Content(path) = source.kind()
            && ignore_funcs.iter().any(|func| func(source, path))
        {
            continue;
        }
        println!(
            "[{}] [{}] [{}]: {}\n",
            message.level(),
            source.name(),
            source.kind(),
            message.message()
        );
        errs = true;
    }

    if errs {
        Err(anyhow!("there were lint errors"))
    } else {
        Ok(())
    }
}

/// License header check that understands this repository's three-line header.
///
/// nexlint's stock `LicenseHeader` only inspects the first four non-empty lines and only
/// skips shebangs for shell and Python. That is no longer enough: files carry the Mysten
/// line, the modification notice, and sometimes upstream Diem/Move notices in front of
/// them, and JS entry points start with a shebang. This linter scans a wider window and
/// skips shebangs for every file type. It keeps the name "license-header" so the
/// external-crates exclusion above still applies.
#[derive(Debug)]
struct HaneulLicenseHeader;

static MYSTEN_LINE: &str = "Copyright (c) Mysten Labs, Inc.";
static SPDX_LINE: &str = "SPDX-License-Identifier: Apache-2.0";
static HOLDER: &str = "Geunhwa Jeong";
const HEADER_WINDOW: usize = 8;

fn has_license_extension(ext: Option<&str>) -> bool {
    matches!(
        ext,
        Some(
            "rs" | "sh"
                | "proto"
                | "js"
                | "jsx"
                | "cjs"
                | "mjs"
                | "ts"
                | "tsx"
                | "mts"
                | "cts"
                | "move"
                | "py"
        )
    )
}

/// Matches `Copyright (c) <year> Geunhwa Jeong`, or the `Modifications Copyright (c)`
/// form when `modifications` is set. The year is deliberately not pinned: it is the year
/// the file was first modified or created here and stays fixed afterwards.
fn is_holder_line(line: &str, modifications: bool) -> bool {
    let rest = if modifications {
        line.strip_prefix("Modifications Copyright (c) ")
    } else {
        line.strip_prefix("Copyright (c) ")
    };
    let Some((year, holder)) = rest.and_then(|r| r.split_once(' ')) else {
        return false;
    };
    year.len() == 4 && year.bytes().all(|b| b.is_ascii_digit()) && holder == HOLDER
}

impl Linter for HaneulLicenseHeader {
    fn name(&self) -> &'static str {
        "license-header"
    }
}

impl ContentLinter for HaneulLicenseHeader {
    fn pre_run<'l>(&self, file_ctx: &FilePathContext<'l>) -> Result<RunStatus<'l>, SystemError> {
        if has_license_extension(file_ctx.extension()) {
            Ok(RunStatus::Executed)
        } else {
            Ok(RunStatus::Skipped(SkipReason::UnsupportedExtension(
                file_ctx.extension(),
            )))
        }
    }

    fn run<'l>(
        &self,
        ctx: &ContentContext<'l>,
        out: &mut LintFormatter<'l, '_>,
    ) -> Result<RunStatus<'l>, SystemError> {
        let Some(content) = ctx.content() else {
            return Ok(RunStatus::Skipped(SkipReason::NonUtf8Content));
        };
        let header: Vec<&str> = content
            .lines()
            .filter(|line| !line.trim().is_empty() && !line.starts_with("#!"))
            .take(HEADER_WINDOW)
            .map(|line| {
                line.trim_start()
                    .trim_start_matches("//")
                    .trim_start_matches('#')
                    .trim()
            })
            .collect();
        let spdx = header.contains(&SPDX_LINE);
        let mysten = header.contains(&MYSTEN_LINE);
        let modified = header.iter().any(|l| is_holder_line(l, true));
        let local = header.iter().any(|l| is_holder_line(l, false));
        let problem = match (spdx, mysten, modified, local) {
            (true, true, true, _) | (true, false, false, true) => None,
            (false, false, false, false) => Some("missing license header"),
            (_, true, false, _) => {
                Some("Mysten Labs header is missing the Modifications Copyright line")
            }
            (false, _, _, _) => Some("missing SPDX-License-Identifier line"),
            _ => Some("missing license header"),
        };
        if let Some(problem) = problem {
            out.write(LintLevel::Error, problem);
        }
        Ok(RunStatus::Executed)
    }
}
