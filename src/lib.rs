mod generated;

use generated::mocap::mocap_grpc;
pub use generated::mocap::mocap;

use generated::info::info_grpc;
pub use generated::info::info;


use grpcio::{ChannelBuilder, EnvBuilder};

pub struct System {
    pub mocap : mocap_grpc::MocapServiceClient,
    pub info : info_grpc::InfoServiceClient,
}

impl System {
    pub fn new(url: Option<String>) -> System {
        let env = std::sync::Arc::new(EnvBuilder::new().build());
        let url = match url {
            Some(x) => x,
            None => String::from("localhost:50051")
        };
        let ch = ChannelBuilder::new(env).connect(url.as_str());

        System {
            mocap: mocap_grpc::MocapServiceClient::new(ch.clone()),
            info: info_grpc::InfoServiceClient::new(ch)
        }
    }
}
