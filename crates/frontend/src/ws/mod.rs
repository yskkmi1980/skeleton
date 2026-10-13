//! WebSocket connection management.

use std::rc::Rc;
use std::sync::{
    atomic::{AtomicBool, AtomicU32},
    Arc, Mutex,
};
use std::time::Duration;

use common::{ClientEvent, ServerEvent};
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::{MessageEvent, WebSocket};

use crate::utils::storage;

/// Maximum reconnection backoff (30 seconds).
const MAX_BACKOFF_MS: u32 = 30_000;

type EventHandler = Rc<dyn Fn(ServerEvent)>;

#[derive(Clone)]
struct WsState {
    ws: Arc<Mutex<Option<WebSocket>>>,
    stopped: Arc<AtomicBool>,
    attempts: Arc<AtomicU32>,
}

impl WsState {
    fn new() -> Self {
        Self {
            ws: Arc::new(Mutex::new(None)),
            stopped: Arc::new(AtomicBool::new(false)),
            attempts: Arc::new(AtomicU32::new(0)),
        }
    }

    fn ws_lock(&self) -> std::sync::MutexGuard<'_, Option<WebSocket>> {
        self.ws.lock().unwrap_or_else(|p| p.into_inner())
    }
}

fn build_url() -> Option<String> {
    let window = web_sys::window()?;
    let host = window.location().host().ok()?;
    let protocol = if window.location().protocol().unwrap_or_default() == "https:" {
        "wss:"
    } else {
        "ws:"
    };
    let mut url = format!("{}//{}/api/ws", protocol, host);
    if let Some(token) = storage::get_stored_token() {
        if !token.is_empty() {
            url.push_str(&format!("?token={}", token));
        }
    }
    Some(url)
}

fn backoff_ms(attempt: u32) -> u32 {
    let base = 1_000u32;
    (base << attempt.min(5)).min(MAX_BACKOFF_MS)
}

fn schedule_reconnect(state: &WsState, handler: &EventHandler) {
    let delay = backoff_ms(state.attempts.load(std::sync::atomic::Ordering::Relaxed));
    state
        .attempts
        .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    let s = state.clone();
    let h = handler.clone();
    wasm_bindgen_futures::spawn_local(async move {
        gloo_timers::future::sleep(Duration::from_millis(delay as u64)).await;
        if !s.stopped.load(std::sync::atomic::Ordering::Relaxed) && s.ws_lock().is_none() {
            open_ws(&s, &h);
        }
    });
}

fn open_ws(state: &WsState, handler: &EventHandler) {
    if state.stopped.load(std::sync::atomic::Ordering::Relaxed) || state.ws_lock().is_some() {
        return;
    }
    let Some(url) = build_url() else {
        schedule_reconnect(state, handler);
        return;
    };

    match WebSocket::new(&url) {
        Ok(ws) => {
            let on_open = {
                let attempts = state.attempts.clone();
                Closure::<dyn FnMut()>::new(move || {
                    attempts.store(0, std::sync::atomic::Ordering::Relaxed);
                })
            };
            ws.set_onopen(Some(on_open.as_ref().unchecked_ref()));
            on_open.forget();

            let on_message = {
                let handler = handler.clone();
                Closure::<dyn FnMut(MessageEvent)>::new(move |e: MessageEvent| {
                    if let Ok(txt) = e.data().dyn_into::<js_sys::JsString>() {
                        let text: String = txt.into();
                        if let Ok(event) = serde_json::from_str::<ServerEvent>(&text) {
                            handler(event);
                        }
                    }
                })
            };
            ws.set_onmessage(Some(on_message.as_ref().unchecked_ref()));
            on_message.forget();

            let on_close = {
                let state_clone = state.clone();
                let handler_clone = handler.clone();
                Closure::<dyn FnMut()>::new(move || {
                    state_clone.ws_lock().take();
                    if !state_clone
                        .stopped
                        .load(std::sync::atomic::Ordering::Relaxed)
                    {
                        schedule_reconnect(&state_clone, &handler_clone);
                    }
                })
            };
            ws.set_onclose(Some(on_close.as_ref().unchecked_ref()));
            on_close.forget();

            *state.ws_lock() = Some(ws);
        }
        Err(_) => {
            schedule_reconnect(state, handler);
        }
    }
}

#[derive(Clone)]
pub struct WsClient {
    state: WsState,
}

impl Default for WsClient {
    fn default() -> Self {
        Self::new()
    }
}

impl WsClient {
    pub fn new() -> Self {
        Self {
            state: WsState::new(),
        }
    }

    pub fn reset(&self) {
        self.state
            .stopped
            .store(false, std::sync::atomic::Ordering::Relaxed);
        self.state
            .attempts
            .store(0, std::sync::atomic::Ordering::Relaxed);
    }

    pub fn connect_if_needed(&self, handler: impl Fn(ServerEvent) + 'static) {
        if self
            .state
            .stopped
            .load(std::sync::atomic::Ordering::Relaxed)
            || self.state.ws_lock().is_some()
        {
            return;
        }
        let handler: EventHandler = Rc::new(handler);
        open_ws(&self.state, &handler);
    }

    pub fn disconnect(&self) {
        self.state
            .stopped
            .store(true, std::sync::atomic::Ordering::Relaxed);
        if let Some(ws) = self.state.ws_lock().take() {
            let _ = ws.close();
        }
    }

    pub fn send_event(&self, event: &ClientEvent) -> bool {
        let Ok(json) = serde_json::to_string(event) else {
            return false;
        };
        match self.state.ws_lock().as_ref() {
            Some(ws) => ws.send_with_str(&json).is_ok(),
            None => false,
        }
    }

    pub fn is_connected(&self) -> bool {
        self.state.ws_lock().is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn backoff_grows_exponentially_and_caps() {
        assert_eq!(backoff_ms(0), 1_000);
        assert_eq!(backoff_ms(1), 2_000);
        assert_eq!(backoff_ms(2), 4_000);
        assert_eq!(backoff_ms(5), 32_000.min(MAX_BACKOFF_MS));
        assert_eq!(backoff_ms(10), MAX_BACKOFF_MS);
    }

    #[test]
    fn backoff_never_exceeds_max() {
        for i in 0..20u32 {
            assert!(backoff_ms(i) <= MAX_BACKOFF_MS);
        }
    }
}
