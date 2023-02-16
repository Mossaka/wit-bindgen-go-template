# wit-bindgen-go-template

This template repository will help you getting started with authoring a wasm component using TinyGo / Go!

## Build App
The Go app is located in /app directory under the root of the repository. The app is built using TinyGo and the build script is provided in the Makefile.

To build the app, run the following command:

```bash
make build-go
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