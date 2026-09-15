//! Repositories: prepared statements, domain DTOs, zero SQL leakage upward.

use rusqlite::{params, Connection, OptionalExtension};

use super::now;
use crate::content::model::{
    BookmarkDto, CompletedLesson, EditorStateDto, ExerciseStat, NoteDto, NoteInput, ProjectStateDto,
    QuizStat, SettingsDto,
};
use crate::error::AppResult;

// ---------- Lesson completions ----------

pub fn list_completed(conn: &Connection) -> AppResult<Vec<CompletedLesson>> {
    let mut stmt = conn.prepare(
        "SELECT track, module_id, lesson_id, completed_at
         FROM lesson_completions ORDER BY completed_at DESC",
    )?;
    let rows = stmt.query_map([], |r| {
        Ok(CompletedLesson {
            track_id: r.get(0)?,
            module_id: r.get(1)?,
            lesson_id: r.get(2)?,
            completed_at: r.get(3)?,
        })
    })?;
    Ok(rows.collect::<Result<Vec<_>, _>>()?)
}

pub fn complete_lesson(
    conn: &Connection,
    track: &str,
    module_id: &str,
    lesson_id: &str,
) -> AppResult<bool> {
    let inserted = conn.execute(
        "INSERT OR IGNORE INTO lesson_completions(track, module_id, lesson_id, completed_at)
         VALUES (?1, ?2, ?3, ?4)",
        params![track, module_id, lesson_id, now()],
    )?;
    Ok(inserted > 0)
}

pub fn uncomplete_lesson(
    conn: &Connection,
    track: &str,
    module_id: &str,
    lesson_id: &str,
) -> AppResult<bool> {
    let removed = conn.execute(
        "DELETE FROM lesson_completions WHERE track=?1 AND module_id=?2 AND lesson_id=?3",
        params![track, module_id, lesson_id],
    )?;
    Ok(removed > 0)
}

pub fn record_quiz_attempt(
    conn: &Connection,
    track: &str,
    module_id: &str,
    lesson_id: &str,
    quiz_id: &str,
    score: i64,
    total: i64,
    passed: bool,
) -> AppResult<()> {
    conn.execute(
        "INSERT INTO quiz_attempts(track, module_id, lesson_id, quiz_id, score, total, passed, attempted_at)
         VALUES (?1,?2,?3,?4,?5,?6,?7,?8)",
        params![track, module_id, lesson_id, quiz_id, score, total, passed as i64, now()],
    )?;
    Ok(())
}

pub fn record_exercise_attempt(
    conn: &Connection,
    track: &str,
    module_id: &str,
    lesson_id: &str,
    exercise_id: &str,
    status: &str,
    solution_revealed: bool,
) -> AppResult<()> {
    conn.execute(
        "INSERT INTO exercise_attempts(track, module_id, lesson_id, exercise_id, status, solution_revealed, attempted_at)
         VALUES (?1,?2,?3,?4,?5,?6,?7)",
        params![track, module_id, lesson_id, exercise_id, status, solution_revealed as i64, now()],
    )?;
    Ok(())
}

/// Per-lesson quiz evidence: attempt count, best percent, ever passed.
pub fn quiz_stats(conn: &Connection) -> AppResult<Vec<QuizStat>> {
    let mut stmt = conn.prepare(
        "SELECT track, module_id, lesson_id, COUNT(*),
                COALESCE(MAX(CASE WHEN total > 0 THEN score * 100 / total ELSE 0 END), 0),
                COALESCE(MAX(passed), 0)
         FROM quiz_attempts
         GROUP BY track, module_id, lesson_id",
    )?;
    let rows = stmt.query_map([], |r| {
        Ok(QuizStat {
            track_id: r.get(0)?,
            module_id: r.get(1)?,
            lesson_id: r.get(2)?,
            attempts: r.get::<_, i64>(3)? as u32,
            best_percent: r.get::<_, i64>(4)? as u32,
            passed: r.get::<_, i64>(5)? > 0,
        })
    })?;
    Ok(rows.collect::<Result<Vec<_>, _>>()?)
}

