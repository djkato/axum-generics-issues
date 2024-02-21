use crate::{AppState, APL};

pub mod manifest;
use axum::{routing::get, Router};
use manifest::manifest;

pub fn create_routes<T: APL>(state: AppState<T>) -> Router<AppState<T>> {
    Router::new()
        .route("/api/manifest", get(manifest))
        .with_state(state)
}
