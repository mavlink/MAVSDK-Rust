use libmavsdk::{mocap, System};
use std::io::{self, Write};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();

    if args.len() > 1 {
        io::stderr()
            .write_all(b"Usage: info [connection_url]\n")
            .unwrap();
        std::process::exit(1);
    }

    let url = args.get(0).cloned();

    let mut system = System::connect(url).await?;

    let mut vision_position_estimate = mocap::VisionPositionEstimate::default();
    vision_position_estimate.pose_covariance.covariance_matrix = vec![std::f32::NAN];

    for _ in 0..500 {
        system
            .mocap
            .set_vision_position_estimate(vision_position_estimate.clone())
            .await?;

        vision_position_estimate.position_body.x_m += 0.1;
        vision_position_estimate.position_body.y_m -= 0.1;
        vision_position_estimate.position_body.z_m -= 0.01;

        tokio::time::sleep(Duration::from_millis(20)).await;
    }

    println!("All sended successfully!");

    Ok(())
}
