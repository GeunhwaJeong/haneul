// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use std::ffi::OsStr;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use anyhow::{Context, anyhow};
use colored::Colorize;
use move_command_line_common::env::MOVE_HOME;
use tar::Archive;
use tracing::debug;

use crate::error::Error;

const CURRENT_COMPILER_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Resolve the path to a `haneul` binary for `version`, downloading and caching it under the user's
/// cache directory (one subdirectory per version) if necessary.
///
/// If `version` is the version of the running binary, the running executable is used directly
/// (avoiding a redundant download of the version already in hand). This is also the precache /
/// warm entry point: calling it ahead of time populates the cache so later runs need no network.
pub fn ensure_binary(version: &str) -> Result<PathBuf, Error> {
    if version == CURRENT_COMPILER_VERSION {
        return std::env::current_exe().map_err(|e| Error::BinaryDownload {
            version: version.to_string(),
            message: format!("could not locate the running executable: {e}"),
        });
    }

    let platform = detect_platform(version)?;
    let binary_name = platform.binary_name();

    let cache_root = binary_cache_dir();
    let version_dir = cache_root.join(version);
    let canonical = version_dir.join("target").join("release").join(binary_name);

    if canonical.exists() {
        touch_last_used(&version_dir);
        return Ok(canonical);
    }

    download_and_install(
        version,
        platform.artifact_str(),
        binary_name,
        &cache_root,
        &version_dir,
    )
    .map_err(|e| Error::BinaryDownload {
        version: version.to_string(),
        message: e.to_string(),
    })?;

    touch_last_used(&version_dir);
    evict_stale(&cache_root, cache_limit());

    Ok(canonical)
}

/// Name of the per-version marker file recording when a cached binary was last used, for LRU
/// eviction.
const LAST_USED_FILE: &str = ".last_used";

/// The number of downloaded `haneul` binaries to keep cached. After each install the least-recently-used
/// versions beyond this are evicted, so the cache stays small in disk-scarce environments (enclaves).
/// Overridable with `HANEUL_BINARY_CACHE_LIMIT`.
const DEFAULT_CACHE_LIMIT: usize = 5;

fn cache_limit() -> usize {
    std::env::var("HANEUL_BINARY_CACHE_LIMIT")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(DEFAULT_CACHE_LIMIT)
}

/// Record that the binary under `version_dir` was just used. Best-effort: a failure here (for
/// example, a read-only cache) must not fail verification.
fn touch_last_used(version_dir: &Path) {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    let _ = fs::write(version_dir.join(LAST_USED_FILE), now.to_string());
}

/// The recorded last-used timestamp for a cached version, or `0` when it has none (a directory
/// predating this bookkeeping), which sorts it as the oldest.
fn last_used(version_dir: &Path) -> u64 {
    fs::read_to_string(version_dir.join(LAST_USED_FILE))
        .ok()
        .and_then(|s| s.trim().parse().ok())
        .unwrap_or(0)
}

/// Evict least-recently-used cached binaries, keeping at most `limit`. Best-effort: eviction failures
/// do not fail verification. Only whole other-version directories are removed; the version in hand
/// keeps its own path, and on unix an executing binary survives removal of its directory. In-progress
/// `.tmp-*` installs (hidden entries) are never touched.
fn evict_stale(cache_root: &Path, limit: usize) {
    let Ok(entries) = fs::read_dir(cache_root) else {
        return;
    };
    let mut versions: Vec<(u64, PathBuf)> = entries
        .flatten()
        .map(|entry| entry.path())
        .filter(|path| path.is_dir() && !is_hidden(path))
        .map(|dir| (last_used(&dir), dir))
        .collect();

    // Newest first; keep the first `limit`, remove the rest.
    versions.sort_by_key(|(last_used, _)| std::cmp::Reverse(*last_used));
    for (_, dir) in versions.into_iter().skip(limit) {
        let _ = fs::remove_dir_all(&dir);
    }
}

/// Whether a cache entry is hidden (dot-prefixed) — an in-progress `.tmp-*` install rather than a
/// cached version, so not a candidate for eviction.
fn is_hidden(path: &Path) -> bool {
    path.file_name()
        .and_then(|name| name.to_str())
        .is_some_and(|name| name.starts_with('.'))
}

