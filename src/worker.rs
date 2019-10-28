pub trait Worker {
  fn on_message(&self, msg: &[u8]);
}