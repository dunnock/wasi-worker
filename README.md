The proper way to create WASM browser service workers.

This crate provides rust library and JS glue code allowing to wrap POSIX compatible code into WASM/WASI target to be able to run in the browser service worker. It also provides imput/output message channel with main web application.

# Why WASI?

> WebAssembly System Interfaces (WASI) is an exciting new specification that allows running POSIX-like applications anywhere, safely and securely with WebAssembly.

POSIX-compatible applications compiled to WASI can now also run in browser, think of code-reuse and delegating server workload to client side. On top of that it appears code compiled to wasm32-wasi target is executing about 2 times faster than code compiled to other wasm32 targets with web bindings, CPU intensive workloads can now execute with performance close to native targets.

# Why might I need wasi-worker?

WASM code which executes as part of web application occupies same javascript thread, hence if wasm code is running complex calculations it will block browser application while working. To make it working in separate thread we can employ [browser service workers](https://developer.mozilla.org/en-US/docs/Web/API/Service_Worker_API/Using_Service_Workers).

As it stated before code compiled to WASI seems to run about 2 times faster (link to benchmark). The only problem is that WASI is not built to be executed from browser, rather it is standard which aims to run WASM code on server side. Leveraging [@wasmer/wasi](https://github.com/wasmerio/wasmer-js) this crate provides browser service worker WASI runtime as well as communication bridge to/from web application.

# Usage example

This example requires WASI JavaScript bindings [which can be deployed with wasi-worker-cli](https://github.com/dunnock/wasi-worker/tree/master/wasi-worker-cli) or [WASI environment](https://github.com/dunnock/wasi-worker/tree/master/examples/myworker) with properly preconfigured filesystem

```rust
use wasi_worker::*;

struct MyWorker;
impl Handler for MyWorker {
  fn on_message(&self, msg: &[u8]) -> std::io::Result<()> {
    println!("My Worker got message: {:?}", msg);
    Ok(())
  }
}

fn main() {
  // JS glue code will hook to /output.bin
  ServiceWorker::initialize(ServiceOptions::default());
  ServiceWorker::set_message_handler(Box::new(MyWorker {}));
  // Send binary message to main browser application
  ServiceWorker::post_message(b"message");
}

// Function will be called from JS on incoming message
pub extern "C" fn message_ready() -> usize {
  ServiceWorker::on_message()
    .expect("ServiceWorker.on_message")
}
```


# JavaScript WASI bindings

JavaScript WASI bindings are built on top of [@wasmer](https://www.npmjs.com/search?q=%40wasmer) libraries can be easily deployed with [wasiworker](https://github.com/dunnock/wasi-worker/tree/master/wasi-worker-cli) tool:
```
cargo install wasi-worker-cli
```

`wasiworker install` will add sample `worker` code and bin target to current crate directory:
```
wasiworker install
```

`wasiworker deploy` will build `worker` bin target and deploy it with JS glue code under `./dist`:
```
wasiworker deploy
```

For hacking [JS glue code source code](https://github.com/dunnock/wasi-worker/tree/master/wasi-worker-cli/js) is located in the same repository.


# More detailed example

```rust
use wasi_worker::*;

struct MyWorker {}
impl Handler for MyWorker {
  fn on_message(&self, msg: &[u8]) -> std::io::Result<()> {
    // Process incoming message
    println!("My Worker got message: {:?}", msg);
    Ok(())
  }
}

fn main() {
  // In WASI setup output will go to /output.bin
  #[cfg(target_os="wasi")]
  let opt = ServiceOptions::default();
  // In user filesystem we operate under current dir
  #[cfg(not(target_os="wasi"))]
  let opt = ServiceOptions { 
    output: FileOptions::File("./testdata/output.bin".to_string()) 
  };
  let output_file = match &opt.output { 
    FileOptions::File(path) => path.clone() 
  };
  ServiceWorker::initialize(opt)
    .expect("ServiceWorker::initialize");

  // Attach Agent to ServiceWorker as message handler singleton
  ServiceWorker::set_message_handler(Box::new(MyWorker {}));

  // Send binary message to main browser application
  // this requires JS glue see wasi-worker-cli
  ServiceWorker::post_message(b"message")
    .expect("ServiceWorker::post_message");

  // It does not autodelete output file
  std::fs::remove_file(output_file)
    .expect("Remove output.bin");
}

// This function will be called from worker.js on new message
// To operate it requires JS glue - see wasi-worker-cli
// Note: It will be substituted by poll_oneoff, 
// though currently poll_oneoff does not transfer control
pub extern "C" fn message_ready() -> usize {
  ServiceWorker::on_message()
    .expect("ServiceWorker.on_message")
}
```


# TODO

- [X] library code with WASI fs interface
- [X] basic example
- [X] documentation
- [X] CLI for worker setup