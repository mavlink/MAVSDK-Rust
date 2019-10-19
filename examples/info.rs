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

    let get_version_result = system.info.get_version();

    match get_version_result {
        Ok(version) => println!("Success: {:?}", version),
        Err(error) => match error {
            RequestError::MavErr(mav_err) => match mav_err {
                info::InfoError::Unknown(s) => println!("Unknown MAVLink error ({:?})", s),
                info::InfoError::InformationNotReceivedYet(s) => {
                    println!("Information not received yet ({:?})", s)
                }
            },
            RequestError::RpcErr(rpc_err) => println!("RPC error: {:?}", rpc_err),
        },
    }
}
