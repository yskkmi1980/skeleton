//! Application Root Component (L3 Coordinator).

use leptos::prelude::*;

use crate::components::layout::{
    CollapsibleGroup, DockPanel, DockSide, Navbar, NodeAccent, NodeFrame,
};
use crate::i18n::provide_i18n;
use crate::theme::provide_theme;

#[component]
pub fn App() -> impl IntoView {
    provide_theme();
    let i18n = provide_i18n();

    let (left_collapsed, set_left_collapsed) = signal(false);
    let (left_width, set_left_width) = signal(280u32);
    let (group_collapsed, set_group_collapsed) = signal(false);

    let on_refresh = Callback::new(move |_| {
        // Refresh handler placeholder
    });
    let _ = i18n;

    view! {
        <div class="h-screen w-screen flex flex-col bg-comfy-canvas text-slate-200 overflow-hidden select-none font-sans">
            // Navigation Bar
            <Navbar
                title="Skeleton App"
                on_refresh=on_refresh
            />

            // Main Workspace Layout
            <div class="flex-1 flex overflow-hidden relative">
                // Left Dock Panel (Collapsible / Resizable)
                <DockPanel
                    title="Navigation".to_string()
                    resize=DockSide::Left
                    icon="fa-solid fa-compass".to_string()
                    size=left_width
                    collapsed=left_collapsed
                    on_toggle=Callback::new(move |_| set_left_collapsed.update(|v| *v = !*v))
                    on_resize=Callback::new(move |s| set_left_width.set(s))
                >
                    <div class="p-2 space-y-2">
                        <CollapsibleGroup
                            title="Quick Links".to_string()
                            icon="fa-solid fa-bolt".to_string()
                            collapsed=group_collapsed
                            on_toggle=Callback::new(move |_| set_group_collapsed.update(|v| *v = !*v))
                        >
                            <div class="text-xs text-slate-400 p-2 bg-comfy-canvas/50 rounded border border-comfy-border">
                                "Collapsible group item"
                            </div>
                        </CollapsibleGroup>
                    </div>
                </DockPanel>

                // Central Main Content Canvas
                <main class="flex-1 overflow-auto p-4 sm:p-6 space-y-4">
                    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
                        // Node Frame 1
                        <NodeFrame
                            title="System Status".to_string()
                            icon="fa-solid fa-server".to_string()
                            accent=NodeAccent::Sky
                            active=true
                        >
                            <div class="text-xs text-slate-300 space-y-2">
                                <p>"Skeleton project ready for new domain implementation."</p>
                                <div class="flex items-center space-x-2">
                                    <span class="w-2 h-2 rounded-full bg-emerald-400 animate-pulse"></span>
                                    <span class="text-emerald-400 font-medium">"Online"</span>
                                </div>
                            </div>
                        </NodeFrame>

                        // Node Frame 2
                        <NodeFrame
                            title="Analytics".to_string()
                            icon="fa-solid fa-chart-line".to_string()
                            accent=NodeAccent::Amber
                        >
                            <div class="text-xs text-slate-400">
                                "Sample card placeholder."
                            </div>
                        </NodeFrame>
                    </div>
                </main>
            </div>
        </div>
    }
}
