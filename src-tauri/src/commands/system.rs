use serde::Serialize;
use tauri::{AppHandle, State};


use crate::db::repo;
use crate::error::AppResult;
use crate::execution;
use crate::state::AppState;

use super::lock_db;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemInfo {
    pub os: String,
    pub arch: String,
    pub app_version: String,
    pub rustc_version: Option<String>,
    pub cargo_version: Option<String>,
    pub db_path: String,
}

#[tauri::command]
pub async fn get_system_info(app: AppHandle, state: State<'_, AppState>) -> AppResult<SystemInfo> {
    let (rustc_version, cargo_version) = execution::toolchain_versions();
    Ok(SystemInfo {
        os: std::env::consts::OS.to_string(),
        arch: std::env::consts::ARCH.to_string(),
        app_version: app.package_info().version.to_string(),
        rustc_version: rustc_version.clone(),
        cargo_version: cargo_version.clone(),
        db_path: state.db_path.display().to_string(),
    })
}

/// Destructive: wipes all user data (progress, notes, bookmarks, editor
/// snapshots, project states, settings). Schema and migrations survive.
#[tauri::command]
pub async fn reset_all_data(state: State<'_, AppState>) -> AppResult<()> {
    let conn = lock_db(&state.db)?;
    repo::reset_user_data(&conn)
}
