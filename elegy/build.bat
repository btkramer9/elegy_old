cargo +nightly build --release --target wasm32-unknown-unknown
wasm-bindgen target/wasm32-unknown-unknown/release/elegy.wasm --out-dir .
npm run serve