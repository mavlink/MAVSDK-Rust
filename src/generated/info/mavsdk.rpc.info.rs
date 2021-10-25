/// System version information.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Version {
    /// Flight software major version
    #[prost(int32, tag = "1")]
    pub flight_sw_major: i32,
    /// Flight software minor version
    #[prost(int32, tag = "2")]
    pub flight_sw_minor: i32,
    /// Flight software patch version
    #[prost(int32, tag = "3")]
    pub flight_sw_patch: i32,
    /// Flight software vendor major version
    #[prost(int32, tag = "4")]
    pub flight_sw_vendor_major: i32,
    /// Flight software vendor minor version
    #[prost(int32, tag = "5")]
    pub flight_sw_vendor_minor: i32,
    /// Flight software vendor patch version
    #[prost(int32, tag = "6")]
    pub flight_sw_vendor_patch: i32,
    /// Operating system software major version
    #[prost(int32, tag = "7")]
    pub os_sw_major: i32,
    /// Operating system software minor version
    #[prost(int32, tag = "8")]
    pub os_sw_minor: i32,
    /// Operating system software patch version
    #[prost(int32, tag = "9")]
    pub os_sw_patch: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetVersionRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetVersionResponse {
    #[prost(message, optional, tag = "1")]
    pub info_result: ::core::option::Option<InfoResult>,
    /// Version information about the system
    #[prost(message, optional, tag = "2")]
    pub version: ::core::option::Option<Version>,
}
/// Result type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InfoResult {
    /// Result enum value
    #[prost(enumeration = "info_result::Result", tag = "1")]
    pub result: i32,
    /// Human-readable English string describing the result
    #[prost(string, tag = "2")]
    pub result_str: ::prost::alloc::string::String,
}
/// Nested message and enum types in `InfoResult`.
pub mod info_result {
    /// Possible results returned for info requests.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Result {
        /// Unknown error
        Unknown = 0,
        /// Request succeeded
        Success = 1,
        /// Information has not been received yet
        InformationNotReceivedYet = 2,
    }
}
#[doc = r" Generated client implementations."]
pub mod info_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Provide infomation about the hardware and/or software of a system."]
    #[derive(Debug, Clone)]
    pub struct InfoServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl InfoServiceClient<tonic::transport::Channel> {
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
    impl<T> InfoServiceClient<T>
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
        ) -> InfoServiceClient<InterceptedService<T, F>>
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
            InfoServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Get the system version information."]
        pub async fn get_version(
            &mut self,
            request: impl tonic::IntoRequest<super::GetVersionRequest>,
        ) -> Result<tonic::Response<super::GetVersionResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/mavsdk.rpc.info.InfoService/GetVersion");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
