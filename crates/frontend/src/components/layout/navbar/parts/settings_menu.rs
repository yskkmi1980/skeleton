//! 表示設定メニュー（言語・タイムゾーン・テーマ選択）。
//!
//! L2 パーツ：ローカル UI 状態（ドロップダウン開閉）のみ保持し、
//! 設定値の所有・永続化は [`I18n`] および [`ThemeContext`] に委譲する。

use leptos::prelude::*;

use crate::i18n::{use_i18n, Language, TIMEZONES};
use crate::theme::{use_theme, THEMES};

/// 選択可能な言語一覧（設定メニューで順に列挙する）。
const LANGUAGE_OPTIONS: [Language; 2] = [Language::En, Language::Ja];

fn lang_button_class(selected: bool) -> &'static str {
    if selected {
        "w-full text-left px-2.5 py-1.5 rounded-md text-xs transition border bg-sky-500/20 text-sky-300 border-sky-500/40"
    } else {
        "w-full text-left px-2.5 py-1.5 rounded-md text-xs transition border text-slate-300 hover:bg-comfy-hover border-transparent"
    }
}

fn tz_button_class(selected: bool) -> &'static str {
    if selected {
        "w-full text-left px-2.5 py-1.5 rounded-md text-xs transition border bg-violet-500/20 text-violet-300 border-violet-500/40"
    } else {
        "w-full text-left px-2.5 py-1.5 rounded-md text-xs transition border text-slate-300 hover:bg-comfy-hover border-transparent"
    }
}

fn theme_button_class(selected: bool) -> &'static str {
    if selected {
        "w-full text-left px-2.5 py-1.5 rounded-md text-xs transition border flex items-center space-x-2 bg-amber-500/20 text-amber-300 border-amber-500/40 font-medium"
    } else {
        "w-full text-left px-2.5 py-1.5 rounded-md text-xs transition border flex items-center space-x-2 text-slate-300 hover:bg-comfy-hover border-transparent"
    }
}

/// ナビバー右端に表示設定（言語 / タイムゾーン / テーマ）のドロップダウンメニュー。
#[component]
pub fn SettingsMenu() -> impl IntoView {
    let i18n = use_i18n();
    let theme_ctx = use_theme();
    let (open, set_open) = signal(false);

    view! {
        <div class="relative flex-shrink-0">
            <button
                type="button"
                on:click=move |_| set_open.update(|v| *v = !*v)
                class="h-7 w-7 sm:h-8 sm:w-8 rounded-lg bg-comfy-hover hover:bg-comfy-borderlight/60 text-slate-300 hover:text-white transition border border-comfy-border flex items-center justify-center"
                title=move || i18n.t("nav.settings_tooltip")
            >
                <i class="fa-solid fa-gear text-xs sm:text-sm"></i>
            </button>

            {move || {
                if open.get() {
                    view! {
                        <>
                            <div
                                class="fixed inset-0 z-40"
                                on:click=move |_| set_open.set(false)
                            ></div>
                            <div class="settings-menu-popup absolute right-0 top-full mt-2 w-64 bg-comfy-node border border-comfy-borderlight rounded-xl shadow-2xl p-3.5 space-y-3 z-50 ring-1 ring-black/20">
                                <div class="text-[11px] font-semibold uppercase tracking-wider text-slate-400 pb-1 border-b border-comfy-border/50">
                                    {move || i18n.t("settings.title")}
                                </div>

                                // Theme Selection
                                <div class="space-y-1.5">
                                    <div class="text-xs text-slate-300 font-medium flex items-center space-x-1.5">
                                        <i class="fa-solid fa-palette text-amber-400 text-[11px]"></i>
                                        <span>{move || i18n.t("settings.theme")}</span>
                                    </div>
                                    <div class="grid grid-cols-2 gap-1">
                                        {THEMES.iter().map(|th| {
                                            let th = *th;
                                            view! {
                                                <button
                                                    type="button"
                                                    on:click=move |_| {
                                                        theme_ctx.set_theme(th);
                                                    }
                                                    class=move || theme_button_class(theme_ctx.theme.get() == th)
                                                >
                                                    <i class=th.icon()></i>
                                                    <span class="truncate">{move || i18n.t(th.i18n_key())}</span>
                                                </button>
                                            }
                                        }).collect::<Vec<_>>()}
                                    </div>
                                </div>

                                // Language Selection
                                <div class="pt-2 border-t border-comfy-border/50 space-y-1.5">
                                    <div class="text-xs text-slate-300 font-medium flex items-center space-x-1.5">
                                        <i class="fa-solid fa-language text-sky-400 text-[11px]"></i>
                                        <span>{move || i18n.t("settings.language")}</span>
                                    </div>
                                    <div class="flex flex-col space-y-1">
                                        {LANGUAGE_OPTIONS.iter().map(|lang| {
                                            let lang = *lang;
                                            view! {
                                                <button
                                                    type="button"
                                                    on:click=move |_| {
                                                        i18n.set_language(lang);
                                                        set_open.set(false);
                                                    }
                                                    class=move || lang_button_class(i18n.lang.get() == lang)
                                                >
                                                    {lang.label()}
                                                </button>
                                            }
                                        }).collect::<Vec<_>>()}
                                    </div>
                                </div>

                                // Timezone Selection
                                <div class="pt-2 border-t border-comfy-border/50 space-y-1.5">
                                    <div class="text-xs text-slate-300 font-medium flex items-center space-x-1.5">
                                        <i class="fa-solid fa-clock text-violet-400 text-[11px]"></i>
                                        <span>{move || i18n.t("settings.timezone")}</span>
                                    </div>
                                    <div class="flex flex-col space-y-1">
                                        {TIMEZONES.iter().map(|tz| {
                                            let tz = *tz;
                                            view! {
                                                <button
                                                    type="button"
                                                    on:click=move |_| {
                                                        i18n.set_timezone_setting(tz);
                                                        set_open.set(false);
                                                    }
                                                    class=move || tz_button_class(i18n.timezone.get() == tz)
                                                >
                                                    {tz.label()}
                                                </button>
                                            }
                                        }).collect::<Vec<_>>()}
                                    </div>
                                </div>
                            </div>
                        </>
                    }
                    .into_any()
                } else {
                    view! { <span></span> }.into_any()
                }
            }}
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::i18n::Timezone;
    use crate::theme::Theme;

    #[test]
    fn language_options_cover_all_languages() {
        assert_eq!(LANGUAGE_OPTIONS.len(), 2);
        assert!(LANGUAGE_OPTIONS.contains(&Language::En));
        assert!(LANGUAGE_OPTIONS.contains(&Language::Ja));
    }

    #[test]
    fn timezone_options_match_catalog() {
        assert_eq!(TIMEZONES.len(), 3);
        assert!(TIMEZONES.contains(&Timezone::Utc));
        assert!(TIMEZONES.contains(&Timezone::Jst));
        assert!(TIMEZONES.contains(&Timezone::BrowserLocal));
    }

    #[test]
    fn theme_options_cover_all_themes() {
        assert_eq!(THEMES.len(), 4);
        assert!(THEMES.contains(&Theme::Dark));
        assert!(THEMES.contains(&Theme::Light));
        assert!(THEMES.contains(&Theme::Modern));
        assert!(THEMES.contains(&Theme::Industrial));
    }

    #[test]
    fn test_theme_button_class() {
        assert!(theme_button_class(true).contains("border-amber-500/40"));
        assert!(theme_button_class(false).contains("border-transparent"));
    }
}
