mod app;
mod components;
mod hooks;
mod error;
mod types;
mod services;
mod pages;

use tracing_wasm::WASMLayerConfigBuilder;
use app::App;

fn main() {
    console_error_panic_hook::set_once();
    tracing_wasm::set_as_global_default_with_config(
        WASMLayerConfigBuilder::new().set_max_level(tracing::Level::DEBUG).build(),
    );
    yew::Renderer::<App>::new().render();
}