/// Download the `haneul` release tarball for `version`, streaming out just the `haneul` binary, and install
/// it atomically into `version_dir`. The binary is written under a temporary directory that is renamed
/// into place only once complete, so concurrent installs of the same version cannot observe a partial
/// tree, and nothing but the `haneul` binary is ever written to disk.
fn download_and_install(
    version: &str,
    platform: &str,
    binary_name: &str,
    cache_root: &Path,
    version_dir: &Path,
) -> anyhow::Result<()> {
    fs::create_dir_all(cache_root).context("creating binary cache directory")?;

    // Temp dir on the same filesystem as the cache so the final rename is atomic.
    let tmp = cache_root.join(format!(".tmp-{version}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&tmp);
    fs::create_dir_all(&tmp).context("creating temporary install directory")?;

    let result = (|| {
        // Stream the release archive into the temp tree, writing only the `haneul` binary. A release
        // archive carries every shipped binary (`haneul-debug`, `haneul-node`, ...), well over a gigabyte,
        // so streaming keeps everything but `haneul` off disk. The completed tree is renamed into place
        // below, so the install stays atomic.
        let release_dir = tmp.join("target").join("release");
        fs::create_dir_all(&release_dir).context("creating release directory")?;
        let staged_binary = release_dir.join(binary_name);
        stream_haneul_binary(version, platform, &staged_binary)?;
        set_executable_permission(staged_binary.as_os_str())?;
        Ok(())
    })();

    if let Err(e) = result {
        let _ = fs::remove_dir_all(&tmp);
        return Err(e);
    }

    match fs::rename(&tmp, version_dir) {
        Ok(()) => Ok(()),
        // Another process installed the same version first; use theirs and drop ours.
        Err(_)
            if version_dir
                .join("target")
                .join("release")
                .join(binary_name)
                .exists() =>
        {
            let _ = fs::remove_dir_all(&tmp);
            Ok(())
        }
        Err(e) => {
            let _ = fs::remove_dir_all(&tmp);
            Err(anyhow!("installing downloaded binary: {e}"))
        }
    }
}

/// Download the `haneul` release tarball for `version` and stream it, extracting only the `haneul` binary
/// to `dest`.
fn stream_haneul_binary(version: &str, platform: &str, dest: &Path) -> anyhow::Result<()> {
    if let Some(tarball) = haneulup_cached_tarball(version, platform) {
        debug!("reusing haneulup's cached release at {}", tarball.display());
        let reader = fs::File::open(&tarball).context("opening haneulup's cached release")?;
        return extract_haneul_from_stream(reader, version, platform, dest);
    }
    let reader = download_reader(version, platform)?;
    extract_haneul_from_stream(reader, version, platform, dest)
}

/// The path to a `haneul` release tarball already cached by `haneulup`, if one exists. `haneulup` keeps
/// release archives under `<cache-dir>/haneulup/releases/haneul-<net>-v<version>-<platform>.tgz`; reusing
/// one avoids re-downloading a release the user already has.
fn haneulup_cached_tarball(version: &str, platform: &str) -> Option<PathBuf> {
    let releases = dirs::cache_dir()?.join("haneulup").join("releases");
    haneulup_tarball_in(&releases, version, platform)
}

/// The path to a matching `haneulup` release tarball under `releases` (checking the mainnet and testnet
/// naming), or `None` if neither is present.
fn haneulup_tarball_in(releases: &Path, version: &str, platform: &str) -> Option<PathBuf> {
    ["mainnet", "testnet"]
        .into_iter()
        .map(|net| releases.join(format!("haneul-{net}-v{version}-{platform}.tgz")))
        .find(|path| path.exists())
}

/// Open a streaming reader over the `haneul` release tarball for `version`, trying the mainnet release
/// first and falling back to the testnet release on a 404.
fn download_reader(version: &str, platform: &str) -> anyhow::Result<impl io::Read> {
    let mainnet_url = format!(
        "https://github.com/GeunhwaJeong/haneul/releases/download/mainnet-v{version}/haneul-mainnet-v{version}-{platform}.tgz",
    );

    // Progress goes to stderr so it does not corrupt a `--json` verification result on stdout.
    eprintln!(
        "{} haneul compiler @ {} (this may take a while)",
        "DOWNLOADING".bold().green(),
        version.yellow()
    );

    let response = match ureq::get(&mainnet_url).call() {
        Ok(response) => response,
        Err(ureq::Error::Status(404, _)) => {
            debug!("no mainnet release for {version}, trying testnet");
            let testnet_url = format!(
                "https://github.com/GeunhwaJeong/haneul/releases/download/testnet-v{version}/haneul-testnet-v{version}-{platform}.tgz",
            );
            ureq::get(&testnet_url).call()?
        }
        Err(e) => return Err(e.into()),
    };
    Ok(response.into_reader())
}

/// Read a gzipped tar archive from `reader` and unpack only the `haneul` binary to `dest`, discarding
/// every other entry as it streams. Errors if the archive for `version` contains no `haneul` binary.
fn extract_haneul_from_stream(
    reader: impl io::Read,
    version: &str,
    platform: &str,
    dest: &Path,
) -> anyhow::Result<()> {
    let tar = flate2::read::GzDecoder::new(reader);
    let mut archive = Archive::new(tar);
    let entries = archive.entries().context("reading release archive")?;

    for entry in entries {
        let mut entry = entry.context("reading release archive entry")?;
        let is_haneul = {
            let path = entry.path().context("reading archive entry path")?;
            matches_haneul(&path, platform)
        };
        if is_haneul {
            entry.unpack(dest).context("unpacking the haneul binary")?;
            return Ok(());
        }
    }
    Err(anyhow!(
        "no haneul binary found in the {version} release archive"
    ))
}

/// Whether `path`, the path of an entry in a release archive, is the `haneul` executable — either the
/// modern root-level `haneul` or the older `target/release/haneul-<platform>`. Other shipped binaries
/// (`haneul-node`, `haneul-tool`, ...) do not match.
fn matches_haneul(path: &Path, platform: &str) -> bool {
    let suffix = if platform == "windows-x86_64" {
        ".exe"
    } else {
        ""
    };
    let names = [
        format!("haneul{suffix}"),
        format!("haneul-{platform}{suffix}"),
    ];
    path.file_name()
        .and_then(|n| n.to_str())
        .is_some_and(|n| names.iter().any(|candidate| candidate == n))
}

/// A platform for which `haneul` release binaries are published.
enum Platform {
    MacosArm64,
    MacosX86_64,
    UbuntuX86_64,
    WindowsX86_64,
}

impl Platform {
    /// The platform string used in release download URLs.
    fn artifact_str(&self) -> &'static str {
        match self {
            Platform::MacosArm64 => "macos-arm64",
            Platform::MacosX86_64 => "macos-x86_64",
            Platform::UbuntuX86_64 => "ubuntu-x86_64",
            Platform::WindowsX86_64 => "windows-x86_64",
        }
    }

    /// The name of the `haneul` executable on this platform.
    fn binary_name(&self) -> &'static str {
        match self {
            Platform::WindowsX86_64 => "haneul.exe",
            _ => "haneul",
        }
    }
}

