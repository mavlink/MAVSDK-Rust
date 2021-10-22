#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendRequest {
    #[prost(message, optional, tag = "1")]
    pub shell_message: ::core::option::Option<ShellMessage>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendResponse {
    #[prost(message, optional, tag = "1")]
    pub shell_result: ::core::option::Option<ShellResult>,
    /// Response message data (if available)
    #[prost(string, tag = "2")]
    pub response_message_data: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShellMessage {
    /// Set if the sender wants the receiver to send a response.
    #[prost(bool, tag = "1")]
    pub need_response: bool,
    /// Timeout (ms).
    #[prost(uint32, tag = "2")]
    pub timeout_ms: u32,
    /// Serial data.
    #[prost(string, tag = "3")]
    pub data: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShellResult {
    /// Result enum value
    #[prost(enumeration = "shell_result::Result", tag = "1")]
    pub result: i32,
    /// Human-readable English string describing the result
    #[prost(string, tag = "2")]
    pub result_str: ::prost::alloc::string::String,
}
/// Nested message and enum types in `ShellResult`.
pub mod shell_result {
    /// Possible results returned for shell requests
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Result {
        /// Unknown error
        Unknown = 0,
        /// Request succeeded
        Success = 1,
        /// No system is connected
        NoSystem = 2,
        /// Connection error
        ConnectionError = 3,
        /// Response was not received
        NoResponse = 4,
        /// Shell busy (transfer in progress)
        Busy = 5,
    }
}
#[doc = r" Generated client implementations."]
pub mod shell_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = "*"]
    #[doc = " Allow to communicate with the vehicle's system shell."]
    #[derive(Debug, Clone)]
    pub struct ShellServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ShellServiceClient<tonic::transport::Channel> {
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
    impl<T> ShellServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + Send + Sync + 'static,
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
        ) -> ShellServiceClient<InterceptedService<T, F>>
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
            ShellServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Communicate with a vehicle's Shell."]
        pub async fn send(
            &mut self,
            request: impl tonic::IntoRequest<super::SendRequest>,
        ) -> Result<tonic::Response<super::SendResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/mavsdk.rpc.shell.ShellService/Send");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
