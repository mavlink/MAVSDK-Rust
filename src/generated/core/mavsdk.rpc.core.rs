#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeConnectionStateRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConnectionStateResponse {
    /// Connection state
    #[prost(message, optional, tag = "1")]
    pub connection_state: ::std::option::Option<ConnectionState>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListRunningPluginsRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListRunningPluginsResponse {
    /// Plugin info
    #[prost(message, repeated, tag = "1")]
    pub plugin_info: ::std::vec::Vec<PluginInfo>,
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
    pub name: std::string::String,
    /// Address where the plugin is running
    #[prost(string, tag = "2")]
    pub address: std::string::String,
    /// Port where the plugin is running
    #[prost(int32, tag = "3")]
    pub port: i32,
}
#[doc = r" Generated server implementations."]
pub mod core_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Access to the connection state and running plugins."]
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
        T::ResponseBody: Body + HttpBody + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as HttpBody>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
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
    impl<T: Clone> Clone for CoreServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
}
