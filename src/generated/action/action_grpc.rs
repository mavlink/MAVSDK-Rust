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

const METHOD_ACTION_SERVICE_ARM: ::grpcio::Method<super::action::ArmRequest, super::action::ArmResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/mavsdk.rpc.action.ActionService/Arm",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_ACTION_SERVICE_DISARM: ::grpcio::Method<super::action::DisarmRequest, super::action::DisarmResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/mavsdk.rpc.action.ActionService/Disarm",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_ACTION_SERVICE_TAKEOFF: ::grpcio::Method<super::action::TakeoffRequest, super::action::TakeoffResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/mavsdk.rpc.action.ActionService/Takeoff",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_ACTION_SERVICE_LAND: ::grpcio::Method<super::action::LandRequest, super::action::LandResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/mavsdk.rpc.action.ActionService/Land",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_ACTION_SERVICE_REBOOT: ::grpcio::Method<super::action::RebootRequest, super::action::RebootResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/mavsdk.rpc.action.ActionService/Reboot",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_ACTION_SERVICE_KILL: ::grpcio::Method<super::action::KillRequest, super::action::KillResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/mavsdk.rpc.action.ActionService/Kill",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_ACTION_SERVICE_RETURN_TO_LAUNCH: ::grpcio::Method<super::action::ReturnToLaunchRequest, super::action::ReturnToLaunchResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/mavsdk.rpc.action.ActionService/ReturnToLaunch",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_ACTION_SERVICE_TRANSITION_TO_FIXED_WING: ::grpcio::Method<super::action::TransitionToFixedWingRequest, super::action::TransitionToFixedWingResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/mavsdk.rpc.action.ActionService/TransitionToFixedWing",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_ACTION_SERVICE_TRANSITION_TO_MULTICOPTER: ::grpcio::Method<super::action::TransitionToMulticopterRequest, super::action::TransitionToMulticopterResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/mavsdk.rpc.action.ActionService/TransitionToMulticopter",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_ACTION_SERVICE_GET_TAKEOFF_ALTITUDE: ::grpcio::Method<super::action::GetTakeoffAltitudeRequest, super::action::GetTakeoffAltitudeResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/mavsdk.rpc.action.ActionService/GetTakeoffAltitude",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_ACTION_SERVICE_SET_TAKEOFF_ALTITUDE: ::grpcio::Method<super::action::SetTakeoffAltitudeRequest, super::action::SetTakeoffAltitudeResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/mavsdk.rpc.action.ActionService/SetTakeoffAltitude",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_ACTION_SERVICE_GET_MAXIMUM_SPEED: ::grpcio::Method<super::action::GetMaximumSpeedRequest, super::action::GetMaximumSpeedResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/mavsdk.rpc.action.ActionService/GetMaximumSpeed",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_ACTION_SERVICE_SET_MAXIMUM_SPEED: ::grpcio::Method<super::action::SetMaximumSpeedRequest, super::action::SetMaximumSpeedResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/mavsdk.rpc.action.ActionService/SetMaximumSpeed",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_ACTION_SERVICE_GET_RETURN_TO_LAUNCH_ALTITUDE: ::grpcio::Method<super::action::GetReturnToLaunchAltitudeRequest, super::action::GetReturnToLaunchAltitudeResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/mavsdk.rpc.action.ActionService/GetReturnToLaunchAltitude",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_ACTION_SERVICE_SET_RETURN_TO_LAUNCH_ALTITUDE: ::grpcio::Method<super::action::SetReturnToLaunchAltitudeRequest, super::action::SetReturnToLaunchAltitudeResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/mavsdk.rpc.action.ActionService/SetReturnToLaunchAltitude",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct ActionServiceClient {
    client: ::grpcio::Client,
}

