# Persistence Architecture

## Storage

Single SQLite database at `<app_data_dir>/rust-mastery.db` (Windows:
`%APPDATA%/dev.rustmastery.offline`). Accessed exclusively through `rusqlite` (bundled SQLite) in
`src-tauri/src/db`. The webview has zero SQL exposure — components see typed DTOs only.

## Migrations

Versioned, ordered, transactional:

```sql
CREATE TABLE IF NOT EXISTS _migrations (
  version INTEGER PRIMARY KEY,
  applied_at INTEGER NOT NULL
);
```

Runner logic: open connection → `PRAGMA foreign_keys = ON` → read applied versions → apply each
pending migration inside one transaction → record version. Re-running is a no-op (idempotent),
and the set of migrations is append-only once shipped.

## Schema v3

| Table                 | Purpose / Key columns                                                        |
|-----------------------|------------------------------------------------------------------------------|
| `settings`            | key/value JSON blobs (theme, editor prefs)                                   |
| `lesson_completions`  | UNIQUE(track, module_id, lesson_id), completed_at                            |
| `quiz_attempts`       | score/total/passed + timestamps (history)                                    |
| `exercise_attempts`   | status(tried/solved), solution_revealed flag                                 |
| `notes`               | scope_type+scope_id, title, body, created/updated                            |
| `bookmarks`           | UNIQUE(item_type, item_id)                                                   |
| `editor_state`        | PK scope_id → language, code, cursor_offset                                  |
| `project_states`      | PK project_id → milestones_json, completed                                   |

Indexes on every non-PK lookup column (`track`, `scope_id`, `item_type`, …). Foreign keys declared
where parent rows exist conceptually; natural-key uniqueness prevents duplicate completions.

## Repository Rules

1. All statements are prepared statements with bound parameters — string interpolation into SQL
   is forbidden (and CI-greppable).
2. Multi-row writes use explicit transactions.
3. Repos return domain DTOs, never `rusqlite::Row`.
4. Timestamps are Unix epoch seconds (`i64`) — timezone-free and trivially serializable.

## Reset & Backup

`reset_all_data()` deletes user tables (not `_migrations`) inside one transaction so schema
versioning survives a reset. Because everything lives in one file, OS-level file copy is a valid
manual backup strategy; documented for users instead of building an auto-backup subsystem.
