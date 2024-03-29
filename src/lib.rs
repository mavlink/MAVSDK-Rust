#![deny(clippy::all)]

use futures_util::future::try_join3;

pub mod info;
pub mod mocap;
pub mod telemetry;

#[derive(Debug, thiserror::Error)]
pub enum RequestError<PluginMavErr>
where
    PluginMavErr: std::error::Error,
{
    #[error("MAVLink error: {0}")]
    Mav(PluginMavErr),
    #[error("RPC error: {0}")]
    Rpc(#[from] tonic::Status),
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
    pub async fn connect(url: Option<String>) -> Result<Self, tonic::transport::Error> {
        let url = url.unwrap_or_else(|| String::from("http://0.0.0.0:50051"));

        let (mocap, info, telemetry) = try_join3(
            mocap::Mocap::connect(url.clone()),
            info::Info::connect(url.clone()),
            telemetry::Telemetry::connect(url),
        )
        .await?;

        Ok(Self {
            mocap,
            info,
            telemetry,
        })
    }
}

#[tonic::async_trait]
trait Connect {
    async fn connect(url: String) -> Result<Self, tonic::transport::Error>
    where
        Self: Sized;
}
