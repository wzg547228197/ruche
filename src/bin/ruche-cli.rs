#[macro_use] extern crate log;

use std::io;
use std::io::{BufRead, Write};
use std::process::exit;
use structopt::StructOpt;
use std::net::SocketAddr;
use ruche::RucheClient;

const DEFAULT_CONNECTING_ADDRESS: &str = "127.0.0.1:8030";

#[derive(StructOpt, Debug)]
#[structopt(name="ruche-server", author, about)]
struct Opt {
    #[structopt(
        long,
        help = "Sets the connecting address",
        value_name = "IP:PORT",
        default_value(DEFAULT_CONNECTING_ADDRESS),
        parse(try_from_str)
    )]
    addr: SocketAddr
}

fn main() {
    env_logger::init();
    info!("ruche-cli {}", env!("CARGO_PKG_VERSION"));
    let opt = Opt::from_args();
    info!("ruche-cli connecting {}...", opt.addr);
    let client = RucheClient::new(opt.addr);
    if let Err(e) = client {
        error!("connection failed {}", e);
        exit(1);
    }

    let mut client = client.unwrap();
    info!("ruche-cli connected!");
    print!("ruche-cli > ");
    io::stdout().flush().unwrap();

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let vec: Vec<&str> = line.trim().split(" ").collect();
        let vec: Vec<&str> = vec.into_iter().filter(|&item| !item.is_empty()).collect();

        match vec.get(0).unwrap() {
            &"get" => {
                let &key = vec.get(1).unwrap();
                let res = client.get(key.to_owned()).unwrap();
                println!("{}", res.unwrap_or_default());
            }
            &"set" => {
                let &key = vec.get(1).unwrap();
                let &value = vec.get(2).unwrap();
                let _res = client.set(key.to_owned(), value.to_owned());
                println!("OK");
            }
            &"rm" => {
                let &key = vec.get(1).unwrap();
                let _res = client.remove(key.to_owned());
                println!("OK");
            }
            _ => {
                println!("Command not support!");
            },
        }

        print!("ruche-cli > ");
        io::stdout().flush().unwrap();
    }
}