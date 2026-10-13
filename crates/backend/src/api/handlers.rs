//! API Handlers and routing.

use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::IntoResponse,
    Json,
};
use common::{ClientEvent, ServerEvent, SystemHealth, SystemStats};
use futures_util::{SinkExt, StreamExt};
use std::sync::Arc;
use tokio::sync::broadcast;
use tracing::{info, warn};

use crate::{api::error::AppError, db::Database};

/// Health check handler.
pub async fn get_system_health() -> Result<Json<SystemHealth>, AppError> {
    Ok(Json(SystemHealth::default()))
}

/// System stats handler.
pub async fn get_system_stats(
    State(db): State<Arc<Database>>,
) -> Result<Json<SystemStats>, AppError> {
    // Read or compute system stats
    let _ = db;
    Ok(Json(SystemStats {
        uptime_seconds: 0,
        active_connections: 0,
        backend_version: env!("CARGO_PKG_VERSION").to_string(),
    }))
}

/// WebSocket endpoint handler.
pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(event_tx): State<broadcast::Sender<ServerEvent>>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, event_tx))
}

async fn handle_socket(socket: WebSocket, event_tx: broadcast::Sender<ServerEvent>) {
    let (mut sender, mut receiver) = socket.split();
    let mut rx = event_tx.subscribe();

    // Spawn task to forward broadcast events to this client
    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            if let Ok(text) = serde_json::to_string(&msg) {
                if sender.send(Message::Text(text.into())).await.is_err() {
                    break;
                }
            }
        }
    });

    // Handle incoming client messages
    let tx_clone = event_tx.clone();
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            match msg {
                Message::Text(text) => {
                    if let Ok(client_event) = serde_json::from_str::<ClientEvent>(&text) {
                        match client_event {
                            ClientEvent::Ping => {
                                let _ = tx_clone.send(ServerEvent::Pong);
                            }
                            ClientEvent::Custom { name, data } => {
                                info!(name = %name, "Received client custom event");
                                let _ = tx_clone.send(ServerEvent::Custom { name, data });
                            }
                        }
                    } else {
                        warn!("Received invalid client event payload: {}", text);
                    }
                }
                Message::Close(_) => break,
                _ => {}
            }
        }
    });

    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    };
}
