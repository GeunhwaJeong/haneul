// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

use std::path::{Path, PathBuf};

use colored::Colorize;
use haneul_package_alt::HaneulFlavor;
use haneul_rpc_api::Client;
use haneul_types::base_types::ObjectID;
use move_package_alt::schema::{Environment, Publication};
use serde::Serialize;

pub mod error;

mod binary;
mod build;
mod compare;
mod onchain;
mod pinning;
mod prebuilt;
mod toolchain_version;

pub use binary::ensure_binary;
pub use error::{AggregateError, Error};

/// The publication metadata a successful [`verify_source`] run relied on: the addresses it checked
/// against, the toolchain it rebuilt with, and the `haneul` binary it used.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VerifiedMetadata {
    /// The package's original (first-published) id.
    pub original_id: ObjectID,
    /// The on-chain address whose bytecode the rebuild was compared against.
    pub published_at: ObjectID,
    /// The `haneul` toolchain version the source was rebuilt with, or `None` when a caller-supplied
    /// binary was used (its version is not known).
    pub toolchain_version: Option<String>,
    /// The path to the `haneul` binary used for the rebuild.
    pub binary_path: PathBuf,
}

/// How to obtain the `haneul` toolchain that rebuilds a package.
pub enum ToolchainSource {
    /// Download and cache a release: the publication's recorded version, or `Some(version)` to
    /// override it.
    Version(Option<String>),
    /// Use the `haneul` binary already at this path, skipping version resolution and download.
    Binary(PathBuf),
}

/// Verify that the Move source package at `source_path` compiles to the on-chain package described
/// by `publication`, matching both its module bytecode and its linkage. On success, returns the
/// [`VerifiedMetadata`] the verification relied on.
///
/// `toolchain` selects the `haneul` binary to rebuild with: a downloaded release (the publication's
/// recorded version, or an override) or a caller-supplied binary. The package is rebuilt against
/// `env`, the resulting `0x0` root address is rewritten to the publication's original id, and the
/// modules and linkage are compared against the package fetched from the publication's published-at
/// address. `client_config` locates the wallet for releases whose build contacts the network.
pub async fn verify_source(
    source_path: &Path,
    publication: &Publication<HaneulFlavor>,
    toolchain: ToolchainSource,
    env: &Environment,
    client: &Client,
    client_config: Option<&Path>,
) -> Result<VerifiedMetadata, AggregateError> {
    // Resolve the binary to rebuild with. A downloaded release has a known version to report; a
    // caller-supplied binary does not.
    let (binary, toolchain_version) = match toolchain {
        ToolchainSource::Binary(path) => (path, None),
        ToolchainSource::Version(override_) => {
            let version = resolve_toolchain(
                publication.metadata.toolchain_version.clone(),
                source_path,
                override_,
            )?;
            check_toolchain_version(&version)?;
            let binary = ensure_binary(&version)?;
            (binary, Some(version))
        }
    };

    // Verification is attempted even for packages whose dependencies are not pinned to commit
    // hashes; only if that attempt fails is the lack of pinning reported, since it explains why the
    // rebuild could not reproduce what was published. The package is built against the environment
    // it is being verified on.
    let generated = build::dump(&binary, source_path, env.name(), client_config)
        .map_err(|e| explain_unpinned_dependencies(source_path, e.into()))?;

    let published_at = ObjectID::from_address(publication.addresses.published_at.0);
    let original_id = publication.addresses.original_id.0;
    let onchain = onchain::fetch(client, published_at).await?;

    // The package the source claims to be (its recorded original id) must be the one actually at
    // `published_at`, otherwise a source could be verified against an unrelated on-chain package.
    if onchain.original_id != original_id {
        return Err(Error::OriginalIdMismatch {
            recorded: original_id,
            on_chain: onchain.original_id,
        }
        .into());
    }

    compare::check(client, generated, onchain)
        .await
        .map_err(|e| explain_unpinned_dependencies(source_path, e))?;

    Ok(VerifiedMetadata {
        original_id: ObjectID::from_address(original_id),
        published_at,
        toolchain_version,
        binary_path: binary,
    })
}

/// Verify that the modules already compiled under `source_path/build` match the on-chain package at
/// `on_chain_id`, without invoking the compiler. Only module bytecode is compared, not linkage. The
/// caller supplies `on_chain_id` directly (there is no publication metadata to read), so this is a
/// local-build-versus-chain diff rather than an authenticity check on recorded source.
pub async fn verify_built(
    source_path: &Path,
    on_chain_id: ObjectID,
    client: &Client,
) -> Result<(), AggregateError> {
    let modules = prebuilt::read_modules(source_path)?;
    let generated = build::GeneratedPackage {
        modules,
        dependencies: vec![],
    };
    let onchain = onchain::fetch(client, on_chain_id).await?;
    compare::check(client, generated, onchain).await
}

