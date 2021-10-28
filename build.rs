fn main() -> std::io::Result<()> {
    let plugins = vec![
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

    for plugin in plugins {
        let proto_path = proto_path(plugin);
        let proto_include_path = proto_include_path(plugin);
        tonic_build::configure()
            .build_server(false)
            .compile(&[proto_path], &[proto_include_path])?;
    }
    Ok(())
}

fn proto_path(plugin_name: &str) -> String {
    format!("proto/protos/{name}/{name}.proto", name = plugin_name)
}

fn proto_include_path(plugin_name: &str) -> String {
    format!("proto/protos/{}", plugin_name)
}
