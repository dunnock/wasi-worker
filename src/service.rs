/// Connects Rust Worker with browser service worker
/// 
/// Example usage:
/// ```
/// use wasi_worker::*;
/// use std::thread_local;
/// use std::cell::RefCell;
//
/// thread_local! {
///   static SERVICE: RefCell<Option<ServiceWorker>> = RefCell::new(None);
/// }
///
/// struct MyWorker {}
/// impl Worker for MyWorker {
///   fn on_message(&self, msg: &[u8]) {
///     // Process incoming message
///     println!("My Worker got message: {:?}", msg);
///   }
/// }
///
/// fn main() {
///   let worker = MyWorker {};
///   let mut service = ServiceWorker::new(Box::new(worker))
///     .expect("ServiceWorker::new");
///   service.post_message(b"message")
///     .expect("ServiceWorker.post_message");
///   SERVICE.with(|local| local.replace(Some(service)));
///   message_ready();
/// }
/// // this function will be called from worker.js when it receives message
/// // In the future it will be substituted by poll_oneoff or thread::yield, 
/// // though currently poll_oneoff does not return control to browser
/// pub extern "C" fn message_ready() -> usize {
///   let mut len: usize = 0;
///   SERVICE.with(move |local| {
///     if let Some(service) = &mut *local.borrow_mut() {
///       len = service.on_message()
///         .expect("ServiceWorker.on_message")
///     } else {
///       panic!("Service not initialized");
///     }
///   });
///   len
/// }
/// ```


use crate::Worker;
use std::io::{self, Read, Write};
use std::fs::File;


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