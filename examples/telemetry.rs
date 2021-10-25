use futures_util::stream::StreamExt;
use libmavsdk::System;
use std::io::{self, Write};

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    if args.len() > 1 {
        io::stderr()
            .write_all(b"Usage: telemetry [connection_url]\n")
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

    let mut stream_odometry = system.telemetry.subscribe_odometry().await.unwrap();
    while let Some(odometry) = stream_odometry.next().await {
        match odometry {
            Ok(odometry) => println!("Received: {:?}", odometry),
            Err(err) => {
                println!("Break: {:?}", err);
                break;
            }
        }
    }
    println!("Exit");
}
