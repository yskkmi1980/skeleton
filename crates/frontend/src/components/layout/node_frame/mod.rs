//! ComfyUI 風のノードフレーム（L1 共通 UI）。
//!
//! タイトルバー + 枠線 + 装飾的なポートドットを持つノード風カード。
//! ステートレス：表示は Props、操作は Callback で受領する。

use leptos::prelude::*;

/// ノードのアクセントカラー種別（ポートドット・リングの色）。
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum NodeAccent {
    /// スカイ（デフォルト）。
    #[default]
    Sky,
    /// アンバー。
    Amber,
    /// エメラルド。
    Emerald,
    /// バイオレット。
    Violet,
}

/// アクセントごとのポートドット色（ComfyUI のポートカラー風）。
pub(crate) fn accent_dot_class(accent: NodeAccent) -> &'static str {
    match accent {
        NodeAccent::Sky => "bg-sky-400",
        NodeAccent::Amber => "bg-amber-400",
        NodeAccent::Emerald => "bg-emerald-400",
        NodeAccent::Violet => "bg-violet-400",
    }
}

/// アクセントごとの選択時リング色。
pub(crate) fn accent_ring_class(accent: NodeAccent) -> &'static str {
    match accent {
        NodeAccent::Sky => "ring-sky-500/30 border-sky-500/60",
        NodeAccent::Amber => "ring-amber-500/30 border-amber-500/60",
        NodeAccent::Emerald => "ring-emerald-500/30 border-emerald-500/60",
        NodeAccent::Violet => "ring-violet-500/30 border-violet-500/60",
    }
}

/// ComfyUI 風のノード風カードフレーム。
///
/// - タイトルバー：左にアイコン+タイトル、右に装飾ポートドット
/// - 枠線：`rounded-lg border bg-comfy-node`、選択時はアクセントリング
/// - ポートドット：タイトルバー左右端の装飾的カラードット（非機能）
#[component]
pub fn NodeFrame(
    /// タイトルテキスト。
    title: String,
    /// タイトルアイコン（FontAwesome クラス文字列）。
    #[prop(optional)]
    icon: Option<String>,
    /// アクセントカラー（ポートドット・リング）。
    #[prop(optional)]
    accent: NodeAccent,
    /// 選択中/アクティブ表示（リング強調）。
    #[prop(optional)]
    active: bool,
    /// タイトルの右側に並べるフィルタやバッジ等。
    #[prop(optional)]
    title_extra: Option<AnyView>,
    /// タイトルバー右端（装飾ドットの左側）に配置するアクション（検索バーなど）。
    #[prop(optional)]
    header_actions: Option<AnyView>,
    /// ノード本体の内容。
    children: ChildrenFn,
) -> impl IntoView {
    let dot = accent_dot_class(accent);
    let title_tooltip = title.clone();
    view! {
        <div class=move || {
            if active {
                format!(
                    "h-full flex flex-col rounded-lg border bg-comfy-node shadow-lg shadow-black/30 ring-2 overflow-hidden {}",
                    accent_ring_class(accent)
                )
            } else {
                "h-full flex flex-col rounded-lg border border-comfy-border bg-comfy-node shadow-lg shadow-black/30 overflow-hidden"
                    .to_string()
            }
        }>
            // Title bar（ComfyUI ノードヘッダー風）
            <div class="flex-shrink-0 flex items-center justify-between gap-2 px-3 py-1.5 sm:py-2 bg-comfy-titlebar border-b border-comfy-border/70 min-w-0">
                <div class="flex items-center space-x-2 min-w-0 flex-1">
                    // 入力ポート風ドット（左端・装飾）
                    <span class=format!("w-2.5 h-2.5 rounded-full flex-shrink-0 ring-2 ring-black/40 {}", dot)></span>
                    {if let Some(ref ic) = icon {
                        view! {
                            <i class=format!("{} text-xs flex-shrink-0", ic.clone())></i>
                        }
                        .into_any()
                    } else {
                        view! { <span></span> }.into_any()
                    }}
                    <span
                        class="text-xs font-semibold tracking-wide text-slate-200 truncate min-w-0"
                        title=title_tooltip
                    >
                        {title}
                    </span>
                    {if let Some(extra) = title_extra {
                        extra
                    } else {
                        view! { <span></span> }.into_any()
                    }}
                </div>
                <div class="flex items-center space-x-2 flex-shrink-0">
                    {if let Some(actions) = header_actions {
                        actions
                    } else {
                        view! { <span></span> }.into_any()
                    }}
                </div>
            </div>

            // Body
            <div class="flex-1 min-h-0 flex flex-col p-3 overflow-hidden">{children()}</div>
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_accent_classes() {
        assert_eq!(accent_dot_class(NodeAccent::Sky), "bg-sky-400");
        assert_eq!(accent_dot_class(NodeAccent::Amber), "bg-amber-400");
        assert!(accent_ring_class(NodeAccent::Sky).contains("ring-sky-500/30"));
    }
}
