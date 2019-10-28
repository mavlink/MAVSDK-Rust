#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ArmRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ArmResponse {
    #[prost(message, optional, tag = "1")]
    pub action_result: ::std::option::Option<ActionResult>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DisarmRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DisarmResponse {
    #[prost(message, optional, tag = "1")]
    pub action_result: ::std::option::Option<ActionResult>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TakeoffRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TakeoffResponse {
    #[prost(message, optional, tag = "1")]
    pub action_result: ::std::option::Option<ActionResult>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LandRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LandResponse {
    #[prost(message, optional, tag = "1")]
    pub action_result: ::std::option::Option<ActionResult>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RebootRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RebootResponse {
    #[prost(message, optional, tag = "1")]
    pub action_result: ::std::option::Option<ActionResult>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KillRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KillResponse {
    #[prost(message, optional, tag = "1")]
    pub action_result: ::std::option::Option<ActionResult>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReturnToLaunchRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReturnToLaunchResponse {
    #[prost(message, optional, tag = "1")]
    pub action_result: ::std::option::Option<ActionResult>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransitionToFixedWingRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransitionToFixedWingResponse {
    #[prost(message, optional, tag = "1")]
    pub action_result: ::std::option::Option<ActionResult>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransitionToMulticopterRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransitionToMulticopterResponse {
    #[prost(message, optional, tag = "1")]
    pub action_result: ::std::option::Option<ActionResult>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTakeoffAltitudeRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTakeoffAltitudeResponse {
    #[prost(message, optional, tag = "1")]
    pub action_result: ::std::option::Option<ActionResult>,
    /// Takeoff altitude relative to ground/takeoff location (in meters)
    #[prost(float, tag = "2")]
    pub altitude: f32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetTakeoffAltitudeRequest {
    /// Takeoff altitude relative to ground/takeoff location (in meters)
    #[prost(float, tag = "1")]
    pub altitude: f32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetTakeoffAltitudeResponse {
    #[prost(message, optional, tag = "1")]
    pub action_result: ::std::option::Option<ActionResult>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMaximumSpeedRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMaximumSpeedResponse {
    #[prost(message, optional, tag = "1")]
    pub action_result: ::std::option::Option<ActionResult>,
    /// Maximum speed (in metres/second)
    #[prost(float, tag = "2")]
    pub speed: f32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetMaximumSpeedRequest {
    /// Maximum speed (in metres/second)
    #[prost(float, tag = "1")]
    pub speed: f32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetMaximumSpeedResponse {
    #[prost(message, optional, tag = "1")]
    pub action_result: ::std::option::Option<ActionResult>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetReturnToLaunchAltitudeRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetReturnToLaunchAltitudeResponse {
    #[prost(message, optional, tag = "1")]
    pub action_result: ::std::option::Option<ActionResult>,
    /// Return altitude relative to takeoff location (in meters)
    #[prost(float, tag = "2")]
    pub relative_altitude_m: f32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetReturnToLaunchAltitudeRequest {
    /// Return altitude relative to takeoff location (in meters)
    #[prost(float, tag = "1")]
    pub relative_altitude_m: f32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetReturnToLaunchAltitudeResponse {
    #[prost(message, optional, tag = "1")]
    pub action_result: ::std::option::Option<ActionResult>,
}
/// Result type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActionResult {
    /// Result enum value
    #[prost(enumeration = "action_result::Result", tag = "1")]
    pub result: i32,
    /// Human-readable English string describing the result
    #[prost(string, tag = "2")]
    pub result_str: std::string::String,
}
pub mod action_result {
    /// Possible results returned for action requests.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Result {
        /// Unknown error
        Unknown = 0,
        /// Success: the action command was accepted by the vehicle
        Success = 1,
        /// No system is connected
        NoSystem = 2,
        /// Connection error
        ConnectionError = 3,
        /// Vehicle is busy
        Busy = 4,
        /// Command refused by vehicle
        CommandDenied = 5,
        /// Command refused because landed state is unknown
        CommandDeniedLandedStateUnknown = 6,
        /// Command refused because vehicle not landed
        CommandDeniedNotLanded = 7,
        /// Request timed out
        Timeout = 8,
        /// Hybrid/VTOL transition refused because VTOL support is unknown
        VtolTransitionSupportUnknown = 9,
        /// Vehicle does not support hybrid/VTOL transitions
        NoVtolTransitionSupport = 10,
        /// Error getting or setting parameter
        ParameterError = 11,
    }
}
#[doc = r" Generated client implementations."]
pub mod client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Enable simple actions such as arming, taking off, and landing."]
    pub struct ActionServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ActionServiceClient<tonic::transport::Channel> {
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
    impl<T> ActionServiceClient<T>
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
        #[doc = " Send command to arm the drone."]
        #[doc = ""]
        #[doc = " Arming a drone normally causes motors to spin at idle."]
        #[doc = " Before arming take all safety precautions and stand clear of the drone!"]
        pub async fn arm(
            &mut self,
            request: impl tonic::IntoRequest<super::ArmRequest>,
        ) -> Result<tonic::Response<super::ArmResponse>, tonic::Status> {
            self.ready().await?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/mavsdk.rpc.action.ActionService/Arm");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = " Send command to disarm the drone."]
        #[doc = ""]
        #[doc = " This will disarm a drone that considers itself landed. If flying, the drone should"]
        #[doc = " reject the disarm command. Disarming means that all motors will stop."]
        pub async fn disarm(
            &mut self,
            request: impl tonic::IntoRequest<super::DisarmRequest>,
        ) -> Result<tonic::Response<super::DisarmResponse>, tonic::Status> {
            self.ready().await?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/mavsdk.rpc.action.ActionService/Disarm");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = " Send command to take off and hover."]
        #[doc = ""]
        #[doc = " This switches the drone into position control mode and commands "]
        #[doc = " it to take off and hover at the takeoff altitude."]
        #[doc = ""]
        #[doc = " Note that the vehicle must be armed before it can take off."]
        pub async fn takeoff(
            &mut self,
            request: impl tonic::IntoRequest<super::TakeoffRequest>,
        ) -> Result<tonic::Response<super::TakeoffResponse>, tonic::Status> {
            self.ready().await?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/mavsdk.rpc.action.ActionService/Takeoff");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = " Send command to land at the current position."]
        #[doc = ""]
        #[doc = " This switches the drone to 'Land' flight mode."]
        pub async fn land(
            &mut self,
            request: impl tonic::IntoRequest<super::LandRequest>,
        ) -> Result<tonic::Response<super::LandResponse>, tonic::Status> {
            self.ready().await?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/mavsdk.rpc.action.ActionService/Land");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = " Send command to reboot the drone components."]
        #[doc = ""]
        #[doc = " This will reboot the autopilot, companion computer, camera and gimbal."]
        pub async fn reboot(
            &mut self,
            request: impl tonic::IntoRequest<super::RebootRequest>,
        ) -> Result<tonic::Response<super::RebootResponse>, tonic::Status> {
            self.ready().await?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/mavsdk.rpc.action.ActionService/Reboot");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = " Send command to kill the drone. "]
        #[doc = ""]
        #[doc = " This will disarm a drone irrespective of whether it is landed or flying."]
        #[doc = " Note that the drone will fall out of the sky if this command is used while flying."]
        pub async fn kill(
            &mut self,
            request: impl tonic::IntoRequest<super::KillRequest>,
        ) -> Result<tonic::Response<super::KillResponse>, tonic::Status> {
            self.ready().await?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/mavsdk.rpc.action.ActionService/Kill");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = " Send command to return to the launch (takeoff) position and land."]
        #[doc = ""]
        #[doc = " This switches the drone into [RTL mode](https://docs.px4.io/en/flight_modes/rtl.html) which"]
        #[doc = " generally means it will rise up to a certain altitude to clear any obstacles before heading"]
        #[doc = " back to the launch (takeoff) position and land there."]
        pub async fn return_to_launch(
            &mut self,
            request: impl tonic::IntoRequest<super::ReturnToLaunchRequest>,
        ) -> Result<tonic::Response<super::ReturnToLaunchResponse>, tonic::Status> {
            self.ready().await?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.action.ActionService/ReturnToLaunch",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = " Send command to transition the drone to fixedwing."]
        #[doc = ""]
        #[doc = " The associated action will only be executed for VTOL vehicles (on other vehicle types the"]
        #[doc = " command will fail). The command will succeed if called when the vehicle"]
        #[doc = " is already in fixedwing mode."]
        pub async fn transition_to_fixed_wing(
            &mut self,
            request: impl tonic::IntoRequest<super::TransitionToFixedWingRequest>,
        ) -> Result<tonic::Response<super::TransitionToFixedWingResponse>, tonic::Status> {
            self.ready().await?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.action.ActionService/TransitionToFixedWing",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = " Send command to transition the drone to multicopter."]
        #[doc = ""]
        #[doc = " The associated action will only be executed for VTOL vehicles (on other vehicle types the"]
        #[doc = " command will fail). The command will succeed if called when the vehicle"]
        #[doc = " is already in multicopter mode."]
        pub async fn transition_to_multicopter(
            &mut self,
            request: impl tonic::IntoRequest<super::TransitionToMulticopterRequest>,
        ) -> Result<tonic::Response<super::TransitionToMulticopterResponse>, tonic::Status>
        {
            self.ready().await?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.action.ActionService/TransitionToMulticopter",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = " Get the takeoff altitude (in meters above ground)."]
        pub async fn get_takeoff_altitude(
            &mut self,
            request: impl tonic::IntoRequest<super::GetTakeoffAltitudeRequest>,
        ) -> Result<tonic::Response<super::GetTakeoffAltitudeResponse>, tonic::Status> {
            self.ready().await?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.action.ActionService/GetTakeoffAltitude",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = " Set takeoff altitude (in meters above ground)."]
        pub async fn set_takeoff_altitude(
            &mut self,
            request: impl tonic::IntoRequest<super::SetTakeoffAltitudeRequest>,
        ) -> Result<tonic::Response<super::SetTakeoffAltitudeResponse>, tonic::Status> {
            self.ready().await?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.action.ActionService/SetTakeoffAltitude",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = " Get the vehicle maximum speed (in metres/second)."]
        pub async fn get_maximum_speed(
            &mut self,
            request: impl tonic::IntoRequest<super::GetMaximumSpeedRequest>,
        ) -> Result<tonic::Response<super::GetMaximumSpeedResponse>, tonic::Status> {
            self.ready().await?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.action.ActionService/GetMaximumSpeed",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = " Set vehicle maximum speed (in metres/second)."]
        pub async fn set_maximum_speed(
            &mut self,
            request: impl tonic::IntoRequest<super::SetMaximumSpeedRequest>,
        ) -> Result<tonic::Response<super::SetMaximumSpeedResponse>, tonic::Status> {
            self.ready().await?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.action.ActionService/SetMaximumSpeed",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = " Get the return to launch minimum return altitude (in meters)."]
        pub async fn get_return_to_launch_altitude(
            &mut self,
            request: impl tonic::IntoRequest<super::GetReturnToLaunchAltitudeRequest>,
        ) -> Result<tonic::Response<super::GetReturnToLaunchAltitudeResponse>, tonic::Status>
        {
            self.ready().await?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.action.ActionService/GetReturnToLaunchAltitude",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = " Set the return to launch minimum return altitude (in meters)."]
        pub async fn set_return_to_launch_altitude(
            &mut self,
            request: impl tonic::IntoRequest<super::SetReturnToLaunchAltitudeRequest>,
        ) -> Result<tonic::Response<super::SetReturnToLaunchAltitudeResponse>, tonic::Status>
        {
            self.ready().await?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.action.ActionService/SetReturnToLaunchAltitude",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for ActionServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
}
