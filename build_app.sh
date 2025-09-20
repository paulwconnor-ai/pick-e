export RUSTFLAGS='--cfg getrandom_backend="wasm_js"'
cargo build --target wasm32-unknown-unknown
cp target/wasm32-unknown-unknown/debug/pick_e.wasm ./pick_e.wasm