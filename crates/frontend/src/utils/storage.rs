//! localStorage アクセスの一元化。
//!
//! 認証トークン・APIキー・言語設定などのブラウザ内永続化は、
//! このモジュール経由でのみ行われること。

const TOKEN_KEY: &str = "rwk_auth_token";
const API_KEY_KEY: &str = "rwk_api_key";
const LANGUAGE_KEY: &str = "rwk_language";

fn storage() -> Option<web_sys::Storage> {
    web_sys::window()?.local_storage().ok()?
}

pub fn get_stored_token() -> Option<String> {
    storage()?.get_item(TOKEN_KEY).ok()?
}

pub fn set_stored_token(token: Option<&str>) {
    if let Some(s) = storage() {
        match token {
            Some(t) => {
                let _ = s.set_item(TOKEN_KEY, t);
            }
            None => {
                let _ = s.remove_item(TOKEN_KEY);
            }
        }
    }
}

pub fn get_stored_api_key() -> Option<String> {
    storage()?.get_item(API_KEY_KEY).ok()?
}

pub fn set_stored_api_key(key: Option<&str>) {
    if let Some(s) = storage() {
        match key {
            Some(k) => {
                let _ = s.set_item(API_KEY_KEY, k);
            }
            None => {
                let _ = s.remove_item(API_KEY_KEY);
            }
        }
    }
}

pub fn get_stored_language() -> Option<String> {
    storage()?.get_item(LANGUAGE_KEY).ok()?
}

pub fn set_stored_language(lang: &str) {
    if let Some(s) = storage() {
        let _ = s.set_item(LANGUAGE_KEY, lang);
    }
}

/// 汎用 bool 値を読み込む（未設定・不正値時は None）。
pub fn get_stored_bool_opt(key: &str) -> Option<bool> {
    storage()
        .and_then(|s| s.get_item(key).ok().flatten())
        .map(|v| v == "true")
}

/// 汎用 bool 値を読み込む（未設定時は false）。
pub fn get_stored_bool(key: &str) -> bool {
    get_stored_bool_opt(key).unwrap_or(false)
}

/// 汎用 bool 値を永続化する。
pub fn set_stored_bool(key: &str, v: bool) {
    if let Some(s) = storage() {
        let _ = s.set_item(key, &v.to_string());
    }
}

/// 汎用 u32 値（px サイズ等）を読み込む（未設定・不正値時は None）。
pub fn get_stored_u32(key: &str) -> Option<u32> {
    storage()
        .and_then(|s| s.get_item(key).ok().flatten())
        .and_then(|v| v.parse().ok())
}

/// 汎用 u32 値（px サイズ等）を永続化する。
pub fn set_stored_u32(key: &str, v: u32) {
    if let Some(s) = storage() {
        let _ = s.set_item(key, &v.to_string());
    }
}

/// 汎用文字列値を読み込む（未設定時は None）。
pub fn get_stored_string(key: &str) -> Option<String> {
    storage().and_then(|s| s.get_item(key).ok().flatten())
}

/// 汎用文字列値を永続化する。
pub fn set_stored_string(key: &str, v: &str) {
    if let Some(s) = storage() {
        let _ = s.set_item(key, v);
    }
}

#[cfg(test)]
mod tests {
    // wasm 環境依存のため、ここではロジックの単体テストは行わない。
    // （web_sys::window() は非 wasm では None を返す）
}
