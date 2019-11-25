//! Tool to create and deploy WASM WASI-based browser service workers
//!
//! This tool provides JavaScript glue for browser service workers built with [wasi-worker](https://crates.io/crates/wasi-worker) library.
//! [More details](https://crates.io/crates/wasi-worker-cli)
//!

mod cli;
mod gc;
#[cfg(test)]
mod test;

pub use cli::Cli;
pub use gc::gc;
