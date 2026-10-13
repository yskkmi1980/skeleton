//! ドックパネル（L1 共通 UI）。
//!
//! ComfyUI ノード風の枠（タイトルバー + ポートドット）を持ち、
//! 左/右/下のいずれかの端にドッキングするコンテナ型ペイン。
//! 折りたたみ・リサイズの状態は親（MainLayout）が所有し、
//! 本コンポーネントは表示と Callback の受領のみを行う。

use leptos::prelude::*;
use wasm_bindgen::JsCast;

use crate::components::layout::node_frame::{accent_dot_class, NodeAccent};

/// タブ定義（タイトルバー内のセグメント切替用）。
#[derive(Clone, PartialEq, Eq)]
pub struct DockTab {
    /// 識別子。
    pub id: String,
    /// 表示ラベル。
    pub label: String,
    /// FontAwesome アイコンクラス。
    pub icon: String,
}

/// ドッキングする端（リサイズ方向・折りたたみレールの向きを決定）。
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum DockSide {
    /// 左端にドッキング（幅でリサイズ、右側にハンドル）。
    Left,
    /// 右端にドッキング（幅でリサイズ、左側にハンドル）。
    Right,
    /// 下端にドッキング（高さでリサイズ、上にハンドル）。
    Bottom,
}

/// ペインの最小サイズ（px）。
const MIN_SIZE: u32 = 240;

/// サイズのクランプ（最小値〜ビューポートの85%まで）。
fn clamp_size(s: f64) -> u32 {
    let viewport = web_sys::window()
        .and_then(|w| w.inner_width().ok())
        .and_then(|v| v.as_f64())
        .unwrap_or(1280.0);
    let max = (viewport * 0.85).max(MIN_SIZE as f64);
    s.clamp(MIN_SIZE as f64, max) as u32
}

/// 折りたたみレールの向きごとの chevron アイコン。
fn collapse_icon(side: DockSide, collapsed: bool) -> &'static str {
    match side {
        DockSide::Left => {
            if collapsed {
                "fa-solid fa-chevron-right"
            } else {
                "fa-solid fa-chevron-left"
            }
        }
        DockSide::Right => {
            if collapsed {
                "fa-solid fa-chevron-left"
            } else {
                "fa-solid fa-chevron-right"
            }
        }
        DockSide::Bottom => {
            if collapsed {
                "fa-solid fa-chevron-up"
            } else {
                "fa-solid fa-chevron-down"
            }
        }
    }
}

/// リサイズハンドル位置のクラス。
///
/// 左右・下ドックは `lg` 未満でドロワー表示になるためハンドルを隠す。
fn handle_class(side: DockSide) -> &'static str {
    match side {
        DockSide::Left => "absolute right-0 inset-y-0 z-10 hidden lg:block w-1.5 cursor-col-resize",
        DockSide::Right => "absolute left-0 inset-y-0 z-10 hidden lg:block w-1.5 cursor-col-resize",
        DockSide::Bottom => {
            "absolute left-0 right-0 top-0 z-10 hidden lg:block h-1.5 cursor-row-resize"
        }
    }
}

/// ボディ領域のクラス。タイトルバー/タブ行の下の残り領域を占有する（高さは flex で決定）。
const BODY_CLASS: &str = "flex-1 min-h-0 overflow-hidden p-3";

