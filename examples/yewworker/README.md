This example can be executed in any WASI environment or shell. It requires access to filesystem and supposes that stdin is available for read and that it can create and write to local dir in user OS or to file /output.bin in WASI filesystem.

Shell script `./run.sh` executes it from native shell:

Shell script `./run_wasi.sh` executes it via wasmtime: https://wasmtime.dev/:

```shell
> wasmtime target/wasm32-wasi/debug/sendmessage.wasm | wasmtime --mapdir=/::./testdata target/wasm32-wasi/debug/main.wasm
Got incoming message: hello
Outgoing file content Ok("\u{0}\u{0}\u{0}\u{0}message\u{1}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{1}\u{5}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}hello")
```