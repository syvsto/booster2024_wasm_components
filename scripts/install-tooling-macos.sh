#/bin/bash

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- --default-toolchain nightly -t wasm32-wasi -t wasm32-unknown-unknown -y
rustup-init --default-toolchain nightly -t wasm32-wasi -t wasm32-unknown-unknown -y
cargo install cargo-component
cargo install wasmtime-cli
cargo install wasm-pack
cargo install --git https://github.com/bytecodealliance/wasi-virt

brew install node

brew install python3
pip3 install componentize-py


