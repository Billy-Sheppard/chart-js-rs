#! /bin/sh

cargo build --release --target wasm32-unknown-unknown --manifest-path examples/Cargo.toml

wasm-bindgen --target web target/wasm32-unknown-unknown/release/chart_js_rs_example.wasm --out-dir examples --no-typescript

# wasm-opt -Os examples/chart_js_rs_example_bg.wasm -o examples/chart_js_rs_example_bg.wasm