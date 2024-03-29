use futures_util::stream::StreamExt;
use libmavsdk::System;
use std::io::{self, Write};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();

    if args.len() > 1 {
        io::stderr()
            .write_all(b"Usage: telemetry [connection_url]\n")
            .unwrap();
        std::process::exit(1);
    }

    let url = args.first().cloned();

    let mut stream_odometry = System::connect(url)
        .await?
        .telemetry
        .subscribe_odometry()
        .await?;

    while let Some(odometry) = stream_odometry.next().await {
        println!("Received: {:?}", odometry?);
    }
    println!("Exit");
    Ok(())
}
