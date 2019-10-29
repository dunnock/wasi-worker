use structopt::StructOpt;
use std::fs;
use std::io;
use std::path::Path;
use std::process::{Command, Stdio};

/// Install JavaScript glue code and WASI toolset for WASI worker to function.
#[derive(Debug, StructOpt)]
enum Cli {
    /// Install static files
    Install,
    /// Build and deploy
    Deploy,
}

impl Cli {
    const WORKER_JS: &'static [u8] = include_bytes!("../js/dist/worker.js");
    const WASM_TRANSFORMER: &'static [u8] = include_bytes!("../js/dist/wasm_transformer_bg.wasm");
    fn exec(&self) -> io::Result<()> {
        match self {
            Self::Install => self.install(),
            Self::Deploy => self.deploy()
        }
    }
    fn install(&self) -> io::Result<()> {
        Ok(())
    }
    fn deploy(&self) -> io::Result<()> {
        println!("Building worker with release settings");
        build_worker()?;
        println!("Output will go to ./dist");
        fs::create_dir_all("dist")?;
        println!("Copying target/wasm32-wasi/release/worker.wasm");
        fs::copy("target/wasm32-wasi/release/worker.wasm", "dist/worker.wasm")?;
        println!("Deploying JavaScript glue code under dist/worker.js");
        fs::write("dist/worker.js", Self::WORKER_JS)?;
        println!("Deploying wasm transformer under dist/wasm_transformer_bg.wasm");
        fs::write("dist/wasm_transformer_bg.wasm", Self::WASM_TRANSFORMER)?;
        Ok(())
    }
}

fn build_worker() -> io::Result<()> {
    // if the submodule has not been checked out, the build will stall
    if !Path::new("./Cargo.toml").exists() {
        panic!("Current dir is not cargo package");
    }

    let mut cmd = Command::new("cargo");
    cmd.args(&[
        "build",
        "--release",
        "--target=wasm32-wasi",
    ])
    .stdout(Stdio::inherit())
    .stderr(Stdio::inherit());
    let output = cmd.output()?;

    let status = output.status;
    if !status.success() {
        panic!(
            "Building worker failed: exit code: {}",
            status.code().unwrap()
        );
    }

    Ok(())
}

fn main() {
    let opt = Cli::from_args();
    opt.exec()
        .expect("command failed");
}

