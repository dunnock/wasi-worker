use wasm_gc::Config;
use std::fs::File;
use std::io::{Read, Write};

pub fn gc(file: &str) -> std::io::Result<()> {
    let mut contents = Vec::new();
    File::open(file)?.read_to_end(&mut contents)?;

    let mut cfg = Config::new();
    cfg.demangle(false);
    let result = cfg.gc(&contents).expect("wasm-gc: failed to parse wasm module");
    File::create(file)?.write_all(&result)
}