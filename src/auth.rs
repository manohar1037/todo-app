use std::sync::Arc;

use axum::{
    extract::{Request, State},
    middleware::Next,
    response::IntoResponse,
};
use axum_auth::AuthBasic;

use crate::{error::AppError, state::AppState};

pub async fn require_basic_auth(
    State(state): State<Arc<AppState>>,
    AuthBasic((username, password)): AuthBasic,
    mut req: Request,
    next: Next,
) -> Result<impl IntoResponse, AppError> {
    if password.is_none() {
        return Err(AppError::Unauthorized);
    }
    let actual_user = username;
    let actual_pass = password.unwrap_or_default();

    if actual_user == state.basic_user() && actual_pass == state.basic_pass() {
        req.extensions_mut().insert::<String>(actual_user);
        Ok(next.run(req).await)
    } else {
        Err(AppError::Unauthorized)
    }
}
