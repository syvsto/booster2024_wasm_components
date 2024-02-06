#/bin/bash

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- --default-toolchain nightly -t wasm32-wasi -t wasm32-unknown-unknown -y
cargo install cargo-component
cargo install wasmtime-cli
cargo install --git https://github.com/bytecodealliance/wasi-virt

pip install componentize-py
