//! Sandboxed Rust code execution: isolated temp workspace, allowlisted env,
//! hard timeouts, cancellation, output caps, deterministic cleanup.
//! See docs/execution-security.md — this module is the enforcement point.

mod registry;
pub mod validator;

use std::path::{Path, PathBuf};
use std::process::Stdio;
use std::sync::OnceLock;
use std::time::{Duration, Instant};


use serde::Serialize;
use tokio::io::AsyncReadExt;
use tokio::process::{Child, Command};

pub use registry::ExecutionRegistry;

use crate::error::{AppError, AppResult};

/// Hard cap for running student binaries — the spec's 5-second rule.
pub const EXEC_TIMEOUT_SECS: u64 = 5;
/// Compilation gets more headroom (cold toolchain warm-up) but is still bounded.
pub const COMPILE_TIMEOUT_SECS: u64 = 30;
pub const TEST_TIMEOUT_SECS: u64 = 120;
const POLL_MS: u64 = 100;
const MAX_OUTPUT_BYTES: usize = 64 * 1024;
const MAX_DIAGNOSTICS: usize = 50;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Diagnostic {
    pub severity: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub span: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RunResult {
    pub execution_id: String,
    pub ok: bool,
    pub timed_out: bool,
    pub cancelled: bool,
    pub duration_ms: u64,
    pub stdout: String,
    pub stderr: String,
    pub diagnostics: Vec<Diagnostic>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TestResult {
    pub execution_id: String,
    pub ok: bool,
    pub timed_out: bool,
    pub cancelled: bool,
    pub duration_ms: u64,
    pub stdout: String,
    pub stderr: String,
    pub passed: u32,
    pub failed: u32,
}

/// Deletes its workspace on drop — deterministic cleanup regardless of
/// success, timeout, cancellation, or early-return error paths.
struct WorkspaceGuard(PathBuf);

impl Drop for WorkspaceGuard {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

fn exe_name(base: &str) -> String {
    if cfg!(windows) {
        format!("{base}.exe")
    } else {
        base.to_string()
    }
}

fn apply_sandbox_env(cmd: &mut Command) {
    cmd.env_clear();
    for (k, v) in validator::build_sandbox_env() {
        cmd.env(k, v);
    }
}

async fn read_stream<R: tokio::io::AsyncRead + Unpin>(mut reader: R) -> String {
    let mut buf: Vec<u8> = Vec::new();
    let mut chunk = [0u8; 4096];
    loop {
        match reader.read(&mut chunk).await {
            Ok(0) | Err(_) => break,
            Ok(n) => {
                let take = n.min(MAX_OUTPUT_BYTES.saturating_sub(buf.len()));
                buf.extend_from_slice(&chunk[..take]);
            }
        }
    }
    String::from_utf8_lossy(&buf).to_string()
}

enum WaitOutcome {
    Done(std::process::ExitStatus),
    TimedOut,
    Cancelled,
}

/// Poll-based wait that reacts to cancellation within ~POLL_MS while piped
/// readers drain concurrently (no pipe-full deadlocks).
async fn wait_with_cancel(
    child: &mut Child,
    timeout_secs: u64,
    cancel: &registry::CancelHandle,
) -> AppResult<WaitOutcome> {
    let start = Instant::now();
    loop {
        if cancel.is_cancelled() {
            let _ = child.kill().await;
            let _ = child.wait().await;
            return Ok(WaitOutcome::Cancelled);
        }
        if let Some(status) = child.try_wait()? {
            return Ok(WaitOutcome::Done(status));
        }
        if start.elapsed() >= Duration::from_secs(timeout_secs) {
            let _ = child.kill().await;
            let _ = child.wait().await;
            return Ok(WaitOutcome::TimedOut);
        }
        tokio::time::sleep(Duration::from_millis(POLL_MS)).await;
    }
}

struct SpawnedOutput {
    status: Option<std::process::ExitStatus>,
    timed_out: bool,
    cancelled: bool,
    stdout: String,
    stderr: String,
}

async fn run_child(
    mut cmd: Command,
    timeout_secs: u64,
    cancel: &registry::CancelHandle,
) -> AppResult<SpawnedOutput> {
    cmd.stdout(Stdio::piped()).stderr(Stdio::piped());
    let mut child = cmd
        .spawn()
        .map_err(|e| AppError::Execution(format!("failed to spawn process: {e}")))?;
    let stdout_pipe = child
        .stdout
        .take()
        .ok_or_else(|| AppError::Execution("stdout pipe unavailable".into()))?;
    let stderr_pipe = child
        .stderr
        .take()
        .ok_or_else(|| AppError::Execution("stderr pipe unavailable".into()))?;
    let out_task = tokio::spawn(read_stream(stdout_pipe));
    let err_task = tokio::spawn(read_stream(stderr_pipe));

    let outcome = wait_with_cancel(&mut child, timeout_secs, cancel).await?;
    let (status, timed_out, cancelled) = match outcome {
        WaitOutcome::Done(status) => (Some(status), false, false),
        WaitOutcome::TimedOut => (None, true, false),
        WaitOutcome::Cancelled => (None, false, true),
    };
    // Readers finish once the pipes close (kill or normal exit); join! hands
    // both results back at once.
    let (stdout_joined, stderr_joined) = tokio::join!(out_task, err_task);
    let stdout = stdout_joined.unwrap_or_default();
    let stderr = stderr_joined.unwrap_or_default();
    Ok(SpawnedOutput { status, timed_out, cancelled, stdout, stderr })
}


/// Parse compiler stderr into structured, teachable diagnostics.
pub fn parse_diagnostics(stderr: &str) -> Vec<Diagnostic> {
    let mut out: Vec<Diagnostic> = Vec::new();
    for line in stderr.lines() {
        let trimmed = line.trim_start();
        if trimmed.starts_with("-->") {
            if let Some(last) = out.last_mut() {
                last.span = Some(trimmed.to_string());
            }
            continue;
        }
        let severity = if trimmed.starts_with("error") {
            Some("error")
        } else if trimmed.starts_with("warning") {
            Some("warning")
        } else if trimmed.starts_with("note") {
            Some("note")
        } else if trimmed.starts_with("help") {
            Some("help")
        } else {
            None
        };
        if let Some(sev) = severity {
            if out.len() >= MAX_DIAGNOSTICS {
                break;
            }
            out.push(Diagnostic {
                severity: sev.to_string(),
                message: trimmed.to_string(),
                span: None,
            });
        }
    }
    out
}

/// Compile-and-run pipeline for a single-file student program.
pub async fn run_rust_code(
    registry: &ExecutionRegistry,
    sandbox_root: &Path,
    code: &str,
) -> AppResult<RunResult> {
    validator::validate_source(code)?;
    let execution_id = uuid::Uuid::new_v4().to_string();
    let handle = registry.register(format!("run:{execution_id}"));
    // Always remove the id when this function exits (guard pattern).
    struct Unregister<'a>(&'a ExecutionRegistry, String);
    impl Drop for Unregister<'_> {
        fn drop(&mut self) {
            self.0.unregister(&self.1);
        }
    }
    let _unregister = Unregister(registry, format!("run:{execution_id}"));

    let workspace = sandbox_root.join(&execution_id);
    std::fs::create_dir_all(&workspace)?;
    let _workspace_guard = WorkspaceGuard(workspace.clone());

    let source_path = workspace.join("main.rs");
    std::fs::write(&source_path, code)?;

    let started = Instant::now();
    let binary = workspace.join(exe_name("main"));

    // ---- Phase 1: compile -------------------------------------------------
    // NOTE: plain `rustc` has no `-O1` shorthand (that's cargo-speak); it
    // parses as an unknown short cluster and aborts with
    // "Unrecognized option: '1'". Default (debug) codegen is also the
    // fastest path for a teaching sandbox, so we simply omit opt-level.
    let mut compile_cmd = Command::new("rustc");
    apply_sandbox_env(&mut compile_cmd);
    compile_cmd
        .arg("--edition=2021")
        .arg(source_path.file_name().expect("file name").to_string_lossy().as_ref())
        .arg("-o")
        .arg(&binary)
        .current_dir(&workspace);


    let compiled = run_child(compile_cmd, COMPILE_TIMEOUT_SECS, &handle).await?;
    let diagnostics = parse_diagnostics(&compiled.stderr);
    if compiled.timed_out || compiled.cancelled || !compiled.status.map(|s| s.success()).unwrap_or(false) {
        return Ok(RunResult {
            execution_id,
            ok: false,
            timed_out: compiled.timed_out,
            cancelled: compiled.cancelled,
            duration_ms: started.elapsed().as_millis() as u64,
            stdout: compiled.stdout,
            stderr: compiled.stderr,
            diagnostics,
        });
    }

    // ---- Phase 2: execute --------------------------------------------------
    let mut run_cmd = Command::new(&binary);
    apply_sandbox_env(&mut run_cmd);
    run_cmd.current_dir(&workspace);
    let ran = run_child(run_cmd, EXEC_TIMEOUT_SECS, &handle).await?;

    Ok(RunResult {
        execution_id,
        ok: !ran.timed_out && !ran.cancelled && ran.status.map(|s| s.success()).unwrap_or(false),
        timed_out: ran.timed_out,
        cancelled: ran.cancelled,
        duration_ms: started.elapsed().as_millis() as u64,
        stdout: ran.stdout,
        stderr: ran.stderr,
        diagnostics,
    })
}

/// Extract `passed`/`failed` counts from a `cargo test` summary line.
pub fn parse_test_counts(output: &str) -> (u32, u32) {
    for line in output.lines() {
        if !line.contains("test result:") {
            continue;
        }
        let mut passed = 0u32;
        let mut failed = 0u32;
        let mut tokens = line.split_whitespace().peekable();
        while let Some(token) = tokens.next() {
            let next = tokens.peek().copied().unwrap_or("");
            if let Ok(n) = token.parse::<u32>() {
                if next.starts_with("passed") {
                    passed += n;
                } else if next.starts_with("failed") {
                    failed += n;
                }
            }
        }
        return (passed, failed);
    }
    (0, 0)
}

/// Run `cargo test --offline` inside a managed workspace. The path is
/// validated against the sandbox roots before any process is spawned.
pub async fn run_cargo_test(
    registry: &ExecutionRegistry,
    workspace_roots: &[PathBuf],
    raw_path: &str,
) -> AppResult<TestResult> {
    let project_dir = validator::validate_cargo_path(raw_path, workspace_roots)?;
    let execution_id = uuid::Uuid::new_v4().to_string();
    let handle = registry.register(format!("test:{execution_id}"));
    struct Unregister<'a>(&'a ExecutionRegistry, String);
    impl Drop for Unregister<'_> {
        fn drop(&mut self) {
            self.0.unregister(&self.1);
        }
    }
    let _unregister = Unregister(registry, format!("test:{execution_id}"));

    // Keep build artifacts inside the project workspace, isolated from the
    // app's own target dir; deterministic cleanup of the run's target tree.
    let target_dir = project_dir.join(".rm-test-target");
    let _target_guard = WorkspaceGuard(target_dir.clone());

    let started = Instant::now();
    let mut cmd = Command::new("cargo");
    apply_sandbox_env(&mut cmd);
    cmd.arg("test")
        .arg("--offline")
        .env("CARGO_TARGET_DIR", &target_dir)
        .current_dir(&project_dir);

    let ran = run_child(cmd, TEST_TIMEOUT_SECS, &handle).await?;
    let (passed, failed) = parse_test_counts(&ran.stdout);
    Ok(TestResult {
        execution_id,
        ok: !ran.timed_out && !ran.cancelled && ran.status.map(|s| s.success()).unwrap_or(false),
        timed_out: ran.timed_out,
        cancelled: ran.cancelled,
        duration_ms: started.elapsed().as_millis() as u64,
        stdout: ran.stdout,
        stderr: ran.stderr,
        passed,
        failed,
    })
}


