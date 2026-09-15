use tauri::State;

use crate::content::model::{BookmarkDto, NoteDto, NoteInput};
use crate::db::repo;
use crate::error::{AppError, AppResult};
use crate::state::AppState;

use super::lock_db;

const MAX_NOTE_BODY: usize = 50_000;
const VALID_SCOPES: [&str; 3] = ["lesson", "project", "general"];

fn validate_note(input: &NoteInput) -> AppResult<()> {
    if !VALID_SCOPES.contains(&input.scope_type.as_str()) {
        return Err(AppError::Validation(format!(
            "scope_type must be one of {VALID_SCOPES:?}"
        )));
    }
    if input.scope_id.len() > 200 {
        return Err(AppError::Validation("scope_id too long".into()));
    }
    if input.title.len() > 200 {
        return Err(AppError::Validation("note title too long (max 200)".into()));
    }
    if input.body.len() > MAX_NOTE_BODY {
        return Err(AppError::Validation("note body too long".into()));
    }
    Ok(())
}

#[tauri::command]
pub async fn save_note(state: State<'_, AppState>, note: NoteInput) -> AppResult<NoteDto> {
    validate_note(&note)?;
    let conn = lock_db(&state.db)?;
    repo::upsert_note(&conn, &note)
}

#[tauri::command]
pub async fn delete_note(state: State<'_, AppState>, id: i64) -> AppResult<bool> {
    let conn = lock_db(&state.db)?;
    repo::delete_note(&conn, id)
}

#[tauri::command]
pub async fn list_notes(
    state: State<'_, AppState>,
    scope_type: Option<String>,
    scope_id: Option<String>,
) -> AppResult<Vec<NoteDto>> {
    let conn = lock_db(&state.db)?;
    match (scope_type, scope_id) {
        (Some(t), Some(s)) => repo::list_notes(&conn, Some((&t, &s))),
        _ => repo::list_notes(&conn, None),
    }
}

#[tauri::command]
pub async fn toggle_bookmark(
    state: State<'_, AppState>,
    item_type: String,
    item_id: String,
) -> AppResult<bool> {
    if item_type != "lesson" && item_type != "project" {
        return Err(AppError::Validation(
            "item_type must be 'lesson' or 'project'".into(),
        ));
    }
    if item_id.is_empty() || item_id.len() > 200 {
        return Err(AppError::Validation("invalid item_id".into()));
    }
    let conn = lock_db(&state.db)?;
    repo::toggle_bookmark(&conn, &item_type, &item_id)
}

#[tauri::command]
pub async fn list_bookmarks(state: State<'_, AppState>) -> AppResult<Vec<BookmarkDto>> {
    let conn = lock_db(&state.db)?;
    repo::list_bookmarks(&conn)
}
