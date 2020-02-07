use wasi_worker::*;

struct MyWorker {}
impl Handler for MyWorker {
    fn on_message(&self, msg: &[u8]) -> std::io::Result<()> {
        // Process incoming message
        println!("My Worker got message: {:?}", msg);
        Ok(())
    }
}

fn main() {
    // In WASI setup output will go to /output.bin
    // In native (test) setup output will go to ./output.bin
    let opt = ServiceOptions::default();
    ServiceWorker::initialize(opt).expect("ServiceWorker::initialize");

    // Attach Agent to ServiceWorker as message handler singleton
    ServiceWorker::set_message_handler(Box::new(MyWorker {}));
    message_ready();

    // Send binary message to main browser application
    // this requires JS glue see wasi-worker-cli
    ServiceWorker::post_message(b"message").expect("ServiceWorker::post_message");
}
