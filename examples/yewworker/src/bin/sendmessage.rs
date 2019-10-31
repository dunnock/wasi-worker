use wasi_worker_yew::*;
use std::io::{Write};
use serde::{Serialize, Deserialize};

/// Id of responses handler.
#[derive(Serialize, Deserialize)]
struct HandlerId(usize, bool);

fn main() {
  let srzd = serde_json::to_string(&HandlerId(0, true)).unwrap();
  let hdl: wasi_worker_yew::HandlerId = serde_json::from_str(&srzd).unwrap();
  let msg = ToWorker::<String>::ProcessInput(hdl, "hello".to_string());
  std::io::stdout().write(&msg.pack())
    .expect("Write to stdout");
}