/// Best-effort removal of stale sandbox workspaces at startup (> 1 h old).
pub fn sweep_stale_workspaces(root: &Path) {
    let cutoff = std::time::SystemTime::now() - Duration::from_secs(3600);
    let Ok(entries) = std::fs::read_dir(root) else { return };
    for entry in entries.flatten() {
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }
        let stale = entry
            .metadata()
            .and_then(|m| m.modified())
            .map(|t| t < cutoff)
            .unwrap_or(false);
        if stale {
            let _ = std::fs::remove_dir_all(path);
        }
    }
}

static TOOLCHAIN_VERSIONS: OnceLock<(Option<String>, Option<String>)> = OnceLock::new();

fn probe_version(program: &str) -> Option<String> {
    let output = std::process::Command::new(program).arg("--version").output().ok()?;
    if output.status.success() {
        Some(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        None
    }
}

/// Cached `(rustc --version, cargo --version)` — probed at most once per app run.
pub fn toolchain_versions() -> &'static (Option<String>, Option<String>) {
    TOOLCHAIN_VERSIONS.get_or_init(|| (probe_version("rustc"), probe_version("cargo")))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn diagnostics_capture_severity_and_span() {
        let stderr = "error[E0382]: borrow of moved value: `s`\n \
                      --> src/main.rs:4:30\n \
                       |\n \
                      warning: unused variable: `x`\n";
        let diags = parse_diagnostics(stderr);
        assert_eq!(diags.len(), 2);
        assert_eq!(diags[0].severity, "error");
        assert!(diags[0].message.starts_with("error[E0382]"));
        assert_eq!(diags[0].span.as_deref(), Some("--> src/main.rs:4:30"));
        assert_eq!(diags[1].severity, "warning");
    }

    #[test]
    fn plain_output_yields_no_diagnostics() {
        assert!(parse_diagnostics("hello world\nsecond line").is_empty());
    }

    #[test]
    fn test_counts_are_parsed_from_summary_line() {
        let out = "running 3 tests\ntest a ... ok\ntest b ... FAILED\ntests ...\n\
                   test result: FAILED. 2 passed; 1 failed; 0 ignored; 0 measured\n";
        assert_eq!(parse_test_counts(out), (2, 1));
        assert_eq!(parse_test_counts("no summary here"), (0, 0));
    }
}



