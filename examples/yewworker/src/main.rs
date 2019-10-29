use wasi_worker_yew::{ThreadedWASI, WASIAgent, ServiceWorker};
use yew::agent::{Agent, AgentLink, Public, HandlerId};
use std::thread_local;
use std::cell::RefCell;
use std::rc::Rc;

thread_local! {
  static SERVICE: RefCell<Option<Rc<RefCell<ServiceWorker<WASIAgent<MyAgent>>>>>> = RefCell::new(None);
}

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
  fn handle(&mut self, msg: Self::Input, who: HandlerId) {
    self.link.response(who, msg);
  }
}

fn main() {
  let _worker = ServiceWorker::<WASIAgent<MyAgent>>::new()
    .expect("ServiceWorker created");
  let worker = Rc::new(RefCell::new(_worker));
  let agent = WASIAgent::<MyAgent>::new(worker.clone());
  agent.run().expect("Agent run");
  worker.borrow_mut().set_message_handler(agent);

  worker.borrow_mut().post_message(b"message")
    .expect("ServiceWorker.post_message");
  SERVICE.with(|local| local.replace(Some(worker.clone())));
  message_ready();
}

// this function will be called from worker.js when it receives message
// In the future it will be substituted by poll_oneoff or thread::yield, 
// though currently poll_oneoff does not return control to browser
pub extern "C" fn message_ready() -> usize {
  let mut len: usize = 0;
  SERVICE.with(move |local| {
    let opt = &mut *local.borrow_mut();
    if let Some(service) = opt {
      len = service.borrow_mut().on_message()
        .expect("ServiceWorker.on_message")
    } else {
      panic!("Service not initialized");
    }
  });
  len
}