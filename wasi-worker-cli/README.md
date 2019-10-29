Tool to create and deploy WASM WASI-based browser service workers

# Installation

```
cargo install wasi-worker-cli
```

# Usage

## Install wasiworker template considering current directory is a crate root

```
wasiworker install
```

It will create `bin/worker.rs` and place relevant target and dependencies in current `Cargo.toml`. Will panic if Cargo.toml was not found.

## Build and deploy `worker` under ./dist with all depencies

```
wasiworker deploy
```

It will run `cargo build --release --target wasm32-wasi --bin worker`, then it will take resulting wasm file and optimize it with


# Building/hacking/contirubintg/debugging

JavaScript glue code is built on top of following great packages. Thanks https://wasmer.io/ for their great work on making WASI easy to use.

 - [@wasmer-js/wasi](https://github.com/wasmerio/wasmer-js/tree/master/packages/wasi)
 - [@wasmer-js/wasmfs](https://github.com/wasmerio/wasmer-js/tree/master/packages/wasmfs)
 - [@wasmer-js/](https://github.com/wasmerio/wasmer-js/tree/master/packages/wasm-transformer)

Code structure:

 - src/main.rs - is the CLI, when compiled it's embedding dist glue code into resulting binary
 - js/* is the source of JavaScript package, which is using rollup and typescript to build distribution files
 - js/dist/* is the latest glue code distributable built with production settings

## Install the cli package from the crate manually

```
cargo install --path . --force
```

It will also embed whaever version of JS glue was placed under the ./dist subfolder.

## Build JS glue for development

```
npm install
npm run build:dev
```

## Build JS glue for distribution
```
npm install
npm run build
```

# TODO

 [X] CLI
 [X] JavaScript glue package
 [X] Embed JavaScript dist into package
 [X] Documentation
 [ ] Cargo build script to pack and install release or debug version, including building js dependencies
 [ ] Add wasm-gc to optimize resulting wasm size
 [ ] CLI install
 [ ] CLI install allows to customize worker name
 [X] CLI deploy release only
 [ ] CLI deploy can compile for debug
