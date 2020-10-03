#[macro_use] extern crate log;

use ruche::*;
use std::net::SocketAddr;
use structopt::StructOpt;
use std::process::exit;

const DEFAULT_LISTENING_ADDRESS: &str = "127.0.0.1:8030";

#[derive(StructOpt, Debug)]
#[structopt(name="ruche-server", author, about)]
struct Opt {
    #[structopt(
        long,
        help = "Sets the listening address",
        value_name = "IP:PORT",
        default_value(DEFAULT_LISTENING_ADDRESS),
        parse(try_from_str)
    )]
    addr: SocketAddr
}

fn main() {
    env_logger::init();
    let opt = Opt::from_args();
    let result = run(opt);
    if let Err(e) = result {
        error!("{}!", e);
        exit(1);
    }
}

fn run(opt: Opt) -> RucheResult<()> {
    let server: RucheServer = RucheServer::new();
    info!("ruche-server {}", env!("CARGO_PKG_VERSION"));
    info!("Listening on {}", opt.addr);

    server.run(opt.addr)
}