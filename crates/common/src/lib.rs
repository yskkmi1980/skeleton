//! Common models, traits, error definitions, and utilities.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// System health & status representation.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SystemHealth {
    pub status: String,
    pub version: String,
    pub timestamp: DateTime<Utc>,
}

impl Default for SystemHealth {
    fn default() -> Self {
        Self {
            status: "ok".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            timestamp: Utc::now(),
        }
    }
}

/// System statistics representation.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct SystemStats {
    pub uptime_seconds: u64,
    pub active_connections: u32,
    pub backend_version: String,
}

/// WebSocket Client-to-Server events.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", content = "payload")]
pub enum ClientEvent {
    Ping,
    Custom {
        name: String,
        data: serde_json::Value,
    },
}

/// WebSocket Server-to-Client events.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", content = "payload")]
pub enum ServerEvent {
    Pong,
    Stats(SystemStats),
    Custom {
        name: String,
        data: serde_json::Value,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_system_health_default() {
        let health = SystemHealth::default();
        assert_eq!(health.status, "ok");
        assert_eq!(health.version, env!("CARGO_PKG_VERSION"));
    }

    #[test]
    fn test_events_serde() {
        let ping = ClientEvent::Ping;
        let serialized = serde_json::to_string(&ping).unwrap();
        assert_eq!(serialized, r#"{"type":"Ping"}"#);

        let parsed: ClientEvent = serde_json::from_str(&serialized).unwrap();
        assert_eq!(parsed, ping);

        let pong = ServerEvent::Pong;
        let s_pong = serde_json::to_string(&pong).unwrap();
        assert_eq!(s_pong, r#"{"type":"Pong"}"#);
    }
}
