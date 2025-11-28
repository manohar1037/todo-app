use axum::{
    Router, middleware,
    response::IntoResponse,
    routing::{get, post},
};
use std::sync::Arc;

use crate::{
    auth::require_basic_auth,
    handlers::{create_task, delete_task, get_task, get_tasks, update_task},
    state::AppState,
};

pub fn create_router(state: Arc<AppState>) -> Router {
    let tasks_router = Router::new()
        .route("/tasks", post(create_task).get(get_tasks))
        .route(
            "/tasks/{id}",
            get(get_task).put(update_task).delete(delete_task),
        )
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            require_basic_auth,
        ));

    Router::new()
        .route("/ping", get(ping))
        .merge(tasks_router)
        .with_state(state)
}

async fn ping() -> impl IntoResponse {
    "Hello"
}
