cargo build --debug --target wasm32-wasi
wasmtime target/wasm32-wasi/debug/sendmessage.wasm | wasmtime --mapdir=/::./tmp target/wasm32-wasi/debug/main.wasm