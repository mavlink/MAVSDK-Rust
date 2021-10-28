macro_rules! include_plugin {
    ($plugin: ident, $path: tt ) => {
        #[allow(clippy::all, clippy::pedantic)]
        pub mod $plugin {
            tonic::include_proto!($path);
        }
    };
}

include_plugin!(info, "mavsdk.rpc.info");
include_plugin!(mocap, "mavsdk.rpc.mocap");
include_plugin!(telemetry, "mavsdk.rpc.telemetry");
