This example can be executed in any WASI environment. It requires access to filesystem and supposes that stdin is available for read and that it can create and write to file /output.bin.

Shell script `./run.sh` executes it via wasmtime: https://wasmtime.dev/:

```shell
> echo "hello from shell" | wasmtime --mapdir=/::./tmp target/wasm32-wasi/debug/yewworker.wasm
My Worker got message: [104, 101, 108, 108, 111, 32, 102, 114, 111, 109, 32, 115, 104, 101, 108, 108, 10]
```