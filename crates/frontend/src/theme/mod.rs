//! テーマ管理（Dark, Light, Modern, Industrial）。
//!
//! localStorage と DOM (`document.documentElement.className`) を同期し、
//! アプリケーション全体に [`ThemeContext`] を提供する。

use leptos::prelude::*;
use serde::{Deserialize, Serialize};

/// 利用可能な 4 種類のテーマ。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Theme {
    /// 定番の ComfyUI 低彩度ダークテーマ
    Dark,
    /// 明るくハイコントラストなライトテーマ
    Light,
    /// インディゴ・サイバーネオン調のモダンテーマ
    Modern,
    /// 警告アンバー・重機ハードウェア調のインダストリアルテーマ
    Industrial,
}

impl Theme {
    pub fn code(&self) -> &'static str {
        match self {
            Self::Dark => "dark",
            Self::Light => "light",
            Self::Modern => "modern",
            Self::Industrial => "industrial",
        }
    }

    pub fn i18n_key(&self) -> &'static str {
        match self {
            Self::Dark => "theme.dark",
            Self::Light => "theme.light",
            Self::Modern => "theme.modern",
            Self::Industrial => "theme.industrial",
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            Self::Dark => "fa-solid fa-moon text-sky-400",
            Self::Light => "fa-solid fa-sun text-amber-500",
            Self::Modern => "fa-solid fa-wand-magic-sparkles text-indigo-400",
            Self::Industrial => "fa-solid fa-industry text-amber-500",
        }
    }

    pub fn class_name(&self) -> &'static str {
        match self {
            Self::Dark => "dark theme-dark",
            Self::Light => "light theme-light",
            Self::Modern => "dark theme-modern",
            Self::Industrial => "dark theme-industrial",
        }
    }

    pub fn is_dark(&self) -> bool {
        !matches!(self, Self::Light)
    }

    pub fn next(&self) -> Self {
        match self {
            Self::Dark => Self::Light,
            Self::Light => Self::Modern,
            Self::Modern => Self::Industrial,
            Self::Industrial => Self::Dark,
        }
    }

    pub fn from_code(code: &str) -> Option<Self> {
        match code {
            "dark" => Some(Self::Dark),
            "light" => Some(Self::Light),
            "modern" => Some(Self::Modern),
            "industrial" => Some(Self::Industrial),
            _ => None,
        }
    }
}

pub const THEMES: [Theme; 4] = [Theme::Dark, Theme::Light, Theme::Modern, Theme::Industrial];

const THEME_KEY: &str = "app_theme";

pub fn get_initial_theme() -> Theme {
    if let Some(w) = web_sys::window() {
        if let Ok(Some(storage)) = w.local_storage() {
            if let Ok(Some(saved)) = storage.get_item(THEME_KEY) {
                if let Some(theme) = Theme::from_code(&saved) {
                    return theme;
                }
            }
        }
    }
    Theme::Dark
}

pub fn apply_theme_to_dom(theme: Theme) {
    if let Some(w) = web_sys::window() {
        if let Some(doc) = w.document() {
            if let Some(el) = doc.document_element() {
                el.set_class_name(theme.class_name());
            }
        }
        if let Ok(Some(storage)) = w.local_storage() {
            let _ = storage.set_item(THEME_KEY, theme.code());
        }
    }
}

#[derive(Clone, Copy)]
pub struct ThemeContext {
    pub theme: ReadSignal<Theme>,
    set_theme_raw: WriteSignal<Theme>,
}

impl ThemeContext {
    pub fn set_theme(&self, new_theme: Theme) {
        apply_theme_to_dom(new_theme);
        self.set_theme_raw.set(new_theme);
    }

    pub fn cycle_theme(&self) {
        let current = self.theme.get();
        self.set_theme(current.next());
    }
}

pub fn provide_theme() -> ThemeContext {
    let initial = get_initial_theme();
    apply_theme_to_dom(initial);
    let (theme, set_theme_raw) = signal(initial);
    let ctx = ThemeContext {
        theme,
        set_theme_raw,
    };
    provide_context(ctx);
    ctx
}

pub fn use_theme() -> ThemeContext {
    use_context::<ThemeContext>().expect("ThemeContext must be provided")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_theme_code_roundtrip() {
        for theme in THEMES {
            let code = theme.code();
            let parsed = Theme::from_code(code).expect("should parse theme code");
            assert_eq!(parsed, theme);
        }
        assert!(Theme::from_code("invalid").is_none());
    }

    #[test]
    fn test_theme_cycle() {
        assert_eq!(Theme::Dark.next(), Theme::Light);
        assert_eq!(Theme::Light.next(), Theme::Modern);
        assert_eq!(Theme::Modern.next(), Theme::Industrial);
        assert_eq!(Theme::Industrial.next(), Theme::Dark);
    }

    #[test]
    fn test_theme_class_name() {
        assert_eq!(Theme::Dark.class_name(), "dark theme-dark");
        assert_eq!(Theme::Light.class_name(), "light theme-light");
        assert_eq!(Theme::Modern.class_name(), "dark theme-modern");
        assert_eq!(Theme::Industrial.class_name(), "dark theme-industrial");
    }

    #[test]
    fn test_theme_is_dark() {
        assert!(Theme::Dark.is_dark());
        assert!(!Theme::Light.is_dark());
        assert!(Theme::Modern.is_dark());
        assert!(Theme::Industrial.is_dark());
    }
}