/// Per-lesson exercise evidence: attempts, solved, solution revealed.
pub fn exercise_stats(conn: &Connection) -> AppResult<Vec<ExerciseStat>> {
    let mut stmt = conn.prepare(
        "SELECT track, module_id, lesson_id, COUNT(*),
                COALESCE(MAX(CASE WHEN status = 'solved' THEN 1 ELSE 0 END), 0),
                COALESCE(MAX(solution_revealed), 0)
         FROM exercise_attempts
         GROUP BY track, module_id, lesson_id",
    )?;
    let rows = stmt.query_map([], |r| {
        Ok(ExerciseStat {
            track_id: r.get(0)?,
            module_id: r.get(1)?,
            lesson_id: r.get(2)?,
            attempts: r.get::<_, i64>(3)? as u32,
            solved: r.get::<_, i64>(4)? > 0,
            solution_revealed: r.get::<_, i64>(5)? > 0,
        })
    })?;
    Ok(rows.collect::<Result<Vec<_>, _>>()?)
}

// ---------- Settings / editor state ----------

pub fn save_setting(conn: &Connection, key: &str, value: &str) -> AppResult<()> {
    conn.execute(
        "INSERT INTO settings(key, value) VALUES (?1, ?2)
         ON CONFLICT(key) DO UPDATE SET value = excluded.value",
        params![key, value],
    )?;
    Ok(())
}

pub fn load_setting(conn: &Connection, key: &str) -> AppResult<Option<String>> {
    let mut stmt = conn.prepare("SELECT value FROM settings WHERE key = ?1")?;
    Ok(stmt.query_row(params![key], |r| r.get::<_, String>(0)).optional()?)
}

pub fn load_app_settings(conn: &Connection) -> AppResult<SettingsDto> {
    match load_setting(conn, "app")? {
        Some(raw) => Ok(serde_json::from_str(&raw).unwrap_or_default()),
        None => Ok(SettingsDto::default()),
    }
}

pub fn save_editor_state(conn: &Connection, state: &EditorStateDto) -> AppResult<()> {
    conn.execute(
        "INSERT INTO editor_state(scope_id, language, code, cursor_offset, updated_at)
         VALUES (?1,?2,?3,?4,?5)
         ON CONFLICT(scope_id) DO UPDATE SET
           language=excluded.language, code=excluded.code,
           cursor_offset=excluded.cursor_offset, updated_at=excluded.updated_at",
        params![state.scope_id, state.language, state.code, state.cursor_offset, now()],
    )?;
    Ok(())
}

pub fn get_editor_state(conn: &Connection, scope_id: &str) -> AppResult<Option<EditorStateDto>> {
    let mut stmt = conn.prepare(
        "SELECT scope_id, language, code, cursor_offset FROM editor_state WHERE scope_id = ?1",
    )?;
    Ok(stmt
        .query_row(params![scope_id], |r| {
            Ok(EditorStateDto {
                scope_id: r.get(0)?,
                language: r.get(1)?,
                code: r.get(2)?,
                cursor_offset: r.get(3)?,
            })
        })
        .optional()?)
}

// ---------- Notes ----------

fn note_from_row(r: &rusqlite::Row<'_>) -> rusqlite::Result<NoteDto> {
    Ok(NoteDto {
        id: r.get(0)?,
        scope_type: r.get(1)?,
        scope_id: r.get(2)?,
        title: r.get(3)?,
        body: r.get(4)?,
        created_at: r.get(5)?,
        updated_at: r.get(6)?,
    })
}

const NOTE_COLS: &str =
    "id, scope_type, scope_id, title, body, created_at, updated_at";

pub fn upsert_note(conn: &Connection, input: &NoteInput) -> AppResult<NoteDto> {
    match input.id {
        Some(id) => {
            conn.execute(
                "UPDATE notes SET scope_type=?1, scope_id=?2, title=?3, body=?4, updated_at=?5 WHERE id=?6",
                params![input.scope_type, input.scope_id, input.title, input.body, now(), id],
            )?;
            get_note(conn, id)?.ok_or_else(|| {
                crate::error::AppError::NotFound(format!("note {id}"))
            })
        }
        None => {
            let ts = now();
            conn.execute(
                "INSERT INTO notes(scope_type, scope_id, title, body, created_at, updated_at)
                 VALUES (?1,?2,?3,?4,?5,?5)",
                params![input.scope_type, input.scope_id, input.title, input.body, ts],
            )?;
            get_note(conn, conn.last_insert_rowid())?
                .ok_or_else(|| crate::error::AppError::Database("inserted note not found".into()))
        }
    }
}

pub fn get_note(conn: &Connection, id: i64) -> AppResult<Option<NoteDto>> {
    let mut stmt = conn.prepare(&format!("SELECT {NOTE_COLS} FROM notes WHERE id = ?1"))?;
    Ok(stmt.query_row(params![id], note_from_row).optional()?)
}

