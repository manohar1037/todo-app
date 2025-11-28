use axum::{Json, http::StatusCode, response::IntoResponse};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("DB error{0}")]
    Db(String),
    #[error("Not Found task for this id {0}")]
    TaskNotFound(String),

    #[error("Invalid input: {0}")]
    BadRequest(String),

    #[error("{0}")]
    NotUpdated(String),

    #[error("{0}")]
    NotDeleted(String),

    #[error(" {0}")]
    FetchError(String),

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Bson Conversion error")]
    BsonConversionError,

    #[error("Must provide a reason for rejection")]
    NoRemarksFound,

    #[error("Status is same ,no need to update")]
    SameStatus,
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let status_code = match self {
            AppError::Db(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::TaskNotFound(_) => StatusCode::NOT_FOUND,
            AppError::BadRequest(_) => StatusCode::BAD_REQUEST,
            AppError::NotDeleted(_) => StatusCode::NOT_FOUND,
            AppError::FetchError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::NotUpdated(_) => StatusCode::NOT_MODIFIED,
            AppError::Unauthorized => StatusCode::UNAUTHORIZED,
            AppError::BsonConversionError => StatusCode::NOT_MODIFIED,
            AppError::NoRemarksFound => StatusCode::BAD_REQUEST,
            AppError::SameStatus => StatusCode::NOT_MODIFIED,
        };
        let message = format!("{self}");
        let body = Json(serde_json::json!({"message":message}));
        (status_code, body).into_response()
    }
}

impl From<mongodb::error::Error> for AppError {
    fn from(e: mongodb::error::Error) -> Self {
        AppError::Db(e.to_string())
    }
}
