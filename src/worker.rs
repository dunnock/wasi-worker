/// Worker process will need to implement this trai to receive messages
pub trait Worker {
  fn on_message(&self, msg: &[u8]) -> std::io::Result<()>;
}