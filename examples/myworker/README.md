This example can be executed in any WASI environment. It requires access to filesystem and supposes that stdin is available for read and that it can create and write to file /output.bin.

Shell script `./run.sh` executes it via wasmtime: https://wasmtime.dev/
