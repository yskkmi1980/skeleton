//! English (EN) translation catalog.

pub(crate) fn t_en(key: &str) -> &'static str {
    match key {
        // Navbar
        "nav.subtitle" => "Application Workspace",
        "nav.refresh_tooltip" => "Refresh Data",
        "nav.logout" => "Sign Out",
        "nav.logout_tooltip" => "Sign Out",
        "nav.connecting" => "Connecting...",
        "nav.settings_tooltip" => "Display Settings",
        "nav.theme_tooltip" => "Theme Switcher",
        "settings.title" => "Display Settings",
        "settings.language" => "Language",
        "settings.timezone" => "Timezone",
        "settings.theme" => "Theme",
        "theme.dark" => "Dark (Default)",
        "theme.light" => "Light",
        "theme.modern" => "Modern (Cyber)",
        "theme.industrial" => "Industrial",

        // Dock & Layout
        "dock.collapse" => "Collapse",
        "dock.expand" => "Expand",
        "dock.resize" => "Resize",

        // General
        "common.loading" => "Loading...",
        "common.error" => "Error occurred",
        "common.save" => "Save",
        "common.cancel" => "Cancel",
        "common.close" => "Close",

        _ => "",
    }
}
