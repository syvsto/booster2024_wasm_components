# syntax=docker/dockerfile:1

FROM ubuntu:22.04
COPY . /app
RUN <<EOF
apt update
apt install curl -y
apt install build-essential -y
apt install pkg-config -y
apt install libssl-dev -y
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- --default-toolchain nightly -t wasm32-wasi -t wasm32-unknown-unknown -y
export PATH=$PATH:$HOME/.cargo/bin
export CARGO_PROFILE_RELEASE_LTO=thin
cargo install cargo-component
cargo install wasmtime-cli
cargo install wasm-pack
apt install clang -y
update-alternatives --set c++ /usr/bin/clang++
cargo install --git https://github.com/bytecodealliance/wasi-virt
apt install python3-pip -y
pip install componentize-py
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.7/install.sh | bash
export NVM_DIR="$HOME/.nvm"
[ -s "$NVM_DIR/nvm.sh" ] && \. "$NVM_DIR/nvm.sh"  # This loads nvm
[ -s "$NVM_DIR/bash_completion" ] && \. "$NVM_DIR/bash_completion"  # This loads nvm bash_completion
nvm install node
apt install git -y
curl -fsSL https://developer.fermyon.com/downloads/install.sh | bash
mv spin /usr/bin/
git clone https://github.com/syvsto/booster2024_wasm_components
EOF
