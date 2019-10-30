use std::io::{self, Read, Write};
use std::fs::File;
use std::cell::RefCell;
use super::{ServiceOptions, FileOptions};

/// Connects Rust Worker with browser service worker via WASI filesystem.
/// ServiceWorker is singleton, can operate only in single threaded environment
/// which is fine when it's run as browser service worker.
///
/// TODO: it requires cleaning of filesystem, add drop implementation
pub struct ServiceWorker {
  output: File,
  input: io::Stdin,
  handler: Option<Box<dyn Handler>>
}

/// Handler for incoming messages via ServiceWorker
pub trait Handler {
  fn on_message(&self, msg: &[u8]) -> std::io::Result<()>;
}

thread_local! {
  static SERVICE: RefCell<Option<ServiceWorker>> = RefCell::new(None);
}

impl ServiceWorker {
  pub const OUTFILE: &'static str = "/output.bin";

  /// Initialize ServiceWorker instance.
  /// ServiceWorker operates as singleton, all struct methods are static.
  /// Unless initialized all methods will result in error io::ErrorKind::NotConnected.
  pub fn initialize(opt: ServiceOptions) -> io::Result<()> {
    let output = match opt.output {
      FileOptions::Default => File::create(Self::OUTFILE)?,
      FileOptions::File(path) => File::create(path)?,
    };
    let sw = ServiceWorker {
      output,
      input: io::stdin(),
      handler: None
    };
    SERVICE.with(|service| service.replace(Some(sw)));
    Ok(())
  }

  /// Message handler is required to process incoming messages. 
  /// Please note, there is no queue therefore messages received before handler initialized will be lost.
  pub fn set_message_handler(handler: Box<dyn Handler>) -> io::Result<()> {
    SERVICE.with(|service| {
      if let Some(sw) = &mut *service.borrow_mut() {
        sw.handler = Some(handler);
        Ok(())
      } else {
        Err(io::Error::new(io::ErrorKind::NotConnected, "Service was not initialized"))
      }
    })
  }

  /// This method is a trigger 
  /// This is workaround while we don't have wasi::poll_oneoff, 
  /// ideally we shall just poll and wait for FD_READ event.
  pub fn on_message() -> io::Result<usize> {
    SERVICE.with(|service| {
      if let Some(sw) = &mut *service.borrow_mut() {
        if let Some(handler) = &sw.handler {
          let mut buf: [u8; 1000] = [0; 1000];
          let len = sw.input.read(&mut buf)?;
          handler.on_message(&buf[0..len])?;
          Ok(len)
        } else {
          Err(io::Error::new(io::ErrorKind::NotConnected, "Worker was not initialized"))
        }
      } else {
        Err(io::Error::new(io::ErrorKind::NotConnected, "Service was not initialized"))
      }
    })
  }

  /// Post message to external consumers
  /// 
  /// Example usage:
  /// ```
  /// ServiceWorker::post_message(b"mymesage")
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
}