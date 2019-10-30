#![feature(trait_alias)]

//! Yew worker compilable to wasm32-wasi target. It should provide about 2x better 
//! performance than wasm32-unknown target.
//! It is experimental implementation, uses customized fork of yew 
//! until PR https://github.com/yewstack/yew/pull/719  accepted

use yew::agent::*;
use std::io;
use wasi_worker::{Handler, ServiceWorker};


/// WASIAgent is the main executor and communication bridge for yew Agent with Reach = Public
/// 
/// Example usage:
/// ```
/// use wasi_worker_yew::{ThreadedWASI, WASIAgent};
/// use yew::agent::*;
/// use wasi_worker::{FileOptions, ServiceOptions, ServiceWorker};
/// 
/// struct MyAgent;
/// impl Agent for MyAgent {
///     type Reach = Public;
///     type Message = String;
///     type Input = String;
///     type Output = String;
///     fn create(_link: AgentLink<Self>) -> Self { MyAgent { } }
///     fn update(&mut self, _msg: Self::Message) { /* ... */ }
///     fn handle(&mut self, _msg: Self::Input, _who: HandlerId) { /* */ }
/// };
/// 
/// let opt = ServiceOptions{output: FileOptions::File("./testdata/outputdoc.bin".to_string())};
/// ServiceWorker::initialize(opt);
/// ServiceWorker::set_message_handler(Box::new(WASIAgent::<MyAgent>::new()));
/// std::fs::remove_file("./testdata/outputdoc.bin");
/// ```
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
    use yew::agent::*;
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
        let opt = ServiceOptions{output: FileOptions::File("./testdata/output.bin".to_string())};
        ServiceWorker::initialize(opt)
            .expect("ServiceWorker::initialize");
        ServiceWorker::set_message_handler(Box::new(WASIAgent::<MyAgent>::new()));
        let message = b"check";
        ServiceWorker::post_message(message)
            .expect("ServiceWorker::post_message");
        let data = std::fs::read("./testdata/output.bin")
            .expect("Read testdata/output.bin");
        assert_eq!(data, message);
        std::fs::remove_file("./testdata/output.bin")
            .expect("Remove testdata/output.bin");
    }
}
