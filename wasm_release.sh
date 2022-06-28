#!/bin/bash

cargo build --release --target wasm32-unknown-unknown
mkdir wasm
wasm-bindgen target/wasm32-unknown-unknown/release/rust-roguelike.wasm --out-dir ./wasm --target web