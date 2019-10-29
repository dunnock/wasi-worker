use crate::Worker;
use std::io::{self, Read, Write};
use std::fs::File;

/// Connects Rust Worker with browser service worker
pub struct ServiceWorker<T: Worker> {
  output: File,
  input: io::Stdin,
  handler: Option<T>
}

impl<T: Worker> ServiceWorker<T> {
  pub const OUTFILE: &'static str = "/output.bin";

  pub fn new() -> io::Result<ServiceWorker<T>> {
    Ok(ServiceWorker {
      output: File::create(Self::OUTFILE)?,
      input: io::stdin(),
      handler: None
    })
  }
  pub fn set_message_handler(&mut self, handler: T) {
    self.handler = Some(handler)
  }
  pub fn on_message(&mut self) -> io::Result<usize> {
    let mut buf: [u8; 1000] = [0; 1000];
    let len = self.input.read(&mut buf)?;
    if let Some(handler) = &self.handler {
      handler.on_message(&buf[0..len])?;
      Ok(len)
    } else {
      Err(io::Error::new(io::ErrorKind::NotConnected, "Worker was not initialized"))
    }
  }
  pub fn post_message(&mut self, msg: &[u8]) -> std::io::Result<()> {
    self.output.write_all(msg)
  }
}