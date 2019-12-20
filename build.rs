fn main() -> Result<(), Box<dyn std::error::Error>> {
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
        "telemetry"
    ];

    for plugin in plugins {
        let proto_path = vec![proto_path(plugin)];
        let proto_include_paths = proto_include_paths(plugin);
        let proto_out_dir = proto_generated_path(plugin);
        tonic_build::configure()
            .build_server(false)
            .out_dir(proto_out_dir)
            .compile(&proto_path, &proto_include_paths)?;
    }
    Ok(())
}

fn proto_path(plugin_name: &str) -> String {
    let protos_path = "proto/protos";
    String::from(format!(
        "{path}/{name}/{name}.proto",
        path = protos_path,
        name = plugin_name
    ))
}

fn proto_generated_path(plugin_name: &str) -> String {
    let generated_path = "src/generated";
    String::from(format!(
        "{root}/{name}",
        root = generated_path,
        name = plugin_name
    ))
}

fn proto_include_paths(plugin_name: &str) -> Vec<String> {
    let protos_path = "proto/protos";
    vec![String::from(format!("{root}/{name}", root = protos_path, name = plugin_name))]
}