/// ドックパネル本体のクラス。
///
/// `lg` 以上では ComfyUI ドック風の並び（`--dock-size`）。
/// `lg` 未満では画面端からせり出すオーバーレイドロワー（fixed z-50）にする。
fn panel_class(side: DockSide) -> &'static str {
    match side {
        DockSide::Left => {
            "fixed inset-y-0 left-0 z-50 w-80 max-w-[85vw] h-full rounded-r-xl border-r border-comfy-border bg-comfy-node shadow-2xl overflow-hidden flex flex-col lg:relative lg:inset-auto lg:z-auto lg:h-auto lg:w-[var(--dock-size)] lg:rounded-lg lg:border lg:shadow-lg lg:shadow-black/30 animate-in slide-in-from-left duration-200"
        }
        DockSide::Right => {
            "fixed inset-y-0 right-0 z-50 w-80 max-w-[85vw] h-full rounded-l-xl border-l border-comfy-border bg-comfy-node shadow-2xl overflow-hidden flex flex-col lg:relative lg:inset-auto lg:z-auto lg:h-auto lg:w-[var(--dock-size)] lg:rounded-lg lg:border lg:shadow-lg lg:shadow-black/30 animate-in slide-in-from-right duration-200"
        }
        DockSide::Bottom => {
            "fixed inset-x-0 bottom-0 z-50 w-full rounded-t-xl border-t border-comfy-border bg-comfy-node shadow-2xl overflow-hidden flex flex-col lg:relative lg:inset-auto lg:z-auto lg:h-auto lg:rounded-lg lg:border lg:shadow-lg lg:shadow-black/30 animate-in slide-in-from-bottom duration-200 dock-drawer-bottom"
        }
    }
}

/// 折りたたみレールのクラス。
///
/// `lg` 未満ではドロワー方式のため初期状態・折りたたみ時は非表示にし、`lg` 以上でのみレールを表示する。
fn rail_class(side: DockSide) -> &'static str {
    match side {
        DockSide::Left | DockSide::Right => {
            "hidden lg:flex items-center justify-center gap-2 lg:h-auto lg:w-12 lg:flex-col rounded-lg border border-comfy-border bg-comfy-node shadow-lg shadow-black/30"
        }
        DockSide::Bottom => {
            "hidden lg:flex items-center justify-center gap-2 h-10 flex-row rounded-lg border border-comfy-border bg-comfy-node shadow-lg shadow-black/30"
        }
    }
}

/// ドックパネル本体のスタイル（幅/高さ）。
fn panel_style(side: DockSide, size: u32) -> String {
    match side {
        DockSide::Left | DockSide::Right => format!("--dock-size: {}px; flex-shrink: 0;", size),
        DockSide::Bottom => format!("height: {}px; flex-shrink: 0;", size),
    }
}

/// タブボタンのクラス（アクティブ/非アクティブ）。
fn tab_button_class(is_active: bool) -> &'static str {
    if is_active {
        "px-2.5 py-1.5 text-xs font-medium transition flex items-center space-x-1.5 border-b-2 border-sky-400 text-sky-400 whitespace-nowrap min-w-0 flex-shrink"
    } else {
        "px-2.5 py-1.5 text-xs font-medium transition flex items-center space-x-1.5 border-b-2 border-transparent text-slate-400 hover:text-white whitespace-nowrap min-w-0 flex-shrink"
    }
}

/// タブの1ボタン（タブ行内）。
fn tab_button(tab: &DockTab, active: ReadSignal<String>, sel: Callback<String>) -> AnyView {
    let id = tab.id.clone();
    let click_id = id.clone();
    let icon_cls = tab.icon.clone();
    let label = tab.label.clone();
    let tooltip = label.clone();
    view! {
        <button
            type="button"
            on:click=move |_| sel.run(click_id.clone())
            class=move || tab_button_class(active.get() == id)
            title=tooltip.clone()
        >
            <i class=format!("{} flex-shrink-0 text-xs", icon_cls)></i>
            <span class="truncate min-w-0">{label.clone()}</span>
        </button>
    }
    .into_any()
}

/// タブ行（タイトルバー内・左寄せで表示）。
fn tab_row(tabs: Vec<DockTab>, active: ReadSignal<String>, sel: Callback<String>) -> AnyView {
    let buttons: Vec<AnyView> = tabs.iter().map(|t| tab_button(t, active, sel)).collect();
    view! {
        <div class="flex items-center space-x-1 min-w-0 flex-1 overflow-hidden">
            {buttons}
        </div>
    }
    .into_any()
}

