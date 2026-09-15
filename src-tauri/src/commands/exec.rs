use tauri::State;

use crate::error::AppResult;
use crate::execution::{self, RunResult, TestResult};
use crate::execution::ExecutionRegistry;
use crate::state::SandboxPaths;

/// Compile and run a student program in an isolated sandbox.
#[tauri::command]
pub async fn run_rust_code(
    registry: State<'_, ExecutionRegistry>,
    sandbox: State<'_, SandboxPaths>,
    code: String,
) -> AppResult<RunResult> {
    execution::run_rust_code(&registry, &sandbox.root, &code).await
}

/// Cancel a live run by its execution id (works for runs and test suites).
#[tauri::command]
pub async fn cancel_execution(
    registry: State<'_, ExecutionRegistry>,
    execution_id: String,
) -> AppResult<bool> {
    if registry.cancel(&format!("run:{execution_id}")) {
        return Ok(true);
    }
    Ok(registry.cancel(&format!("test:{execution_id}")))
}

/// Run `cargo test --offline` for a managed workspace project.
#[tauri::command]
pub async fn run_cargo_test(
    registry: State<'_, ExecutionRegistry>,
    sandbox: State<'_, SandboxPaths>,
    path: String,
) -> AppResult<TestResult> {
    let roots = vec![sandbox.root.clone()];
    execution::run_cargo_test(&registry, &roots, &path).await
}

