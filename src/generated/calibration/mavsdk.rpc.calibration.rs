#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeCalibrateGyroRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CalibrateGyroResponse {
    #[prost(message, optional, tag = "1")]
    pub calibration_result: ::core::option::Option<CalibrationResult>,
    /// Progress data
    #[prost(message, optional, tag = "2")]
    pub progress_data: ::core::option::Option<ProgressData>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeCalibrateAccelerometerRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CalibrateAccelerometerResponse {
    #[prost(message, optional, tag = "1")]
    pub calibration_result: ::core::option::Option<CalibrationResult>,
    /// Progress data
    #[prost(message, optional, tag = "2")]
    pub progress_data: ::core::option::Option<ProgressData>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeCalibrateMagnetometerRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CalibrateMagnetometerResponse {
    #[prost(message, optional, tag = "1")]
    pub calibration_result: ::core::option::Option<CalibrationResult>,
    /// Progress data
    #[prost(message, optional, tag = "2")]
    pub progress_data: ::core::option::Option<ProgressData>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeCalibrateGimbalAccelerometerRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CalibrateGimbalAccelerometerResponse {
    #[prost(message, optional, tag = "1")]
    pub calibration_result: ::core::option::Option<CalibrationResult>,
    /// Progress data
    #[prost(message, optional, tag = "2")]
    pub progress_data: ::core::option::Option<ProgressData>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CancelRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CancelResponse {}
/// Result type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CalibrationResult {
    /// Result enum value
    #[prost(enumeration = "calibration_result::Result", tag = "1")]
    pub result: i32,
    /// Human-readable English string describing the result
    #[prost(string, tag = "2")]
    pub result_str: ::prost::alloc::string::String,
}
/// Nested message and enum types in `CalibrationResult`.
pub mod calibration_result {
    /// Possible results returned for calibration commands
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Result {
        /// Unknown error
        Unknown = 0,
        /// The calibration process succeeded
        Success = 1,
        /// Intermediate message showing progress of the calibration process
        InProgress = 2,
        /// Intermediate message giving instructions on the next steps required by the process
        Instruction = 3,
        /// Calibration failed
        Failed = 4,
        /// No system is connected
        NoSystem = 5,
        /// Connection error
        ConnectionError = 6,
        /// Vehicle is busy
        Busy = 7,
        /// Command refused by vehicle
        CommandDenied = 8,
        /// Command timed out
        Timeout = 9,
        /// Calibration process got cancelled
        Cancelled = 10,
    }
}
///
/// Progress data coming from calibration.
///
/// Can be a progress percentage, or an instruction text.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProgressData {
    #[prost(bool, tag = "1")]
    pub has_progress: bool,
    /// Progress (percentage)
    #[prost(float, tag = "2")]
    pub progress: f32,
    #[prost(bool, tag = "3")]
    pub has_status_text: bool,
    /// Instruction text
    #[prost(string, tag = "4")]
    pub status_text: ::prost::alloc::string::String,
}
#[doc = r" Generated client implementations."]
pub mod calibration_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Enable to calibrate sensors of a drone such as gyro, accelerometer, and magnetometer."]
    #[derive(Debug, Clone)]
    pub struct CalibrationServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl CalibrationServiceClient<tonic::transport::Channel> {
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
    impl<T> CalibrationServiceClient<T>
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
        ) -> CalibrationServiceClient<InterceptedService<T, F>>
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
            CalibrationServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Perform gyro calibration."]
        pub async fn subscribe_calibrate_gyro(
            &mut self,
            request: impl tonic::IntoRequest<super::SubscribeCalibrateGyroRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::CalibrateGyroResponse>>,
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
                "/mavsdk.rpc.calibration.CalibrationService/SubscribeCalibrateGyro",
            );
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        #[doc = " Perform accelerometer calibration."]
        pub async fn subscribe_calibrate_accelerometer(
            &mut self,
            request: impl tonic::IntoRequest<super::SubscribeCalibrateAccelerometerRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::CalibrateAccelerometerResponse>>,
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
                "/mavsdk.rpc.calibration.CalibrationService/SubscribeCalibrateAccelerometer",
            );
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        #[doc = " Perform magnetometer caliration."]
        pub async fn subscribe_calibrate_magnetometer(
            &mut self,
            request: impl tonic::IntoRequest<super::SubscribeCalibrateMagnetometerRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::CalibrateMagnetometerResponse>>,
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
                "/mavsdk.rpc.calibration.CalibrationService/SubscribeCalibrateMagnetometer",
            );
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        #[doc = " Perform gimbal accelerometer calibration."]
        pub async fn subscribe_calibrate_gimbal_accelerometer(
            &mut self,
            request: impl tonic::IntoRequest<super::SubscribeCalibrateGimbalAccelerometerRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::CalibrateGimbalAccelerometerResponse>>,
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
                "/mavsdk.rpc.calibration.CalibrationService/SubscribeCalibrateGimbalAccelerometer",
            );
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        #[doc = " Cancel ongoing calibration process."]
        pub async fn cancel(
            &mut self,
            request: impl tonic::IntoRequest<super::CancelRequest>,
        ) -> Result<tonic::Response<super::CancelResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.calibration.CalibrationService/Cancel",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
