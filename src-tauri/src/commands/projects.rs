use tauri::State;

use crate::content::model::ProjectStateDto;
use crate::db::repo;
use crate::error::{AppError, AppResult};
use crate::state::AppState;

use super::lock_db;

#[tauri::command]
pub async fn save_project_state(state: State<'_, AppState>, project: ProjectStateDto) -> AppResult<()> {
    // Validate against the embedded catalog — unknown project ids are rejected.
    let projects = crate::content::load_projects()?;
    if !projects.iter().any(|p| p.id == project.project_id) {
        return Err(AppError::NotFound(format!("project '{}'", project.project_id)));
    }
    if project.milestones.len() > 100 || project.milestones.iter().any(|m| m.len() > 120) {
        return Err(AppError::Validation("invalid milestone list".into()));
    }
    let conn = lock_db(&state.db)?;
    repo::save_project_state(&conn, &project)
}

#[tauri::command]
pub async fn get_project_states(state: State<'_, AppState>) -> AppResult<Vec<ProjectStateDto>> {
    let conn = lock_db(&state.db)?;
    repo::list_project_states(&conn)
}
