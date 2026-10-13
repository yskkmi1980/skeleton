pub mod error;

use gloo_net::http::{Request, RequestBuilder, Response};

pub use crate::utils::storage::{get_stored_token, set_stored_token};
pub use error::ApiError;

/// Add Authorization header (Bearer token) if present.
pub(crate) fn auth_request(builder: RequestBuilder) -> RequestBuilder {
    if let Some(token) = get_stored_token() {
        if !token.is_empty() {
            return builder.header("Authorization", &format!("Bearer {}", token));
        }
    }
    builder
}

/// Perform authenticated GET request.
pub async fn get(url: &str) -> Result<Response, ApiError> {
    let resp = auth_request(Request::get(url))
        .send()
        .await
        .map_err(|e| ApiError::Network(e.to_string()))?;
    check_response(resp).await
}

/// Perform authenticated POST request with JSON body.
pub async fn post<T: serde::Serialize>(url: &str, body: &T) -> Result<Response, ApiError> {
    let resp = auth_request(Request::post(url))
        .json(body)
        .map_err(|e| ApiError::Parse(e.to_string()))?
        .send()
        .await
        .map_err(|e| ApiError::Network(e.to_string()))?;
    check_response(resp).await
}

/// Perform authenticated PUT request with JSON body.
pub async fn put<T: serde::Serialize>(url: &str, body: &T) -> Result<Response, ApiError> {
    let resp = auth_request(Request::put(url))
        .json(body)
        .map_err(|e| ApiError::Parse(e.to_string()))?
        .send()
        .await
        .map_err(|e| ApiError::Network(e.to_string()))?;
    check_response(resp).await
}

/// Perform authenticated DELETE request.
pub async fn delete(url: &str) -> Result<Response, ApiError> {
    let resp = auth_request(Request::delete(url))
        .send()
        .await
        .map_err(|e| ApiError::Network(e.to_string()))?;
    check_response(resp).await
}

/// Check response status, returning ApiError::Http on failure.
pub async fn check_response(resp: Response) -> Result<Response, ApiError> {
    if resp.ok() {
        Ok(resp)
    } else {
        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        Err(ApiError::Http { status, body })
    }
}

/// Parse JSON response into target type.
pub async fn parse_json<T: serde::de::DeserializeOwned>(resp: Response) -> Result<T, ApiError> {
    resp.json::<T>()
        .await
        .map_err(|e| ApiError::Parse(e.to_string()))
}

/// Fetch system health status.
pub async fn fetch_health() -> Result<common::SystemHealth, ApiError> {
    let resp = get("/api/health").await?;
    parse_json(resp).await
}
