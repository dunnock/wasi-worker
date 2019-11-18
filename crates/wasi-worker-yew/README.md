# Yew worker compiled to wasm32-wasi

This library allows to compile and deploy [yew]() worker with wasm32-wasi target. 
It allows to compile and run POSIX-like applications, having access to timer, random numbers and 
to emulated file system (memfs).

On some operations [wasi workers run faster than wasm-bindgen or stdweb](https:://github.com/dunnock/wabench).

## Example usage:

### In your `Cargo.toml`
```
wasi_worker_yew = "0.4"
```

### In `src/bin/worker.rs`
```
use wasi_worker_yew::{ThreadedWASI, WASIAgent};
use yew::agent::*;
use wasi_worker::{FileOptions, ServiceOptions, ServiceWorker};

pub struct MyAgent;
impl Agent for MyAgent {
    type Reach = Public;
    type Message = String;
    type Input = String;
    type Output = String;
    fn create(_link: AgentLink<Self>) -> Self { MyAgent { } }
    fn update(&mut self, _msg: Self::Message) { /* ... */ }
    fn handle(&mut self, _msg: Self::Input, _who: HandlerId) { /* */ }
    fn name_of_resource() -> &'static str {
        "worker.js"
    }
};

fn main() {
  let opt = ServiceOptions::default().with_cleanup();
  ServiceWorker::initialize(opt)
    .expect("ServiceWorker::initialize");
  ServiceWorker::set_message_handler(Box::new(WASIAgent::<MyAgent>::new()));
}
```

### Deploy

To simplify build and deploy you may use [wasi-worker-cli](https://crates.io/crates/wasi-worker-cli):
```
cargo install wasi-worker-cli --force
```

Deployment script will place compiled and optimized worker target and JS glue code under `./dist`:
```
wasiworker deploy
```