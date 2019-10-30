The proper way to create WASM browser service workers.

This crate provides rust library and JS glue code to compose browser service worker on WASI.

# Why specifically WASI?

It seems that code compiled to wasm32-wasi target is executing about 2 times faster than code compiled to other wasm32 targets with web bindings. It makes sense to use it for CPU intensive workloads.

# Why might I need wasi-worker?

WASM code which executes as part of web application occupies same javascript thread, hence if wasm code is running complex calculations it will block browser application while working. To make it working in separate thread we can employ [browser service workers]().

As it stated before code compiled to WASI seems to run about 2 times faster (link to benchmark). The only problem is that WASI is not built to be executed from browser, rather it is standard which aims to run WASM code on server side, hence it is missing proper JavaScript bindings. Thankfully to beautiful [wasmer-js](https://github.com/wasmerio/wasmer-js) this crate provides browser service worker WASI runtime as well as communication bridge to/from web application.

# Example

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
  // In usual WASI setup with JS glue all output will be posted to /output.bin
  // Though in user filesystem to be able to run from shell we operate under current dir
  #[cfg(target_os="wasi")]
  let opt = ServiceOptions::default();
  #[cfg(not(target_os="wasi"))]
  let opt = ServiceOptions { output: FileOptions::File("./testdata/output.bin".to_string()) };
  let output_file = match &opt.output { FileOptions::File(path) => path.clone() };
  ServiceWorker::initialize(opt)
    .expect("ServiceWorker::initialize");

  // Attach Agent to ServiceWorker as message handler singleton
  ServiceWorker::set_message_handler(Box::new(MyWorker {}))
    .expect("ServiceWorker::set_message_handler");

  // Send binary message to main browser application
  // this requires JS glue see wasi-worker-cli
  ServiceWorker::post_message(b"message")
    .expect("ServiceWorker::post_message");

  // It does not autodelete output file
  std::fs::remove_file(output_file)
    .expect("Remove output.bin");
}

// This function will be called from worker.js when it receives message
// To operate it requires JS glue - see wasi-worker-cli
// Note: In the future it will be substituted by poll_oneoff or thread::yield, 
// though currently poll_oneoff does not return control to browser
pub extern "C" fn message_ready() -> usize {
  ServiceWorker::on_message()
    .expect("ServiceWorker.on_message")
}
```


# TODO

[X] library code with WASI fs interface
[X] basic example
[X] documentation
[ ] CLI for worker setup