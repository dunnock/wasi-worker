use wasi_worker::*;

struct MyAgent {}
impl Handler for MyAgent {
  fn on_message(&self, msg: &[u8]) -> std::io::Result<()> {
    // Process incoming message
    println!("My Worker got message: {:?}", msg);
    Ok(())
  }
}

fn main() {
  ServiceWorker::initialize(ServiceOptions::default())
    .expect("ServiceWorker::initialize");
  ServiceWorker::set_message_handler(Box::new(MyAgent {}))
    .expect("ServiceWorker::set_message_handler");
  ServiceWorker::post_message(b"message")
    .expect("ServiceWorker::post_message");
  message_ready();
}

// this function will be called from worker.js when it receives message
// In the future it will be substituted by poll_oneoff or thread::yield, 
// though currently poll_oneoff does not return control to browser
pub extern "C" fn message_ready() -> usize {
  ServiceWorker::on_message()
    .expect("ServiceWorker.on_message")
}