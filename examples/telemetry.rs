extern crate libmavsdk;

use libmavsdk::*;
use std::io::{self, Write};
use futures::stream::StreamExt;

#[tokio::main]
async fn main() {

    let mut args = Vec::new();

    for arg in std::env::args().skip(1) {
        args.push(arg.clone());
    }

    if args.len() > 1 {
        io::stderr()
            .write_all(b"Usage: telemetry [connection_url]\n")
            .unwrap();
        std::process::exit(1);
    }

    let url: Option<String> = if args.len() == 0 {
        Option::None
    } else {
        Option::Some(args[0].clone())
    };

    let mut system = match System::connect(url).await {
        Ok(system) => system,
        Err(err) => {
            println!("Connection error: {:?}", err);
            return
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
    };
    println!("Exit");
}