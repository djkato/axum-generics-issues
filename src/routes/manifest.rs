use crate::{anyhow_errors::AppError, AppState, APL};
use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    response::Json,
};

struct AuthToken {
    auth_token: String,
}

pub async fn manifest<A: APL>(
    _headers: HeaderMap,
    Json(_auth_token): Json<AuthToken>,
    State(_state): State<AppState<A>>,
) -> Result<StatusCode, AppError> {
    Ok(StatusCode::OK)
}
