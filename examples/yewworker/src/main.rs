use wasi_worker_yew::{ThreadedWASI, WASIAgent};
use yew::agent::{Agent, AgentLink, Public, HandlerId};
use wasi_worker::{ServiceWorker, ServiceOptions};

struct MyAgent {
  link: AgentLink<Self>
}

impl Agent for MyAgent {
  type Reach = Public;
  type Message = String;
  type Input = String;
  type Output = String;
  // Create an instance with a link to agent's environment.
  fn create(link: AgentLink<Self>) -> Self {
    MyAgent { link }
  }

  // Handle inner messages (of services of `send_back` callbacks)
  fn update(&mut self, _msg: Self::Message) { /* ... */ }

  // Handle incoming messages from components of other agents.
  fn handle(&mut self, msg: Self::Input, _who: HandlerId) {
    println!("Got message: {}", msg);
    //self.link.response(who, msg);
  }
}

fn main() {
  ServiceWorker::initialize(ServiceOptions::default())
    .expect("ServiceWorker created");
  let agent = WASIAgent::<MyAgent>::new();
  agent.run().expect("Agent run");
  ServiceWorker::set_message_handler(Box::new(agent))
    .expect("ServiceWorker set_message_handler");
  ServiceWorker::post_message(b"message")
    .expect("ServiceWorker.post_message");
  message_ready();
}

// this function will be called from worker.js when it receives message
// In the future it will be substituted by poll_oneoff or thread::yield, 
// though currently poll_oneoff does not return control to browser
pub extern "C" fn message_ready() -> usize {
  ServiceWorker::on_message()
    .expect("ServiceWorker.on_message")
}