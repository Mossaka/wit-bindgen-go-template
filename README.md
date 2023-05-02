# wit-bindgen-go-template

This template repository will help you getting started with authoring a wasm component using TinyGo / Go!

## Prerequisites

- `TinyGo 0.27`
- `Go 1.20`
- `wasm-tools 1.0.28`
- [cargo-component 2101df558d](https://github.com/bytecodealliance/cargo-component/commits/2101df558d116f6bebab808bb6f29c672410ef7b)
- `wit-bindgen-cli` tag "wit-bindgen-0.4.0"

## Build App
The Go app is located in /app directory under the root of the repository. The app is built using TinyGo and the build script is provided in the Makefile.

To build the app, run the following command:

```bash
make build-go
```

To build js app, run the following command:

```bash
make install-js
make build-js
```

To build rs app, run the following command:

```bash
make build-rust
```

## Run App
To run the app, run the following command:

```bash
make run
```

This will starts a local HTTP server listening on port 3001. You can access the app at http://localhost:3001.

## Test App
To test the app, run the following command:

```bash
curl -X GET http://localhost:3001
```

## Compatibility

This repo follows the [component model compatibility matrix](https://bytecodealliance.org/articles/component-model-tooling-compatibility).
