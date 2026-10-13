//! 折りたたみ可能なグループセクション（L1 共通 UI）。
//!
//! DockPanel の中に複数配置し、各グループを独立して折りたたむ。
//! 状態は親が所有・Callback で受領するステートレス設計。

use leptos::prelude::*;

/// 折りたたみ chevron アイコンのクラス。
pub(crate) fn chevron_icon_class(collapsed: bool) -> &'static str {
    if collapsed {
        "fa-solid fa-chevron-down text-[10px] text-slate-500 flex-shrink-0"
    } else {
        "fa-solid fa-chevron-up text-[10px] text-slate-500 flex-shrink-0"
    }
}

/// グループヘッダーのアイコン + タイトル + chevron ボタン。
#[component]
pub fn CollapsibleGroup(
    /// グループタイトルテキスト。
    title: String,
    /// FontAwesome アイコンクラス（例: "fa-solid fa-house"）。
    icon: String,
    /// 折りたたみ状態（親が所有）。
    collapsed: ReadSignal<bool>,
    /// トグル通知。
    on_toggle: Callback<()>,
    /// グループ内容。
    children: ChildrenFn,
) -> impl IntoView {
    let title_clone = title.clone();
    view! {
        <div class="rounded-lg border border-slate-700/60 bg-slate-900/40 overflow-hidden">
            // Header（クリックで折りたたみ）
            <button
                type="button"
                on:click=move |_| on_toggle.run(())
                class="w-full flex items-center justify-between px-3 py-2 text-left hover:bg-slate-800/40 transition group overflow-hidden min-w-0"
                title=title_clone.clone()
            >
                <div class="flex items-center space-x-2 min-w-0 flex-1 mr-2 overflow-hidden">
                    <i class=format!("{} text-xs text-slate-400 flex-shrink-0", icon.clone())></i>
                    <span class="text-[11px] font-semibold uppercase tracking-wider text-slate-300 group-hover:text-white transition truncate min-w-0">
                        {title}
                    </span>
                </div>
                <i class=move || chevron_icon_class(collapsed.get())></i>
            </button>

            // Body（展開時のみ表示）
            {move || {
                if !collapsed.get() {
                    view! {
                        <div class="px-2 pb-2 pt-1 space-y-2">
                            {children()}
                        </div>
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

    #[test]
    fn test_chevron_icon_class() {
        assert!(chevron_icon_class(true).contains("fa-chevron-down"));
        assert!(chevron_icon_class(false).contains("fa-chevron-up"));
    }
}
