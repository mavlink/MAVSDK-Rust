// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

const METHOD_TELEMETRY_SERVICE_SUBSCRIBE_POSITION: ::grpcio::Method<super::telemetry::SubscribePositionRequest, super::telemetry::PositionResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ServerStreaming,
    name: "/mavsdk.rpc.telemetry.TelemetryService/SubscribePosition",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_TELEMETRY_SERVICE_SUBSCRIBE_HOME: ::grpcio::Method<super::telemetry::SubscribeHomeRequest, super::telemetry::HomeResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ServerStreaming,
    name: "/mavsdk.rpc.telemetry.TelemetryService/SubscribeHome",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_TELEMETRY_SERVICE_SUBSCRIBE_IN_AIR: ::grpcio::Method<super::telemetry::SubscribeInAirRequest, super::telemetry::InAirResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ServerStreaming,
    name: "/mavsdk.rpc.telemetry.TelemetryService/SubscribeInAir",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_TELEMETRY_SERVICE_SUBSCRIBE_LANDED_STATE: ::grpcio::Method<super::telemetry::SubscribeLandedStateRequest, super::telemetry::LandedStateResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ServerStreaming,
    name: "/mavsdk.rpc.telemetry.TelemetryService/SubscribeLandedState",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_TELEMETRY_SERVICE_SUBSCRIBE_ARMED: ::grpcio::Method<super::telemetry::SubscribeArmedRequest, super::telemetry::ArmedResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ServerStreaming,
    name: "/mavsdk.rpc.telemetry.TelemetryService/SubscribeArmed",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_TELEMETRY_SERVICE_SUBSCRIBE_ATTITUDE_QUATERNION: ::grpcio::Method<super::telemetry::SubscribeAttitudeQuaternionRequest, super::telemetry::AttitudeQuaternionResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ServerStreaming,
    name: "/mavsdk.rpc.telemetry.TelemetryService/SubscribeAttitudeQuaternion",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_TELEMETRY_SERVICE_SUBSCRIBE_ATTITUDE_EULER: ::grpcio::Method<super::telemetry::SubscribeAttitudeEulerRequest, super::telemetry::AttitudeEulerResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ServerStreaming,
    name: "/mavsdk.rpc.telemetry.TelemetryService/SubscribeAttitudeEuler",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_TELEMETRY_SERVICE_SUBSCRIBE_ATTITUDE_ANGULAR_VELOCITY_BODY: ::grpcio::Method<super::telemetry::SubscribeAttitudeAngularVelocityBodyRequest, super::telemetry::AttitudeAngularVelocityBodyResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ServerStreaming,
    name: "/mavsdk.rpc.telemetry.TelemetryService/SubscribeAttitudeAngularVelocityBody",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_TELEMETRY_SERVICE_SUBSCRIBE_CAMERA_ATTITUDE_QUATERNION: ::grpcio::Method<super::telemetry::SubscribeCameraAttitudeQuaternionRequest, super::telemetry::CameraAttitudeQuaternionResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ServerStreaming,
    name: "/mavsdk.rpc.telemetry.TelemetryService/SubscribeCameraAttitudeQuaternion",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_TELEMETRY_SERVICE_SUBSCRIBE_CAMERA_ATTITUDE_EULER: ::grpcio::Method<super::telemetry::SubscribeCameraAttitudeEulerRequest, super::telemetry::CameraAttitudeEulerResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ServerStreaming,
    name: "/mavsdk.rpc.telemetry.TelemetryService/SubscribeCameraAttitudeEuler",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_TELEMETRY_SERVICE_SUBSCRIBE_GROUND_SPEED_NED: ::grpcio::Method<super::telemetry::SubscribeGroundSpeedNedRequest, super::telemetry::GroundSpeedNedResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ServerStreaming,
    name: "/mavsdk.rpc.telemetry.TelemetryService/SubscribeGroundSpeedNed",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_TELEMETRY_SERVICE_SUBSCRIBE_GPS_INFO: ::grpcio::Method<super::telemetry::SubscribeGpsInfoRequest, super::telemetry::GpsInfoResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ServerStreaming,
    name: "/mavsdk.rpc.telemetry.TelemetryService/SubscribeGpsInfo",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_TELEMETRY_SERVICE_SUBSCRIBE_BATTERY: ::grpcio::Method<super::telemetry::SubscribeBatteryRequest, super::telemetry::BatteryResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ServerStreaming,
    name: "/mavsdk.rpc.telemetry.TelemetryService/SubscribeBattery",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_TELEMETRY_SERVICE_SUBSCRIBE_FLIGHT_MODE: ::grpcio::Method<super::telemetry::SubscribeFlightModeRequest, super::telemetry::FlightModeResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ServerStreaming,
    name: "/mavsdk.rpc.telemetry.TelemetryService/SubscribeFlightMode",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_TELEMETRY_SERVICE_SUBSCRIBE_HEALTH: ::grpcio::Method<super::telemetry::SubscribeHealthRequest, super::telemetry::HealthResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ServerStreaming,
    name: "/mavsdk.rpc.telemetry.TelemetryService/SubscribeHealth",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_TELEMETRY_SERVICE_SUBSCRIBE_RC_STATUS: ::grpcio::Method<super::telemetry::SubscribeRcStatusRequest, super::telemetry::RcStatusResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ServerStreaming,
    name: "/mavsdk.rpc.telemetry.TelemetryService/SubscribeRcStatus",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_TELEMETRY_SERVICE_SUBSCRIBE_STATUS_TEXT: ::grpcio::Method<super::telemetry::SubscribeStatusTextRequest, super::telemetry::StatusTextResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ServerStreaming,
    name: "/mavsdk.rpc.telemetry.TelemetryService/SubscribeStatusText",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_TELEMETRY_SERVICE_SUBSCRIBE_ACTUATOR_CONTROL_TARGET: ::grpcio::Method<super::telemetry::SubscribeActuatorControlTargetRequest, super::telemetry::ActuatorControlTargetResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ServerStreaming,
    name: "/mavsdk.rpc.telemetry.TelemetryService/SubscribeActuatorControlTarget",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_TELEMETRY_SERVICE_SUBSCRIBE_ACTUATOR_OUTPUT_STATUS: ::grpcio::Method<super::telemetry::SubscribeActuatorOutputStatusRequest, super::telemetry::ActuatorOutputStatusResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ServerStreaming,
    name: "/mavsdk.rpc.telemetry.TelemetryService/SubscribeActuatorOutputStatus",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_TELEMETRY_SERVICE_SUBSCRIBE_ODOMETRY: ::grpcio::Method<super::telemetry::SubscribeOdometryRequest, super::telemetry::OdometryResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ServerStreaming,
    name: "/mavsdk.rpc.telemetry.TelemetryService/SubscribeOdometry",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct TelemetryServiceClient {
    client: ::grpcio::Client,
}

