#![feature(trait_alias)]

//! Worker compilable to wasm32-wasi target
/*  This is copy-paste from https://github.com/yewstack/yew/blob/master/src/agent.rs 
    as a PoC for compilation purpose. */

use yew::agent::{Agent, HandlerId, AgentScope, AgentLink, AgentUpdate, Responder, Public, FromWorker, ToWorker, Packed};
use std::io;
use std::rc::Rc;
use std::cell::RefCell;
pub use wasi_worker::{Worker, ServiceWorker};


pub trait PublicAgent = Agent<Reach = Public>;


/// WASIAgent is the main executor and communication bridge for yew Agent with Reach = Public
/// 
/// Example usage:
/// ```
/// use wasi_worker_yew::{ThreadedWASI, WASIAgent, ServiceWorker}
/// 
/// let worker = Rc::new(RefCell::new(ServiceWorker::new()));
/// let agent = WASIAgent::new(MyAgent::new(), worker.clone());
/// worker.on_message(|msg| agent.on_message(msg));
/// ```
pub struct WASIAgent<T: PublicAgent> {
    scope: AgentScope<T>,
    worker: Rc<RefCell<ServiceWorker<WASIAgent<T>>>>
}

impl<T: PublicAgent> WASIAgent<T> {
    pub fn new(worker: Rc<RefCell<ServiceWorker<WASIAgent<T>>>>) -> Self {
        Self {
            scope: AgentScope::<T>::new(),
            worker
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


impl<T: PublicAgent> ThreadedWASI for WASIAgent<T>
{
    fn run(&self) -> io::Result<()> {
        let scope = AgentScope::<T>::new();
        let responder = WASIResponder { worker: self.worker.clone() };
        let link = AgentLink::connect(&scope, responder);
        let upd = AgentUpdate::Create(link);
        scope.send(upd);
        let loaded: FromWorker<T::Output> = FromWorker::WorkerLoaded;
        let loaded = loaded.pack();
        if let Ok(mut worker) = self.worker.try_borrow_mut() {
            worker.post_message(&loaded)
        } else {
            Err(io::Error::new(io::ErrorKind::WouldBlock, "Service worker was busy"))
        }
    }
}

impl<T: PublicAgent> Worker for WASIAgent<T>
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

struct WASIResponder<T: PublicAgent> {
    worker: Rc<RefCell<ServiceWorker<WASIAgent<T>>>>
}

/// Sending message from worker via ServiceWorker channel
impl<T: PublicAgent> Responder<T> for WASIResponder<T>
{
    fn response(&self, id: HandlerId, output: T::Output) {
        let msg = FromWorker::ProcessOutput(id, output);
        let data = msg.pack();
        if let Ok(mut worker) = self.worker.try_borrow_mut() {
            if let Err(err) = worker.post_message(&data) {
                eprintln!("Worker failed to send message: {:?}", err);    
            };
        } else {
            eprintln!("Worker failed to send message - worker was busy!");
        }
    }
}



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
