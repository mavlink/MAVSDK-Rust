#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetIntParamRequest {
    /// Name of the parameter
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetIntParamResponse {
    #[prost(message, optional, tag = "1")]
    pub param_result: ::core::option::Option<ParamResult>,
    /// Value of the requested parameter
    #[prost(int32, tag = "2")]
    pub value: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetIntParamRequest {
    /// Name of the parameter to set
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Value the parameter should be set to
    #[prost(int32, tag = "2")]
    pub value: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetIntParamResponse {
    #[prost(message, optional, tag = "1")]
    pub param_result: ::core::option::Option<ParamResult>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetFloatParamRequest {
    /// Name of the parameter
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetFloatParamResponse {
    #[prost(message, optional, tag = "1")]
    pub param_result: ::core::option::Option<ParamResult>,
    /// Value of the requested parameter
    #[prost(float, tag = "2")]
    pub value: f32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetFloatParamRequest {
    /// Name of the parameter to set
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Value the parameter should be set to
    #[prost(float, tag = "2")]
    pub value: f32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetFloatParamResponse {
    #[prost(message, optional, tag = "1")]
    pub param_result: ::core::option::Option<ParamResult>,
}
/// Result type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ParamResult {
    /// Result enum value
    #[prost(enumeration = "param_result::Result", tag = "1")]
    pub result: i32,
    /// Human-readable English string describing the result
    #[prost(string, tag = "2")]
    pub result_str: ::prost::alloc::string::String,
}
/// Nested message and enum types in `ParamResult`.
pub mod param_result {
    /// Possible results returned for param requests.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Result {
        /// Unknown error
        Unknown = 0,
        /// Request succeeded
        Success = 1,
        /// Request timed out
        Timeout = 2,
        /// Connection error
        ConnectionError = 3,
        /// Wrong type
        WrongType = 4,
        /// Parameter name too long (> 16)
        ParamNameTooLong = 5,
    }
}
#[doc = r" Generated client implementations."]
pub mod param_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Provide raw access to get and set parameters."]
    #[derive(Debug, Clone)]
    pub struct ParamServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ParamServiceClient<tonic::transport::Channel> {
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
    impl<T> ParamServiceClient<T>
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
        ) -> ParamServiceClient<InterceptedService<T, F>>
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
            ParamServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = ""]
        #[doc = " Get an int parameter."]
        #[doc = ""]
        #[doc = " If the type is wrong, the result will be `WRONG_TYPE`."]
        pub async fn get_int_param(
            &mut self,
            request: impl tonic::IntoRequest<super::GetIntParamRequest>,
        ) -> Result<tonic::Response<super::GetIntParamResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/mavsdk.rpc.param.ParamService/GetIntParam");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = " Set an int parameter."]
        #[doc = ""]
        #[doc = " If the type is wrong, the result will be `WRONG_TYPE`."]
        pub async fn set_int_param(
            &mut self,
            request: impl tonic::IntoRequest<super::SetIntParamRequest>,
        ) -> Result<tonic::Response<super::SetIntParamResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/mavsdk.rpc.param.ParamService/SetIntParam");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = " Get a float parameter."]
        #[doc = ""]
        #[doc = " If the type is wrong, the result will be `WRONG_TYPE`."]
        pub async fn get_float_param(
            &mut self,
            request: impl tonic::IntoRequest<super::GetFloatParamRequest>,
        ) -> Result<tonic::Response<super::GetFloatParamResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.param.ParamService/GetFloatParam",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = " Set a float parameter."]
        #[doc = ""]
        #[doc = " If the type is wrong, the result will be `WRONG_TYPE`."]
        pub async fn set_float_param(
            &mut self,
            request: impl tonic::IntoRequest<super::SetFloatParamRequest>,
        ) -> Result<tonic::Response<super::SetFloatParamResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.param.ParamService/SetFloatParam",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
