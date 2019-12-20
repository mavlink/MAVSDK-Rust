#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UploadMissionRequest {
    /// List of mission items
    #[prost(message, repeated, tag = "1")]
    pub mission_items: ::std::vec::Vec<MissionItem>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UploadMissionResponse {
    #[prost(message, optional, tag = "1")]
    pub mission_result: ::std::option::Option<MissionResult>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CancelMissionUploadRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CancelMissionUploadResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DownloadMissionRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DownloadMissionResponse {
    #[prost(message, optional, tag = "1")]
    pub mission_result: ::std::option::Option<MissionResult>,
    /// List of downloaded mission items
    #[prost(message, repeated, tag = "2")]
    pub mission_items: ::std::vec::Vec<MissionItem>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CancelMissionDownloadRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CancelMissionDownloadResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StartMissionRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StartMissionResponse {
    #[prost(message, optional, tag = "1")]
    pub mission_result: ::std::option::Option<MissionResult>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PauseMissionRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PauseMissionResponse {
    #[prost(message, optional, tag = "1")]
    pub mission_result: ::std::option::Option<MissionResult>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClearMissionRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClearMissionResponse {
    #[prost(message, optional, tag = "1")]
    pub mission_result: ::std::option::Option<MissionResult>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetCurrentMissionItemIndexRequest {
    /// Index of the mission item to be set as the next one (0-based)
    #[prost(int32, tag = "1")]
    pub index: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetCurrentMissionItemIndexResponse {
    #[prost(message, optional, tag = "1")]
    pub mission_result: ::std::option::Option<MissionResult>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IsMissionFinishedRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IsMissionFinishedResponse {
    /// True if the mission is finished and the last mission item has been reached
    #[prost(bool, tag = "1")]
    pub is_finished: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeMissionProgressRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MissionProgressResponse {
    /// Mission progress
    #[prost(message, optional, tag = "1")]
    pub mission_progress: ::std::option::Option<MissionProgress>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetReturnToLaunchAfterMissionRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetReturnToLaunchAfterMissionResponse {
    /// If true, trigger an RTL at the end of the mission
    #[prost(bool, tag = "1")]
    pub enable: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetReturnToLaunchAfterMissionRequest {
    /// If true, trigger an RTL at the end of the mission
    #[prost(bool, tag = "1")]
    pub enable: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetReturnToLaunchAfterMissionResponse {}
///
/// Type representing a mission item.
///
/// A MissionItem can contain a position and/or actions.
/// Mission items are building blocks to assemble a mission,
/// which can be sent to (or received from) a system.
/// They cannot be used independently.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MissionItem {
    /// Latitude in degrees (range: -90 to +90)
    #[prost(double, tag = "1")]
    pub latitude_deg: f64,
    /// Longitude in degrees (range: -180 to +180)
    #[prost(double, tag = "2")]
    pub longitude_deg: f64,
    /// Altitude relative to takeoff altitude in metres
    #[prost(float, tag = "3")]
    pub relative_altitude_m: f32,
    /// Speed to use after this mission item (in metres/second)
    #[prost(float, tag = "4")]
    pub speed_m_s: f32,
    /// True will make the drone fly through without stopping, while false will make the drone stop on the waypoint
    #[prost(bool, tag = "5")]
    pub is_fly_through: bool,
    /// Gimbal pitch (in degrees)
    #[prost(float, tag = "6")]
    pub gimbal_pitch_deg: f32,
    /// Gimbal yaw (in degrees)
    #[prost(float, tag = "7")]
    pub gimbal_yaw_deg: f32,
    /// Camera action to trigger at this mission item
    #[prost(enumeration = "mission_item::CameraAction", tag = "8")]
    pub camera_action: i32,
    /// Loiter time (in seconds)
    #[prost(float, tag = "9")]
    pub loiter_time_s: f32,
    /// Camera photo interval to use after this mission item (in seconds)
    #[prost(double, tag = "10")]
    pub camera_photo_interval_s: f64,
}
pub mod mission_item {
    /// Possible camera actions at a mission item.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum CameraAction {
        /// No action
        None = 0,
        /// Take a single photo
        TakePhoto = 1,
        /// Start capturing photos at regular intervals
        StartPhotoInterval = 2,
        /// Stop capturing photos at regular intervals
        StopPhotoInterval = 3,
        /// Start capturing video
        StartVideo = 4,
        /// Stop capturing video
        StopVideo = 5,
    }
}
/// Mission progress type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MissionProgress {
    /// Current mission item index (0-based)
    #[prost(int32, tag = "1")]
    pub current_item_index: i32,
    /// Total number of mission items
    #[prost(int32, tag = "2")]
    pub mission_count: i32,
}
/// Result type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MissionResult {
    /// Result enum value
    #[prost(enumeration = "mission_result::Result", tag = "1")]
    pub result: i32,
    /// Human-readable English string describing the result
    #[prost(string, tag = "2")]
    pub result_str: std::string::String,
}
pub mod mission_result {
    /// Possible results returned for action requests.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Result {
        /// Unknown error
        Unknown = 0,
        /// Request succeeded
        Success = 1,
        /// Error
        Error = 2,
        /// Too many mission items in the mission
        TooManyMissionItems = 3,
        /// Vehicle is busy
        Busy = 4,
        /// Request timed out
        Timeout = 5,
        /// Invalid argument
        InvalidArgument = 6,
        /// Mission downloaded from the system is not supported
        Unsupported = 7,
        /// No mission available on the system
        NoMissionAvailable = 8,
        /// Failed to open the QGroundControl plan
        FailedToOpenQgcPlan = 9,
        /// Failed to parse the QGroundControl plan
        FailedToParseQgcPlan = 10,
        /// Unsupported mission command
        UnsupportedMissionCmd = 11,
        /// Mission transfer (upload or download) has been cancelled
        TransferCancelled = 12,
    }
}
#[doc = r" Generated server implementations."]
pub mod mission_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Enable waypoint missions."]
    pub struct MissionServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl MissionServiceClient<tonic::transport::Channel> {
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
    impl<T> MissionServiceClient<T>
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
        #[doc = ""]
        #[doc = " Upload a list of mission items to the system."]
        #[doc = ""]
        #[doc = " The mission items are uploaded to a drone. Once uploaded the mission can be started and"]
        #[doc = " executed even if the connection is lost."]
        pub async fn upload_mission(
            &mut self,
            request: impl tonic::IntoRequest<super::UploadMissionRequest>,
        ) -> Result<tonic::Response<super::UploadMissionResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.mission.MissionService/UploadMission",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = " Cancel an ongoing mission upload."]
        pub async fn cancel_mission_upload(
            &mut self,
            request: impl tonic::IntoRequest<super::CancelMissionUploadRequest>,
        ) -> Result<tonic::Response<super::CancelMissionUploadResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.mission.MissionService/CancelMissionUpload",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = " Download a list of mission items from the system (asynchronous)."]
        #[doc = ""]
        #[doc = " Will fail if any of the downloaded mission items are not supported"]
        #[doc = " by the MAVSDK API."]
        pub async fn download_mission(
            &mut self,
            request: impl tonic::IntoRequest<super::DownloadMissionRequest>,
        ) -> Result<tonic::Response<super::DownloadMissionResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.mission.MissionService/DownloadMission",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = " Cancel an ongoing mission download."]
        pub async fn cancel_mission_download(
            &mut self,
            request: impl tonic::IntoRequest<super::CancelMissionDownloadRequest>,
        ) -> Result<tonic::Response<super::CancelMissionDownloadResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.mission.MissionService/CancelMissionDownload",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = " Start the mission."]
        #[doc = ""]
        #[doc = " A mission must be uploaded to the vehicle before this can be called."]
        pub async fn start_mission(
            &mut self,
            request: impl tonic::IntoRequest<super::StartMissionRequest>,
        ) -> Result<tonic::Response<super::StartMissionResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.mission.MissionService/StartMission",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = " Pause the mission."]
        #[doc = ""]
        #[doc = " Pausing the mission puts the vehicle into"]
        #[doc = " [HOLD mode](https://docs.px4.io/en/flight_modes/hold.html)."]
        #[doc = " A multicopter should just hover at the spot while a fixedwing vehicle should loiter"]
        #[doc = " around the location where it paused."]
        pub async fn pause_mission(
            &mut self,
            request: impl tonic::IntoRequest<super::PauseMissionRequest>,
        ) -> Result<tonic::Response<super::PauseMissionResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.mission.MissionService/PauseMission",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = " Clear the mission saved on the vehicle."]
        pub async fn clear_mission(
            &mut self,
            request: impl tonic::IntoRequest<super::ClearMissionRequest>,
        ) -> Result<tonic::Response<super::ClearMissionResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.mission.MissionService/ClearMission",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = " Sets the mission item index to go to."]
        #[doc = ""]
        #[doc = " By setting the current index to 0, the mission is restarted from the beginning. If it is set"]
        #[doc = " to a specific index of a mission item, the mission will be set to this item."]
        #[doc = ""]
        #[doc = " Note that this is not necessarily true for general missions using MAVLink if loop counters"]
        #[doc = " are used."]
        pub async fn set_current_mission_item_index(
            &mut self,
            request: impl tonic::IntoRequest<super::SetCurrentMissionItemIndexRequest>,
        ) -> Result<tonic::Response<super::SetCurrentMissionItemIndexResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.mission.MissionService/SetCurrentMissionItemIndex",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = " Check if the mission has been finished."]
        pub async fn is_mission_finished(
            &mut self,
            request: impl tonic::IntoRequest<super::IsMissionFinishedRequest>,
        ) -> Result<tonic::Response<super::IsMissionFinishedResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.mission.MissionService/IsMissionFinished",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = " Subscribe to mission progress updates."]
        pub async fn subscribe_mission_progress(
            &mut self,
            request: impl tonic::IntoRequest<super::SubscribeMissionProgressRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::MissionProgressResponse>>,
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
                "/mavsdk.rpc.mission.MissionService/SubscribeMissionProgress",
            );
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        #[doc = ""]
        #[doc = " Get whether to trigger Return-to-Launch (RTL) after mission is complete."]
        #[doc = ""]
        #[doc = " Before getting this option, it needs to be set, or a mission"]
        #[doc = " needs to be downloaded."]
        pub async fn get_return_to_launch_after_mission(
            &mut self,
            request: impl tonic::IntoRequest<super::GetReturnToLaunchAfterMissionRequest>,
        ) -> Result<tonic::Response<super::GetReturnToLaunchAfterMissionResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.mission.MissionService/GetReturnToLaunchAfterMission",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = " Set whether to trigger Return-to-Launch (RTL) after the mission is complete."]
        #[doc = ""]
        #[doc = " This will only take effect for the next mission upload, meaning that"]
        #[doc = " the mission may have to be uploaded again."]
        pub async fn set_return_to_launch_after_mission(
            &mut self,
            request: impl tonic::IntoRequest<super::SetReturnToLaunchAfterMissionRequest>,
        ) -> Result<tonic::Response<super::SetReturnToLaunchAfterMissionResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.mission.MissionService/SetReturnToLaunchAfterMission",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for MissionServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
}
