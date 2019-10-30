//!  This crate provides rust library and JS glue code to compose service worker
//!  
//!  Example usage:
//!  ```
//!  use wasi_worker::*;
//! 
//!  struct MyWorker {}
//!  impl Handler for MyWorker {
//!    fn on_message(&self, msg: &[u8]) -> std::io::Result<()> {
//!      // Process incoming message
//!      println!("My Worker got message: {:?}", msg);
//!      Ok(())
//!    }
//!  }
//! 
//!  fn main() {
//!    let opt = ServiceOptions{output: FileOptions::File("./testdata/output.bin".to_string())};
//!    ServiceWorker::initialize(opt)
//!      .expect("ServiceWorker::initialize");
//!    ServiceWorker::set_message_handler(Box::new(MyWorker {}));
//!    ServiceWorker::post_message(b"message")
//!      .expect("ServiceWorker.post_message");
//!    std::fs::remove_file("./testdata/output.bin")
//!      .expect("Remove ./testdata/output.bin");
//!  }
//!  // this function will be called from worker.js when it receives message
//!  // In the future it will be substituted by poll_oneoff or thread::yield, 
//!  // though currently poll_oneoff does not return control to browser
//!  pub extern "C" fn message_ready() -> usize {
//!    ServiceWorker::on_message()
//!      .expect("ServiceWorker.on_message")
//!  }
//!  ```
mod service;

pub use service::{ServiceWorker, Handler};

pub enum FileOptions {
  Default,
  File(String)
}

pub struct ServiceOptions {
// TODO:  input: FileOptions,
  pub output: FileOptions,
}

impl Default for ServiceOptions {
  fn default() -> Self {
    Self {
      output: FileOptions::Default
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
