build-go:
	cd app && go generate && tinygo build -target=wasi main.go &&mv main.wasm ../main.wasm
	wasm-tools component embed --world world wit/world.wit main.wasm > main.embed.wasm
	wasm-tools component new main.embed.wasm -o main.component.wasm --adapt artifacts/wasi_snapshot_preview1.wasm
	wasm-tools validate main.component.wasm --features component-model 

build-rust:
	cd app-rust && cargo build --target wasm32-wasi --release && mv target/wasm32-wasi/release/app_rust.wasm ../rust.wasm
	wasm-tools component new rust.wasm -o rust.component.wasm --adapt artifacts/wasi_snapshot_preview1.wasm
	wasm-tools validate rust.component.wasm --features component-model

run: build-go build-rust
	cargo run -- main.component.wasm 