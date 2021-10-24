#![deny(clippy::all)]

#[allow(clippy::all)]
mod generated;

use futures::future::try_join3;
pub use generated::info;
pub use generated::mocap;
pub use generated::telemetry;

#[derive(Debug)]
pub enum RequestError<PluginMavErr> {
    MavErr(PluginMavErr),
    RpcErr(tonic::Status),
}

pub type RequestResult<SuccessType, PluginMavErr> = Result<SuccessType, RequestError<PluginMavErr>>;

type TonicResult<T> = Result<tonic::Response<T>, tonic::Status>;

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
        let url = url.unwrap_or_else(|| String::from("http://0.0.0.0:50051"));

        let (mocap, info, telemetry) = try_join3(
            mocap::Mocap::connect(&url),
            info::Info::connect(&url),
            telemetry::Telemetry::connect(&url),
        )
        .await?;

        Ok(System {
            mocap,
            info,
            telemetry,
        })
    }
}

#[tonic::async_trait]
trait Connect {
    #[allow(clippy::ptr_arg)]
    async fn connect(url: &String) -> Result<Self, tonic::transport::Error>
    where
        Self: Sized;
}
