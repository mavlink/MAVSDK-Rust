use libmavsdk::System;
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
        Err(e) => println!("{}", e),
    };
}
