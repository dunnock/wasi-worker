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
//!    // In WASI setup with JS glue all output will be posted to memfs::/output.bin
//!    // In native OS to be able to run test from shell output goes to ./output.bin
//!    let opt = ServiceOptions::default().with_cleanup();
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
//!  }
//!  ```
mod service;

pub use service::{Handler, ServiceWorker};

/// Instructs on file descriptor configuration for ServiceWorker
pub enum FileOptions {
    File(String),
}

/// Options for ServiceWorker
pub struct ServiceOptions {
    // TODO:  input: FileOptions,
    pub cleanup: bool,
    pub output: FileOptions,
}

impl ServiceOptions {
    pub fn with_cleanup(mut self) -> Self {
        self.cleanup = true;
        self
    }
}

impl Default for ServiceOptions {
    fn default() -> Self {
        Self {
            output: if cfg!(target_os = "wasi") {
                FileOptions::File("/output.bin".to_string())
            } else {
                FileOptions::File("./output.bin".to_string())
            },
            cleanup: false,
        }
    }
}

// This function will be called from worker.js on new message
// To operate it requires JS glue - see wasi-worker-cli
// Note: It will be substituted by poll_oneoff,
// though currently poll_oneoff does not transfer control
#[no_mangle]
pub extern "C" fn message_ready() -> usize {
    ServiceWorker::on_message().expect("ServiceWorker.on_message")
}


#[cfg(test)]
mod tests {
    use super::{FileOptions, ServiceOptions, ServiceWorker};

    #[test]
    fn cleanup() {
        {
            let opt = ServiceOptions {
                output: FileOptions::File("./testdata/output.bin".to_string()),
                cleanup: true,
            };
            ServiceWorker::initialize(opt).expect("ServiceWorker::initialize");
            std::fs::File::open("./testdata/output.bin")
                .expect("/testdata/output.bin should been created");
            ServiceWorker::kill();
        }
        std::fs::File::open("./testdata/output.bin")
            .expect_err("/testdata/output.bin should been cleaned up");
    }
}
