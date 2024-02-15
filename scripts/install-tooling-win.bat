winget search buildtools
winget install Microsoft.VisualStudio.2022.Community --silent --override "--wait --quiet --add ProductLang En-us --add Microsoft.VisualStudio.Workload.NativeDesktop --includeRecommended"
winget search rustlang
winget install Rustlang.Rustup

rustup override set nightly
cargo install cargo-component
cargo install wasmtime-cli
cargo install wasm-pack
cargo install --git https://github.com/bytecodealliance/wasi-virt

npm install -g @bytecodealliance/componentize-js
pip install componentize-py