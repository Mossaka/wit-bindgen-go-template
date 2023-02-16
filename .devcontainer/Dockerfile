# See here for image contents: https://github.com/microsoft/vscode-dev-containers/tree/v0.234.0/containers/python-3/.devcontainer/base.Dockerfile

# [Choice] Debian OS version (use bullseye on local arm64/Apple Silicon): buster, bullseye
ARG VARIANT="bullseye"
FROM mcr.microsoft.com/vscode/devcontainers/rust:${VARIANT}

RUN rustup default stable
RUN rustup target add wasm32-wasi

# TinyGo installation, see https://tinygo.org/getting-started/install/linux/ for instructions
ARG TINYGO_URL="https://github.com/tinygo-org/tinygo/releases/download/v0.27.0/tinygo_0.27.0_amd64.deb"
RUN curl -sL "$TINYGO_URL" -o tinygo_amd64.deb && dpkg -i tinygo_amd64.deb && rm tinygo_amd64.deb

RUN cargo install wit-bindgen-cli --git https://github.com/bytecodealliance/wit-bindgen --rev 16994da9d568a4392ee846d1ed7c2a5d1d2086c2
RUN cargo install wasm-tools --git https://github.com/bytecodealliance/wasm-tools --rev c4c9125419684c4cf64fc88eeee2661fa0c36f14
