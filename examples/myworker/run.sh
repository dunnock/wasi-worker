echo "hello from shell" | wasmtime --mapdir=/::./tmp target/wasm32-wasi/debug/myworker.wasm