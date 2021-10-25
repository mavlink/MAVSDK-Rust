const PROTO_INCLUDE_PATH: &str = "proto/protos";

const PLUGINS: &[&str] = &[
    //"action",
    //"calibration",
    //"camera",
    //"core",
    //"geofence",
    //"gimbal",
    "info",
    //"mission",
    "mocap",
    //"offboard",
    //"param",
    //"shell",
    "telemetry",
];

fn main() -> std::io::Result<()> {
    for plugin in PLUGINS {
        let proto_path = format!("{0}/{1}/{1}.proto", PROTO_INCLUDE_PATH, plugin);

        tonic_build::configure()
            .build_server(false)
            .compile(&[proto_path.as_str()], &[PROTO_INCLUDE_PATH])?;
    }
    Ok(())
}