/// Determine which toolchain version to rebuild with.
///
/// `recorded` is the version from the package's publication metadata, if any. When it is absent, the
/// legacy `Move.lock` is consulted (older packages record the version only there). `override_` is
/// the user's `--toolchain-version`: it is used when nothing is recorded, and otherwise takes
/// precedence with a warning, so a package whose recorded version cannot be built can still be
/// rebuilt with a working one.
fn resolve_toolchain(
    recorded: Option<String>,
    source_path: &Path,
    override_: Option<String>,
) -> Result<String, Error> {
    let recorded = recorded.or_else(|| toolchain_version::legacy_move_lock_version(source_path));

    match (override_, recorded) {
        (Some(override_), Some(recorded)) if override_ != recorded => {
            eprintln!(
                "{} rebuilding with toolchain {} instead of the recorded {}",
                "WARNING".bold().yellow(),
                override_.yellow(),
                recorded.yellow(),
            );
            Ok(override_)
        }
        (Some(version), _) | (None, Some(version)) => Ok(version),
        (None, None) => Err(Error::ToolchainVersionNotFound),
    }
}

/// Append an explanation for each dependency that is not pinned to a commit hash. Such a package
/// resolves its dependencies to whatever they point at now rather than at publish time, which is
/// the usual reason a rebuild neither compiles nor matches.
fn explain_unpinned_dependencies(source_path: &Path, mut error: AggregateError) -> AggregateError {
    for moving in pinning::moving_revisions(source_path) {
        error.0.push(Error::NonReproducibleDependency {
            dependency: moving.dependency,
            rev: moving.rev,
        });
    }
    error
}

/// Fail up front for releases known not to work for verification, naming a nearby release that
/// does — so a package recording such a toolchain gets a precise, actionable error rather than an
/// opaque download or build failure. Unparseable versions (e.g. a nightly) are let through to be
/// attempted.
///
/// The known-bad list describes this chain's own release history, which starts at v1.0.0 with no
/// gaps in verification support, so it is currently empty. Populate it if a future release turns
/// out to be unusable for rebuilds (e.g. it pins a framework revision that later leaves the
/// repository).
fn check_toolchain_version(_version: &str) -> Result<(), Error> {
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::check_toolchain_version;

    fn suggestion(version: &str) -> Option<String> {
        check_toolchain_version(version)
            .err()
            .map(|e| e.to_string())
    }

    #[test]
    fn flags_known_unusable_versions() {
        // The known-bad list is empty for this chain's release line: every released version can
        // verify, and unparseable versions are let through to be attempted.
        assert!(suggestion("1.0.0").is_none());
        assert!(suggestion("1.5.0").is_none());
        assert!(suggestion("1.7.0").is_none());
        assert!(suggestion("nightly").is_none());
    }

    /// The `--json` output uses stable, camelCase keys and renders addresses as `0x` strings.
    #[test]
    fn metadata_serializes_with_camel_case_keys() {
        use super::{ObjectID, PathBuf, VerifiedMetadata};

        let metadata = VerifiedMetadata {
            original_id: ObjectID::from_hex_literal("0x1").unwrap(),
            published_at: ObjectID::from_hex_literal("0x2").unwrap(),
            toolchain_version: Some("1.71.1".to_string()),
            binary_path: PathBuf::from("/cache/1.71.1/target/release/haneul"),
        };

        insta::assert_json_snapshot!(metadata, @r###"
        {
          "originalId": "0x0000000000000000000000000000000000000000000000000000000000000001",
          "publishedAt": "0x0000000000000000000000000000000000000000000000000000000000000002",
          "toolchainVersion": "1.71.1",
          "binaryPath": "/cache/1.71.1/target/release/haneul"
        }
        "###);

        // A caller-supplied binary has no known version, so `toolchainVersion` is null.
        let local = VerifiedMetadata {
            toolchain_version: None,
            ..metadata
        };
        insta::assert_json_snapshot!(local, @r###"
        {
          "originalId": "0x0000000000000000000000000000000000000000000000000000000000000001",
          "publishedAt": "0x0000000000000000000000000000000000000000000000000000000000000002",
          "toolchainVersion": null,
          "binaryPath": "/cache/1.71.1/target/release/haneul"
        }
        "###);
    }
}
