use tauri::State;

use crate::content::model::{EditorStateDto, SettingsDto};
use crate::db::repo;
use crate::error::{AppError, AppResult};
use crate::state::AppState;

use super::lock_db;

const MAX_EDITOR_CODE: usize = 512 * 1024;
const VALID_THEMES: [&str; 3] = ["dark", "light", "system"];

#[tauri::command]
pub async fn save_settings(state: State<'_, AppState>, settings: SettingsDto) -> AppResult<()> {
    if !VALID_THEMES.contains(&settings.theme.as_str()) {
        return Err(AppError::Validation(format!(
            "theme must be one of {VALID_THEMES:?}"
        )));
    }
    if !(10..=28).contains(&settings.font_size) {
        return Err(AppError::Validation("font_size must be 10–28".into()));
    }
    if !(2..=8).contains(&settings.tab_size) {
        return Err(AppError::Validation("tab_size must be 2–8".into()));
    }
    let json = serde_json::to_string(&settings)?;
    let conn = lock_db(&state.db)?;
    repo::save_setting(&conn, "app", &json)
}

#[tauri::command]
pub async fn save_editor_state(
    state: State<'_, AppState>,
    scope_id: String,
    language: String,
    code: String,
    cursor_offset: i64,
) -> AppResult<()> {
    if scope_id.is_empty() || scope_id.len() > 200 {
        return Err(AppError::Validation("invalid editor scope".into()));
    }
    if code.len() > MAX_EDITOR_CODE {
        return Err(AppError::Validation("editor snapshot too large".into()));
    }
    let conn = lock_db(&state.db)?;
    repo::save_editor_state(
        &conn,
        &EditorStateDto { scope_id, language, code, cursor_offset },
    )
}

#[tauri::command]
pub async fn get_editor_state(
    state: State<'_, AppState>,
    scope_id: String,
) -> AppResult<Option<EditorStateDto>> {
    let conn = lock_db(&state.db)?;
    repo::get_editor_state(&conn, &scope_id)
}
