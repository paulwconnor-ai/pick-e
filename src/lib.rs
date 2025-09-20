#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn start() {
    real_start();
}

pub fn real_start() {
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    crate::app::build_app().run();
}

mod app;
mod bundles;
mod constants;
mod systems;
