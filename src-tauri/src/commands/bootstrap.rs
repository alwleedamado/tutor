use tauri::State;

use crate::content::{self, model::Bootstrap};
use crate::db::repo;
use crate::engine;
use crate::error::AppResult;
use crate::state::AppState;

use super::lock_db;

/// One-shot startup payload: everything the UI needs on first paint.
#[tauri::command]
pub async fn get_bootstrap(state: State<'_, AppState>) -> AppResult<Bootstrap> {
    let catalog = content::load_catalog()?;
    let projects = content::load_projects()?;
    let capstone = content::load_capstone()?;
    let skill_graph = content::load_skill_graph().ok();
    let conn = lock_db(&state.db)?;
    let completed = repo::list_completed(&conn)?;
    let settings = repo::load_app_settings(&conn)?;
    let quiz_stats = repo::quiz_stats(&conn)?;
    let exercise_stats = repo::exercise_stats(&conn)?;
    let project_states = repo::list_project_states(&conn)?;
    drop(conn);
    let completed_projects: Vec<String> = project_states
        .into_iter()
        .filter(|p| p.completed)
        .map(|p| p.project_id)
        .collect();
    let progress = engine::summarize_full(
        &catalog,
        &completed,
        &capstone.prerequisites,
        skill_graph.as_ref(),
        &quiz_stats,
        &exercise_stats,
        &projects,
        &completed_projects,
    );
    Ok(Bootstrap { catalog, projects, capstone, progress, settings, skill_graph })
}
