#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendRequest {
    #[prost(message, optional, tag = "1")]
    pub shell_message: ::std::option::Option<ShellMessage>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendResponse {
    #[prost(message, optional, tag = "1")]
    pub shell_result: ::std::option::Option<ShellResult>,
    /// Response message data (if available)
    #[prost(string, tag = "2")]
    pub response_message_data: std::string::String,
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
    pub data: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShellResult {
    /// Result enum value
    #[prost(enumeration = "shell_result::Result", tag = "1")]
    pub result: i32,
    /// Human-readable English string describing the result
    #[prost(string, tag = "2")]
    pub result_str: std::string::String,
}
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
#[doc = r" Generated server implementations."]
pub mod shell_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "*"]
    #[doc = " Allow to communicate with the vehicle's system shell."]
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
        T::ResponseBody: Body + HttpBody + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as HttpBody>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
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
    impl<T: Clone> Clone for ShellServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
}
