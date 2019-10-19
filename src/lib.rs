mod generated;

pub use generated::info;
pub use generated::mocap;

use grpcio::{ChannelBuilder, EnvBuilder};

pub struct System {
    pub mocap: mocap::Mocap,
    pub info: info::Info,
}

#[derive(Debug)]
pub enum RequestError<PluginMavErr> {
    MavErr(PluginMavErr),
    RpcErr(grpcio::Error),
}

trait FromRpcResult<T> {
    fn from_rpc_result(rpc_result: ::grpcio::Result<T>) -> Self;
}

pub type RequestResult<SuccessType, PluginMavErr> = Result<SuccessType, RequestError<PluginMavErr>>;

trait FromChannel {
    fn new(channel: &::grpcio::Channel) -> Self;
}

impl System {
    pub fn new(url: Option<String>) -> System {
        let env = std::sync::Arc::new(EnvBuilder::new().build());
        let url = match url {
            Some(x) => x,
            None => String::from("localhost:50051"),
        };
        let ch = ChannelBuilder::new(env).connect(url.as_str());

        System {
            mocap: mocap::Mocap::new(&ch),
            info: info::Info::new(&ch),
        }
    }
}