/// コンテナ型ドックペイン（折りたたみ・リサイズ対応）。
///
/// - 展開時：ノード風タイトルバー + リサイズハンドル + 内容コンテナ
/// - 折りたたみ時：端に張り付いた細いレール（アイコン+chevron のみ）
#[component]
pub fn DockPanel(
    /// タイトルテキスト。
    title: String,
    /// タイトルアイコン（FontAwesome クラス文字列）。
    #[prop(optional)]
    icon: Option<String>,
    /// アクセントカラー（ポートドット）。
    #[prop(optional)]
    accent: NodeAccent,
    /// 折りたたみ状態（親が所有・永続化）。
    collapsed: ReadSignal<bool>,
    /// 折りたたみ/展開のトグル。
    on_toggle: Callback<()>,
    /// ドッキングする端。
    resize: DockSide,
    /// 現在の幅/高さ（px、親が所有・永続化）。
    size: ReadSignal<u32>,
    /// リサイズ通知（クランプ後の px）。
    on_resize: Callback<u32>,
    /// タブ一覧（指定時はタイトルバーにセグメント切替を表示）。
    #[prop(optional)]
    tabs: Option<Vec<DockTab>>,
    /// アクティブタブ識別子。
    #[prop(optional)]
    active_tab: Option<ReadSignal<String>>,
    /// タブ選択通知。
    #[prop(optional)]
    on_select_tab: Option<Callback<String>>,
    /// ペイン内容（任意の UI を差し込めるコンテナ）。
    children: ChildrenFn,
) -> impl IntoView {
    let dot = accent_dot_class(accent);
    // ドラッグ開始位置（X/Y）と開始時のサイズ。
    let (drag_start, set_drag_start) = signal(Option::<(f64, f64, u32)>::None);

    let on_pointer_down = move |e: web_sys::PointerEvent| {
        if let Some(el) = e
            .current_target()
            .map(|t| t.unchecked_into::<web_sys::Element>())
        {
            let _ = el.set_pointer_capture(e.pointer_id());
        }
        set_drag_start.set(Some((e.client_x() as f64, e.client_y() as f64, size.get())));
    };

    let on_pointer_move = move |e: web_sys::PointerEvent| {
        if let Some((start_x, start_y, start_s)) = drag_start.get() {
            let next = match resize {
                DockSide::Left => start_s as f64 + (e.client_x() as f64 - start_x),
                DockSide::Right => start_s as f64 - (e.client_x() as f64 - start_x),
                DockSide::Bottom => start_s as f64 - (e.client_y() as f64 - start_y),
            };
            on_resize.run(clamp_size(next));
        }
    };

    let on_pointer_up = move |e: web_sys::PointerEvent| {
        if let Some(el) = e
            .current_target()
            .map(|t| t.unchecked_into::<web_sys::Element>())
        {
            let _ = el.release_pointer_capture(e.pointer_id());
        }
        set_drag_start.set(None);
    };

    view! {
        // 展開時：ノード風ドックペイン。折りたたみ時：細いレール。
        {move || {
            if collapsed.get() {
                view! {
                    <div class=rail_class(resize)>
                        <span class=format!("w-2.5 h-2.5 rounded-full ring-2 ring-black/40 {}", dot)></span>
                        {if let Some(ref ic) = icon {
                            view! { <i class=format!("{} text-xs text-slate-300", ic.clone())></i> }
                                .into_any()
                        } else {
                            view! { <span></span> }.into_any()
                        }}
                        <button
                            type="button"
                            on:click=move |_| on_toggle.run(())
                            class="p-1.5 text-slate-400 hover:text-white rounded-lg hover:bg-comfy-hover transition"
                            title=title.clone()
                        >
                            <i class=format!("{} text-sm", collapse_icon(resize, true))></i>
                        </button>
                    </div>
                }
                .into_any()
            } else {
                view! {
                    <div
                        class="fixed inset-0 bg-black/60 backdrop-blur-sm z-40 lg:hidden transition-opacity"
                        on:click=move |_| on_toggle.run(())
                    ></div>
                    <div class=panel_class(resize) style=move || panel_style(resize, size.get())>
                        // Title bar（ComfyUI ノードヘッダー風。タブ指定時はタイトル位置にタブを統合描画）
                        <div class="flex items-center justify-between gap-2 px-3 py-1.5 bg-comfy-titlebar border-b border-comfy-border/70 min-h-[36px] min-w-0 overflow-hidden">
                            <div class="flex items-center space-x-2 min-w-0 flex-1 overflow-hidden">
                                <span class=format!("w-2.5 h-2.5 rounded-full flex-shrink-0 ring-2 ring-black/40 {}", dot)></span>
                                {if let (Some(tabs), Some(active), Some(sel)) =
                                    (&tabs, &active_tab, &on_select_tab)
                                {
                                    tab_row(tabs.clone(), *active, *sel)
                                } else {
                                    view! {
                                        <div class="flex items-center space-x-2 min-w-0 flex-1 overflow-hidden">
                                            {if let Some(ref ic) = icon {
                                                view! { <i class=format!("{} text-xs flex-shrink-0", ic.clone())></i> }
                                                    .into_any()
                                            } else {
                                                view! { <span></span> }.into_any()
                                            }}
                                            <span
                                                class="text-xs font-semibold tracking-wide text-slate-200 truncate min-w-0"
                                                title=title.clone()
                                            >
                                                {title.clone()}
                                            </span>
                                        </div>
                                    }
                                    .into_any()
                                }}
                            </div>

                            <button
                                type="button"
                                on:click=move |_| on_toggle.run(())
                                class="p-1.5 text-slate-400 hover:text-white rounded-lg hover:bg-comfy-hover transition flex-shrink-0"
                                title=title.clone()
                            >
                                <i class="lg:hidden fa-solid fa-xmark text-sm"></i>
                                <i class=format!("hidden lg:inline {} text-sm", collapse_icon(resize, false))></i>
                            </button>
                        </div>

                        // Resize Handle（端に張り付く）
                        <div
                            on:pointerdown=on_pointer_down
                            on:pointermove=on_pointer_move
                            on:pointerup=on_pointer_up
                            class=format!("{} touch-none select-none", handle_class(resize))
                        >
                            {if resize == DockSide::Bottom {
                                view! { <div class="w-full h-full flex items-center justify-center"><div class="w-10 h-1 rounded-full bg-comfy-borderlight/70"></div></div> }
                                    .into_any()
                            } else {
                                view! { <div class="w-full h-full flex items-center justify-center"><div class="h-10 w-1 rounded-full bg-comfy-borderlight/70"></div></div> }
                                    .into_any()
                            }}
                        </div>

                        // Body（内容コンテナ：任意の UI を差し込める）
                        <div class=BODY_CLASS>{children()}</div>
                    </div>
                }
                .into_any()
            }
        }}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collapse_icon() {
        assert_eq!(
            collapse_icon(DockSide::Left, true),
            "fa-solid fa-chevron-right"
        );
        assert_eq!(
            collapse_icon(DockSide::Left, false),
            "fa-solid fa-chevron-left"
        );
        assert_eq!(
            collapse_icon(DockSide::Right, true),
            "fa-solid fa-chevron-left"
        );
        assert_eq!(
            collapse_icon(DockSide::Right, false),
            "fa-solid fa-chevron-right"
        );
        assert_eq!(
            collapse_icon(DockSide::Bottom, true),
            "fa-solid fa-chevron-up"
        );
        assert_eq!(
            collapse_icon(DockSide::Bottom, false),
            "fa-solid fa-chevron-down"
        );
    }

    #[test]
    fn test_panel_style() {
        assert_eq!(
            panel_style(DockSide::Left, 320),
            "--dock-size: 320px; flex-shrink: 0;"
        );
        assert_eq!(
            panel_style(DockSide::Right, 400),
            "--dock-size: 400px; flex-shrink: 0;"
        );
        assert_eq!(
            panel_style(DockSide::Bottom, 200),
            "height: 200px; flex-shrink: 0;"
        );
    }

    #[test]
    fn test_classes() {
        assert!(handle_class(DockSide::Left).contains("cursor-col-resize"));
        assert!(handle_class(DockSide::Bottom).contains("cursor-row-resize"));
        assert!(panel_class(DockSide::Left).contains("--dock-size"));
        assert!(rail_class(DockSide::Left).contains("lg:w-12"));
        assert_eq!(
            tab_button_class(true),
            "px-2.5 py-1.5 text-xs font-medium transition flex items-center space-x-1.5 border-b-2 border-sky-400 text-sky-400 whitespace-nowrap min-w-0 flex-shrink"
        );
    }
}
