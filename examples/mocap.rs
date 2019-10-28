extern crate libmavsdk;

use libmavsdk::*;
use std::io::{self, Write};
use std::time::Duration;

use async_std::task;

#[tokio::main]
async fn main() {
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

    let mut system = match System::connect(url).await {
        Ok(system) => system,
        Err(err) => {
            println!("Connection error: {:?}", err);
            return
        }
    };

    let mut vision_position_estimate = mocap::VisionPositionEstimate::default();
    vision_position_estimate.pose_covariance.covariance_matrix = vec![std::f32::NAN];


    let mut counter = 500;

    while counter > 0 {
        match system.mocap.set_vision_position_estimate(vision_position_estimate.clone()).await {
            Ok(_) => {},
            Err(err) =>  {
                match err {
                    RequestError::MavErr(mav_err) => println!("MAVLink error: {:?}", mav_err),
                    RequestError::RpcErr(rpc_err) => println!("RPC error: {:?}", rpc_err.message())
                }
                return;
            }
        };
        counter = counter - 1;
        vision_position_estimate.position_body.x_m+= 0.1;
        vision_position_estimate.position_body.y_m-= 0.1;
        vision_position_estimate.position_body.z_m-= 0.01;

        task::sleep(Duration::from_millis(20)).await;

    }

    println!("All sended successfully!");
}