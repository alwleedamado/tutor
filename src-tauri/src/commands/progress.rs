use tauri::State;

use crate::content::model::{CompletedLesson, ProgressSummary};
use crate::content::Catalog;
use crate::db::repo;
use crate::engine;
use crate::error::{AppError, AppResult};
use crate::state::AppState;

use super::{find_lesson, lock_db};

fn fresh_summary(
    catalog: &Catalog,
    capstone_prerequisites: &[String],
    conn: &rusqlite::Connection,
) -> AppResult<ProgressSummary> {
    let completed = repo::list_completed(conn)?;
    let quiz_stats = repo::quiz_stats(conn)?;
    let exercise_stats = repo::exercise_stats(conn)?;
    let projects = crate::content::load_projects()?;
    let project_states = repo::list_project_states(conn)?;
    let completed_projects: Vec<String> = project_states
        .into_iter()
        .filter(|p| p.completed)
        .map(|p| p.project_id)
        .collect();
    let graph = crate::content::load_skill_graph().ok();
    Ok(engine::summarize_full(
        catalog,
        &completed,
        capstone_prerequisites,
        graph.as_ref(),
        &quiz_stats,
        &exercise_stats,
        &projects,
        &completed_projects,
    ))
}

fn load_catalog_and_capstone() -> AppResult<(Catalog, Vec<String>)> {
    let catalog = crate::content::load_catalog()?;
    let capstone = crate::content::load_capstone()?;
    Ok((catalog, capstone.prerequisites))
}

#[tauri::command]
pub async fn complete_lesson(
    state: State<'_, AppState>,
    track_id: String,
    module_id: String,
    lesson_id: String,
) -> AppResult<ProgressSummary> {
    let (catalog, capstone_prereqs) = load_catalog_and_capstone()?;
    // Validate against the real catalog before persisting anything.
    let lesson = find_lesson(&catalog, &track_id, &module_id, &lesson_id)?;
    let (canonical_track, canonical_module, canonical_lesson) =
        (track_id.clone(), lesson.module_id.clone(), lesson.id.clone());
    let conn = lock_db(&state.db)?;
    repo::complete_lesson(&conn, &canonical_track, &canonical_module, &canonical_lesson)?;
    fresh_summary(&catalog, &capstone_prereqs, &conn)
}

#[tauri::command]
pub async fn uncomplete_lesson(
    state: State<'_, AppState>,
    track_id: String,
    module_id: String,
    lesson_id: String,
) -> AppResult<ProgressSummary> {
    let (catalog, capstone_prereqs) = load_catalog_and_capstone()?;
    let lesson = find_lesson(&catalog, &track_id, &module_id, &lesson_id)?;
    let (canonical_track, canonical_module, canonical_lesson) =
        (track_id.clone(), lesson.module_id.clone(), lesson.id.clone());
    let conn = lock_db(&state.db)?;
    repo::uncomplete_lesson(&conn, &canonical_track, &canonical_module, &canonical_lesson)?;
    fresh_summary(&catalog, &capstone_prereqs, &conn)
}

#[tauri::command]
pub async fn record_quiz_attempt(
    state: State<'_, AppState>,
    track_id: String,
    module_id: String,
    lesson_id: String,
    quiz_id: String,
    score: i64,
    total: i64,
) -> AppResult<()> {
    let catalog = crate::content::load_catalog()?;
    find_lesson(&catalog, &track_id, &module_id, &lesson_id)?;
    if !(0..=100).contains(&total) || !(0..=total).contains(&score) {
        return Err(AppError::Validation(
            "quiz scores must satisfy 0 ≤ score ≤ total ≤ 100".into(),
        ));
    }
    let conn = lock_db(&state.db)?;
    repo::record_quiz_attempt(
        &conn,
        &track_id,
        &module_id,
        &lesson_id,
        &quiz_id,
        score,
        total,
        total > 0 && (score * 100) >= (total * 70),
    )
}

#[tauri::command]
pub async fn record_exercise_attempt(
    state: State<'_, AppState>,
    track_id: String,
    module_id: String,
    lesson_id: String,
    exercise_id: String,
    status: String,
    solution_revealed: bool,
) -> AppResult<()> {
    let catalog = crate::content::load_catalog()?;
    find_lesson(&catalog, &track_id, &module_id, &lesson_id)?;
    if status != "tried" && status != "solved" {
        return Err(AppError::Validation(
            "exercise status must be 'tried' or 'solved'".into(),
        ));
    }
    let conn = lock_db(&state.db)?;
    repo::record_exercise_attempt(
        &conn,
        &track_id,
        &module_id,
        &lesson_id,
        &exercise_id,
        &status,
        solution_revealed,
    )
}

/// Re-exported for tests that need the raw row type.
pub type CompletedRow = CompletedLesson;
