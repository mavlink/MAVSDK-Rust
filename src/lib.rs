mod generated;

pub use generated::mocap;
pub use generated::info;

use grpcio::{ChannelBuilder, EnvBuilder};

pub struct System {
    pub mocap : mocap::Mocap,
    pub info : info::Info
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
            mocap: mocap::Mocap::new(ch.clone()),
            info: info::Info::new(ch)
        }
    }
}
