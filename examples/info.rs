use libmavsdk::{info, RequestError, System};
use std::io::{self, Write};

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    if args.len() > 1 {
        io::stderr()
            .write_all(b"Usage: info [connection_url]\n")
            .unwrap();
        std::process::exit(1);
    }

    let url = args.get(0).cloned();

    let mut system = match System::connect(url).await {
        Ok(system) => system,
        Err(err) => {
            println!("Connection error: {:?}", err);
            return;
        }
    };

    match system.info.get_version().await {
        Ok(v) => println!("Version received: {:?}", v),
        Err(RequestError::MavErr(info::InfoError::Unknown(s))) => {
            println!("Unknown MAVLink error ({:?})", s);
        }
        Err(RequestError::MavErr(info::InfoError::InformationNotReceivedYet(s))) => {
            println!("{}", s);
        }
        Err(RequestError::RpcErr(rpc_err)) => println!("RPC error: {:?}", rpc_err),
    };
}
