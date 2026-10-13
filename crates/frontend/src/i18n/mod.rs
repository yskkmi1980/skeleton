//! Localized presentation infrastructure (Language & Timezone).

mod en;
mod ja;

use leptos::prelude::*;
use serde::{Deserialize, Serialize};

use en::t_en;
use ja::t_ja;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Language {
    En,
    Ja,
}

impl Language {
    pub fn code(&self) -> &'static str {
        match self {
            Self::En => "en",
            Self::Ja => "ja",
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Self::En => "English",
            Self::Ja => "日本語",
        }
    }
}

/// Display timezone.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Timezone {
    Utc,
    Jst,
    BrowserLocal,
}

impl Timezone {
    pub fn code(&self) -> &'static str {
        match self {
            Self::Utc => "utc",
            Self::Jst => "jst",
            Self::BrowserLocal => "local",
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Self::Utc => "UTC",
            Self::Jst => "JST",
            Self::BrowserLocal => "Local",
        }
    }

    pub fn offset_minutes(&self) -> Option<i32> {
        match self {
            Self::Utc => Some(0),
            Self::Jst => Some(9 * 60),
            Self::BrowserLocal => None,
        }
    }

    pub fn from_code(code: &str) -> Option<Self> {
        match code {
            "utc" => Some(Self::Utc),
            "jst" => Some(Self::Jst),
            "local" => Some(Self::BrowserLocal),
            _ => None,
        }
    }
}

pub const TIMEZONES: [Timezone; 3] = [Timezone::Utc, Timezone::Jst, Timezone::BrowserLocal];

const LANGUAGE_KEY: &str = "app_language";
const TIMEZONE_KEY: &str = "app_timezone";

pub fn get_initial_language() -> Language {
    if let Some(w) = web_sys::window() {
        if let Ok(Some(storage)) = w.local_storage() {
            if let Ok(Some(saved)) = storage.get_item(LANGUAGE_KEY) {
                if saved == "ja" {
                    return Language::Ja;
                } else if saved == "en" {
                    return Language::En;
                }
            }
        }
        if let Ok(nav) = js_sys::Reflect::get(&w, &wasm_bindgen::JsValue::from_str("navigator")) {
            if !nav.is_undefined() && !nav.is_null() {
                if let Ok(lang_val) =
                    js_sys::Reflect::get(&nav, &wasm_bindgen::JsValue::from_str("language"))
                {
                    if let Some(lang_str) = lang_val.as_string() {
                        if lang_str.to_lowercase().starts_with("ja") {
                            return Language::Ja;
                        } else if lang_str.to_lowercase().starts_with("en") {
                            return Language::En;
                        }
                    }
                }
            }
        }
    }
    Language::Ja
}

pub fn set_stored_language(lang: Language) {
    if let Some(w) = web_sys::window() {
        if let Ok(Some(storage)) = w.local_storage() {
            let _ = storage.set_item(LANGUAGE_KEY, lang.code());
        }
    }
}

pub fn get_initial_timezone() -> Timezone {
    if let Some(w) = web_sys::window() {
        if let Ok(Some(storage)) = w.local_storage() {
            if let Ok(Some(saved)) = storage.get_item(TIMEZONE_KEY) {
                if let Some(tz) = Timezone::from_code(&saved) {
                    return tz;
                }
            }
        }
    }
    Timezone::BrowserLocal
}

pub fn set_stored_timezone(tz: Timezone) {
    if let Some(w) = web_sys::window() {
        if let Ok(Some(storage)) = w.local_storage() {
            let _ = storage.set_item(TIMEZONE_KEY, tz.code());
        }
    }
}

pub fn format_datetime(tz: Timezone, dt: &chrono::DateTime<chrono::Utc>, fmt: &str) -> String {
    match tz {
        Timezone::Utc => dt.format(fmt).to_string(),
        Timezone::Jst => {
            let offset =
                chrono::FixedOffset::east_opt(Timezone::Jst.offset_minutes().unwrap_or(0) * 60)
                    .expect("JST offset is always valid");
            dt.with_timezone(&offset).format(fmt).to_string()
        }
        Timezone::BrowserLocal => browser_local_format(dt, fmt),
    }
}

fn browser_local_format(dt: &chrono::DateTime<chrono::Utc>, fmt: &str) -> String {
    #[cfg(target_arch = "wasm32")]
    if web_sys::window().is_some() {
        let millis = wasm_bindgen::JsValue::from_f64(dt.timestamp_millis() as f64);
        let date = js_sys::Date::new(&millis);
        let east_seconds = (-date.get_timezone_offset()).round() as i32 * 60;
        if let Some(fixed) = chrono::FixedOffset::east_opt(east_seconds) {
            return dt.with_timezone(&fixed).format(fmt).to_string();
        }
    }
    dt.format(fmt).to_string()
}

#[derive(Clone, Copy)]
pub struct I18n {
    pub lang: ReadSignal<Language>,
    pub set_lang: WriteSignal<Language>,
    pub timezone: ReadSignal<Timezone>,
    pub set_timezone: WriteSignal<Timezone>,
}

impl I18n {
    pub fn t(&self, key: &str) -> &'static str {
        t(self.lang.get(), key)
    }

    pub fn set_language(&self, lang: Language) {
        set_stored_language(lang);
        self.set_lang.set(lang);
    }

    pub fn toggle(&self) {
        let next = match self.lang.get() {
            Language::En => Language::Ja,
            Language::Ja => Language::En,
        };
        self.set_language(next);
    }

    pub fn set_timezone_setting(&self, tz: Timezone) {
        set_stored_timezone(tz);
        self.set_timezone.set(tz);
    }

    pub fn fmt_dt(&self, dt: &chrono::DateTime<chrono::Utc>, fmt: &str) -> String {
        format_datetime(self.timezone.get(), dt, fmt)
    }
}

pub fn provide_i18n() -> I18n {
    let (lang, set_lang) = signal(get_initial_language());
    let (timezone, set_timezone) = signal(get_initial_timezone());
    let i18n = I18n {
        lang,
        set_lang,
        timezone,
        set_timezone,
    };
    provide_context(i18n);
    i18n
}

pub fn use_i18n() -> I18n {
    use_context::<I18n>().unwrap_or_else(|| {
        let (lang, set_lang) = signal(Language::Ja);
        let (timezone, set_timezone) = signal(Timezone::BrowserLocal);
        I18n {
            lang,
            set_lang,
            timezone,
            set_timezone,
        }
    })
}

pub fn t(lang: Language, key: &str) -> &'static str {
    match lang {
        Language::En => t_en(key),
        Language::Ja => t_ja(key),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_language_and_timezone() {
        assert_eq!(Language::En.code(), "en");
        assert_eq!(Language::Ja.code(), "ja");
        assert_eq!(Timezone::Utc.code(), "utc");
        assert_eq!(Timezone::Jst.offset_minutes(), Some(540));
        assert_eq!(Timezone::from_code("jst"), Some(Timezone::Jst));
        assert_eq!(Timezone::from_code("invalid"), None);
    }

    #[test]
    fn test_translations() {
        assert_eq!(t(Language::En, "nav.refresh_tooltip"), "Refresh Data");
        assert_eq!(t(Language::Ja, "nav.refresh_tooltip"), "最新情報に更新");
    }
}
