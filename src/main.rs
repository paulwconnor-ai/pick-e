#[cfg(target_arch = "wasm32")]
fn main() {
    // No-op on native when targeting WASM — do nothing.
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    pick_e::real_start();
}
