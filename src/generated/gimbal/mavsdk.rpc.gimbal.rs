#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetPitchAndYawRequest {
    /// Pitch angle in degrees (negative points down)
    #[prost(float, tag = "1")]
    pub pitch_deg: f32,
    /// Yaw angle in degrees (positive is clock-wise, range: -180 to 180 or 0 to 360)
    #[prost(float, tag = "2")]
    pub yaw_deg: f32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetPitchAndYawResponse {
    #[prost(message, optional, tag = "1")]
    pub gimbal_result: ::std::option::Option<GimbalResult>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetModeRequest {
    /// The mode to be set.
    #[prost(enumeration = "GimbalMode", tag = "1")]
    pub gimbal_mode: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetModeResponse {
    #[prost(message, optional, tag = "1")]
    pub gimbal_result: ::std::option::Option<GimbalResult>,
}
/// Result type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GimbalResult {
    /// Result enum value
    #[prost(enumeration = "gimbal_result::Result", tag = "1")]
    pub result: i32,
    /// Human-readable English string describing the result
    #[prost(string, tag = "2")]
    pub result_str: std::string::String,
}
pub mod gimbal_result {
    /// Possible results returned for gimbal commands.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Result {
        /// Unknown error
        Unknown = 0,
        /// Command was accepted
        Success = 1,
        /// Error occurred sending the command
        Error = 2,
        /// Command timed out
        Timeout = 3,
    }
}
/// Gimbal mode type.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum GimbalMode {
    /// Yaw follow will point the gimbal to the vehicle heading
    YawFollow = 0,
    /// Yaw lock will fix the gimbal poiting to an absolute direction
    YawLock = 1,
}
#[doc = r" Generated client implementations."]
pub mod client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Provide control over a gimbal."]
    pub struct GimbalServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl GimbalServiceClient<tonic::transport::Channel> {
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
    impl<T> GimbalServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + HttpBody + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as HttpBody>::Error: Into<StdError> + Send,
        <T::ResponseBody as HttpBody>::Data: Into<bytes::Bytes> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        #[doc = r" Check if the service is ready."]
        pub async fn ready(&mut self) -> Result<(), tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })
        }
        #[doc = ""]
        #[doc = ""]
        #[doc = " Set gimbal pitch and yaw angles."]
        #[doc = ""]
        #[doc = " This sets the desired pitch and yaw angles of a gimbal."]
        #[doc = " Will return when the command is accepted, however, it might"]
        #[doc = " take the gimbal longer to actually be set to the new angles."]
        pub async fn set_pitch_and_yaw(
            &mut self,
            request: impl tonic::IntoRequest<super::SetPitchAndYawRequest>,
        ) -> Result<tonic::Response<super::SetPitchAndYawResponse>, tonic::Status> {
            self.ready().await?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.gimbal.GimbalService/SetPitchAndYaw",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = " Set gimbal mode."]
        #[doc = ""]
        #[doc = " This sets the desired yaw mode of a gimbal."]
        #[doc = " Will return when the command is accepted. However, it might"]
        #[doc = " take the gimbal longer to actually be set to the new angles."]
        pub async fn set_mode(
            &mut self,
            request: impl tonic::IntoRequest<super::SetModeRequest>,
        ) -> Result<tonic::Response<super::SetModeResponse>, tonic::Status> {
            self.ready().await?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/mavsdk.rpc.gimbal.GimbalService/SetMode");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for GimbalServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
}
