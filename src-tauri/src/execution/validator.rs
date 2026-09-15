//! Input validation for the execution boundary. Frontend data is untrusted;
//! every check here is a security control (see docs/execution-security.md).

use std::collections::BTreeMap;
use std::path::{Component, Path, PathBuf};

use crate::error::{AppError, AppResult};

pub const MAX_SOURCE_BYTES: usize = 64 * 1024;

/// Whitelisted environment variables passed to sandboxed child processes.
/// Anything else (app secrets, tool credentials, proxy config…) is stripped.
pub const ALLOWED_ENV: &[&str] = &[
    "PATH",
    "SYSTEMROOT",
    "SYSTEMDRIVE",
    "WINDIR",
    "TEMP",
    "TMP",
    "USERPROFILE",
    "HOMEDRIVE",
    "HOMEPATH",
    "COMSPEC",
    "PATHEXT",
    "NUMBER_OF_PROCESSORS",
    "PROCESSOR_ARCHITECTURE",
];

pub fn validate_source(code: &str) -> AppResult<()> {
    if code.len() > MAX_SOURCE_BYTES {
        return Err(AppError::Validation(format!(
            "source exceeds {} bytes (got {})",
            MAX_SOURCE_BYTES,
            code.len()
        )));
    }
    if code.contains('\0') {
        return Err(AppError::Validation("source contains NUL bytes".into()));
    }
    if code.trim().is_empty() {
        return Err(AppError::Validation("source is empty".into()));
    }
    Ok(())
}

/// Lexically resolve `..` components without touching the filesystem.
pub fn normalize_lexical(path: &Path) -> PathBuf {
    let mut out = PathBuf::new();
    for component in path.components() {
        match component {
            Component::ParentDir => {
                out.pop();
            }
            Component::CurDir => {}
            other => out.push(other.as_os_str()),
        }
    }
    out
}

pub fn ensure_within_roots(candidate: &Path, roots: &[PathBuf]) -> AppResult<PathBuf> {
    let normalized = normalize_lexical(candidate);
    for root in roots {
        if normalized.starts_with(root) {
            return Ok(normalized);
        }
    }
    Err(AppError::Validation(format!(
        "path is outside managed workspaces: {}",
        candidate.display()
    )))
}

/// Validate a user-supplied cargo project path: must resolve (lexically) to a
/// directory containing `Cargo.toml` inside one of the managed workspace roots.
pub fn validate_cargo_path(raw: &str, roots: &[PathBuf]) -> AppResult<PathBuf> {
    if raw.trim().is_empty() || raw.contains('\0') {
        return Err(AppError::Validation("invalid project path".into()));
    }
    let supplied = PathBuf::from(raw);
    let candidate = if supplied.is_absolute() {
        supplied
    } else {
        let base = roots
            .first()
            .ok_or_else(|| AppError::Validation("no workspace roots configured".into()))?;
        base.join(supplied)
    };
    let within = ensure_within_roots(&candidate, roots)?;
    if !within.join("Cargo.toml").is_file() {
        return Err(AppError::NotFound(format!(
            "no Cargo.toml found under {}",
            within.display()
        )));
    }
    Ok(within)
}

/// Build the sanitized environment map handed to sandboxed children.
pub fn build_sandbox_env() -> BTreeMap<String, String> {
    let mut env = BTreeMap::new();
    for (key, value) in std::env::vars() {
        if ALLOWED_ENV.iter().any(|a| a.eq_ignore_ascii_case(&key)) {
            env.insert(key, value);
        }
    }
    env
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temp_root(name: &str) -> PathBuf {
        let dir = std::env::temp_dir()
            .join("rm-tests")
            .join(format!("{name}-{}", uuid::Uuid::new_v4()));
        std::fs::create_dir_all(dir.join("proj")).unwrap();
        std::fs::write(dir.join("proj").join("Cargo.toml"), "[package]\nname=\"x\"\n").unwrap();
        dir
    }

    #[test]
    fn source_validation_rejects_bad_input() {
        assert!(validate_source("").is_err(), "empty rejected");
        assert!(validate_source("fn main(){}\0").is_err(), "NUL rejected");
        let big = "x".repeat(MAX_SOURCE_BYTES + 1);
        assert!(validate_source(&big).is_err(), "oversize rejected");
        assert!(validate_source("fn main() {}").is_ok());
    }

    #[test]
    fn lexical_normalization_collapses_parent_dirs() {
        let p = PathBuf::from("/a/b/../c/./d");
        assert_eq!(normalize_lexical(&p), PathBuf::from("/a/c/d"));
    }

    #[test]
    fn traversal_outside_root_is_rejected() {
        let root = temp_root("traversal");
        let sneaky = root.join("..").join("elsewhere");
        assert!(ensure_within_roots(&sneaky, &[root.clone()]).is_err());
        std::fs::remove_dir_all(&root).ok();
    }

    #[test]
    fn cargo_path_requires_manifest_inside_root() {
        let root = temp_root("cargo-path");
        // Inside root, manifest present → ok (relative form resolves against root).
        let rel = format!("proj{}", std::path::MAIN_SEPARATOR_STR);
        assert!(validate_cargo_path(&rel, &[root.clone()]).is_ok());
        // Absolute outside → rejected.
        assert!(validate_cargo_path("C:\\Windows", &[root.clone()]).is_err());
        // Inside root but missing Cargo.toml → NotFound.
        let empty_rel = format!(".{}", std::path::MAIN_SEPARATOR_STR);
        match validate_cargo_path(&empty_rel, &[root.clone()]) {
            Err(AppError::NotFound(_)) => {}
            other => panic!("expected NotFound, got {other:?}"),
        }
        std::fs::remove_dir_all(&root).ok();
    }

    #[test]
    fn sandbox_env_only_contains_allowlisted_keys() {
        std::env::set_var("RM_DEFINITELY_NOT_ALLOWED", "1");
        let env = build_sandbox_env();
        assert!(!env.contains_key("RM_DEFINITELY_NOT_ALLOWED"));
        assert!(env.keys().all(|k| ALLOWED_ENV.iter().any(|a| a.eq_ignore_ascii_case(k))));
    }
}
