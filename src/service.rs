use crate::Worker;
use std::io::{self, Read, Write};
use std::fs::File;


/// Connects Rust Worker with browser service worker
/// 
/// Example usage:
/// ```myworker.rs
/// use wasi_worker::*;
/// 
/// struct MyWorker {}
/// impl Worker for MyWorker {
///   on_message(msg: &[u8]) {
///     // Process incoming message
///     println!("My Worker got message: {:?}", msg);
///   }
/// }
/// 
/// fn main() {
///   let worker = MyWorker::new();
///   let mut service = ServiceWorker::mew(Box::new(worker))
///     .expect("ServiceWorker::new");
///   service.post_message(b"message")
///     .expect("ServiceWorker.post_message");
/// }
/// ```
pub struct ServiceWorker {
  output: File,
  input: io::Stdin,
  worker: Box<dyn Worker>
}

impl ServiceWorker {
  pub const OUTFILE: &'static str = "/output.bin";

  pub fn new(worker: Box<dyn Worker>) -> io::Result<ServiceWorker> {
    Ok(ServiceWorker {
      output: File::create(Self::OUTFILE)?,
      input: io::stdin(),
      worker
    })
  }
  pub fn post_message(&mut self, msg: &[u8]) -> std::io::Result<()> {
    self.output.write_all(msg)
  }
  pub fn on_message(&mut self) -> io::Result<usize> {
    let mut buf: [u8; 1000] = [0; 1000];
    let len = self.input.read(&mut buf)?;
    self.worker.on_message(&buf[0..len]);
    Ok(len)
  }
}