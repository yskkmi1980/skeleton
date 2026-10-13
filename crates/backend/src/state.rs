//! Application shared state.

use axum::extract::FromRef;
use std::sync::Arc;
use tokio::sync::broadcast;

use crate::db::Database;
use common::ServerEvent;

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<Database>,
    pub http_client: reqwest::Client,
    pub event_tx: broadcast::Sender<ServerEvent>,
}

impl AppState {
    pub fn new(db: Arc<Database>) -> Self {
        let (event_tx, _) = broadcast::channel(100);
        Self {
            db,
            http_client: reqwest::Client::new(),
            event_tx,
        }
    }
}

impl FromRef<AppState> for Arc<Database> {
    fn from_ref(app_state: &AppState) -> Self {
        app_state.db.clone()
    }
}

impl FromRef<AppState> for broadcast::Sender<ServerEvent> {
    fn from_ref(app_state: &AppState) -> Self {
        app_state.event_tx.clone()
    }
}
