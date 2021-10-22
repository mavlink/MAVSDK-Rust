#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetVisionPositionEstimateRequest {
    #[prost(message, optional, tag = "1")]
    pub vision_position_estimate: ::core::option::Option<VisionPositionEstimate>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetVisionPositionEstimateResponse {
    #[prost(message, optional, tag = "1")]
    pub mocap_result: ::core::option::Option<MocapResult>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetAttitudePositionMocapRequest {
    #[prost(message, optional, tag = "1")]
    pub attitude_position_mocap: ::core::option::Option<AttitudePositionMocap>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetAttitudePositionMocapResponse {
    #[prost(message, optional, tag = "1")]
    pub mocap_result: ::core::option::Option<MocapResult>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetOdometryRequest {
    #[prost(message, optional, tag = "1")]
    pub odometry: ::core::option::Option<Odometry>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetOdometryResponse {
    #[prost(message, optional, tag = "1")]
    pub mocap_result: ::core::option::Option<MocapResult>,
}
/// Global position/attitude estimate from a vision source.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VisionPositionEstimate {
    /// PositionBody frame timestamp UNIX Epoch time (0 to use Backend timestamp)
    #[prost(uint64, tag = "1")]
    pub time_usec: u64,
    /// Global position (m)
    #[prost(message, optional, tag = "2")]
    pub position_body: ::core::option::Option<PositionBody>,
    /// Body angle (rad).
    #[prost(message, optional, tag = "3")]
    pub angle_body: ::core::option::Option<AngleBody>,
    /// Pose cross-covariance matrix.
    #[prost(message, optional, tag = "4")]
    pub pose_covariance: ::core::option::Option<Covariance>,
}
/// Motion capture attitude and position
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AttitudePositionMocap {
    /// PositionBody frame timestamp UNIX Epoch time (0 to use Backend timestamp)
    #[prost(uint64, tag = "1")]
    pub time_usec: u64,
    /// Attitude quaternion (w, x, y, z order, zero-rotation is 1, 0, 0, 0)
    #[prost(message, optional, tag = "2")]
    pub q: ::core::option::Option<Quaternion>,
    /// Body Position (NED)
    #[prost(message, optional, tag = "3")]
    pub position_body: ::core::option::Option<PositionBody>,
    /// Pose cross-covariance matrix.
    #[prost(message, optional, tag = "4")]
    pub pose_covariance: ::core::option::Option<Covariance>,
}
/// Odometry message to communicate odometry information with an external interface.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Odometry {
    /// Timestamp (0 to use Backend timestamp).
    #[prost(uint64, tag = "1")]
    pub time_usec: u64,
    /// Coordinate frame of reference for the pose data.
    #[prost(enumeration = "odometry::MavFrame", tag = "2")]
    pub frame_id: i32,
    /// Body Position.
    #[prost(message, optional, tag = "3")]
    pub position_body: ::core::option::Option<PositionBody>,
    /// Quaternion components, w, x, y, z (1 0 0 0 is the null-rotation).
    #[prost(message, optional, tag = "4")]
    pub q: ::core::option::Option<Quaternion>,
    /// Linear speed (m/s).
    #[prost(message, optional, tag = "5")]
    pub speed_body: ::core::option::Option<SpeedBody>,
    /// Angular speed (rad/s).
    #[prost(message, optional, tag = "6")]
    pub angular_velocity_body: ::core::option::Option<AngularVelocityBody>,
    /// Pose cross-covariance matrix.
    #[prost(message, optional, tag = "7")]
    pub pose_covariance: ::core::option::Option<Covariance>,
    /// Velocity cross-covariance matrix.
    #[prost(message, optional, tag = "8")]
    pub velocity_covariance: ::core::option::Option<Covariance>,
}
/// Nested message and enum types in `Odometry`.
pub mod odometry {
    /// Mavlink frame id
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum MavFrame {
        /// MAVLink number: 14. Odometry local coordinate frame of data given by a motion capture system, Z-down (x: north, y: east, z: down).
        MocapNed = 0,
        /// MAVLink number: 20. Forward, Right, Down coordinate frame. This is a local frame with Z-down and arbitrary F/R alignment (i.e. not aligned with NED/earth frame). Replacement for MAV_FRAME_MOCAP_NED, MAV_FRAME_VISION_NED, MAV_FRAME_ESTIM_NED.
        LocalFrd = 1,
    }
}
/// Body position type
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PositionBody {
    /// X position in metres.
    #[prost(float, tag = "1")]
    pub x_m: f32,
    /// Y position in metres.
    #[prost(float, tag = "2")]
    pub y_m: f32,
    /// Z position in metres.
    #[prost(float, tag = "3")]
    pub z_m: f32,
}
/// Body angle type
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AngleBody {
    /// Roll angle in radians.
    #[prost(float, tag = "1")]
    pub roll_rad: f32,
    /// Pitch angle in radians.
    #[prost(float, tag = "2")]
    pub pitch_rad: f32,
    /// Yaw angle in radians.
    #[prost(float, tag = "3")]
    pub yaw_rad: f32,
}
/// Speed type, represented in the Body (X Y Z) frame and in metres/second.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpeedBody {
    /// Velocity in X in metres/second.
    #[prost(float, tag = "1")]
    pub x_m_s: f32,
    /// Velocity in Y in metres/second.
    #[prost(float, tag = "2")]
    pub y_m_s: f32,
    /// Velocity in Z in metres/second.
    #[prost(float, tag = "3")]
    pub z_m_s: f32,
}
/// Angular velocity type
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AngularVelocityBody {
    /// Roll angular velocity in radians/second.
    #[prost(float, tag = "1")]
    pub roll_rad_s: f32,
    /// Pitch angular velocity in radians/second.
    #[prost(float, tag = "2")]
    pub pitch_rad_s: f32,
    /// Yaw angular velocity in radians/second.
    #[prost(float, tag = "3")]
    pub yaw_rad_s: f32,
}
/// Covariance type.
/// Row-major representation of a 6x6 cross-covariance matrix
/// upper right triangle.
/// Set first to NaN if unknown.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Covariance {
    #[prost(float, repeated, tag = "1")]
    pub covariance_matrix: ::prost::alloc::vec::Vec<f32>,
}
///
/// Quaternion type.
///
/// All rotations and axis systems follow the right-hand rule.
/// The Hamilton quaternion product definition is used.
/// A zero-rotation quaternion is represented by (1,0,0,0).
/// The quaternion could also be written as w + xi + yj + zk.
///
/// For more info see: https://en.wikipedia.org/wiki/Quaternion
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
/// Result type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MocapResult {
    /// Result enum value
    #[prost(enumeration = "mocap_result::Result", tag = "1")]
    pub result: i32,
    /// Human-readable English string describing the result
    #[prost(string, tag = "2")]
    pub result_str: ::prost::alloc::string::String,
}
/// Nested message and enum types in `MocapResult`.
pub mod mocap_result {
    /// Possible results returned for mocap requests
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
        /// Invalid request data
        InvalidRequestData = 4,
    }
}
#[doc = r" Generated client implementations."]
pub mod mocap_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = "*"]
    #[doc = " Motion Capture allow vehicles to navigate when a global"]
    #[doc = " position source is unavailable or unreliable"]
    #[doc = " (e.g. indoors, or when flying under a bridge. etc.)."]
    #[derive(Debug, Clone)]
    pub struct MocapServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl MocapServiceClient<tonic::transport::Channel> {
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
    impl<T> MocapServiceClient<T>
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
        ) -> MocapServiceClient<InterceptedService<T, F>>
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
            MocapServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Send Global position/attitude estimate from a vision source."]
        pub async fn set_vision_position_estimate(
            &mut self,
            request: impl tonic::IntoRequest<super::SetVisionPositionEstimateRequest>,
        ) -> Result<tonic::Response<super::SetVisionPositionEstimateResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.mocap.MocapService/SetVisionPositionEstimate",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Send motion capture attitude and position."]
        pub async fn set_attitude_position_mocap(
            &mut self,
            request: impl tonic::IntoRequest<super::SetAttitudePositionMocapRequest>,
        ) -> Result<tonic::Response<super::SetAttitudePositionMocapResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.mocap.MocapService/SetAttitudePositionMocap",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Send odometry information with an external interface."]
        pub async fn set_odometry(
            &mut self,
            request: impl tonic::IntoRequest<super::SetOdometryRequest>,
        ) -> Result<tonic::Response<super::SetOdometryResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/mavsdk.rpc.mocap.MocapService/SetOdometry");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
