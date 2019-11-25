use wasi_worker_yew::*;

struct MyAgent {
    link: AgentLink<Self>,
}

impl Agent for MyAgent {
    type Reach = Public;
    type Message = String;
    type Input = String;
    type Output = String;
    // Create an instance with a link to agent's environment.
    fn create(link: AgentLink<Self>) -> Self {
        MyAgent { link }
    }

    // Handle inner messages (of services of `send_back` callbacks)
    fn update(&mut self, _msg: Self::Message) { /* ... */
    }

    // Handle incoming messages from components of other agents.
    fn handle(&mut self, msg: Self::Input, who: HandlerId) {
        println!("Got incoming message: {}", msg);
        self.link.response(who, msg);
    }
}

fn main() {
    // In usual WASI setup with JS glue all output will be posted to /output.bin
    // Though in user filesystem to be able to run from shell output goes to ./output.bin
    let opt = ServiceOptions::default().with_cleanup();
    let output_file = match &opt.output {
        FileOptions::File(path) => path.clone(),
    };
    ServiceWorker::initialize(opt).expect("ServiceWorker created");

    // Following will create and initialize Agent
    let agent = WASIAgent::<MyAgent>::new();
    // It will run ThreadedWASI::run() to start Agent in WASI compatible context
    agent.run().expect("Agent run");
    // Attach Agent to ServiceWorker as message handler singleton
    ServiceWorker::set_message_handler(Box::new(agent));
    ServiceWorker::post_message(b"message").expect("ServiceWorker.post_message");

    // Supposedly we also received "hello" via stdin (see ./run.sh)
    // If that is the case output.bin and output.bin.snapshot will match
    message_ready();
    let output_dump = std::fs::read(&output_file).unwrap();
    println!(
        "Outgoing file content {:?}",
        String::from_utf8(output_dump.clone())
    );
    assert_eq!(
        output_dump,
        std::fs::read(format!("{}.snapshot", output_file)).unwrap()
    );
}

// this function will be called from worker.js when it receives message
// In the future it will be substituted by poll_oneoff or thread::yield,
// though currently poll_oneoff does not return control to browser
#[no_mangle]
pub extern "C" fn message_ready() -> usize {
    ServiceWorker::on_message().expect("ServiceWorker.on_message")
}
