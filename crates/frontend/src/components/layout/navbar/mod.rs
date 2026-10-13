pub mod parts;

use leptos::prelude::*;

use crate::components::layout::navbar::parts::{AppLogo, SettingsMenu};
use crate::i18n::use_i18n;

#[component]
pub fn Navbar(
    #[prop(optional, into)] title: Option<String>,
    #[prop(optional, into)] subtitle: Option<String>,
    #[prop(optional)] on_refresh: Option<Callback<()>>,
) -> impl IntoView {
    let i18n = use_i18n();
    let app_title = title.unwrap_or_else(|| "App".to_string());

    view! {
        <header class="bg-comfy-node/90 backdrop-blur border-b border-comfy-border sticky top-0 z-40 px-3 sm:px-4 py-1.5 sm:py-2">
            <div class="w-full flex items-center justify-between gap-2">
                <div class="flex items-center space-x-2 sm:space-x-2.5 flex-shrink-0">
                    <AppLogo />
                    <div>
                        <div class="flex items-center space-x-1.5 sm:space-x-2">
                            <h1 class="text-base sm:text-lg font-bold tracking-tight bg-gradient-to-r from-slate-900 via-slate-700 to-slate-500 dark:from-white dark:via-slate-200 dark:to-slate-400 bg-clip-text text-transparent">
                                {app_title}
                            </h1>
                            <span class="text-[10px] sm:text-xs font-semibold px-1.5 sm:px-2 py-0.5 rounded-full bg-sky-500/20 text-sky-400 border border-sky-500/30">
                                "v0.1.0"
                            </span>
                        </div>
                        <p class="hidden xl:block text-[11px] text-slate-400">
                            {move || subtitle.clone().unwrap_or_else(|| i18n.t("nav.subtitle").to_string())}
                        </p>
                    </div>
                </div>

                <div class="flex items-center space-x-1.5 sm:space-x-2 flex-shrink-0">
                    // Refresh Button
                    {move || {
                        if let Some(cb) = on_refresh {
                            view! {
                                <button
                                    on:click=move |_| cb.run(())
                                    class="h-7 w-7 sm:h-8 sm:w-8 rounded-lg bg-comfy-hover hover:bg-comfy-borderlight/60 text-slate-300 hover:text-white transition border border-comfy-border flex items-center justify-center flex-shrink-0"
                                    title=move || i18n.t("nav.refresh_tooltip")
                                >
                                    <i class="fa-solid fa-arrows-rotate text-xs sm:text-sm"></i>
                                </button>
                            }.into_any()
                        } else {
                            view! { <span></span> }.into_any()
                        }
                    }}

                    // Settings (Gear)
                    <SettingsMenu />
                </div>
            </div>
        </header>
    }
}
