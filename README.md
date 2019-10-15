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
cargo run --example info
Finished dev [unoptimized + debuginfo] target(s) in 3.79s
 Running `target/debug/examples/info`
Ok(info_result {result: SUCCESS result_str: "Success"} version {flight_sw_major: 1 flight_sw_minor: 10 os_sw_major: 7 os_sw_minor: 29})
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
