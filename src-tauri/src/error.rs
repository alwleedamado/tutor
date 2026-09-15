use serde::{Deserialize, Serialize};

/// Application-wide structured error. Crosses IPC as `{ kind, message }` — never a bare string.
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("database error: {0}")]
    Database(String),
    #[error("content error: {0}")]
    Content(String),
    #[error("validation error: {0}")]
    Validation(String),
    #[error("execution error: {0}")]
    Execution(String),
    #[error("not found: {0}")]
    NotFound(String),
    #[error("io error: {0}")]
    Io(String),
}

pub type AppResult<T> = Result<T, AppError>;

impl From<rusqlite::Error> for AppError {
    fn from(e: rusqlite::Error) -> Self {
        AppError::Database(e.to_string())
    }
}

impl From<std::io::Error> for AppError {
    fn from(e: std::io::Error) -> Self {
        AppError::Io(e.to_string())
    }
}

impl From<serde_json::Error> for AppError {
    fn from(e: serde_json::Error) -> Self {
        AppError::Content(e.to_string())
    }
}

impl AppError {
    fn kind(&self) -> &'static str {
        match self {
            AppError::Database(_) => "database",
            AppError::Content(_) => "content",
            AppError::Validation(_) => "validation",
            AppError::Execution(_) => "execution",
            AppError::NotFound(_) => "notFound",
            AppError::Io(_) => "io",
        }
    }
}

/// IPC-safe projection of [`AppError`].
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppErrorDto {
    pub kind: String,
    pub message: String,
}

impl From<&AppError> for AppErrorDto {
    fn from(e: &AppError) -> Self {
        AppErrorDto { kind: e.kind().to_string(), message: e.to_string() }
    }
}

impl Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        AppErrorDto::from(self).serialize(serializer)
    }
}
