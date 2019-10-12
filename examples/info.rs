extern crate libmavsdk;

use libmavsdk::*;

fn main() {
    let mut args = Vec::new();

    for arg in std::env::args.skip(1) {
        args.push(&arg);
    }

    if args.len() == 0 {
        writeln!(std::io::stderr(), "Usage: info <CONNECTION_URL>").unwrap();
        std::process::exit(1);
    }
}
