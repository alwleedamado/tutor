pub mod bootstrap;
pub mod exec;
pub mod notes;
pub mod progress;
pub mod projects;
pub mod settings;
pub mod system;

use crate::content::model::{Catalog, Lesson};
use crate::error::{AppError, AppResult};

/// Resolve a lesson by composite key — shared validation for progress commands.
pub(crate) fn find_lesson<'a>(
    catalog: &'a Catalog,
    track_id: &str,
    module_slug_or_id: &str,
    lesson_slug_or_id: &str,
) -> AppResult<&'a Lesson> {
    // Accept either ids or slugs so routes can stay slug-based while the DB
    // stores canonical ids.
    let track = catalog
        .tracks
        .iter()
        .find(|t| t.id == track_id)
        .ok_or_else(|| AppError::NotFound(format!("track '{track_id}'")))?;
    let module = track
        .modules
        .iter()
        .find(|m| m.id == module_slug_or_id || m.slug == module_slug_or_id)
        .ok_or_else(|| AppError::NotFound(format!("module '{module_slug_or_id}'")))?;
    module
        .lessons
        .iter()
        .find(|l| l.id == lesson_slug_or_id || l.slug == lesson_slug_or_id)
        .ok_or_else(|| AppError::NotFound(format!("lesson '{lesson_slug_or_id}'")))
}

/// Lock a poisoned mutex without panicking — recover the guard instead.
pub(crate) fn lock_db<T>(mutex: &std::sync::Mutex<T>) -> AppResult<std::sync::MutexGuard<'_, T>> {
    mutex.lock().map_err(|poisoned| {
        AppError::Database(format!("state mutex recovered from poison: {poisoned}"))
    })
}
