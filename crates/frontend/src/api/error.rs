//! API クライアント共通エラー型。
//!
//! 従来の `Result<T, String>` に代わり、エラー種別（ネットワーク / HTTP / パース）
//! を区別して伝播する。表示は `Display` で従来と同等のメッセージを生成する。

use std::fmt;

#[derive(Debug)]
pub enum ApiError {
    /// ネットワーク層での失敗（接続不能、DNS 等）
    Network(String),
    /// HTTP レスポンスが非 2xx を返した場合
    Http { status: u16, body: String },
    /// レスポンス本文のパース失敗
    Parse(String),
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Network(msg) => write!(f, "Network error: {}", msg),
            Self::Http { status, body } => {
                if body.trim().is_empty() {
                    write!(f, "HTTP {}", status)
                } else {
                    write!(f, "HTTP {}: {}", status, body)
                }
            }
            Self::Parse(msg) => write!(f, "Failed to parse response: {}", msg),
        }
    }
}

impl std::error::Error for ApiError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_network() {
        let e = ApiError::Network("timeout".to_string());
        assert_eq!(e.to_string(), "Network error: timeout");
    }

    #[test]
    fn display_http_with_body() {
        let e = ApiError::Http {
            status: 403,
            body: "forbidden".to_string(),
        };
        assert_eq!(e.to_string(), "HTTP 403: forbidden");
    }

    #[test]
    fn display_http_without_body() {
        let e = ApiError::Http {
            status: 500,
            body: "   ".to_string(),
        };
        assert_eq!(e.to_string(), "HTTP 500");
    }

    #[test]
    fn display_parse() {
        let e = ApiError::Parse("bad json".to_string());
        assert_eq!(e.to_string(), "Failed to parse response: bad json");
    }
}
