use wasi_worker_cli::Cli;
use structopt::StructOpt;

fn main() {
    let opt = Cli::from_args();
    opt.exec()
        .expect("command failed");
}