/// The [`Platform`] for the current OS/architecture, or an error explaining how to sideload a binary
/// if there is no downloadable release for this platform.
fn detect_platform(version: &str) -> Result<Platform, Error> {
    match (std::env::consts::OS, std::env::consts::ARCH) {
        ("macos", "aarch64") => Ok(Platform::MacosArm64),
        ("macos", "x86_64") => Ok(Platform::MacosX86_64),
        ("linux", "x86_64") => Ok(Platform::UbuntuX86_64),
        ("windows", "x86_64") => Ok(Platform::WindowsX86_64),
        (os, arch) => Err(Error::BinaryDownload {
            version: version.to_string(),
            message: format!(
                "no downloadable haneul {version} release for your platform \
                 (OS: {os}, architecture: {arch}); pass --toolchain <path> to build with a local binary"
            ),
        }),
    }
}

/// Directory under which downloaded `haneul` binaries are cached, one subdirectory per version. Uses the
/// platform cache directory (as other Haneul tooling does), falling back to `$MOVE_HOME` if it cannot be
/// determined.
fn binary_cache_dir() -> PathBuf {
    dirs::cache_dir()
        .unwrap_or_else(|| PathBuf::from(&*MOVE_HOME))
        .join("haneul")
        .join("source-verification")
        .join("binaries")
}

#[cfg(unix)]
fn set_executable_permission(path: &OsStr) -> anyhow::Result<()> {
    use std::os::unix::prelude::PermissionsExt;
    let mut perms = fs::metadata(path)?.permissions();
    perms.set_mode(0o755);
    fs::set_permissions(path, perms)?;
    Ok(())
}

