cargo build --target wasm32-wasi
wasmtime target/wasm32-wasi/debug/sendmessage.wasm | wasmtime --mapdir=.::. target/wasm32-wasi/debug/main.wasm