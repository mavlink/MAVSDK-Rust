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

    let mut vision_position_estimate = mocap::VisionPositionEstimate::default();

    let mut angle_body = mocap::AngleBody::default();
    angle_body.roll_rad = 0.0;
    angle_body.pitch_rad = 0.0;
    angle_body.yaw_rad = 0.0;
    vision_position_estimate.angle_body = angle_body;

    let mut position_body = mocap::PositionBody::default();
    position_body.x_m = 0.0;
    position_body.y_m = 0.0;
    position_body.z_m = 0.0;
    vision_position_estimate.position_body = position_body;

    let mut pose_covariance = mocap::Covariance::default();
    pose_covariance.covariance_matrix = vec![std::f32::NAN];
    vision_position_estimate.pose_covariance = pose_covariance;

    let result = system
        .mocap
        .set_vision_position_estimate(vision_position_estimate);

    println!("{:?}", result);
}
