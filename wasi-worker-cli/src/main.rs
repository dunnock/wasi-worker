use structopt::StructOpt;
use std::fs;
use std::io;
use std::path::Path;
use std::process::{Command, Stdio};
use toml_edit::{Document, value, Item, Table, array};

fn worker_table() -> Table {
    let mut table = Table::new();
    table["name"] = value("worker");
    table["path"] = value("src/bin/worker.rs");
    table
}

const WASI_WORKER_VERSION: &str = "0.3";

/// Install JavaScript glue code and WASM toolset for wasi-worker browser worker to function.
/// 
/// Details https://crates.io/crates/wasi-worker
#[derive(Debug, StructOpt)]
enum Cli {
    /// Install static files and worker.rs template in current crate. 
    /// 
    /// Note! it adds [[bin]] target to ./Cargo.toml and sets wasi-worker dependency
    Install,
    /// Build with `--bin worker` and deploy with glue code under ./dist
    /// 
    /// Resulting dependency is still quite big, use wasm-gc to shrink it:
    /// % wasm-gc dist/worker.wasm
    Deploy,
}

impl Cli {
    const WORKER_JS: &'static [u8] = include_bytes!("../js/dist/worker.js");
    const WASM_TRANSFORMER: &'static [u8] = include_bytes!("../js/dist/wasm_transformer_bg.wasm");
    const WORKER_RS: &'static [u8] = include_bytes!("../worker/worker.rs");
    fn exec(&self) -> io::Result<()> {
        match self {
            Self::Install => self.install(),
            Self::Deploy => self.deploy()
        }
    }
    fn install(&self) -> io::Result<()> {
        Self::install_worker()
    }
    fn deploy(&self) -> io::Result<()> {
        println!("Building worker with release settings");
        Self::build_worker()?;
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

    fn install_worker() -> io::Result<()> {
        // if the submodule has not been checked out, the build will stall
        if !Path::new("./Cargo.toml").exists() {
            panic!("Current dir is not cargo package");
        }
        println!("Copying worker.rs template to src/bin/worker.rs");
        fs::create_dir_all("src/bin")?;
        fs::write("src/bin/worker.rs", Self::WORKER_RS)?;

        println!("Checking Cargo.toml for bin worker target...");
        let cargo_toml = fs::read_to_string("./Cargo.toml")?;
        let mut toml = cargo_toml.parse::<Document>()
            .expect("Invalid Cargo.toml, bin target not installed but can be built");
        // Insert only when there is no existing bin target with name worker
        let changed = match &mut toml["bin"] {
            Item::ArrayOfTables(tables) =>
                if tables.iter().filter(
                        |table| table["name"].as_str().filter(|val| val == &"worker").is_some()
                    ).count() == 0 {
                        tables.append(worker_table());
                        true
                } else {
                    false
                }
            _ => {
                toml["bin"] = array();
                toml["bin"].as_array_of_tables_mut().map(|arr| arr.append(worker_table()));
                true
            }
        };
        toml["dependencies"]["wasi-worker"] = value(WASI_WORKER_VERSION);
        if changed {
            // Note: it will overwrite Cargo.toml file
            println!("Adding bin worker target to Cargo.toml");
            fs::write("./Cargo.toml", toml.to_string())?;
        }
        Ok(())
    }

    fn build_worker() -> io::Result<()> {
        // if the submodule has not been checked out, the build will stall
        if !Path::new("./Cargo.toml").exists() {
            panic!("Current dir is not cargo package");
        }

        let mut cmd = Command::new("cargo");
        cmd.args(&[
            "build",
            "--bin=worker",
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
}

fn main() {
    let opt = Cli::from_args();
    opt.exec()
        .expect("command failed");
}

