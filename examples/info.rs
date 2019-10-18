extern crate libmavsdk;

use libmavsdk::*;
use std::io::{self, Write};

fn main() {
    let mut args = Vec::new();

    for arg in std::env::args().skip(1) {
        args.push(arg.clone());
    }

    if args.len() > 1 {
        io::stderr()
            .write_all(b"Usage: info [connection_url]\n")
            .unwrap();
        std::process::exit(1);
    }

    let url: Option<String> = if args.len() == 0 {
        Option::None
    } else {
        Option::Some(args[0].clone())
    };

    let system = System::new(url);

    let version = system.info.get_version();

    println!("{:?}", version);
}