impl TelemetryServiceClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        TelemetryServiceClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn subscribe_position_opt(&self, req: &super::telemetry::SubscribePositionRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::telemetry::PositionResponse>> {
        self.client.server_streaming(&METHOD_TELEMETRY_SERVICE_SUBSCRIBE_POSITION, req, opt)
    }

    pub fn subscribe_position(&self, req: &super::telemetry::SubscribePositionRequest) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::telemetry::PositionResponse>> {
        self.subscribe_position_opt(req, ::grpcio::CallOption::default())
    }

    pub fn subscribe_home_opt(&self, req: &super::telemetry::SubscribeHomeRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::telemetry::HomeResponse>> {
        self.client.server_streaming(&METHOD_TELEMETRY_SERVICE_SUBSCRIBE_HOME, req, opt)
    }

    pub fn subscribe_home(&self, req: &super::telemetry::SubscribeHomeRequest) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::telemetry::HomeResponse>> {
        self.subscribe_home_opt(req, ::grpcio::CallOption::default())
    }

    pub fn subscribe_in_air_opt(&self, req: &super::telemetry::SubscribeInAirRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::telemetry::InAirResponse>> {
        self.client.server_streaming(&METHOD_TELEMETRY_SERVICE_SUBSCRIBE_IN_AIR, req, opt)
    }

    pub fn subscribe_in_air(&self, req: &super::telemetry::SubscribeInAirRequest) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::telemetry::InAirResponse>> {
        self.subscribe_in_air_opt(req, ::grpcio::CallOption::default())
    }

    pub fn subscribe_landed_state_opt(&self, req: &super::telemetry::SubscribeLandedStateRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::telemetry::LandedStateResponse>> {
        self.client.server_streaming(&METHOD_TELEMETRY_SERVICE_SUBSCRIBE_LANDED_STATE, req, opt)
    }

    pub fn subscribe_landed_state(&self, req: &super::telemetry::SubscribeLandedStateRequest) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::telemetry::LandedStateResponse>> {
        self.subscribe_landed_state_opt(req, ::grpcio::CallOption::default())
    }

    pub fn subscribe_armed_opt(&self, req: &super::telemetry::SubscribeArmedRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::telemetry::ArmedResponse>> {
        self.client.server_streaming(&METHOD_TELEMETRY_SERVICE_SUBSCRIBE_ARMED, req, opt)
    }

    pub fn subscribe_armed(&self, req: &super::telemetry::SubscribeArmedRequest) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::telemetry::ArmedResponse>> {
        self.subscribe_armed_opt(req, ::grpcio::CallOption::default())
    }

    pub fn subscribe_attitude_quaternion_opt(&self, req: &super::telemetry::SubscribeAttitudeQuaternionRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::telemetry::AttitudeQuaternionResponse>> {
        self.client.server_streaming(&METHOD_TELEMETRY_SERVICE_SUBSCRIBE_ATTITUDE_QUATERNION, req, opt)
    }

    pub fn subscribe_attitude_quaternion(&self, req: &super::telemetry::SubscribeAttitudeQuaternionRequest) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::telemetry::AttitudeQuaternionResponse>> {
        self.subscribe_attitude_quaternion_opt(req, ::grpcio::CallOption::default())
    }

    pub fn subscribe_attitude_euler_opt(&self, req: &super::telemetry::SubscribeAttitudeEulerRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::telemetry::AttitudeEulerResponse>> {
        self.client.server_streaming(&METHOD_TELEMETRY_SERVICE_SUBSCRIBE_ATTITUDE_EULER, req, opt)
    }

    pub fn subscribe_attitude_euler(&self, req: &super::telemetry::SubscribeAttitudeEulerRequest) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::telemetry::AttitudeEulerResponse>> {
        self.subscribe_attitude_euler_opt(req, ::grpcio::CallOption::default())
    }

    pub fn subscribe_attitude_angular_velocity_body_opt(&self, req: &super::telemetry::SubscribeAttitudeAngularVelocityBodyRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::telemetry::AttitudeAngularVelocityBodyResponse>> {
        self.client.server_streaming(&METHOD_TELEMETRY_SERVICE_SUBSCRIBE_ATTITUDE_ANGULAR_VELOCITY_BODY, req, opt)
    }

    pub fn subscribe_attitude_angular_velocity_body(&self, req: &super::telemetry::SubscribeAttitudeAngularVelocityBodyRequest) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::telemetry::AttitudeAngularVelocityBodyResponse>> {
        self.subscribe_attitude_angular_velocity_body_opt(req, ::grpcio::CallOption::default())
    }

    pub fn subscribe_camera_attitude_quaternion_opt(&self, req: &super::telemetry::SubscribeCameraAttitudeQuaternionRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::telemetry::CameraAttitudeQuaternionResponse>> {
        self.client.server_streaming(&METHOD_TELEMETRY_SERVICE_SUBSCRIBE_CAMERA_ATTITUDE_QUATERNION, req, opt)
    }

    pub fn subscribe_camera_attitude_quaternion(&self, req: &super::telemetry::SubscribeCameraAttitudeQuaternionRequest) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::telemetry::CameraAttitudeQuaternionResponse>> {
        self.subscribe_camera_attitude_quaternion_opt(req, ::grpcio::CallOption::default())
    }

    pub fn subscribe_camera_attitude_euler_opt(&self, req: &super::telemetry::SubscribeCameraAttitudeEulerRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::telemetry::CameraAttitudeEulerResponse>> {
        self.client.server_streaming(&METHOD_TELEMETRY_SERVICE_SUBSCRIBE_CAMERA_ATTITUDE_EULER, req, opt)
    }

    pub fn subscribe_camera_attitude_euler(&self, req: &super::telemetry::SubscribeCameraAttitudeEulerRequest) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::telemetry::CameraAttitudeEulerResponse>> {
        self.subscribe_camera_attitude_euler_opt(req, ::grpcio::CallOption::default())
    }

    pub fn subscribe_ground_speed_ned_opt(&self, req: &super::telemetry::SubscribeGroundSpeedNedRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::telemetry::GroundSpeedNedResponse>> {
        self.client.server_streaming(&METHOD_TELEMETRY_SERVICE_SUBSCRIBE_GROUND_SPEED_NED, req, opt)
    }

    pub fn subscribe_ground_speed_ned(&self, req: &super::telemetry::SubscribeGroundSpeedNedRequest) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::telemetry::GroundSpeedNedResponse>> {
        self.subscribe_ground_speed_ned_opt(req, ::grpcio::CallOption::default())
    }

    pub fn subscribe_gps_info_opt(&self, req: &super::telemetry::SubscribeGpsInfoRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::telemetry::GpsInfoResponse>> {
        self.client.server_streaming(&METHOD_TELEMETRY_SERVICE_SUBSCRIBE_GPS_INFO, req, opt)
    }

    pub fn subscribe_gps_info(&self, req: &super::telemetry::SubscribeGpsInfoRequest) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::telemetry::GpsInfoResponse>> {
        self.subscribe_gps_info_opt(req, ::grpcio::CallOption::default())
    }

    pub fn subscribe_battery_opt(&self, req: &super::telemetry::SubscribeBatteryRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::telemetry::BatteryResponse>> {
        self.client.server_streaming(&METHOD_TELEMETRY_SERVICE_SUBSCRIBE_BATTERY, req, opt)
    }

    pub fn subscribe_battery(&self, req: &super::telemetry::SubscribeBatteryRequest) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::telemetry::BatteryResponse>> {
        self.subscribe_battery_opt(req, ::grpcio::CallOption::default())
    }

    pub fn subscribe_flight_mode_opt(&self, req: &super::telemetry::SubscribeFlightModeRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::telemetry::FlightModeResponse>> {
        self.client.server_streaming(&METHOD_TELEMETRY_SERVICE_SUBSCRIBE_FLIGHT_MODE, req, opt)
    }

    pub fn subscribe_flight_mode(&self, req: &super::telemetry::SubscribeFlightModeRequest) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::telemetry::FlightModeResponse>> {
        self.subscribe_flight_mode_opt(req, ::grpcio::CallOption::default())
    }

    pub fn subscribe_health_opt(&self, req: &super::telemetry::SubscribeHealthRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::telemetry::HealthResponse>> {
        self.client.server_streaming(&METHOD_TELEMETRY_SERVICE_SUBSCRIBE_HEALTH, req, opt)
    }

    pub fn subscribe_health(&self, req: &super::telemetry::SubscribeHealthRequest) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::telemetry::HealthResponse>> {
        self.subscribe_health_opt(req, ::grpcio::CallOption::default())
    }

    pub fn subscribe_rc_status_opt(&self, req: &super::telemetry::SubscribeRcStatusRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::telemetry::RcStatusResponse>> {
        self.client.server_streaming(&METHOD_TELEMETRY_SERVICE_SUBSCRIBE_RC_STATUS, req, opt)
    }

    pub fn subscribe_rc_status(&self, req: &super::telemetry::SubscribeRcStatusRequest) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::telemetry::RcStatusResponse>> {
        self.subscribe_rc_status_opt(req, ::grpcio::CallOption::default())
    }

    pub fn subscribe_status_text_opt(&self, req: &super::telemetry::SubscribeStatusTextRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::telemetry::StatusTextResponse>> {
        self.client.server_streaming(&METHOD_TELEMETRY_SERVICE_SUBSCRIBE_STATUS_TEXT, req, opt)
    }

    pub fn subscribe_status_text(&self, req: &super::telemetry::SubscribeStatusTextRequest) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::telemetry::StatusTextResponse>> {
        self.subscribe_status_text_opt(req, ::grpcio::CallOption::default())
    }

    pub fn subscribe_actuator_control_target_opt(&self, req: &super::telemetry::SubscribeActuatorControlTargetRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::telemetry::ActuatorControlTargetResponse>> {
        self.client.server_streaming(&METHOD_TELEMETRY_SERVICE_SUBSCRIBE_ACTUATOR_CONTROL_TARGET, req, opt)
    }

    pub fn subscribe_actuator_control_target(&self, req: &super::telemetry::SubscribeActuatorControlTargetRequest) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::telemetry::ActuatorControlTargetResponse>> {
        self.subscribe_actuator_control_target_opt(req, ::grpcio::CallOption::default())
    }

    pub fn subscribe_actuator_output_status_opt(&self, req: &super::telemetry::SubscribeActuatorOutputStatusRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::telemetry::ActuatorOutputStatusResponse>> {
        self.client.server_streaming(&METHOD_TELEMETRY_SERVICE_SUBSCRIBE_ACTUATOR_OUTPUT_STATUS, req, opt)
    }

    pub fn subscribe_actuator_output_status(&self, req: &super::telemetry::SubscribeActuatorOutputStatusRequest) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::telemetry::ActuatorOutputStatusResponse>> {
        self.subscribe_actuator_output_status_opt(req, ::grpcio::CallOption::default())
    }

    pub fn subscribe_odometry_opt(&self, req: &super::telemetry::SubscribeOdometryRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::telemetry::OdometryResponse>> {
        self.client.server_streaming(&METHOD_TELEMETRY_SERVICE_SUBSCRIBE_ODOMETRY, req, opt)
    }

    pub fn subscribe_odometry(&self, req: &super::telemetry::SubscribeOdometryRequest) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::telemetry::OdometryResponse>> {
        self.subscribe_odometry_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait TelemetryService {
    fn subscribe_position(&mut self, ctx: ::grpcio::RpcContext, req: super::telemetry::SubscribePositionRequest, sink: ::grpcio::ServerStreamingSink<super::telemetry::PositionResponse>);
    fn subscribe_home(&mut self, ctx: ::grpcio::RpcContext, req: super::telemetry::SubscribeHomeRequest, sink: ::grpcio::ServerStreamingSink<super::telemetry::HomeResponse>);
    fn subscribe_in_air(&mut self, ctx: ::grpcio::RpcContext, req: super::telemetry::SubscribeInAirRequest, sink: ::grpcio::ServerStreamingSink<super::telemetry::InAirResponse>);
    fn subscribe_landed_state(&mut self, ctx: ::grpcio::RpcContext, req: super::telemetry::SubscribeLandedStateRequest, sink: ::grpcio::ServerStreamingSink<super::telemetry::LandedStateResponse>);
    fn subscribe_armed(&mut self, ctx: ::grpcio::RpcContext, req: super::telemetry::SubscribeArmedRequest, sink: ::grpcio::ServerStreamingSink<super::telemetry::ArmedResponse>);
    fn subscribe_attitude_quaternion(&mut self, ctx: ::grpcio::RpcContext, req: super::telemetry::SubscribeAttitudeQuaternionRequest, sink: ::grpcio::ServerStreamingSink<super::telemetry::AttitudeQuaternionResponse>);
    fn subscribe_attitude_euler(&mut self, ctx: ::grpcio::RpcContext, req: super::telemetry::SubscribeAttitudeEulerRequest, sink: ::grpcio::ServerStreamingSink<super::telemetry::AttitudeEulerResponse>);
    fn subscribe_attitude_angular_velocity_body(&mut self, ctx: ::grpcio::RpcContext, req: super::telemetry::SubscribeAttitudeAngularVelocityBodyRequest, sink: ::grpcio::ServerStreamingSink<super::telemetry::AttitudeAngularVelocityBodyResponse>);
    fn subscribe_camera_attitude_quaternion(&mut self, ctx: ::grpcio::RpcContext, req: super::telemetry::SubscribeCameraAttitudeQuaternionRequest, sink: ::grpcio::ServerStreamingSink<super::telemetry::CameraAttitudeQuaternionResponse>);
    fn subscribe_camera_attitude_euler(&mut self, ctx: ::grpcio::RpcContext, req: super::telemetry::SubscribeCameraAttitudeEulerRequest, sink: ::grpcio::ServerStreamingSink<super::telemetry::CameraAttitudeEulerResponse>);
    fn subscribe_ground_speed_ned(&mut self, ctx: ::grpcio::RpcContext, req: super::telemetry::SubscribeGroundSpeedNedRequest, sink: ::grpcio::ServerStreamingSink<super::telemetry::GroundSpeedNedResponse>);
    fn subscribe_gps_info(&mut self, ctx: ::grpcio::RpcContext, req: super::telemetry::SubscribeGpsInfoRequest, sink: ::grpcio::ServerStreamingSink<super::telemetry::GpsInfoResponse>);
    fn subscribe_battery(&mut self, ctx: ::grpcio::RpcContext, req: super::telemetry::SubscribeBatteryRequest, sink: ::grpcio::ServerStreamingSink<super::telemetry::BatteryResponse>);
    fn subscribe_flight_mode(&mut self, ctx: ::grpcio::RpcContext, req: super::telemetry::SubscribeFlightModeRequest, sink: ::grpcio::ServerStreamingSink<super::telemetry::FlightModeResponse>);
    fn subscribe_health(&mut self, ctx: ::grpcio::RpcContext, req: super::telemetry::SubscribeHealthRequest, sink: ::grpcio::ServerStreamingSink<super::telemetry::HealthResponse>);
    fn subscribe_rc_status(&mut self, ctx: ::grpcio::RpcContext, req: super::telemetry::SubscribeRcStatusRequest, sink: ::grpcio::ServerStreamingSink<super::telemetry::RcStatusResponse>);
    fn subscribe_status_text(&mut self, ctx: ::grpcio::RpcContext, req: super::telemetry::SubscribeStatusTextRequest, sink: ::grpcio::ServerStreamingSink<super::telemetry::StatusTextResponse>);
    fn subscribe_actuator_control_target(&mut self, ctx: ::grpcio::RpcContext, req: super::telemetry::SubscribeActuatorControlTargetRequest, sink: ::grpcio::ServerStreamingSink<super::telemetry::ActuatorControlTargetResponse>);
    fn subscribe_actuator_output_status(&mut self, ctx: ::grpcio::RpcContext, req: super::telemetry::SubscribeActuatorOutputStatusRequest, sink: ::grpcio::ServerStreamingSink<super::telemetry::ActuatorOutputStatusResponse>);
    fn subscribe_odometry(&mut self, ctx: ::grpcio::RpcContext, req: super::telemetry::SubscribeOdometryRequest, sink: ::grpcio::ServerStreamingSink<super::telemetry::OdometryResponse>);
}

pub fn create_telemetry_service<S: TelemetryService + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_server_streaming_handler(&METHOD_TELEMETRY_SERVICE_SUBSCRIBE_POSITION, move |ctx, req, resp| {
        instance.subscribe_position(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_server_streaming_handler(&METHOD_TELEMETRY_SERVICE_SUBSCRIBE_HOME, move |ctx, req, resp| {
        instance.subscribe_home(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_server_streaming_handler(&METHOD_TELEMETRY_SERVICE_SUBSCRIBE_IN_AIR, move |ctx, req, resp| {
        instance.subscribe_in_air(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_server_streaming_handler(&METHOD_TELEMETRY_SERVICE_SUBSCRIBE_LANDED_STATE, move |ctx, req, resp| {
        instance.subscribe_landed_state(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_server_streaming_handler(&METHOD_TELEMETRY_SERVICE_SUBSCRIBE_ARMED, move |ctx, req, resp| {
        instance.subscribe_armed(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_server_streaming_handler(&METHOD_TELEMETRY_SERVICE_SUBSCRIBE_ATTITUDE_QUATERNION, move |ctx, req, resp| {
        instance.subscribe_attitude_quaternion(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_server_streaming_handler(&METHOD_TELEMETRY_SERVICE_SUBSCRIBE_ATTITUDE_EULER, move |ctx, req, resp| {
        instance.subscribe_attitude_euler(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_server_streaming_handler(&METHOD_TELEMETRY_SERVICE_SUBSCRIBE_ATTITUDE_ANGULAR_VELOCITY_BODY, move |ctx, req, resp| {
        instance.subscribe_attitude_angular_velocity_body(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_server_streaming_handler(&METHOD_TELEMETRY_SERVICE_SUBSCRIBE_CAMERA_ATTITUDE_QUATERNION, move |ctx, req, resp| {
        instance.subscribe_camera_attitude_quaternion(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_server_streaming_handler(&METHOD_TELEMETRY_SERVICE_SUBSCRIBE_CAMERA_ATTITUDE_EULER, move |ctx, req, resp| {
        instance.subscribe_camera_attitude_euler(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_server_streaming_handler(&METHOD_TELEMETRY_SERVICE_SUBSCRIBE_GROUND_SPEED_NED, move |ctx, req, resp| {
        instance.subscribe_ground_speed_ned(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_server_streaming_handler(&METHOD_TELEMETRY_SERVICE_SUBSCRIBE_GPS_INFO, move |ctx, req, resp| {
        instance.subscribe_gps_info(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_server_streaming_handler(&METHOD_TELEMETRY_SERVICE_SUBSCRIBE_BATTERY, move |ctx, req, resp| {
        instance.subscribe_battery(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_server_streaming_handler(&METHOD_TELEMETRY_SERVICE_SUBSCRIBE_FLIGHT_MODE, move |ctx, req, resp| {
        instance.subscribe_flight_mode(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_server_streaming_handler(&METHOD_TELEMETRY_SERVICE_SUBSCRIBE_HEALTH, move |ctx, req, resp| {
        instance.subscribe_health(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_server_streaming_handler(&METHOD_TELEMETRY_SERVICE_SUBSCRIBE_RC_STATUS, move |ctx, req, resp| {
        instance.subscribe_rc_status(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_server_streaming_handler(&METHOD_TELEMETRY_SERVICE_SUBSCRIBE_STATUS_TEXT, move |ctx, req, resp| {
        instance.subscribe_status_text(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_server_streaming_handler(&METHOD_TELEMETRY_SERVICE_SUBSCRIBE_ACTUATOR_CONTROL_TARGET, move |ctx, req, resp| {
        instance.subscribe_actuator_control_target(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_server_streaming_handler(&METHOD_TELEMETRY_SERVICE_SUBSCRIBE_ACTUATOR_OUTPUT_STATUS, move |ctx, req, resp| {
        instance.subscribe_actuator_output_status(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_server_streaming_handler(&METHOD_TELEMETRY_SERVICE_SUBSCRIBE_ODOMETRY, move |ctx, req, resp| {
        instance.subscribe_odometry(ctx, req, resp)
    });
    builder.build()
}
