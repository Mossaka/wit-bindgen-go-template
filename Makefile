check-tinygo:
	# check tinygo version is 0.27
	tinygo version
	if [ $$? -ne 0 ]; then \
		echo "tinygo not found, please install tinygo 0.27"; \
		exit 1; \
	fi

check-go:
	# check go version
	go version
	if [ $$? -ne 0 ]; then \
		echo "go not found, please install go"; \
		exit 1; \
	fi

install-wit-bindgen:
	cargo install wit-bindgen-cli --git https://github.com/bytecodealliance/wit-bindgen --tag "wit-bindgen-0.4.0"

install-wasm-tools:
	cargo install wasm-tools@1.0.28

install-cargo-component:
	cargo install cargo-component --git https://github.com/bytecodealliance/cargo-component --rev 2101df558d116f6bebab808bb6f29c672410ef7b

install-deps: check-tinygo check-go install-wit-bindgen install-wasm-tools install-cargo-component
	# finish
	echo "install deps success"

build-go:
	cd app && go generate && tinygo build -target=wasi main.go &&mv main.wasm ../main.wasm
	wasm-tools component embed --world world wit/world.wit main.wasm > main.embed.wasm
	wasm-tools component new main.embed.wasm -o main.component.wasm --adapt artifacts/wasi_snapshot_preview1.wasm
	wasm-tools validate main.component.wasm --features component-model 

build-rust:
	cd app-rust && cargo component build --release  && mv target/wasm32-wasi/release/app_rust.wasm ../rust.component.wasm
	wasm-tools validate rust.component.wasm --features component-model

install-js:
	cd app-js && npm install

build-js:
	cd app-js && ./node_modules/.bin/jco componentize app.js -w ../wit/world.wit -o ../js.component.wasm

clean:
	rm -rf *.wasm

run: clean build-go build-rust 
	cargo run --release -- main.component.wasm 

test:
	curl -X GET http://127.0.0.1:3001

.PHONY: check-tinygo check-go install-wit-bindgen install-wasm-tools install-cargo-component install-deps build-go build-rust run test clean