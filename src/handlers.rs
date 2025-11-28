use std::sync::Arc;

use crate::{
    error::AppError,
    model::{CreateTask, UpdateTask},
    state::AppState,
    taskrepo::{
        create_task_service, delete_task_serivce, fetch_all_tasks, fetch_task, update_task_service,
    },
};
use axum::{
    Extension, Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};

pub async fn get_tasks(State(state): State<Arc<AppState>>) -> Result<impl IntoResponse, AppError> {
    let tasks = fetch_all_tasks(state).await?;

    Ok((StatusCode::OK, Json(tasks)))
}

pub async fn get_task(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let task = fetch_task(state, &id).await?;
    match task {
        Some(t) => Ok((StatusCode::OK, Json(t))),
        None => Err(AppError::TaskNotFound(id)),
    }
}

pub async fn create_task(
    State(state): State<Arc<AppState>>,
    Extension(username): Extension<String>,
    Json(payload): Json<CreateTask>,
) -> Result<impl IntoResponse, AppError> {
    let task = create_task_service(state, username, payload).await?;

    Ok((StatusCode::OK, Json(task)).into_response())
}

pub async fn update_task(
    State(state): State<Arc<AppState>>,
    Extension(username): Extension<String>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateTask>,
) -> Result<impl IntoResponse, AppError> {
    let task = update_task_service(state, username, id, payload).await?;

    match task {
        Some(t) => Ok((StatusCode::OK, Json(t))),
        None => Err(AppError::FetchError(
            "Internal Error occurred while fetching the updated task".into(),
        )),
    }
}

pub async fn delete_task(
    State(state): State<Arc<AppState>>,
    Extension(username): Extension<String>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let _ = delete_task_serivce(state, username, id).await;

    Ok((StatusCode::NO_CONTENT, ()))
}
