use frontend::App;

fn main() {
    console_error_panic_hook::set_once();
    tracing_wasm::set_as_global_default();
    leptos::mount::mount_to_body(App);
}
