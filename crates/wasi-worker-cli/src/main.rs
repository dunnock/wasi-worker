use structopt::StructOpt;
use wasi_worker_cli::Cli;

fn main() {
    let opt = Cli::from_args();
    opt.exec().expect("command failed");
}
