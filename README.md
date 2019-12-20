# MAVSDK-Rust

This is the Rust wrapper for MAVSDK.

The Rust wrapper is based on a gRPC client communicating with the gRPC server written in C++.
The wrapper is potentially auto-generated from the message definitions ([proto files](https://github.com/mavlink/MAVSDK-Proto)).

# Update Rust for async/await support (first time) 
```bash
$ rustup update
```

# Build

```bash
$ cargo build
```

# Run examples

1. Run **MAVSDK Backend** on `localhost:50051`
2. Run examples
```bash
$ cargo run --example info
```
```bash
$ cargo run --example mocap
```
```bash
$ cargo run --example telemetry
```

# Expected examples output

- `info`
```bash
$ cargo run --example info
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/examples/info`
Version received: Version { flight_sw_major: 1, flight_sw_minor: 10, flight_sw_patch: 0, flight_sw_vendor_major: 0, flight_sw_vendor_minor: 0, flight_sw_vendor_patch: 0, os_sw_major: 8, os_sw_minor: 2, os_sw_patch: 0 }
```
- `mocap`
```bash
$ cargo run --example mocap
Finished dev [unoptimized + debuginfo] target(s) in 0.31s
 Running `target/debug/examples/mocap`
All sended successfully!
```
- `telemetry`
```bash
$ cargo run --example telemery
   Compiling libmavsdk v0.1.0 (/home/ildar/sw/mavsdk-rust)
    Finished dev [unoptimized + debuginfo] target(s) in 4.92s
     Running `target/debug/examples/telemetry`
Odometry: Odometry { time_usec: 0, frame_id: EstimNed, child_frame_id: Undef, position_body: PositionBody { x_m: 0.0, y_m: 0.0, z_m: -3.483048 }, q: Quaternion { w: 0.6384722, x: -0.004061609, y: 0.079110526, z: 0.76555747 }, speed_body: SpeedBody { velocity_x_m_s: 0.0042169667, velocity_y_m_s: -0.0015938352, velocity_z_m_s: -0.014632007 }, angular_velocity_body: AngularVelocityBody { roll_rad_s: 0.0005086092, pitch_rad_s: 0.00023366197, yaw_rad_s: -0.0002803828 }, pose_covariance: Covariance { covariance_matrix: [0.0079130605, 0.0, 0.0, 0.0, 0.0, 0.0, 0.007913225, 0.0, 0.0, 0.0, 0.0, 0.044821125, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0] }, velocity_covariance: Covariance { covariance_matrix: [0.0052988436, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0052990587, 0.0, 0.0, 0.0, 0.0, 0.0045366324, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0] } }
...
```
