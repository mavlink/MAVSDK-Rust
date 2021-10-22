#[allow(clippy::all)]
mod generated;

pub use generated::info;
pub use generated::mocap;
pub use generated::telemetry;

#[derive(Debug)]
pub enum RequestError<PluginMavErr> {
    MavErr(PluginMavErr),
    RpcErr(tonic::Status),
}

pub type RequestResult<SuccessType, PluginMavErr> =
    std::result::Result<SuccessType, RequestError<PluginMavErr>>;

type TonicResult<T> = std::result::Result<tonic::Response<T>, tonic::Status>;

trait FromRpcResponse<T> {
    fn from_rpc_response(rpc_result: TonicResult<T>) -> Self;
}
pub struct System {
    pub mocap: mocap::Mocap,
    pub info: info::Info,
    pub telemetry: telemetry::Telemetry,
}

impl System {
    pub async fn connect(url: Option<String>) -> Result<System, tonic::transport::Error> {
        let url = match url {
            Some(x) => x,
            None => String::from("http://0.0.0.0:50051"),
        };

        Ok(System {
            mocap: mocap::Mocap::connect(&url).await?,
            info: info::Info::connect(&url).await?,
            telemetry: telemetry::Telemetry::connect(&url).await?,
        })
    }
}

#[tonic::async_trait]
trait Connect {
    async fn connect(url: &String) -> Result<Self, tonic::transport::Error>
    where
        Self: Sized;
}
