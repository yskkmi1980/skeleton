//! Japanese (JA) translation catalog.

use super::en::t_en;

pub(crate) fn t_ja(key: &str) -> &'static str {
    match key {
        // Navbar
        "nav.subtitle" => "アプリケーション ワークスペース",
        "nav.refresh_tooltip" => "最新情報に更新",
        "nav.logout" => "サインアウト",
        "nav.logout_tooltip" => "ログアウト",
        "nav.connecting" => "接続中...",
        "nav.settings_tooltip" => "表示設定",
        "nav.theme_tooltip" => "テーマ切替",
        "settings.title" => "表示設定",
        "settings.language" => "言語",
        "settings.timezone" => "タイムゾーン",
        "settings.theme" => "テーマ",
        "theme.dark" => "ダーク (標準)",
        "theme.light" => "ライト",
        "theme.modern" => "モダン (サイバー)",
        "theme.industrial" => "インダストリアル",

        // Dock & Layout
        "dock.collapse" => "折りたたむ",
        "dock.expand" => "展開する",
        "dock.resize" => "サイズ変更",

        // General
        "common.loading" => "読み込み中...",
        "common.error" => "エラーが発生しました",
        "common.save" => "保存",
        "common.cancel" => "キャンセル",
        "common.close" => "閉じる",

        // フォールバック: 未翻訳キーは英語定義を参照
        _ => t_en(key),
    }
}
