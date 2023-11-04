use libmavsdk::System;
use std::io::{self, Write};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();

    if args.len() > 1 {
        io::stderr()
            .write_all(b"Usage: info [connection_url]\n")
            .unwrap();
        std::process::exit(1);
    }

    let url = args.first().cloned();

    let version = System::connect(url).await?.info.get_version().await?;

    println!("Version received: {:?}", version);

    Ok(())
}
