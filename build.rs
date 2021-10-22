fn main() -> std::io::Result<()> {
    let plugins = vec![
        "action",
        "calibration",
        "camera",
        "core",
        "geofence",
        "gimbal",
        "info",
        "mission",
        "mocap",
        "offboard",
        "param",
        "shell",
        "telemetry",
    ];

    for plugin in plugins {
        let proto_path = proto_path(plugin);
        let proto_include_paths = proto_include_paths(plugin);
        let proto_out_dir = proto_generated_path(plugin);
        tonic_build::configure()
            .build_server(false)
            .out_dir(proto_out_dir)
            .compile(&[proto_path], &proto_include_paths)?;
    }
    Ok(())
}

fn proto_path(plugin_name: &str) -> String {
    format!("proto/protos/{name}/{name}.proto", name = plugin_name)
}

fn proto_generated_path(plugin_name: &str) -> String {
    format!("src/generated/{name}", name = plugin_name)
}

fn proto_include_paths(plugin_name: &str) -> Vec<String> {
    vec![format!("proto/protos/{name}", name = plugin_name)]
}