#[cfg(not(unix))]
fn set_executable_permission(_path: &OsStr) -> anyhow::Result<()> {
    // On Windows an executable is runnable by virtue of its extension, and the freshly-written file
    // is already owned by the current user, so there is no permission bit to set.
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::io::Write;

    use super::*;

    /// Build a gzipped tar archive from `(path, contents)` entries, as the release download stream
    /// would deliver it.
    fn make_tgz(entries: &[(&str, &[u8])]) -> Vec<u8> {
        let mut tar = Vec::new();
        {
            let mut builder = tar::Builder::new(&mut tar);
            for (name, data) in entries {
                let mut header = tar::Header::new_gnu();
                header.set_size(data.len() as u64);
                header.set_mode(0o755);
                builder.append_data(&mut header, name, *data).unwrap();
            }
            builder.finish().unwrap();
        }
        let mut gz = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::fast());
        gz.write_all(&tar).unwrap();
        gz.finish().unwrap()
    }

    /// `matches_haneul` accepts the modern root-level `haneul` and the older `target/release/haneul-<platform>`
    /// layouts, ignores other shipped binaries, and honours the windows `.exe` suffix.
    #[test]
    fn matches_haneul_by_layout() {
        assert!(matches_haneul(Path::new("haneul"), "macos-arm64"));
        assert!(matches_haneul(
            Path::new("target/release/haneul-macos-arm64"),
            "macos-arm64"
        ));
        assert!(!matches_haneul(Path::new("haneul-node"), "macos-arm64"));
        assert!(!matches_haneul(
            Path::new("target/release/haneul-tool"),
            "macos-arm64"
        ));
        assert!(matches_haneul(Path::new("haneul.exe"), "windows-x86_64"));
        assert!(!matches_haneul(Path::new("haneul"), "windows-x86_64"));
    }

    /// Streaming extraction writes only the `haneul` binary (with its exact bytes) and nothing else,
    /// even when other binaries precede and follow it in the archive.
    #[test]
    fn extract_takes_only_haneul() {
        let tgz = make_tgz(&[
            ("haneul-node", b"NODE"),
            ("haneul", b"HANEUL-BINARY"),
            ("haneul-tool", b"TOOL"),
        ]);
        let dir = tempfile::tempdir().unwrap();
        let dest = dir.path().join("haneul");

        extract_haneul_from_stream(tgz.as_slice(), "1.0.0", "macos-arm64", &dest).unwrap();

        assert_eq!(fs::read(&dest).unwrap(), b"HANEUL-BINARY");
        // Nothing but `haneul` landed on disk.
        assert_eq!(fs::read_dir(dir.path()).unwrap().count(), 1);
    }

    /// An archive with no `haneul` binary is an error rather than a silent success.
    #[test]
    fn extract_errors_without_haneul() {
        let tgz = make_tgz(&[("haneul-node", b"NODE"), ("haneul-tool", b"TOOL")]);
        let dir = tempfile::tempdir().unwrap();
        let dest = dir.path().join("haneul");

        let err =
            extract_haneul_from_stream(tgz.as_slice(), "1.0.0", "macos-arm64", &dest).unwrap_err();
        assert!(err.to_string().contains("no haneul binary"));
    }

    /// Eviction keeps the most-recently-used versions and leaves an in-progress install untouched.
    #[test]
    fn evict_keeps_most_recently_used() {
        let cache = tempfile::tempdir().unwrap();
        let seed = |name: &str, last_used: u64| {
            let dir = cache.path().join(name);
            fs::create_dir_all(&dir).unwrap();
            fs::write(dir.join(LAST_USED_FILE), last_used.to_string()).unwrap();
        };
        seed("1.0.0", 100);
        seed("1.1.0", 300);
        seed("1.2.0", 200);
        seed("1.3.0", 50);
        // A concurrent install's temp directory must survive eviction despite having no marker.
        fs::create_dir_all(cache.path().join(".tmp-9.9.9-1")).unwrap();

        evict_stale(cache.path(), 2);

        // The two most recently used remain; the two oldest are gone.
        assert!(cache.path().join("1.1.0").exists());
        assert!(cache.path().join("1.2.0").exists());
        assert!(!cache.path().join("1.0.0").exists());
        assert!(!cache.path().join("1.3.0").exists());
        // The in-progress install is never evicted.
        assert!(cache.path().join(".tmp-9.9.9-1").exists());
    }

    /// `haneulup_tarball_in` matches a cached release under either the mainnet or testnet naming.
    #[test]
    fn finds_haneulup_cached_tarball() {
        let releases = tempfile::tempdir().unwrap();
        assert!(haneulup_tarball_in(releases.path(), "1.0.0", "macos-arm64").is_none());

        fs::write(
            releases
                .path()
                .join("haneul-testnet-v1.0.0-macos-arm64.tgz"),
            b"",
        )
        .unwrap();
        let found = haneulup_tarball_in(releases.path(), "1.0.0", "macos-arm64").unwrap();
        assert_eq!(
            found.file_name().unwrap(),
            "haneul-testnet-v1.0.0-macos-arm64.tgz"
        );

        // A different version does not match.
        assert!(haneulup_tarball_in(releases.path(), "2.0.0", "macos-arm64").is_none());
    }
}
