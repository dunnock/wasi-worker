use std::io::{self, Read, Write};
use std::fs::File;
use std::cell::RefCell;
use super::{ServiceOptions, FileOptions};

/// Connects Rust Handler with browser service worker via WASI filesystem.
/// 
/// ServiceWorker is a singleton which holds input and output file handles and
/// owns woker via Handler trait. Worker is supposedly reactive, usually operating
/// on incoming events (on_message) and posting messages to main browser application
/// via ServiceWorker::post_message().
/// 
/// Note: ServiceWorker supposed to operate in single threaded environment
/// like a browser service worker.
///
/// TODO: it requires cleaning of filesystem, add drop implementation
pub struct ServiceWorker {
  output: File,
  input: io::Stdin,
  options: ServiceOptions
}

/// Handler for incoming messages via ServiceWorker
pub trait Handler {
  fn on_message(&self, msg: &[u8]) -> std::io::Result<()>;
}

thread_local! {
  static SERVICE: RefCell<Option<ServiceWorker>> = RefCell::new(None);
  static HANDLER: RefCell<Option<Box<dyn Handler>>> = RefCell::new(None);
}

impl ServiceWorker {
  /// Initialize ServiceWorker instance.
  /// ServiceWorker operates as singleton, all struct methods are static.
  /// Unless initialized all methods will result in error io::ErrorKind::NotConnected.
  pub fn initialize(options: ServiceOptions) -> io::Result<()> {
    let output = match &options.output { FileOptions::File(path) => File::create(path)? };
    let sw = ServiceWorker {
      output,
      input: io::stdin(),
      options
    };
    SERVICE.with(|service| service.replace(Some(sw)));
    Ok(())
  }

  /// Message handler is required to process incoming messages. 
  /// Please note, there is no queue therefore messages received before handler initialized will be lost.
  pub fn set_message_handler(new_handler: Box<dyn Handler>) {
    HANDLER.with(|handler| handler.replace(Some(new_handler)));
  }

  /// This method is a trigger 
  /// This is workaround while we don't have wasi::poll_oneoff, 
  /// ideally we shall just poll and wait for FD_READ event.
  pub fn on_message() -> io::Result<usize> {
    let mut buf: [u8; 1000] = [0; 1000];
    let len = 
      SERVICE.with(|service| {
        if let Some(sw) = &mut *service.borrow_mut() {
          sw.input.read(&mut buf)
        } else {
          Err(io::Error::new(io::ErrorKind::ConnectionRefused, "Cannot borrow service mutably"))
        }
      })?;
    HANDLER.with(|handler| {
      if let Some(handler) = &*handler.borrow() {
        handler.on_message(&buf[0..len])?;
        Ok(len)
      } else {
        Err(io::Error::new(io::ErrorKind::NotConnected, "Worker was not initialized"))
      }
    })
  }

  /// Post message to external consumers
  /// 
  /// Example usage:
  /// ```
  /// use wasi_worker::ServiceWorker;
  /// ServiceWorker::post_message(b"mymesage");
  /// ```
  pub fn post_message(msg: &[u8]) -> std::io::Result<()> {
    SERVICE.with(|service| {
      if let Some(sw) = &mut *service.borrow_mut() {
        sw.output.write_all(msg)
      } else {
        Err(io::Error::new(io::ErrorKind::NotConnected, "Service was not initialized"))
      }
    })
  }

  pub fn kill() -> () {
    SERVICE.with(|service| service.replace(None));
    HANDLER.with(|handler| handler.replace(None));
  }
}

impl Drop for ServiceWorker {
  fn drop(&mut self) {
    if self.options.cleanup {
      let clr = match &self.options.output {
        FileOptions::File(output) => std::fs::remove_file(output)
      };
      match clr {
        Ok(_) => (),
        Err(err) => eprintln!("Failed to remove file {}", err)
      }
    }
  }
}
