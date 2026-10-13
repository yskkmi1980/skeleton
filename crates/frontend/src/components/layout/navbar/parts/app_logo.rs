use leptos::prelude::*;

/// アプリロゴ（ブランドグラデーション背景 ＋ アイコンバッジ）。
#[component]
pub fn AppLogo() -> impl IntoView {
    view! {
        <div class="w-7 h-7 sm:w-8 sm:h-8 rounded-lg bg-gradient-to-tr from-sky-500 via-indigo-500 to-purple-600 border border-sky-400/30 flex items-center justify-center flex-shrink-0 shadow-sm ring-1 ring-white/10">
            <i class="fa-solid fa-layer-group text-white text-sm sm:text-base"></i>
        </div>
    }
}
