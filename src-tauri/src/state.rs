use std::path::PathBuf;
use std::sync::Mutex;

use rusqlite::Connection;

/// Managed application state. Holds the SQLite connection behind a mutex so
/// commands share one connection while long operations never block on it.
pub struct AppState {
    pub db: Mutex<Connection>,
    /// Location of the SQLite file (surfaced by get_system_info).
    pub db_path: PathBuf,
}

/// Filesystem locations owned by the execution subsystem.
pub struct SandboxPaths {
    /// Root directory under which every student-code workspace is created.
    pub root: PathBuf,
}

impl AppState {
    pub fn new(db: Connection, db_path: PathBuf) -> Self {
        Self { db: Mutex::new(db), db_path }
    }
}

