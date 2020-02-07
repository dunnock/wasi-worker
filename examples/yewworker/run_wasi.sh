cargo build --target wasm32-wasi
wasmtime ../../target/wasm32-wasi/debug/sendmessage.wasm | wasmtime --mapdir=/::./testdata ../../target/wasm32-wasi/debug/main.wasm