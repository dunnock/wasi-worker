//! Yew worker compilable to wasm32-wasi target. It allows to compile and run POSIX-like
//! applications, having access to random and to emulated file system (memfs).
//! On some operations [wasi workers run faster than wasm-bindgen or stdweb](https:://github.com/dunnock/wabench).
//! 
//! It depends on [wasi-worker-cli](https://crates.io/crates/wasi-worker-cli) for deployment.
//! 
//! Example usage:
//! ```
//! use wasi_worker_yew::{ThreadedWASI, WASIAgent};
//! use yew::agent::*;
//! use wasi_worker::{FileOptions, ServiceOptions, ServiceWorker};
//! 
//! pub struct MyAgent;
//! impl Agent for MyAgent {
//!     type Reach = Public;
//!     type Message = String;
//!     type Input = String;
//!     type Output = String;
//!     fn create(_link: AgentLink<Self>) -> Self { MyAgent { } }
//!     fn update(&mut self, _msg: Self::Message) { /* ... */ }
//!     fn handle(&mut self, _msg: Self::Input, _who: HandlerId) { /* */ }
//!     // link to the JavaScript runner, worker instantiated from:
//!     fn name_of_resource() -> &'static str { "worker.js" }
//! };
//! 
//! // In usual WASI setup with JS glue all output will be posted to /output.bin
//! // Though in user filesystem output goes under ./output.bin
//! let opt = ServiceOptions::default().with_cleanup();
//! let output_file = match &opt.output { FileOptions::File(path) => path.clone() };
//! ServiceWorker::initialize(opt)
//!   .expect("ServiceWorker::initialize");
//! ServiceWorker::set_message_handler(Box::new(WASIAgent::<MyAgent>::new()));
//! ```

pub use yew::agent::{Agent, AgentLink, Public, HandlerId, ToWorker, FromWorker, Packed};
pub use wasi_worker::{FileOptions, ServiceOptions, ServiceWorker};

use yew::agent::{AgentUpdate, Responder, AgentScope};
use std::io;
use wasi_worker::{Handler};


/// WASIAgent is the main executor and communication bridge for yew Agent with Reach = Public 
pub struct WASIAgent<T: Agent<Reach = Public>> {
    scope: AgentScope<T>
}

impl<T: Agent<Reach = Public>> WASIAgent<T> {
    pub fn new() -> Self {
        Self {
            scope: AgentScope::<T>::new()
        }
    }
}

/// Implements rules to register a worker in a separate thread.
pub trait ThreadedWASI {
    /// Creates Agent Scope, initialized AgentLink
    /// It will also create ServiceWorker and return it's instance
    /// ServiceWorker should be used by context to pass messages via on_message
    fn run(&self) -> io::Result<()>;
}


impl<T: Agent<Reach = Public>> ThreadedWASI for WASIAgent<T>
{
    fn run(&self) -> io::Result<()> {
        let responder = WASIResponder { };
        let link = AgentLink::connect(&self.scope, responder);
        let upd = AgentUpdate::Create(link);
        self.scope.send(upd);
        let loaded: FromWorker<T::Output> = FromWorker::WorkerLoaded;
        let loaded = loaded.pack();
        ServiceWorker::post_message(&loaded)
    }
}

impl<T: Agent<Reach = Public>> Handler for WASIAgent<T>
{
    fn on_message(&self, data: &[u8]) -> io::Result<()> {
        let msg = ToWorker::<T::Input>::unpack(&data);
        match msg {
            ToWorker::Connected(id) => {
                let upd = AgentUpdate::Connected(id);
                self.scope.send(upd);
            }
            ToWorker::ProcessInput(id, value) => {
                let upd = AgentUpdate::Input(value, id);
                self.scope.send(upd);
            }
            ToWorker::Disconnected(id) => {
                let upd = AgentUpdate::Disconnected(id);
                self.scope.send(upd);
            }
            ToWorker::Destroy => {
                let upd = AgentUpdate::Destroy;
                self.scope.send(upd);
                std::process::exit(1);
            }
        };
        Ok(())
    }
}

struct WASIResponder {
}

// Sending message from worker via ServiceWorker channel
// 
// In case of sending message failed it will place error to stderr, which should print to console.
impl<T: Agent<Reach = Public>> Responder<T> for WASIResponder
{
    fn response(&self, id: HandlerId, output: T::Output) {
        let msg = FromWorker::ProcessOutput(id, output);
        let data = msg.pack();
        if let Err(err) = ServiceWorker::post_message(&data) {
            eprintln!("Worker failed to send message: {:?}", err);    
        };
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use wasi_worker::{FileOptions, ServiceOptions};


    struct MyAgent;
    impl Agent for MyAgent {
        type Reach = Public;
        type Message = String;
        type Input = String;
        type Output = String;
        fn create(_link: AgentLink<Self>) -> Self { MyAgent { } }
        fn update(&mut self, _msg: Self::Message) { /* ... */ }
        fn handle(&mut self, _msg: Self::Input, _who: HandlerId) { /* */ }
    }


    #[test]
    fn it_works() {
        let opt = ServiceOptions { 
            output: FileOptions::File("./testdata/output.bin".to_string()), 
            cleanup: true 
        };
        ServiceWorker::initialize(opt)
            .expect("ServiceWorker::initialize");
        ServiceWorker::set_message_handler(Box::new(WASIAgent::<MyAgent>::new()));
        let message = b"check";
        ServiceWorker::post_message(message)
            .expect("ServiceWorker::post_message");
        let data = std::fs::read("./testdata/output.bin")
            .expect("Read testdata/output.bin");
        assert_eq!(data, message);
    }
}
