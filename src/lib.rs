//!  This crate provides rust library and JS glue code to compose service worker
//!  
//!  Example usage:
//!  ```
//!  use wasi_worker::*;
//!  use std::thread_local;
//!  use std::cell::RefCell;
//! 
//!  thread_local! {
//!    static SERVICE: RefCell<Option<ServiceWorker>> = RefCell::new(None);
//!  }
//! 
//!  struct MyWorker {}
//!  impl Worker for MyWorker {
//!    fn on_message(&self, msg: &[u8]) {
//!      // Process incoming message
//!      println!("My Worker got message: {:?}", msg);
//!    }
//!  }
//! 
//!  fn main() {
//!    let worker = MyWorker {};
//!    let mut service = ServiceWorker::new(Box::new(worker))
//!      .expect("ServiceWorker::new");
//!    service.post_message(b"message")
//!      .expect("ServiceWorker.post_message");
//!    SERVICE.with(|local| local.replace(Some(service)));
//!    message_ready();
//!  }
//!  // this function will be called from worker.js when it receives message
//!  // In the future it will be substituted by poll_oneoff or thread::yield, 
//!  // though currently poll_oneoff does not return control to browser
//!  pub extern "C" fn message_ready() -> usize {
//!    let mut len: usize = 0;
//!    SERVICE.with(move |local| {
//!      if let Some(service) = &mut *local.borrow_mut() {
//!        len = service.on_message()
//!          .expect("ServiceWorker.on_message")
//!      } else {
//!        panic!("Service not initialized");
//!      }
//!    });
//!    len
//!  }
//!  ```

mod worker;
mod service;

pub use worker::Worker;
pub use service::ServiceWorker;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
