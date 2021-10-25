#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeConnectionStateRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConnectionStateResponse {
    /// Connection state
    #[prost(message, optional, tag = "1")]
    pub connection_state: ::core::option::Option<ConnectionState>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListRunningPluginsRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListRunningPluginsResponse {
    /// Plugin info
    #[prost(message, repeated, tag = "1")]
    pub plugin_info: ::prost::alloc::vec::Vec<PluginInfo>,
}
/// Connection state type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConnectionState {
    /// UUID of the vehicle
    #[prost(uint64, tag = "1")]
    pub uuid: u64,
    /// Whether the vehicle got connected or disconnected
    #[prost(bool, tag = "2")]
    pub is_connected: bool,
}
/// Plugin info type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PluginInfo {
    /// Name of the plugin
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Address where the plugin is running
    #[prost(string, tag = "2")]
    pub address: ::prost::alloc::string::String,
    /// Port where the plugin is running
    #[prost(int32, tag = "3")]
    pub port: i32,
}
#[doc = r" Generated client implementations."]
pub mod core_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Access to the connection state and running plugins."]
    #[derive(Debug, Clone)]
    pub struct CoreServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl CoreServiceClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> CoreServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> CoreServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            CoreServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        #[doc = r" Compress requests with `gzip`."]
        #[doc = r""]
        #[doc = r" This requires the server to support it otherwise it might respond with an"]
        #[doc = r" error."]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        #[doc = r" Enable decompressing responses with `gzip`."]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        #[doc = " Subscribe to 'connection state' updates."]
        pub async fn subscribe_connection_state(
            &mut self,
            request: impl tonic::IntoRequest<super::SubscribeConnectionStateRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::ConnectionStateResponse>>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.core.CoreService/SubscribeConnectionState",
            );
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        #[doc = " Get a list of currently running plugins."]
        pub async fn list_running_plugins(
            &mut self,
            request: impl tonic::IntoRequest<super::ListRunningPluginsRequest>,
        ) -> Result<tonic::Response<super::ListRunningPluginsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.core.CoreService/ListRunningPlugins",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
