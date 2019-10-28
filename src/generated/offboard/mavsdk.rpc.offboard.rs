#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StartRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StartResponse {
    #[prost(message, optional, tag = "1")]
    pub offboard_result: ::std::option::Option<OffboardResult>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StopRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StopResponse {
    #[prost(message, optional, tag = "1")]
    pub offboard_result: ::std::option::Option<OffboardResult>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IsActiveRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IsActiveResponse {
    /// True if offboard is active
    #[prost(bool, tag = "1")]
    pub is_active: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetAttitudeRequest {
    /// Attitude roll, pitch and yaw along with thrust
    #[prost(message, optional, tag = "1")]
    pub attitude: ::std::option::Option<Attitude>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetAttitudeResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetActuatorControlRequest {
    /// Actuator control values
    #[prost(message, optional, tag = "1")]
    pub actuator_control: ::std::option::Option<ActuatorControl>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetActuatorControlResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetAttitudeRateRequest {
    /// Attitude rate roll, pitch and yaw angular rate along with thrust
    #[prost(message, optional, tag = "1")]
    pub attitude_rate: ::std::option::Option<AttitudeRate>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetAttitudeRateResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetPositionNedRequest {
    /// Position and yaw
    #[prost(message, optional, tag = "1")]
    pub position_ned_yaw: ::std::option::Option<PositionNedYaw>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetPositionNedResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetVelocityBodyRequest {
    /// Velocity and yaw angular rate
    #[prost(message, optional, tag = "1")]
    pub velocity_body_yawspeed: ::std::option::Option<VelocityBodyYawspeed>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetVelocityBodyResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetVelocityNedRequest {
    /// Velocity and yaw
    #[prost(message, optional, tag = "1")]
    pub velocity_ned_yaw: ::std::option::Option<VelocityNedYaw>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetVelocityNedResponse {}
/// Type for attitude body angles in NED reference frame (roll, pitch, yaw and thrust)
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Attitude {
    /// Roll angle (in degrees, positive is right side down)
    #[prost(float, tag = "1")]
    pub roll_deg: f32,
    /// Pitch angle (in degrees, positive is nose up)
    #[prost(float, tag = "2")]
    pub pitch_deg: f32,
    /// Yaw angle (in degrees, positive is move nose to the right)
    #[prost(float, tag = "3")]
    pub yaw_deg: f32,
    /// Thrust (range: 0 to 1)
    #[prost(float, tag = "4")]
    pub thrust_value: f32,
}
///
/// Eight controls that will be given to the group. Each control is a normalized
/// (-1..+1) command value, which will be mapped and scaled through the mixer.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActuatorControlGroup {
    /// Controls in the group
    #[prost(float, repeated, tag = "1")]
    pub controls: ::std::vec::Vec<f32>,
}
///
/// Type for actuator control.
///
/// Control members should be normed to -1..+1 where 0 is neutral position.
/// Throttle for single rotation direction motors is 0..1, negative range for reverse direction.
///
/// One group support eight controls.
///
/// Up to 16 actuator controls can be set. To ignore an output group, set all it conrols to NaN.
/// If one or more controls in group is not NaN, then all NaN controls will sent as zero.
/// The first 8 actuator controls internally map to control group 0, the latter 8 actuator
/// controls map to control group 1. Depending on what controls are set (instead of NaN) 1 or 2
/// MAVLink messages are actually sent.
///
/// In PX4 v1.9.0 Only first four Control Groups are supported
/// (https://github.com/PX4/Firmware/blob/v1.9.0/src/modules/mavlink/mavlink_receiver.cpp#L980).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActuatorControl {
    /// Control groups.
    #[prost(message, repeated, tag = "1")]
    pub groups: ::std::vec::Vec<ActuatorControlGroup>,
}
/// Type for attitude rate commands in body coordinates (roll, pitch, yaw angular rate and thrust)
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AttitudeRate {
    /// Roll angular rate (in degrees/second, positive for clock-wise looking from front)
    #[prost(float, tag = "1")]
    pub roll_deg_s: f32,
    /// Pitch angular rate (in degrees/second, positive for head/front moving up)
    #[prost(float, tag = "2")]
    pub pitch_deg_s: f32,
    /// Yaw angular rate (in degrees/second, positive for clock-wise looking from above)
    #[prost(float, tag = "3")]
    pub yaw_deg_s: f32,
    /// Thrust (range: 0 to 1)
    #[prost(float, tag = "4")]
    pub thrust_value: f32,
}
/// Type for position commands in NED (North East Down) coordinates and yaw.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PositionNedYaw {
    /// Position North (in metres)
    #[prost(float, tag = "1")]
    pub north_m: f32,
    /// Position East (in metres)
    #[prost(float, tag = "2")]
    pub east_m: f32,
    /// Position Down (in metres)
    #[prost(float, tag = "3")]
    pub down_m: f32,
    /// Yaw in degrees (0 North, positive is clock-wise looking from above)
    #[prost(float, tag = "4")]
    pub yaw_deg: f32,
}
/// Type for velocity commands in body coordinates.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VelocityBodyYawspeed {
    /// Velocity forward (in metres/second)
    #[prost(float, tag = "1")]
    pub forward_m_s: f32,
    /// Velocity right (in metres/second)
    #[prost(float, tag = "2")]
    pub right_m_s: f32,
    /// Velocity down (in metres/second)
    #[prost(float, tag = "3")]
    pub down_m_s: f32,
    /// Yaw angular rate (in degrees/second, positive for clock-wise looking from above)
    #[prost(float, tag = "4")]
    pub yawspeed_deg_s: f32,
}
/// Type for velocity commands in NED (North East Down) coordinates and yaw.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VelocityNedYaw {
    /// Velocity North (in metres/second)
    #[prost(float, tag = "1")]
    pub north_m_s: f32,
    /// Velocity East (in metres/second)
    #[prost(float, tag = "2")]
    pub east_m_s: f32,
    /// Velocity Down (in metres/second)
    #[prost(float, tag = "3")]
    pub down_m_s: f32,
    /// Yaw in degrees (0 North, positive is clock-wise looking from above)
    #[prost(float, tag = "4")]
    pub yaw_deg: f32,
}
/// Result type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OffboardResult {
    /// Result enum value
    #[prost(enumeration = "offboard_result::Result", tag = "1")]
    pub result: i32,
    /// Human-readable English string describing the result
    #[prost(string, tag = "2")]
    pub result_str: std::string::String,
}
pub mod offboard_result {
    /// Possible results returned for offboard requests
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
        /// Vehicle is busy
        Busy = 4,
        /// Command denied
        CommandDenied = 5,
        /// Request timed out
        Timeout = 6,
        /// Cannot start without setpoint set
        NoSetpointSet = 7,
    }
}
#[doc = r" Generated client implementations."]
pub mod client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "*"]
    #[doc = " Control a drone with position, velocity, attitude or motor commands."]
    #[doc = ""]
    #[doc = " The module is called offboard because the commands can be sent from external sources"]
    #[doc = " as opposed to onboard control right inside the autopilot \"board\"."]
    #[doc = ""]
    #[doc = " Client code must specify a setpoint before starting offboard mode."]
    #[doc = " Mavsdk automatically sends setpoints at 20Hz (PX4 Offboard mode requires that setpoints"]
    #[doc = " are minimally sent at 2Hz)."]
    pub struct OffboardServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl OffboardServiceClient<tonic::transport::Channel> {
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
    impl<T> OffboardServiceClient<T>
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
        #[doc = " Start offboard control."]
        pub async fn start(
            &mut self,
            request: impl tonic::IntoRequest<super::StartRequest>,
        ) -> Result<tonic::Response<super::StartResponse>, tonic::Status> {
            self.ready().await?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/mavsdk.rpc.offboard.OffboardService/Start");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = " Stop offboard control."]
        #[doc = ""]
        #[doc = " The vehicle will be put into Hold mode: https://docs.px4.io/en/flight_modes/hold.html"]
        pub async fn stop(
            &mut self,
            request: impl tonic::IntoRequest<super::StopRequest>,
        ) -> Result<tonic::Response<super::StopResponse>, tonic::Status> {
            self.ready().await?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/mavsdk.rpc.offboard.OffboardService/Stop");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = " Check if offboard control is active."]
        #[doc = ""]
        #[doc = " True means that the vehicle is in offboard mode and we are actively sending"]
        #[doc = " setpoints."]
        pub async fn is_active(
            &mut self,
            request: impl tonic::IntoRequest<super::IsActiveRequest>,
        ) -> Result<tonic::Response<super::IsActiveResponse>, tonic::Status> {
            self.ready().await?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.offboard.OffboardService/IsActive",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = " Set the attitude in terms of roll, pitch and yaw in degrees with thrust."]
        pub async fn set_attitude(
            &mut self,
            request: impl tonic::IntoRequest<super::SetAttitudeRequest>,
        ) -> Result<tonic::Response<super::SetAttitudeResponse>, tonic::Status> {
            self.ready().await?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.offboard.OffboardService/SetAttitude",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = " Set direct actuator control values to groups #0 and #1."]
        #[doc = ""]
        #[doc = " First 8 controls will go to control group 0, the following 8 controls to control group 1 (if"]
        #[doc = " actuator_control.num_controls more than 8)."]
        pub async fn set_actuator_control(
            &mut self,
            request: impl tonic::IntoRequest<super::SetActuatorControlRequest>,
        ) -> Result<tonic::Response<super::SetActuatorControlResponse>, tonic::Status> {
            self.ready().await?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.offboard.OffboardService/SetActuatorControl",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = " Set the attitude rate in terms of pitch, roll and yaw angular rate along with thrust."]
        pub async fn set_attitude_rate(
            &mut self,
            request: impl tonic::IntoRequest<super::SetAttitudeRateRequest>,
        ) -> Result<tonic::Response<super::SetAttitudeRateResponse>, tonic::Status> {
            self.ready().await?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.offboard.OffboardService/SetAttitudeRate",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = " Set the position in NED coordinates and yaw."]
        pub async fn set_position_ned(
            &mut self,
            request: impl tonic::IntoRequest<super::SetPositionNedRequest>,
        ) -> Result<tonic::Response<super::SetPositionNedResponse>, tonic::Status> {
            self.ready().await?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.offboard.OffboardService/SetPositionNed",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = " Set the velocity in body coordinates and yaw angular rate."]
        pub async fn set_velocity_body(
            &mut self,
            request: impl tonic::IntoRequest<super::SetVelocityBodyRequest>,
        ) -> Result<tonic::Response<super::SetVelocityBodyResponse>, tonic::Status> {
            self.ready().await?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.offboard.OffboardService/SetVelocityBody",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = " Set the velocity in NED coordinates and yaw."]
        pub async fn set_velocity_ned(
            &mut self,
            request: impl tonic::IntoRequest<super::SetVelocityNedRequest>,
        ) -> Result<tonic::Response<super::SetVelocityNedResponse>, tonic::Status> {
            self.ready().await?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.offboard.OffboardService/SetVelocityNed",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for OffboardServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
}