impl ActionServiceClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        ActionServiceClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn arm_opt(&self, req: &super::action::ArmRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::action::ArmResponse> {
        self.client.unary_call(&METHOD_ACTION_SERVICE_ARM, req, opt)
    }

    pub fn arm(&self, req: &super::action::ArmRequest) -> ::grpcio::Result<super::action::ArmResponse> {
        self.arm_opt(req, ::grpcio::CallOption::default())
    }

    pub fn arm_async_opt(&self, req: &super::action::ArmRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::action::ArmResponse>> {
        self.client.unary_call_async(&METHOD_ACTION_SERVICE_ARM, req, opt)
    }

    pub fn arm_async(&self, req: &super::action::ArmRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::action::ArmResponse>> {
        self.arm_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn disarm_opt(&self, req: &super::action::DisarmRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::action::DisarmResponse> {
        self.client.unary_call(&METHOD_ACTION_SERVICE_DISARM, req, opt)
    }

    pub fn disarm(&self, req: &super::action::DisarmRequest) -> ::grpcio::Result<super::action::DisarmResponse> {
        self.disarm_opt(req, ::grpcio::CallOption::default())
    }

    pub fn disarm_async_opt(&self, req: &super::action::DisarmRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::action::DisarmResponse>> {
        self.client.unary_call_async(&METHOD_ACTION_SERVICE_DISARM, req, opt)
    }

    pub fn disarm_async(&self, req: &super::action::DisarmRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::action::DisarmResponse>> {
        self.disarm_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn takeoff_opt(&self, req: &super::action::TakeoffRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::action::TakeoffResponse> {
        self.client.unary_call(&METHOD_ACTION_SERVICE_TAKEOFF, req, opt)
    }

    pub fn takeoff(&self, req: &super::action::TakeoffRequest) -> ::grpcio::Result<super::action::TakeoffResponse> {
        self.takeoff_opt(req, ::grpcio::CallOption::default())
    }

    pub fn takeoff_async_opt(&self, req: &super::action::TakeoffRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::action::TakeoffResponse>> {
        self.client.unary_call_async(&METHOD_ACTION_SERVICE_TAKEOFF, req, opt)
    }

    pub fn takeoff_async(&self, req: &super::action::TakeoffRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::action::TakeoffResponse>> {
        self.takeoff_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn land_opt(&self, req: &super::action::LandRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::action::LandResponse> {
        self.client.unary_call(&METHOD_ACTION_SERVICE_LAND, req, opt)
    }

    pub fn land(&self, req: &super::action::LandRequest) -> ::grpcio::Result<super::action::LandResponse> {
        self.land_opt(req, ::grpcio::CallOption::default())
    }

    pub fn land_async_opt(&self, req: &super::action::LandRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::action::LandResponse>> {
        self.client.unary_call_async(&METHOD_ACTION_SERVICE_LAND, req, opt)
    }

    pub fn land_async(&self, req: &super::action::LandRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::action::LandResponse>> {
        self.land_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn reboot_opt(&self, req: &super::action::RebootRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::action::RebootResponse> {
        self.client.unary_call(&METHOD_ACTION_SERVICE_REBOOT, req, opt)
    }

    pub fn reboot(&self, req: &super::action::RebootRequest) -> ::grpcio::Result<super::action::RebootResponse> {
        self.reboot_opt(req, ::grpcio::CallOption::default())
    }

    pub fn reboot_async_opt(&self, req: &super::action::RebootRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::action::RebootResponse>> {
        self.client.unary_call_async(&METHOD_ACTION_SERVICE_REBOOT, req, opt)
    }

    pub fn reboot_async(&self, req: &super::action::RebootRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::action::RebootResponse>> {
        self.reboot_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn kill_opt(&self, req: &super::action::KillRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::action::KillResponse> {
        self.client.unary_call(&METHOD_ACTION_SERVICE_KILL, req, opt)
    }

    pub fn kill(&self, req: &super::action::KillRequest) -> ::grpcio::Result<super::action::KillResponse> {
        self.kill_opt(req, ::grpcio::CallOption::default())
    }

    pub fn kill_async_opt(&self, req: &super::action::KillRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::action::KillResponse>> {
        self.client.unary_call_async(&METHOD_ACTION_SERVICE_KILL, req, opt)
    }

    pub fn kill_async(&self, req: &super::action::KillRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::action::KillResponse>> {
        self.kill_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn return_to_launch_opt(&self, req: &super::action::ReturnToLaunchRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::action::ReturnToLaunchResponse> {
        self.client.unary_call(&METHOD_ACTION_SERVICE_RETURN_TO_LAUNCH, req, opt)
    }

    pub fn return_to_launch(&self, req: &super::action::ReturnToLaunchRequest) -> ::grpcio::Result<super::action::ReturnToLaunchResponse> {
        self.return_to_launch_opt(req, ::grpcio::CallOption::default())
    }

    pub fn return_to_launch_async_opt(&self, req: &super::action::ReturnToLaunchRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::action::ReturnToLaunchResponse>> {
        self.client.unary_call_async(&METHOD_ACTION_SERVICE_RETURN_TO_LAUNCH, req, opt)
    }

    pub fn return_to_launch_async(&self, req: &super::action::ReturnToLaunchRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::action::ReturnToLaunchResponse>> {
        self.return_to_launch_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn transition_to_fixed_wing_opt(&self, req: &super::action::TransitionToFixedWingRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::action::TransitionToFixedWingResponse> {
        self.client.unary_call(&METHOD_ACTION_SERVICE_TRANSITION_TO_FIXED_WING, req, opt)
    }

    pub fn transition_to_fixed_wing(&self, req: &super::action::TransitionToFixedWingRequest) -> ::grpcio::Result<super::action::TransitionToFixedWingResponse> {
        self.transition_to_fixed_wing_opt(req, ::grpcio::CallOption::default())
    }

    pub fn transition_to_fixed_wing_async_opt(&self, req: &super::action::TransitionToFixedWingRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::action::TransitionToFixedWingResponse>> {
        self.client.unary_call_async(&METHOD_ACTION_SERVICE_TRANSITION_TO_FIXED_WING, req, opt)
    }

    pub fn transition_to_fixed_wing_async(&self, req: &super::action::TransitionToFixedWingRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::action::TransitionToFixedWingResponse>> {
        self.transition_to_fixed_wing_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn transition_to_multicopter_opt(&self, req: &super::action::TransitionToMulticopterRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::action::TransitionToMulticopterResponse> {
        self.client.unary_call(&METHOD_ACTION_SERVICE_TRANSITION_TO_MULTICOPTER, req, opt)
    }

    pub fn transition_to_multicopter(&self, req: &super::action::TransitionToMulticopterRequest) -> ::grpcio::Result<super::action::TransitionToMulticopterResponse> {
        self.transition_to_multicopter_opt(req, ::grpcio::CallOption::default())
    }

    pub fn transition_to_multicopter_async_opt(&self, req: &super::action::TransitionToMulticopterRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::action::TransitionToMulticopterResponse>> {
        self.client.unary_call_async(&METHOD_ACTION_SERVICE_TRANSITION_TO_MULTICOPTER, req, opt)
    }

    pub fn transition_to_multicopter_async(&self, req: &super::action::TransitionToMulticopterRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::action::TransitionToMulticopterResponse>> {
        self.transition_to_multicopter_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_takeoff_altitude_opt(&self, req: &super::action::GetTakeoffAltitudeRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::action::GetTakeoffAltitudeResponse> {
        self.client.unary_call(&METHOD_ACTION_SERVICE_GET_TAKEOFF_ALTITUDE, req, opt)
    }

    pub fn get_takeoff_altitude(&self, req: &super::action::GetTakeoffAltitudeRequest) -> ::grpcio::Result<super::action::GetTakeoffAltitudeResponse> {
        self.get_takeoff_altitude_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_takeoff_altitude_async_opt(&self, req: &super::action::GetTakeoffAltitudeRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::action::GetTakeoffAltitudeResponse>> {
        self.client.unary_call_async(&METHOD_ACTION_SERVICE_GET_TAKEOFF_ALTITUDE, req, opt)
    }

    pub fn get_takeoff_altitude_async(&self, req: &super::action::GetTakeoffAltitudeRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::action::GetTakeoffAltitudeResponse>> {
        self.get_takeoff_altitude_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn set_takeoff_altitude_opt(&self, req: &super::action::SetTakeoffAltitudeRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::action::SetTakeoffAltitudeResponse> {
        self.client.unary_call(&METHOD_ACTION_SERVICE_SET_TAKEOFF_ALTITUDE, req, opt)
    }

    pub fn set_takeoff_altitude(&self, req: &super::action::SetTakeoffAltitudeRequest) -> ::grpcio::Result<super::action::SetTakeoffAltitudeResponse> {
        self.set_takeoff_altitude_opt(req, ::grpcio::CallOption::default())
    }

    pub fn set_takeoff_altitude_async_opt(&self, req: &super::action::SetTakeoffAltitudeRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::action::SetTakeoffAltitudeResponse>> {
        self.client.unary_call_async(&METHOD_ACTION_SERVICE_SET_TAKEOFF_ALTITUDE, req, opt)
    }

    pub fn set_takeoff_altitude_async(&self, req: &super::action::SetTakeoffAltitudeRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::action::SetTakeoffAltitudeResponse>> {
        self.set_takeoff_altitude_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_maximum_speed_opt(&self, req: &super::action::GetMaximumSpeedRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::action::GetMaximumSpeedResponse> {
        self.client.unary_call(&METHOD_ACTION_SERVICE_GET_MAXIMUM_SPEED, req, opt)
    }

    pub fn get_maximum_speed(&self, req: &super::action::GetMaximumSpeedRequest) -> ::grpcio::Result<super::action::GetMaximumSpeedResponse> {
        self.get_maximum_speed_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_maximum_speed_async_opt(&self, req: &super::action::GetMaximumSpeedRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::action::GetMaximumSpeedResponse>> {
        self.client.unary_call_async(&METHOD_ACTION_SERVICE_GET_MAXIMUM_SPEED, req, opt)
    }

    pub fn get_maximum_speed_async(&self, req: &super::action::GetMaximumSpeedRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::action::GetMaximumSpeedResponse>> {
        self.get_maximum_speed_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn set_maximum_speed_opt(&self, req: &super::action::SetMaximumSpeedRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::action::SetMaximumSpeedResponse> {
        self.client.unary_call(&METHOD_ACTION_SERVICE_SET_MAXIMUM_SPEED, req, opt)
    }

    pub fn set_maximum_speed(&self, req: &super::action::SetMaximumSpeedRequest) -> ::grpcio::Result<super::action::SetMaximumSpeedResponse> {
        self.set_maximum_speed_opt(req, ::grpcio::CallOption::default())
    }

    pub fn set_maximum_speed_async_opt(&self, req: &super::action::SetMaximumSpeedRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::action::SetMaximumSpeedResponse>> {
        self.client.unary_call_async(&METHOD_ACTION_SERVICE_SET_MAXIMUM_SPEED, req, opt)
    }

    pub fn set_maximum_speed_async(&self, req: &super::action::SetMaximumSpeedRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::action::SetMaximumSpeedResponse>> {
        self.set_maximum_speed_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_return_to_launch_altitude_opt(&self, req: &super::action::GetReturnToLaunchAltitudeRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::action::GetReturnToLaunchAltitudeResponse> {
        self.client.unary_call(&METHOD_ACTION_SERVICE_GET_RETURN_TO_LAUNCH_ALTITUDE, req, opt)
    }

    pub fn get_return_to_launch_altitude(&self, req: &super::action::GetReturnToLaunchAltitudeRequest) -> ::grpcio::Result<super::action::GetReturnToLaunchAltitudeResponse> {
        self.get_return_to_launch_altitude_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_return_to_launch_altitude_async_opt(&self, req: &super::action::GetReturnToLaunchAltitudeRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::action::GetReturnToLaunchAltitudeResponse>> {
        self.client.unary_call_async(&METHOD_ACTION_SERVICE_GET_RETURN_TO_LAUNCH_ALTITUDE, req, opt)
    }

    pub fn get_return_to_launch_altitude_async(&self, req: &super::action::GetReturnToLaunchAltitudeRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::action::GetReturnToLaunchAltitudeResponse>> {
        self.get_return_to_launch_altitude_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn set_return_to_launch_altitude_opt(&self, req: &super::action::SetReturnToLaunchAltitudeRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::action::SetReturnToLaunchAltitudeResponse> {
        self.client.unary_call(&METHOD_ACTION_SERVICE_SET_RETURN_TO_LAUNCH_ALTITUDE, req, opt)
    }

    pub fn set_return_to_launch_altitude(&self, req: &super::action::SetReturnToLaunchAltitudeRequest) -> ::grpcio::Result<super::action::SetReturnToLaunchAltitudeResponse> {
        self.set_return_to_launch_altitude_opt(req, ::grpcio::CallOption::default())
    }

    pub fn set_return_to_launch_altitude_async_opt(&self, req: &super::action::SetReturnToLaunchAltitudeRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::action::SetReturnToLaunchAltitudeResponse>> {
        self.client.unary_call_async(&METHOD_ACTION_SERVICE_SET_RETURN_TO_LAUNCH_ALTITUDE, req, opt)
    }

    pub fn set_return_to_launch_altitude_async(&self, req: &super::action::SetReturnToLaunchAltitudeRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::action::SetReturnToLaunchAltitudeResponse>> {
        self.set_return_to_launch_altitude_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait ActionService {
    fn arm(&mut self, ctx: ::grpcio::RpcContext, req: super::action::ArmRequest, sink: ::grpcio::UnarySink<super::action::ArmResponse>);
    fn disarm(&mut self, ctx: ::grpcio::RpcContext, req: super::action::DisarmRequest, sink: ::grpcio::UnarySink<super::action::DisarmResponse>);
    fn takeoff(&mut self, ctx: ::grpcio::RpcContext, req: super::action::TakeoffRequest, sink: ::grpcio::UnarySink<super::action::TakeoffResponse>);
    fn land(&mut self, ctx: ::grpcio::RpcContext, req: super::action::LandRequest, sink: ::grpcio::UnarySink<super::action::LandResponse>);
    fn reboot(&mut self, ctx: ::grpcio::RpcContext, req: super::action::RebootRequest, sink: ::grpcio::UnarySink<super::action::RebootResponse>);
    fn kill(&mut self, ctx: ::grpcio::RpcContext, req: super::action::KillRequest, sink: ::grpcio::UnarySink<super::action::KillResponse>);
    fn return_to_launch(&mut self, ctx: ::grpcio::RpcContext, req: super::action::ReturnToLaunchRequest, sink: ::grpcio::UnarySink<super::action::ReturnToLaunchResponse>);
    fn transition_to_fixed_wing(&mut self, ctx: ::grpcio::RpcContext, req: super::action::TransitionToFixedWingRequest, sink: ::grpcio::UnarySink<super::action::TransitionToFixedWingResponse>);
    fn transition_to_multicopter(&mut self, ctx: ::grpcio::RpcContext, req: super::action::TransitionToMulticopterRequest, sink: ::grpcio::UnarySink<super::action::TransitionToMulticopterResponse>);
    fn get_takeoff_altitude(&mut self, ctx: ::grpcio::RpcContext, req: super::action::GetTakeoffAltitudeRequest, sink: ::grpcio::UnarySink<super::action::GetTakeoffAltitudeResponse>);
    fn set_takeoff_altitude(&mut self, ctx: ::grpcio::RpcContext, req: super::action::SetTakeoffAltitudeRequest, sink: ::grpcio::UnarySink<super::action::SetTakeoffAltitudeResponse>);
    fn get_maximum_speed(&mut self, ctx: ::grpcio::RpcContext, req: super::action::GetMaximumSpeedRequest, sink: ::grpcio::UnarySink<super::action::GetMaximumSpeedResponse>);
    fn set_maximum_speed(&mut self, ctx: ::grpcio::RpcContext, req: super::action::SetMaximumSpeedRequest, sink: ::grpcio::UnarySink<super::action::SetMaximumSpeedResponse>);
    fn get_return_to_launch_altitude(&mut self, ctx: ::grpcio::RpcContext, req: super::action::GetReturnToLaunchAltitudeRequest, sink: ::grpcio::UnarySink<super::action::GetReturnToLaunchAltitudeResponse>);
    fn set_return_to_launch_altitude(&mut self, ctx: ::grpcio::RpcContext, req: super::action::SetReturnToLaunchAltitudeRequest, sink: ::grpcio::UnarySink<super::action::SetReturnToLaunchAltitudeResponse>);
}

pub fn create_action_service<S: ActionService + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_ACTION_SERVICE_ARM, move |ctx, req, resp| {
        instance.arm(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_ACTION_SERVICE_DISARM, move |ctx, req, resp| {
        instance.disarm(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_ACTION_SERVICE_TAKEOFF, move |ctx, req, resp| {
        instance.takeoff(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_ACTION_SERVICE_LAND, move |ctx, req, resp| {
        instance.land(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_ACTION_SERVICE_REBOOT, move |ctx, req, resp| {
        instance.reboot(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_ACTION_SERVICE_KILL, move |ctx, req, resp| {
        instance.kill(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_ACTION_SERVICE_RETURN_TO_LAUNCH, move |ctx, req, resp| {
        instance.return_to_launch(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_ACTION_SERVICE_TRANSITION_TO_FIXED_WING, move |ctx, req, resp| {
        instance.transition_to_fixed_wing(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_ACTION_SERVICE_TRANSITION_TO_MULTICOPTER, move |ctx, req, resp| {
        instance.transition_to_multicopter(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_ACTION_SERVICE_GET_TAKEOFF_ALTITUDE, move |ctx, req, resp| {
        instance.get_takeoff_altitude(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_ACTION_SERVICE_SET_TAKEOFF_ALTITUDE, move |ctx, req, resp| {
        instance.set_takeoff_altitude(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_ACTION_SERVICE_GET_MAXIMUM_SPEED, move |ctx, req, resp| {
        instance.get_maximum_speed(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_ACTION_SERVICE_SET_MAXIMUM_SPEED, move |ctx, req, resp| {
        instance.set_maximum_speed(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_ACTION_SERVICE_GET_RETURN_TO_LAUNCH_ALTITUDE, move |ctx, req, resp| {
        instance.get_return_to_launch_altitude(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_ACTION_SERVICE_SET_RETURN_TO_LAUNCH_ALTITUDE, move |ctx, req, resp| {
        instance.set_return_to_launch_altitude(ctx, req, resp)
    });
    builder.build()
}
