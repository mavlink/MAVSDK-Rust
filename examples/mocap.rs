use libmavsdk::{mocap, RequestError, System};
use std::io::{self, Write};
use std::time::Duration;

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

    let mut vision_position_estimate = mocap::VisionPositionEstimate::default();
    vision_position_estimate.pose_covariance.covariance_matrix = vec![std::f32::NAN];

    let mut counter: i32 = 500;

    while counter > 0 {
        if let Err(error) = system
            .mocap
            .set_vision_position_estimate(vision_position_estimate.clone())
            .await
        {
            match error {
                RequestError::MavErr(mav_err) => println!("MAVLink error: {:?}", mav_err),
                RequestError::RpcErr(rpc_err) => println!("RPC error: {:?}", rpc_err.message()),
            }
            return;
        };
        counter -= 1;
        vision_position_estimate.position_body.x_m += 0.1;
        vision_position_estimate.position_body.y_m -= 0.1;
        vision_position_estimate.position_body.z_m -= 0.01;

        tokio::time::sleep(Duration::from_millis(20)).await;
    }

    println!("All sended successfully!");
}
