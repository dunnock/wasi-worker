Tool to create and package WASM WASI-based browser service workers

This tool provides JavaScript glue for browser service workers built with [wasi-worker](https://crates.io/crates/wasi-worker) library.

## Installation

```
cargo install wasi-worker-cli
```

## Usage

```shell
% wasiworker 
wasi-worker-cli 0.1.1
Install JavaScript glue code and WASI toolset for WASI worker to function.

USAGE:
    wasiworker <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    deploy     Build with `--bin worker` and deploy with glue code under ./dist
    help       Prints this message or the help of the given subcommand(s)
    install    Not available yet: Install static files and worker.rs template
```

1. Build and deploy `worker` under ./dist with all depencies

```
wasiworker deploy
```

It will run `cargo build --release --target wasm32-wasi --bin worker`, copy resulting worker.wasm under ./dist and copy JavaScript glue code under ./dist/worker.js. It will also add [wasm_transformer](https://github.com/wasmerio/wasmer-js/tree/master/packages/wasm-transformer) to be able to run in browser.

2. Install wasiworker template considering current directory is a crate root

```
wasiworker install
```

It will create `bin/worker.rs` and place relevant target and dependencies in current `Cargo.toml`. Will panic if Cargo.toml was not found.


## Building/hacking

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

# Attributions

JavaScript glue code is built on top of following great packages. Thanks https://wasmer.io/ for their great work on making WASI easy to use.

 - [@wasmer-js/wasi](https://github.com/wasmerio/wasmer-js/tree/master/packages/wasi)
 - [@wasmer-js/wasmfs](https://github.com/wasmerio/wasmer-js/tree/master/packages/wasmfs)
 - [@wasmer-js/wasm-transformer](https://github.com/wasmerio/wasmer-js/tree/master/packages/wasm-transformer)

# TODO

- [X] CLI
- [X] JavaScript glue package
- [X] Embed JavaScript dist into package
- [X] Documentation
- [ ] Cargo build script to pack and install release or debug version, including building js dependencies
- [ ] Add wasm-gc to optimize resulting wasm size
- [ ] CLI install
- [ ] CLI install allows to customize worker name
- [X] CLI deploy release only
- [ ] CLI deploy can compile for debug
