# MAVSDK-Rust

This is the Rust wrapper for MAVSDK.

The Rust wrapper is based on a gRPC client communicating with the gRPC server written in C++.
The wrapper is potentially auto-generated from the message definitions ([proto files](https://github.com/mavlink/MAVSDK-Proto)).

# Build

```bash
cargo build
```

# Run examples

1. Run **MAVSDK Backend** on localhost
2. Run examples
```bash
cargo run --example info
```
```bash
cargo run --example mocap
```

# Expected examples output

- `info`
```bash
$ cargo run --example info
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/examples/info`
Ok(GetVersionResponse { info_result: InfoResult { result: Success, result_str: "Success" }, version: Some(Version { flight_sw_major: 1, flight_sw_minor: 10, flight_sw_patch: 0, flight_sw_vendor_major: 0, flight_sw_vendor_minor: 0, flight_sw_vendor_patch: 0, os_sw_major: 7, os_sw_minor: 29, os_sw_patch: 0 }) })
```
- `mocap`
```bash
cargo run --example mocap
Finished dev [unoptimized + debuginfo] target(s) in 0.31s
 Running `target/debug/examples/mocap`
Ok(mocap_result {result: SUCCESS result_str: "Success"})
```

# Protoc gen

```bash
tools/generate_from_protos.sh
```
