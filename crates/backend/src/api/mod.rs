pub mod error;
pub mod handlers;

use axum::{routing::get, Router};
use std::sync::Arc;

use crate::state::AppState;

pub fn build_api_router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/health", get(handlers::get_system_health))
        .route("/stats", get(handlers::get_system_stats))
        .route("/ws", get(handlers::ws_handler))
        .with_state((*state).clone())
}
