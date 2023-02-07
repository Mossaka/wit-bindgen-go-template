build-go:
	cd app && tinygo build -target=wasi main.go &&mv main.wasm ../main.wasm
	wasm-tools component embed --world world wit/world.wit main.wasm > main.embed.wasm
	wasm-tools component new main.embed.wasm -o main.component.wasm --adapt artifacts/wasi_snapshot_preview1.wasm
	wasm-tools validate main.component.wasm --features component-model 

run: build-go
	cargo run -- main.component.wasm 