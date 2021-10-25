#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribePositionRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PositionResponse {
    /// The next position
    #[prost(message, optional, tag = "1")]
    pub position: ::core::option::Option<Position>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeHomeRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HomeResponse {
    /// The next home position
    #[prost(message, optional, tag = "1")]
    pub home: ::core::option::Option<Position>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeInAirRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InAirResponse {
    /// The next 'in-air' state
    #[prost(bool, tag = "1")]
    pub is_in_air: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeLandedStateRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LandedStateResponse {
    #[prost(enumeration = "LandedState", tag = "1")]
    pub landed_state: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeArmedRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ArmedResponse {
    /// The next 'armed' state
    #[prost(bool, tag = "1")]
    pub is_armed: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeAttitudeQuaternionRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AttitudeQuaternionResponse {
    /// The next attitude (quaternion)
    #[prost(message, optional, tag = "1")]
    pub attitude_quaternion: ::core::option::Option<Quaternion>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeAttitudeEulerRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AttitudeEulerResponse {
    /// The next attitude (euler)
    #[prost(message, optional, tag = "1")]
    pub attitude_euler: ::core::option::Option<EulerAngle>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeAttitudeAngularVelocityBodyRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AttitudeAngularVelocityBodyResponse {
    /// The next angular velocity (rad/s)
    #[prost(message, optional, tag = "1")]
    pub attitude_angular_velocity_body: ::core::option::Option<AngularVelocityBody>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeCameraAttitudeQuaternionRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CameraAttitudeQuaternionResponse {
    /// The next camera attitude (quaternion)
    #[prost(message, optional, tag = "1")]
    pub attitude_quaternion: ::core::option::Option<Quaternion>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeCameraAttitudeEulerRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CameraAttitudeEulerResponse {
    /// The next camera attitude (euler)
    #[prost(message, optional, tag = "1")]
    pub attitude_euler: ::core::option::Option<EulerAngle>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeGroundSpeedNedRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroundSpeedNedResponse {
    /// The next ground speed (NED)
    #[prost(message, optional, tag = "1")]
    pub ground_speed_ned: ::core::option::Option<SpeedNed>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeGpsInfoRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GpsInfoResponse {
    /// The next 'GPS info' state
    #[prost(message, optional, tag = "1")]
    pub gps_info: ::core::option::Option<GpsInfo>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeBatteryRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatteryResponse {
    /// The next 'battery' state
    #[prost(message, optional, tag = "1")]
    pub battery: ::core::option::Option<Battery>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeFlightModeRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FlightModeResponse {
    /// The next flight mode
    #[prost(enumeration = "FlightMode", tag = "1")]
    pub flight_mode: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeHealthRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HealthResponse {
    /// The next 'health' state
    #[prost(message, optional, tag = "1")]
    pub health: ::core::option::Option<Health>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeRcStatusRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RcStatusResponse {
    /// The next RC status
    #[prost(message, optional, tag = "1")]
    pub rc_status: ::core::option::Option<RcStatus>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeStatusTextRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StatusTextResponse {
    /// The next 'status text'
    #[prost(message, optional, tag = "1")]
    pub status_text: ::core::option::Option<StatusText>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeActuatorControlTargetRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActuatorControlTargetResponse {
    /// Actuator control target
    #[prost(message, optional, tag = "1")]
    pub actuator_control_target: ::core::option::Option<ActuatorControlTarget>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeActuatorOutputStatusRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActuatorOutputStatusResponse {
    /// Actuator output status
    #[prost(message, optional, tag = "1")]
    pub actuator_output_status: ::core::option::Option<ActuatorOutputStatus>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeOdometryRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OdometryResponse {
    /// Odometry
    #[prost(message, optional, tag = "1")]
    pub odometry: ::core::option::Option<Odometry>,
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
/// Angular velocity type
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AngularVelocityBody {
    /// Roll angular velocity
    #[prost(float, tag = "1")]
    pub roll_rad_s: f32,
    /// Pitch angular velocity
    #[prost(float, tag = "2")]
    pub pitch_rad_s: f32,
    /// Yaw angular velocity
    #[prost(float, tag = "3")]
    pub yaw_rad_s: f32,
}
/// Speed type, represented in the NED (North East Down) frame and in metres/second.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpeedNed {
    /// Velocity in North direction in metres/second
    #[prost(float, tag = "1")]
    pub velocity_north_m_s: f32,
    /// Velocity in East direction in metres/second
    #[prost(float, tag = "2")]
    pub velocity_east_m_s: f32,
    /// Velocity in Down direction in metres/second
    #[prost(float, tag = "3")]
    pub velocity_down_m_s: f32,
}
/// GPS information type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GpsInfo {
    /// Number of visible satellites in use
    #[prost(int32, tag = "1")]
    pub num_satellites: i32,
    /// Fix type
    #[prost(enumeration = "FixType", tag = "2")]
    pub fix_type: i32,
}
/// Battery type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Battery {
    /// Voltage in volts
    #[prost(float, tag = "1")]
    pub voltage_v: f32,
    /// Estimated battery remaining (range: 0.0 to 1.0)
    #[prost(float, tag = "2")]
    pub remaining_percent: f32,
}
/// Health type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Health {
    /// True if the gyrometer is calibrated
    #[prost(bool, tag = "1")]
    pub is_gyrometer_calibration_ok: bool,
    /// True if the accelerometer is calibrated
    #[prost(bool, tag = "2")]
    pub is_accelerometer_calibration_ok: bool,
    /// True if the magnetometer is calibrated
    #[prost(bool, tag = "3")]
    pub is_magnetometer_calibration_ok: bool,
    /// True if the vehicle has a valid level calibration
    #[prost(bool, tag = "4")]
    pub is_level_calibration_ok: bool,
    /// True if the local position estimate is good enough to fly in 'position control' mode
    #[prost(bool, tag = "5")]
    pub is_local_position_ok: bool,
    /// True if the global position estimate is good enough to fly in 'position control' mode
    #[prost(bool, tag = "6")]
    pub is_global_position_ok: bool,
    /// True if the home position has been initialized properly
    #[prost(bool, tag = "7")]
    pub is_home_position_ok: bool,
}
/// Remote control status type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RcStatus {
    /// True if an RC signal has been available once
    #[prost(bool, tag = "1")]
    pub was_available_once: bool,
    /// True if the RC signal is available now
    #[prost(bool, tag = "2")]
    pub is_available: bool,
    /// Signal strength (range: 0 to 100)
    #[prost(float, tag = "3")]
    pub signal_strength_percent: f32,
}
/// StatusText information type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StatusText {
    /// Message type
    #[prost(enumeration = "status_text::StatusType", tag = "1")]
    pub r#type: i32,
    /// MAVLink status message
    #[prost(string, tag = "2")]
    pub text: ::prost::alloc::string::String,
}
/// Nested message and enum types in `StatusText`.
pub mod status_text {
    /// Status types.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum StatusType {
        /// Information or other
        Info = 0,
        /// Warning
        Warning = 1,
        /// Critical
        Critical = 2,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActuatorControlTarget {
    #[prost(int32, tag = "1")]
    pub group: i32,
    #[prost(float, repeated, tag = "2")]
    pub controls: ::prost::alloc::vec::Vec<f32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActuatorOutputStatus {
    #[prost(uint32, tag = "1")]
    pub active: u32,
    #[prost(float, repeated, tag = "2")]
    pub actuator: ::prost::alloc::vec::Vec<f32>,
}
/// Odometry message type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Odometry {
    /// Timestamp (0 to use Backend timestamp).
    #[prost(uint64, tag = "1")]
    pub time_usec: u64,
    /// Coordinate frame of reference for the pose data.
    #[prost(enumeration = "odometry::MavFrame", tag = "2")]
    pub frame_id: i32,
    /// Coordinate frame of reference for the velocity in free space (twist) data.
    #[prost(enumeration = "odometry::MavFrame", tag = "3")]
    pub child_frame_id: i32,
    /// Position.
    #[prost(message, optional, tag = "4")]
    pub position_body: ::core::option::Option<PositionBody>,
    /// Quaternion components, w, x, y, z (1 0 0 0 is the null-rotation).
    #[prost(message, optional, tag = "5")]
    pub q: ::core::option::Option<Quaternion>,
    /// Linear speed (m/s).
    #[prost(message, optional, tag = "6")]
    pub speed_body: ::core::option::Option<SpeedBody>,
    /// Angular speed (rad/s).
    #[prost(message, optional, tag = "7")]
    pub angular_velocity_body: ::core::option::Option<AngularVelocityBody>,
    /// Pose cross-covariance matrix.
    #[prost(message, optional, tag = "8")]
    pub pose_covariance: ::core::option::Option<Covariance>,
    /// Velocity cross-covariance matrix.
    #[prost(message, optional, tag = "9")]
    pub velocity_covariance: ::core::option::Option<Covariance>,
}
/// Nested message and enum types in `Odometry`.
pub mod odometry {
    /// Mavlink frame id
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum MavFrame {
        Undef = 0,
        /// Setpoint in body NED frame. This makes sense if all position control is externalized - e.g. useful to command 2 m/s^2 acceleration to the right.
        BodyNed = 8,
        /// Odometry local coordinate frame of data given by a vision estimation system, Z-down (x: north, y: east, z: down).
        VisionNed = 16,
        /// Odometry local coordinate frame of data given by an estimator running onboard the vehicle, Z-down (x: north, y: east, z: down).
        EstimNed = 18,
    }
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
/// Speed type, represented in the Body (X Y Z) frame and in metres/second.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpeedBody {
    /// Velocity in X in metres/second
    #[prost(float, tag = "1")]
    pub velocity_x_m_s: f32,
    /// Velocity in Y in metres/second
    #[prost(float, tag = "2")]
    pub velocity_y_m_s: f32,
    /// Velocity in Z in metres/second
    #[prost(float, tag = "3")]
    pub velocity_z_m_s: f32,
}
/// Position type, represented in the Body (X Y Z) frame
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PositionBody {
    /// X Position in metres.
    #[prost(float, tag = "1")]
    pub x_m: f32,
    /// Y Position in metres.
    #[prost(float, tag = "2")]
    pub y_m: f32,
    /// Z Position in metres.
    #[prost(float, tag = "3")]
    pub z_m: f32,
}
/// Fix type.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum FixType {
    /// No GPS connected
    NoGps = 0,
    /// No position information, GPS is connected
    NoFix = 1,
    /// 2D position
    Fix2d = 2,
    /// 3D position
    Fix3d = 3,
    /// DGPS/SBAS aided 3D position
    FixDgps = 4,
    /// RTK float, 3D position
    RtkFloat = 5,
    /// RTK Fixed, 3D position
    RtkFixed = 6,
}
///
/// Flight modes.
///
/// For more information about flight modes, check out
/// <https://docs.px4.io/en/config/flight_mode.html.>
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum FlightMode {
    /// Mode not known
    Unknown = 0,
    /// Armed and ready to take off
    Ready = 1,
    /// Taking off
    Takeoff = 2,
    /// Holding (hovering in place (or circling for fixed-wing vehicles)
    Hold = 3,
    /// In mission
    Mission = 4,
    /// Returning to launch position (then landing)
    ReturnToLaunch = 5,
    /// Landing
    Land = 6,
    /// In 'offboard' mode
    Offboard = 7,
    /// In 'follow-me' mode
    FollowMe = 8,
}
/// Landed State enumeration
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum LandedState {
    Unknown = 0,
    OnGround = 1,
    InAir = 2,
    TakingOff = 3,
    Landing = 4,
}
#[doc = r" Generated client implementations."]
pub mod telemetry_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = ""]
    #[doc = " Allow users to get vehicle telemetry and state information"]
    #[doc = " (e.g. battery, GPS, RC connection, flight mode etc.) and set telemetry update rates."]
    #[derive(Debug, Clone)]
    pub struct TelemetryServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl TelemetryServiceClient<tonic::transport::Channel> {
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
    impl<T> TelemetryServiceClient<T>
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
        ) -> TelemetryServiceClient<InterceptedService<T, F>>
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
            TelemetryServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Subscribe to 'position' updates."]
        pub async fn subscribe_position(
            &mut self,
            request: impl tonic::IntoRequest<super::SubscribePositionRequest>,
        ) -> Result<tonic::Response<tonic::codec::Streaming<super::PositionResponse>>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.telemetry.TelemetryService/SubscribePosition",
            );
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        #[doc = " Subscribe to 'home position' updates."]
        pub async fn subscribe_home(
            &mut self,
            request: impl tonic::IntoRequest<super::SubscribeHomeRequest>,
        ) -> Result<tonic::Response<tonic::codec::Streaming<super::HomeResponse>>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.telemetry.TelemetryService/SubscribeHome",
            );
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        #[doc = " Subscribe to in-air updates."]
        pub async fn subscribe_in_air(
            &mut self,
            request: impl tonic::IntoRequest<super::SubscribeInAirRequest>,
        ) -> Result<tonic::Response<tonic::codec::Streaming<super::InAirResponse>>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.telemetry.TelemetryService/SubscribeInAir",
            );
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        #[doc = " Subscribe to landed state updates"]
        pub async fn subscribe_landed_state(
            &mut self,
            request: impl tonic::IntoRequest<super::SubscribeLandedStateRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::LandedStateResponse>>,
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
                "/mavsdk.rpc.telemetry.TelemetryService/SubscribeLandedState",
            );
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        #[doc = " Subscribe to armed updates."]
        pub async fn subscribe_armed(
            &mut self,
            request: impl tonic::IntoRequest<super::SubscribeArmedRequest>,
        ) -> Result<tonic::Response<tonic::codec::Streaming<super::ArmedResponse>>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.telemetry.TelemetryService/SubscribeArmed",
            );
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        #[doc = " Subscribe to 'attitude' updates (quaternion)."]
        pub async fn subscribe_attitude_quaternion(
            &mut self,
            request: impl tonic::IntoRequest<super::SubscribeAttitudeQuaternionRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::AttitudeQuaternionResponse>>,
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
                "/mavsdk.rpc.telemetry.TelemetryService/SubscribeAttitudeQuaternion",
            );
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        #[doc = " Subscribe to 'attitude' updates (euler)."]
        pub async fn subscribe_attitude_euler(
            &mut self,
            request: impl tonic::IntoRequest<super::SubscribeAttitudeEulerRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::AttitudeEulerResponse>>,
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
                "/mavsdk.rpc.telemetry.TelemetryService/SubscribeAttitudeEuler",
            );
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        #[doc = " Subscribe to 'attitude' updates (angular velocity)"]
        pub async fn subscribe_attitude_angular_velocity_body(
            &mut self,
            request: impl tonic::IntoRequest<super::SubscribeAttitudeAngularVelocityBodyRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::AttitudeAngularVelocityBodyResponse>>,
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
                "/mavsdk.rpc.telemetry.TelemetryService/SubscribeAttitudeAngularVelocityBody",
            );
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        #[doc = " Subscribe to 'camera attitude' updates (quaternion)."]
        pub async fn subscribe_camera_attitude_quaternion(
            &mut self,
            request: impl tonic::IntoRequest<super::SubscribeCameraAttitudeQuaternionRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::CameraAttitudeQuaternionResponse>>,
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
                "/mavsdk.rpc.telemetry.TelemetryService/SubscribeCameraAttitudeQuaternion",
            );
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        #[doc = " Subscribe to 'camera attitude' updates (euler)."]
        pub async fn subscribe_camera_attitude_euler(
            &mut self,
            request: impl tonic::IntoRequest<super::SubscribeCameraAttitudeEulerRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::CameraAttitudeEulerResponse>>,
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
                "/mavsdk.rpc.telemetry.TelemetryService/SubscribeCameraAttitudeEuler",
            );
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        #[doc = " Subscribe to 'ground speed' updates (NED)."]
        pub async fn subscribe_ground_speed_ned(
            &mut self,
            request: impl tonic::IntoRequest<super::SubscribeGroundSpeedNedRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::GroundSpeedNedResponse>>,
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
                "/mavsdk.rpc.telemetry.TelemetryService/SubscribeGroundSpeedNed",
            );
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        #[doc = " Subscribe to 'GPS info' updates."]
        pub async fn subscribe_gps_info(
            &mut self,
            request: impl tonic::IntoRequest<super::SubscribeGpsInfoRequest>,
        ) -> Result<tonic::Response<tonic::codec::Streaming<super::GpsInfoResponse>>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.telemetry.TelemetryService/SubscribeGpsInfo",
            );
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        #[doc = " Subscribe to 'battery' updates."]
        pub async fn subscribe_battery(
            &mut self,
            request: impl tonic::IntoRequest<super::SubscribeBatteryRequest>,
        ) -> Result<tonic::Response<tonic::codec::Streaming<super::BatteryResponse>>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.telemetry.TelemetryService/SubscribeBattery",
            );
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        #[doc = " Subscribe to 'flight mode' updates."]
        pub async fn subscribe_flight_mode(
            &mut self,
            request: impl tonic::IntoRequest<super::SubscribeFlightModeRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::FlightModeResponse>>,
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
                "/mavsdk.rpc.telemetry.TelemetryService/SubscribeFlightMode",
            );
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        #[doc = " Subscribe to 'health' updates."]
        pub async fn subscribe_health(
            &mut self,
            request: impl tonic::IntoRequest<super::SubscribeHealthRequest>,
        ) -> Result<tonic::Response<tonic::codec::Streaming<super::HealthResponse>>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.telemetry.TelemetryService/SubscribeHealth",
            );
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        #[doc = " Subscribe to 'RC status' updates."]
        pub async fn subscribe_rc_status(
            &mut self,
            request: impl tonic::IntoRequest<super::SubscribeRcStatusRequest>,
        ) -> Result<tonic::Response<tonic::codec::Streaming<super::RcStatusResponse>>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.telemetry.TelemetryService/SubscribeRcStatus",
            );
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        #[doc = " Subscribe to 'status text' updates."]
        pub async fn subscribe_status_text(
            &mut self,
            request: impl tonic::IntoRequest<super::SubscribeStatusTextRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::StatusTextResponse>>,
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
                "/mavsdk.rpc.telemetry.TelemetryService/SubscribeStatusText",
            );
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        #[doc = " Subscribe to 'actuator control target' updates."]
        pub async fn subscribe_actuator_control_target(
            &mut self,
            request: impl tonic::IntoRequest<super::SubscribeActuatorControlTargetRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::ActuatorControlTargetResponse>>,
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
                "/mavsdk.rpc.telemetry.TelemetryService/SubscribeActuatorControlTarget",
            );
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        #[doc = " Subscribe to 'actuator output status' updates."]
        pub async fn subscribe_actuator_output_status(
            &mut self,
            request: impl tonic::IntoRequest<super::SubscribeActuatorOutputStatusRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::ActuatorOutputStatusResponse>>,
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
                "/mavsdk.rpc.telemetry.TelemetryService/SubscribeActuatorOutputStatus",
            );
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        #[doc = " Subscribe to 'odometry' updates."]
        pub async fn subscribe_odometry(
            &mut self,
            request: impl tonic::IntoRequest<super::SubscribeOdometryRequest>,
        ) -> Result<tonic::Response<tonic::codec::Streaming<super::OdometryResponse>>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.telemetry.TelemetryService/SubscribeOdometry",
            );
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
    }
}
