pub mod repo;

use rusqlite::Connection;

use crate::error::AppResult;

/// Append-only migration set. Once shipped, entries are never edited —
/// fixes go into new migrations (see docs/persistence.md).
pub const MIGRATIONS: &[&str] = &[M001_INITIAL, M002_NOTE_INDEX];

const M001_INITIAL: &str = r#"
CREATE TABLE IF NOT EXISTS settings (
    key         TEXT PRIMARY KEY,
    value       TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS lesson_completions (
    id           INTEGER PRIMARY KEY AUTOINCREMENT,
    track        TEXT NOT NULL,
    module_id    TEXT NOT NULL,
    lesson_id    TEXT NOT NULL,
    completed_at INTEGER NOT NULL,
    UNIQUE(track, module_id, lesson_id)
);
CREATE INDEX IF NOT EXISTS idx_lc_track ON lesson_completions(track);

CREATE TABLE IF NOT EXISTS quiz_attempts (
    id           INTEGER PRIMARY KEY AUTOINCREMENT,
    track        TEXT NOT NULL,
    module_id    TEXT NOT NULL,
    lesson_id    TEXT NOT NULL,
    quiz_id      TEXT NOT NULL,
    score        INTEGER NOT NULL,
    total        INTEGER NOT NULL,
    passed       INTEGER NOT NULL,
    attempted_at INTEGER NOT NULL
);
CREATE INDEX IF NOT EXISTS idx_qa_lesson ON quiz_attempts(track, module_id, lesson_id);

CREATE TABLE IF NOT EXISTS exercise_attempts (
    id                INTEGER PRIMARY KEY AUTOINCREMENT,
    track             TEXT NOT NULL,
    module_id         TEXT NOT NULL,
    lesson_id         TEXT NOT NULL,
    exercise_id       TEXT NOT NULL,
    status            TEXT NOT NULL,
    solution_revealed INTEGER NOT NULL DEFAULT 0,
    attempted_at      INTEGER NOT NULL
);
CREATE INDEX IF NOT EXISTS idx_ea_lesson ON exercise_attempts(track, module_id, lesson_id);

CREATE TABLE IF NOT EXISTS notes (
    id         INTEGER PRIMARY KEY AUTOINCREMENT,
    scope_type TEXT NOT NULL,
    scope_id   TEXT NOT NULL,
    title      TEXT NOT NULL DEFAULT '',
    body       TEXT NOT NULL DEFAULT '',
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL
);
CREATE INDEX IF NOT EXISTS idx_notes_scope ON notes(scope_type, scope_id);

CREATE TABLE IF NOT EXISTS bookmarks (
    id        INTEGER PRIMARY KEY AUTOINCREMENT,
    item_type TEXT NOT NULL,
    item_id   TEXT NOT NULL,
    created_at INTEGER NOT NULL,
    UNIQUE(item_type, item_id)
);

CREATE TABLE IF NOT EXISTS editor_state (
    scope_id      TEXT PRIMARY KEY,
    language      TEXT NOT NULL,
    code          TEXT NOT NULL,
    cursor_offset INTEGER NOT NULL DEFAULT 0,
    updated_at    INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS project_states (
    project_id      TEXT PRIMARY KEY,
    milestones_json TEXT NOT NULL,
    completed       INTEGER NOT NULL DEFAULT 0,
    updated_at      INTEGER NOT NULL
);
"#;

const M002_NOTE_INDEX: &str =
    "CREATE INDEX IF NOT EXISTS idx_notes_updated ON notes(updated_at DESC);";

pub fn now() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0)
}

pub fn open(path: &std::path::Path) -> AppResult<Connection> {
    let conn = Connection::open(path)?;
    conn.pragma_update(None, "foreign_keys", "ON")?;
    Ok(conn)
}

/// Apply pending migrations transactionally; returns how many were applied.
/// Re-running is a no-op.
pub fn migrate(conn: &mut Connection) -> AppResult<usize> {
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS _migrations (
            version    INTEGER PRIMARY KEY,
            applied_at INTEGER NOT NULL
        );",
    )?;
    let mut applied = Vec::new();
    {
        let mut stmt = conn.prepare("SELECT version FROM _migrations")?;
        let rows = stmt.query_map([], |row| row.get::<_, i64>(0))?;
        for row in rows {
            applied.push(row?);
        }
    }
    let mut count = 0usize;
    for (i, sql) in MIGRATIONS.iter().enumerate() {
        let version = (i + 1) as i64;
        if applied.contains(&version) {
            continue;
        }
        let tx = conn.transaction()?;
        tx.execute_batch(sql)?;
        tx.execute(
            "INSERT INTO _migrations(version, applied_at) VALUES (?1, ?2)",
            rusqlite::params![version, now()],
        )?;
        tx.commit()?;
        count += 1;
    }
    Ok(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn migrations_apply_and_are_idempotent() {
        let mut conn = Connection::open_in_memory().unwrap();
        assert_eq!(migrate(&mut conn).unwrap(), MIGRATIONS.len());
        assert_eq!(migrate(&mut conn).unwrap(), 0, "second run must be a no-op");

        let tables: Vec<String> = {
            let mut stmt = conn
                .prepare("SELECT name FROM sqlite_master WHERE type='table' ORDER BY name")
                .unwrap();
            let rows = stmt.query_map([], |r| r.get::<_, String>(0)).unwrap();
            rows.map(|r| r.unwrap()).collect()
        };
        for expected in [
            "_migrations",
            "settings",
            "lesson_completions",
            "quiz_attempts",
            "exercise_attempts",
            "notes",
            "bookmarks",
            "editor_state",
            "project_states",
        ] {
            assert!(tables.iter().any(|t| t == expected), "missing table {expected}");
        }
    }

    #[test]
    fn unique_constraint_prevents_duplicate_completions() {
        let mut conn = Connection::open_in_memory().unwrap();
        migrate(&mut conn).unwrap();
        let first = repo::complete_lesson(&conn, "rust", "rust-01", "l1").unwrap();
        let second = repo::complete_lesson(&conn, "rust", "rust-01", "l1").unwrap();
        assert!(first);
        assert!(!second, "duplicate completion must be ignored");
        assert_eq!(repo::list_completed(&conn).unwrap().len(), 1);
    }
}
