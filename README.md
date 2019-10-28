This crate provides rust library and JS glue code to compose browser service worker on WASI.

# Why specifically WASI?

It seems that code compiled to wasm32-wasi target is executing about 2 times faster than code compiled to other wasm32 targets with web bindings. It makes sense to use it for CPU intensive workloads.

# Why might I need wasi-worker?

WASM code which executes as part of web application occupies same javascript thread, hence if wasm code is running complex calculations it will block browser application while working. To make it working in separate thread we can employ [browser service workers]().

As it stated before code compiled to WASI seems to run about 2 times faster (link to benchmark). The only problem is that WASI is not built to be executed from browser, rather it is standard which aims to run WASM code on server side, hence it is missing proper JavaScript bindings. Thankfully to beautiful [wasmer-js](https://github.com/wasmerio/wasmer-js) this crate provides browser service worker WASI runtime as well as communication bridge to/from web application.

# Example

```rust
use wasi_worker::*;
use std::thread_local;
use std::cell::RefCell;

thread_local! {
  static SERVICE: RefCell<Option<ServiceWorker>> = RefCell::new(None);
}

struct MyWorker {}
impl Worker for MyWorker {
  fn on_message(&self, msg: &[u8]) {
    // Process incoming message
    println!("My Worker got message: {:?}", msg);
  }
}

fn main() {
  let worker = MyWorker {};
  let mut service = ServiceWorker::new(Box::new(worker))
    .expect("ServiceWorker::new");
  service.post_message(b"message")
    .expect("ServiceWorker.post_message");
  SERVICE.with(|local| local.replace(Some(service)));
  message_ready();
}

// this function will be called from worker.js when it receives message
// In the future it will be substituted by poll_oneoff or thread::yield, 
// though currently poll_oneoff does not return control to browser
pub extern "C" fn message_ready() -> usize {
  let mut len: usize = 0;
  SERVICE.with(move |local| {
    if let Some(service) = &mut *local.borrow_mut() {
      len = service.on_message()
        .expect("ServiceWorker.on_message")
    } else {
      panic!("Service not initialized");
    }
  });
  len
}
```


# TODO

[X] library code with WASI fs interface
[X] basic example
[ ] documentation
[ ] CLI for worker setup