use bevy::prelude::*;

// Only include wasm_bindgen when targeting the browser
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub fn main() {
    // Set up panic logging in the browser console
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, hello)
        .run();
}

fn hello() {
    bevy::log::info!("Hello from Bevy in WASM!");
}
