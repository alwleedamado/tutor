pub mod commands;
pub mod content;
pub mod db;
pub mod engine;
pub mod error;
pub mod execution;
pub mod state;

use tauri::Manager;

use crate::execution::ExecutionRegistry;
use crate::state::{AppState, SandboxPaths};

/// Application entry point wired by both the dev binary and bundled releases.
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            // ---- Persistence ------------------------------------------------
            let data_dir = app.path().app_data_dir()?;
            std::fs::create_dir_all(&data_dir)?;
            let db_path = data_dir.join("rust-mastery.db");
            let mut conn = db::open(&db_path)?;
            let applied = db::migrate(&mut conn)?;
            if applied > 0 {
                eprintln!("[rust-mastery] applied {applied} database migration(s)");
            }
            app.manage(AppState::new(conn, db_path));

            // ---- Execution sandbox -------------------------------------------
            let sandbox_root = std::env::temp_dir().join("rust-mastery-sandbox");
            std::fs::create_dir_all(&sandbox_root)?;
            execution::sweep_stale_workspaces(&sandbox_root);
            app.manage(ExecutionRegistry::default());
            app.manage(SandboxPaths { root: sandbox_root });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::bootstrap::get_bootstrap,
            commands::progress::complete_lesson,
            commands::progress::uncomplete_lesson,
            commands::progress::record_quiz_attempt,
            commands::progress::record_exercise_attempt,
            commands::notes::save_note,
            commands::notes::delete_note,
            commands::notes::list_notes,
            commands::notes::toggle_bookmark,
            commands::notes::list_bookmarks,
            commands::settings::save_settings,
            commands::settings::save_editor_state,
            commands::settings::get_editor_state,
            commands::projects::save_project_state,
            commands::projects::get_project_states,
            commands::system::get_system_info,
            commands::system::reset_all_data,
            commands::exec::run_rust_code,
            commands::exec::cancel_execution,
            commands::exec::run_cargo_test,
        ])
        .run(tauri::generate_context!())
        .expect("error while running Rust Mastery Offline");
}
