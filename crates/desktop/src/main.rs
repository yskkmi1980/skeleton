//! Desktop shell (Tauri v2).

use tauri::{WebviewUrl, WebviewWindowBuilder};

/// Default backend origin.
const DEFAULT_BACKEND_URL: &str = "http://localhost:3000";

/// Window title.
const WINDOW_TITLE: &str = "Skeleton Application";

/// Initial window size (width, height).
const INITIAL_SIZE: (f64, f64) = (1440.0, 900.0);

/// Minimum window size (width, height).
const MIN_SIZE: (f64, f64) = (1024.0, 700.0);

fn backend_url() -> String {
    std::env::var("APP_BACKEND_URL")
        .ok()
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| DEFAULT_BACKEND_URL.to_string())
}

fn main() {
    let url = backend_url();
    tauri::Builder::default()
        .setup(move |app| {
            let url = url
                .parse::<tauri::Url>()
                .expect("APP_BACKEND_URL is not a valid URL");
            WebviewWindowBuilder::new(app, "main", WebviewUrl::External(url))
                .title(WINDOW_TITLE)
                .inner_size(INITIAL_SIZE.0, INITIAL_SIZE.1)
                .min_inner_size(MIN_SIZE.0, MIN_SIZE.1)
                .build()?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running desktop shell");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_backend_url_is_valid() {
        let url: String = DEFAULT_BACKEND_URL.parse().unwrap();
        assert!(url.starts_with("http://"));
    }
}
