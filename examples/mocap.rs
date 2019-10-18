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

    let mut vision_position_estimate = mocap::VisionPositionEstimate::new();

    let mut angle_body = mocap::AngleBody::new();
    angle_body.set_roll_rad(0.0);
    angle_body.set_pitch_rad(0.0);
    angle_body.set_yaw_rad(0.0);
    vision_position_estimate.set_angle_body(angle_body);

    let mut position_body = mocap::PositionBody::new();
    position_body.set_x_m(0.0);
    position_body.set_y_m(0.0);
    position_body.set_z_m(0.0);
    vision_position_estimate.set_position_body(position_body);

    let mut pose_covariance = mocap::Covariance::new();
    pose_covariance.set_covariance_matrix(vec![std::f32::NAN]);
    vision_position_estimate.set_pose_covariance(pose_covariance);

    let result = system
        .mocap
        .set_vision_position_estimate(vision_position_estimate);

    println!("{:?}", result);
}