pub fn delete_note(conn: &Connection, id: i64) -> AppResult<bool> {
    Ok(conn.execute("DELETE FROM notes WHERE id = ?1", params![id])? > 0)
}

pub fn list_notes(
    conn: &Connection,
    scope: Option<(&str, &str)>,
) -> AppResult<Vec<NoteDto>> {
    let (sql, bind): (String, Vec<String>) = match scope {
        Some((t, s)) => (
            format!("SELECT {NOTE_COLS} FROM notes WHERE scope_type=?1 AND scope_id=?2 ORDER BY updated_at DESC"),
            vec![t.to_string(), s.to_string()],
        ),
        None => (
            format!("SELECT {NOTE_COLS} FROM notes ORDER BY updated_at DESC"),
            vec![],
        ),
    };
    let mut stmt = conn.prepare(&sql)?;
    let rows = stmt.query_map(rusqlite::params_from_iter(bind.iter()), note_from_row)?;
    Ok(rows.collect::<Result<Vec<_>, _>>()?)
}

// ---------- Bookmarks ----------

pub fn toggle_bookmark(
    conn: &Connection,
    item_type: &str,
    item_id: &str,
) -> AppResult<bool> {
    let exists: bool = conn
        .query_row(
            "SELECT EXISTS(SELECT 1 FROM bookmarks WHERE item_type=?1 AND item_id=?2)",
            params![item_type, item_id],
            |r| r.get::<_, i64>(0),
        )?
        > 0;
    if exists {
        conn.execute(
            "DELETE FROM bookmarks WHERE item_type=?1 AND item_id=?2",
            params![item_type, item_id],
        )?;
        Ok(false)
    } else {
        conn.execute(
            "INSERT INTO bookmarks(item_type, item_id, created_at) VALUES (?1,?2,?3)",
            params![item_type, item_id, now()],
        )?;
        Ok(true)
    }
}

pub fn list_bookmarks(conn: &Connection) -> AppResult<Vec<BookmarkDto>> {
    let mut stmt = conn.prepare(
        "SELECT item_type, item_id, created_at FROM bookmarks ORDER BY created_at DESC",
    )?;
    let rows = stmt.query_map([], |r| {
        Ok(BookmarkDto {
            item_type: r.get(0)?,
            item_id: r.get(1)?,
            created_at: r.get(2)?,
        })
    })?;
    Ok(rows.collect::<Result<Vec<_>, _>>()?)
}

// ---------- Project states ----------

pub fn save_project_state(conn: &Connection, state: &ProjectStateDto) -> AppResult<()> {
    let milestones = serde_json::to_string(&state.milestones)?;
    conn.execute(
        "INSERT INTO project_states(project_id, milestones_json, completed, updated_at)
         VALUES (?1,?2,?3,?4)
         ON CONFLICT(project_id) DO UPDATE SET
           milestones_json=excluded.milestones_json,
           completed=excluded.completed,
           updated_at=excluded.updated_at",
        params![state.project_id, milestones, state.completed as i64, now()],
    )?;
    Ok(())
}

pub fn list_project_states(conn: &Connection) -> AppResult<Vec<ProjectStateDto>> {
    let mut stmt = conn.prepare(
        "SELECT project_id, milestones_json, completed, updated_at FROM project_states",
    )?;
    let rows = stmt.query_map([], |r| {
        let raw: String = r.get(1)?;
        Ok(ProjectStateDto {
            project_id: r.get(0)?,
            milestones: serde_json::from_str(&raw).unwrap_or_default(),
            completed: r.get::<_, i64>(2)? > 0,
            updated_at: r.get(3)?,
        })
    })?;
    Ok(rows.collect::<Result<Vec<_>, _>>()?)
}

// ---------- Reset ----------

/// Wipe user data inside one transaction; schema/migrations survive.
pub fn reset_user_data(conn: &Connection) -> AppResult<()> {
    conn.execute_batch(
        "BEGIN;
         DELETE FROM lesson_completions;
         DELETE FROM quiz_attempts;
         DELETE FROM exercise_attempts;
         DELETE FROM notes;
         DELETE FROM bookmarks;
         DELETE FROM editor_state;
         DELETE FROM project_states;
         DELETE FROM settings;
         COMMIT;",
    )?;
    Ok(())
}

