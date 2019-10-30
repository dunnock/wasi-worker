//!  This crate provides rust library to easily compose WASM/WASI browser service worker.
//!  
//!  # General overview
//! 
//!  ServiceWorker is a singleton which holds input and output file handles and
//!  owns worker via Handler trait. Worker is supposedly reactive, usually operating
//!  on incoming events (on_message) and posting messages to main browser application
//!  via ServiceWorker::post_message().
//! 
//!  # Example usage:
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
//!    // In usual WASI setup with JS glue all output will be posted to /output.bin
//!    // Though in user filesystem to be able to run from shell we operate under current dir
//!    #[cfg(target_os="wasi")]
//!    let opt = ServiceOptions::default();
//!    #[cfg(not(target_os="wasi"))]
//!    let opt = ServiceOptions { output: FileOptions::File("./testdata/output.bin".to_string()) };
//!    let output_file = match &opt.output { FileOptions::File(path) => path.clone() };
//!    ServiceWorker::initialize(opt)
//!      .expect("ServiceWorker::initialize");
//! 
//!    // Attach Agent to ServiceWorker as message handler singleton
//!    ServiceWorker::set_message_handler(Box::new(MyWorker {}));
//! 
//!    // Send binary message to main browser application
//!    ServiceWorker::post_message(b"message")
//!      .expect("ServiceWorker.post_message");
//!    // It still requires cleanup (TODO impl Drop)
//!    std::fs::remove_file(output_file)
//!      .expect("Remove ./testdata/output.bin");
//!  }
//! 
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

/// Instructs on file descriptor configuration for ServiceWorker
pub enum FileOptions {
  File(String)
}

/// Options for ServiceWorker
pub struct ServiceOptions {
// TODO:  input: FileOptions,
  pub output: FileOptions,
}

impl Default for ServiceOptions {
  fn default() -> Self {
    Self {
      output: FileOptions::File("/output.bin".to_string())
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
