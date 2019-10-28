use structopt::StructOpt;

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
    fn exec(&self) -> std::io::Result<()> {
        match self {
            Self::Install => self.install(),
            Self::Deploy => self.deploy()
        }
    }
    fn install(&self) -> std::io::Result<()> {
        Ok(())
    }
    fn deploy(&self) -> std::io::Result<()> {
        std::fs::write("dist/worker.js", Self::WORKER_JS)?;
        std::fs::write("dist/wasm_transformer_bg.js", Self::WASM_TRANSFORMER)
    }
}

fn main() {
    let opt = Cli::from_args();
    opt.exec()
        .expect("command failed");
}

