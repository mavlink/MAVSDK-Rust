#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TakePhotoRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TakePhotoResponse {
    #[prost(message, optional, tag = "1")]
    pub camera_result: ::core::option::Option<CameraResult>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StartPhotoIntervalRequest {
    /// Interval between photos (in seconds)
    #[prost(float, tag = "1")]
    pub interval_s: f32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StartPhotoIntervalResponse {
    #[prost(message, optional, tag = "1")]
    pub camera_result: ::core::option::Option<CameraResult>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StopPhotoIntervalRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StopPhotoIntervalResponse {
    #[prost(message, optional, tag = "1")]
    pub camera_result: ::core::option::Option<CameraResult>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StartVideoRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StartVideoResponse {
    #[prost(message, optional, tag = "1")]
    pub camera_result: ::core::option::Option<CameraResult>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StopVideoRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StopVideoResponse {
    #[prost(message, optional, tag = "1")]
    pub camera_result: ::core::option::Option<CameraResult>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StartVideoStreamingRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StartVideoStreamingResponse {
    #[prost(message, optional, tag = "1")]
    pub camera_result: ::core::option::Option<CameraResult>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StopVideoStreamingRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StopVideoStreamingResponse {
    #[prost(message, optional, tag = "1")]
    pub camera_result: ::core::option::Option<CameraResult>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetModeRequest {
    /// Camera mode to set
    #[prost(enumeration = "CameraMode", tag = "1")]
    pub camera_mode: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetModeResponse {
    #[prost(message, optional, tag = "1")]
    pub camera_result: ::core::option::Option<CameraResult>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeModeRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModeResponse {
    /// Camera mode
    #[prost(enumeration = "CameraMode", tag = "1")]
    pub camera_mode: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeVideoStreamInfoRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VideoStreamInfoResponse {
    /// Video stream info
    #[prost(message, optional, tag = "1")]
    pub video_stream_info: ::core::option::Option<VideoStreamInfo>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeCaptureInfoRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CaptureInfoResponse {
    /// Capture info
    #[prost(message, optional, tag = "1")]
    pub capture_info: ::core::option::Option<CaptureInfo>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeCameraStatusRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CameraStatusResponse {
    /// Camera status
    #[prost(message, optional, tag = "1")]
    pub camera_status: ::core::option::Option<CameraStatus>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeCurrentSettingsRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CurrentSettingsResponse {
    /// List of current settings
    #[prost(message, repeated, tag = "1")]
    pub current_settings: ::prost::alloc::vec::Vec<Setting>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribePossibleSettingOptionsRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PossibleSettingOptionsResponse {
    /// List of settings that can be changed
    #[prost(message, repeated, tag = "1")]
    pub setting_options: ::prost::alloc::vec::Vec<SettingOptions>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetSettingRequest {
    /// Desired setting
    #[prost(message, optional, tag = "1")]
    pub setting: ::core::option::Option<Setting>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetSettingResponse {
    #[prost(message, optional, tag = "1")]
    pub camera_result: ::core::option::Option<CameraResult>,
}
/// Result type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CameraResult {
    /// Result enum value
    #[prost(enumeration = "camera_result::Result", tag = "1")]
    pub result: i32,
    /// Human-readable English string describing the result
    #[prost(string, tag = "2")]
    pub result_str: ::prost::alloc::string::String,
}
/// Nested message and enum types in `CameraResult`.
pub mod camera_result {
    /// Possible results returned for camera commands
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Result {
        /// Unknown error
        Unknown = 0,
        /// Command executed successfully
        Success = 1,
        /// Command in progress
        InProgress = 2,
        /// Camera is busy and rejected command
        Busy = 3,
        /// Camera denied the command
        Denied = 4,
        /// An error has occured while executing the command
        Error = 5,
        /// Command timed out
        Timeout = 6,
        /// Command has wrong argument(s)
        WrongArgument = 7,
    }
}
/// Information about a picture just captured.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CaptureInfo {
    /// Location where the picture was taken
    #[prost(message, optional, tag = "1")]
    pub position: ::core::option::Option<Position>,
    /// Attitude of the camera when the picture was taken (quaternion)
    #[prost(message, optional, tag = "2")]
    pub attitude_quaternion: ::core::option::Option<Quaternion>,
    /// Attitude of the camera when the picture was taken (euler angle)
    #[prost(message, optional, tag = "3")]
    pub attitude_euler_angle: ::core::option::Option<EulerAngle>,
    /// Timestamp in UTC (since UNIX epoch) in microseconds
    #[prost(uint64, tag = "4")]
    pub time_utc_us: u64,
    /// True if the capture was successful
    #[prost(bool, tag = "5")]
    pub is_success: bool,
    /// Zero-based index of this image since vehicle was armed
    #[prost(int32, tag = "6")]
    pub index: i32,
    /// Download URL of this image
    #[prost(string, tag = "7")]
    pub file_url: ::prost::alloc::string::String,
}
/// Position type in global coordinates.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Position {
    /// Latitude in degrees (range: -90 to +90)
    #[prost(double, tag = "1")]
    pub latitude_deg: f64,
    /// Longitude in degrees (range: -180 to +180)
    #[prost(double, tag = "2")]
    pub longitude_deg: f64,
    /// Altitude AMSL (above mean sea level) in metres
    #[prost(float, tag = "3")]
    pub absolute_altitude_m: f32,
    /// Altitude relative to takeoff altitude in metres
    #[prost(float, tag = "4")]
    pub relative_altitude_m: f32,
}
///
/// Quaternion type.
///
/// All rotations and axis systems follow the right-hand rule.
/// The Hamilton quaternion product definition is used.
/// A zero-rotation quaternion is represented by (1,0,0,0).
/// The quaternion could also be written as w + xi + yj + zk.
///
/// For more info see: <https://en.wikipedia.org/wiki/Quaternion>
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Quaternion {
    /// Quaternion entry 0, also denoted as a
    #[prost(float, tag = "1")]
    pub w: f32,
    /// Quaternion entry 1, also denoted as b
    #[prost(float, tag = "2")]
    pub x: f32,
    /// Quaternion entry 2, also denoted as c
    #[prost(float, tag = "3")]
    pub y: f32,
    /// Quaternion entry 3, also denoted as d
    #[prost(float, tag = "4")]
    pub z: f32,
}
///
/// Euler angle type.
///
/// All rotations and axis systems follow the right-hand rule.
/// The Euler angles follow the convention of a 3-2-1 intrinsic Tait-Bryan rotation sequence.
///
/// For more info see <https://en.wikipedia.org/wiki/Euler_angles>
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EulerAngle {
    /// Roll angle in degrees, positive is banking to the right
    #[prost(float, tag = "1")]
    pub roll_deg: f32,
    /// Pitch angle in degrees, positive is pitching nose up
    #[prost(float, tag = "2")]
    pub pitch_deg: f32,
    /// Yaw angle in degrees, positive is clock-wise seen from above
    #[prost(float, tag = "3")]
    pub yaw_deg: f32,
}
/// Type for video stream settings.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VideoStreamSettings {
    /// Frames per second
    #[prost(float, tag = "1")]
    pub frame_rate_hz: f32,
    /// Horizontal resolution (in pixels)
    #[prost(uint32, tag = "2")]
    pub horizontal_resolution_pix: u32,
    /// Vertical resolution (in pixels)
    #[prost(uint32, tag = "3")]
    pub vertical_resolution_pix: u32,
    /// Bit rate (in bits per second)
    #[prost(uint32, tag = "4")]
    pub bit_rate_b_s: u32,
    /// Video image rotation (clockwise, 0-359 degrees)
    #[prost(uint32, tag = "5")]
    pub rotation_deg: u32,
    /// Video stream URI
    #[prost(string, tag = "6")]
    pub uri: ::prost::alloc::string::String,
}
/// Information about the video stream.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VideoStreamInfo {
    /// Video stream settings
    #[prost(message, optional, tag = "1")]
    pub video_stream_settings: ::core::option::Option<VideoStreamSettings>,
    /// Current status of video streaming
    #[prost(enumeration = "video_stream_info::VideoStreamStatus", tag = "2")]
    pub video_stream_status: i32,
}
/// Nested message and enum types in `VideoStreamInfo`.
pub mod video_stream_info {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum VideoStreamStatus {
        /// Video stream is not running.
        NotRunning = 0,
        /// Video stream is running.
        InProgress = 1,
    }
}
/// Information about the camera status.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CameraStatus {
    #[prost(bool, tag = "1")]
    pub video_on: bool,
    #[prost(bool, tag = "2")]
    pub photo_interval_on: bool,
    /// Used storage (in MiB)
    #[prost(float, tag = "3")]
    pub used_storage_mib: f32,
    /// Available storage (in MiB)
    #[prost(float, tag = "4")]
    pub available_storage_mib: f32,
    /// Total storage (in MiB)
    #[prost(float, tag = "5")]
    pub total_storage_mib: f32,
    /// Elapsed time since starting the video recording (in seconds)
    #[prost(float, tag = "6")]
    pub recording_time_s: f32,
    /// Current folder name where media are saved
    #[prost(string, tag = "7")]
    pub media_folder_name: ::prost::alloc::string::String,
    /// Storage status
    #[prost(enumeration = "camera_status::StorageStatus", tag = "8")]
    pub storage_status: i32,
}
/// Nested message and enum types in `CameraStatus`.
pub mod camera_status {
    /// Storage status type.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum StorageStatus {
        /// Status not available
        NotAvailable = 0,
        /// Storage is not formatted (i.e. has no recognized file system)
        Unformatted = 1,
        /// Storage is formatted (i.e. has recognized a file system)
        Formatted = 2,
    }
}
/// Type to represent a setting with a selected option.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Setting {
    /// Name of a setting (machine readable)
    #[prost(string, tag = "1")]
    pub setting_id: ::prost::alloc::string::String,
    /// Description of the setting (human readable)
    #[prost(string, tag = "2")]
    pub setting_description: ::prost::alloc::string::String,
    /// Selected option
    #[prost(message, optional, tag = "3")]
    pub option: ::core::option::Option<Option>,
}
/// Type to represent a setting option.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Option {
    /// Name of the option (machine readable)
    #[prost(string, tag = "1")]
    pub option_id: ::prost::alloc::string::String,
    /// Description of the option (human readable)
    #[prost(string, tag = "2")]
    pub option_description: ::prost::alloc::string::String,
}
/// Type to represent a setting with a list of options to choose from.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SettingOptions {
    /// Name of the setting (machine readable)
    #[prost(string, tag = "1")]
    pub setting_id: ::prost::alloc::string::String,
    /// Description of the setting (human readable)
    #[prost(string, tag = "2")]
    pub setting_description: ::prost::alloc::string::String,
    /// List of options
    #[prost(message, repeated, tag = "3")]
    pub options: ::prost::alloc::vec::Vec<Option>,
}
/// Camera mode type.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CameraMode {
    /// Unknown
    Unknown = 0,
    /// Photo mode
    Photo = 1,
    /// Video mode
    Video = 2,
}
#[doc = r" Generated client implementations."]
pub mod camera_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = ""]
    #[doc = " Can be used to manage cameras that implement the MAVLink"]
    #[doc = " Camera Protocol: https://mavlink.io/en/protocol/camera.html."]
    #[doc = ""]
    #[doc = " Currently only a single camera is supported."]
    #[doc = " When multiple cameras are supported the plugin will need to be"]
    #[doc = " instantiated separately for every camera and the camera selected using"]
    #[doc = " `select_camera`."]
    #[derive(Debug, Clone)]
    pub struct CameraServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl CameraServiceClient<tonic::transport::Channel> {
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
    impl<T> CameraServiceClient<T>
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
        ) -> CameraServiceClient<InterceptedService<T, F>>
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
            CameraServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Take one photo."]
        pub async fn take_photo(
            &mut self,
            request: impl tonic::IntoRequest<super::TakePhotoRequest>,
        ) -> Result<tonic::Response<super::TakePhotoResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/mavsdk.rpc.camera.CameraService/TakePhoto");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = " Start photo timelapse with a given interval."]
        pub async fn start_photo_interval(
            &mut self,
            request: impl tonic::IntoRequest<super::StartPhotoIntervalRequest>,
        ) -> Result<tonic::Response<super::StartPhotoIntervalResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.camera.CameraService/StartPhotoInterval",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = " Stop a running photo timelapse."]
        pub async fn stop_photo_interval(
            &mut self,
            request: impl tonic::IntoRequest<super::StopPhotoIntervalRequest>,
        ) -> Result<tonic::Response<super::StopPhotoIntervalResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.camera.CameraService/StopPhotoInterval",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = " Start a video recording."]
        pub async fn start_video(
            &mut self,
            request: impl tonic::IntoRequest<super::StartVideoRequest>,
        ) -> Result<tonic::Response<super::StartVideoResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/mavsdk.rpc.camera.CameraService/StartVideo");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = " Stop a running video recording."]
        pub async fn stop_video(
            &mut self,
            request: impl tonic::IntoRequest<super::StopVideoRequest>,
        ) -> Result<tonic::Response<super::StopVideoResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/mavsdk.rpc.camera.CameraService/StopVideo");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = " Start video streaming."]
        pub async fn start_video_streaming(
            &mut self,
            request: impl tonic::IntoRequest<super::StartVideoStreamingRequest>,
        ) -> Result<tonic::Response<super::StartVideoStreamingResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.camera.CameraService/StartVideoStreaming",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = " Stop current video streaming."]
        pub async fn stop_video_streaming(
            &mut self,
            request: impl tonic::IntoRequest<super::StopVideoStreamingRequest>,
        ) -> Result<tonic::Response<super::StopVideoStreamingResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.camera.CameraService/StopVideoStreaming",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = " Set camera mode."]
        pub async fn set_mode(
            &mut self,
            request: impl tonic::IntoRequest<super::SetModeRequest>,
        ) -> Result<tonic::Response<super::SetModeResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/mavsdk.rpc.camera.CameraService/SetMode");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = " Subscribe to camera mode updates."]
        pub async fn subscribe_mode(
            &mut self,
            request: impl tonic::IntoRequest<super::SubscribeModeRequest>,
        ) -> Result<tonic::Response<tonic::codec::Streaming<super::ModeResponse>>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.camera.CameraService/SubscribeMode",
            );
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        #[doc = ""]
        #[doc = " Subscribe to video stream info updates."]
        pub async fn subscribe_video_stream_info(
            &mut self,
            request: impl tonic::IntoRequest<super::SubscribeVideoStreamInfoRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::VideoStreamInfoResponse>>,
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
                "/mavsdk.rpc.camera.CameraService/SubscribeVideoStreamInfo",
            );
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        #[doc = ""]
        #[doc = " Subscribe to capture info updates."]
        pub async fn subscribe_capture_info(
            &mut self,
            request: impl tonic::IntoRequest<super::SubscribeCaptureInfoRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::CaptureInfoResponse>>,
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
                "/mavsdk.rpc.camera.CameraService/SubscribeCaptureInfo",
            );
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        #[doc = ""]
        #[doc = " Subscribe to camera status updates."]
        pub async fn subscribe_camera_status(
            &mut self,
            request: impl tonic::IntoRequest<super::SubscribeCameraStatusRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::CameraStatusResponse>>,
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
                "/mavsdk.rpc.camera.CameraService/SubscribeCameraStatus",
            );
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        #[doc = ""]
        #[doc = " Get the list of current camera settings."]
        pub async fn subscribe_current_settings(
            &mut self,
            request: impl tonic::IntoRequest<super::SubscribeCurrentSettingsRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::CurrentSettingsResponse>>,
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
                "/mavsdk.rpc.camera.CameraService/SubscribeCurrentSettings",
            );
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        #[doc = ""]
        #[doc = " Get the list of settings that can be changed."]
        pub async fn subscribe_possible_setting_options(
            &mut self,
            request: impl tonic::IntoRequest<super::SubscribePossibleSettingOptionsRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::PossibleSettingOptionsResponse>>,
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
                "/mavsdk.rpc.camera.CameraService/SubscribePossibleSettingOptions",
            );
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        #[doc = ""]
        #[doc = " Set a setting to some value."]
        pub async fn set_setting(
            &mut self,
            request: impl tonic::IntoRequest<super::SetSettingRequest>,
        ) -> Result<tonic::Response<super::SetSettingResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/mavsdk.rpc.camera.CameraService/SetSetting");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
