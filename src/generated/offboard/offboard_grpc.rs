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

const METHOD_OFFBOARD_SERVICE_START: ::grpcio::Method<super::offboard::StartRequest, super::offboard::StartResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/mavsdk.rpc.offboard.OffboardService/Start",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_OFFBOARD_SERVICE_STOP: ::grpcio::Method<super::offboard::StopRequest, super::offboard::StopResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/mavsdk.rpc.offboard.OffboardService/Stop",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_OFFBOARD_SERVICE_IS_ACTIVE: ::grpcio::Method<super::offboard::IsActiveRequest, super::offboard::IsActiveResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/mavsdk.rpc.offboard.OffboardService/IsActive",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_OFFBOARD_SERVICE_SET_ATTITUDE: ::grpcio::Method<super::offboard::SetAttitudeRequest, super::offboard::SetAttitudeResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/mavsdk.rpc.offboard.OffboardService/SetAttitude",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_OFFBOARD_SERVICE_SET_ACTUATOR_CONTROL: ::grpcio::Method<super::offboard::SetActuatorControlRequest, super::offboard::SetActuatorControlResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/mavsdk.rpc.offboard.OffboardService/SetActuatorControl",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_OFFBOARD_SERVICE_SET_ATTITUDE_RATE: ::grpcio::Method<super::offboard::SetAttitudeRateRequest, super::offboard::SetAttitudeRateResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/mavsdk.rpc.offboard.OffboardService/SetAttitudeRate",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_OFFBOARD_SERVICE_SET_POSITION_NED: ::grpcio::Method<super::offboard::SetPositionNedRequest, super::offboard::SetPositionNedResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/mavsdk.rpc.offboard.OffboardService/SetPositionNed",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_OFFBOARD_SERVICE_SET_VELOCITY_BODY: ::grpcio::Method<super::offboard::SetVelocityBodyRequest, super::offboard::SetVelocityBodyResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/mavsdk.rpc.offboard.OffboardService/SetVelocityBody",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_OFFBOARD_SERVICE_SET_VELOCITY_NED: ::grpcio::Method<super::offboard::SetVelocityNedRequest, super::offboard::SetVelocityNedResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/mavsdk.rpc.offboard.OffboardService/SetVelocityNed",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct OffboardServiceClient {
    client: ::grpcio::Client,
}

impl OffboardServiceClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        OffboardServiceClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn start_opt(&self, req: &super::offboard::StartRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::offboard::StartResponse> {
        self.client.unary_call(&METHOD_OFFBOARD_SERVICE_START, req, opt)
    }

    pub fn start(&self, req: &super::offboard::StartRequest) -> ::grpcio::Result<super::offboard::StartResponse> {
        self.start_opt(req, ::grpcio::CallOption::default())
    }

    pub fn start_async_opt(&self, req: &super::offboard::StartRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::offboard::StartResponse>> {
        self.client.unary_call_async(&METHOD_OFFBOARD_SERVICE_START, req, opt)
    }

    pub fn start_async(&self, req: &super::offboard::StartRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::offboard::StartResponse>> {
        self.start_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn stop_opt(&self, req: &super::offboard::StopRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::offboard::StopResponse> {
        self.client.unary_call(&METHOD_OFFBOARD_SERVICE_STOP, req, opt)
    }

    pub fn stop(&self, req: &super::offboard::StopRequest) -> ::grpcio::Result<super::offboard::StopResponse> {
        self.stop_opt(req, ::grpcio::CallOption::default())
    }

    pub fn stop_async_opt(&self, req: &super::offboard::StopRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::offboard::StopResponse>> {
        self.client.unary_call_async(&METHOD_OFFBOARD_SERVICE_STOP, req, opt)
    }

    pub fn stop_async(&self, req: &super::offboard::StopRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::offboard::StopResponse>> {
        self.stop_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn is_active_opt(&self, req: &super::offboard::IsActiveRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::offboard::IsActiveResponse> {
        self.client.unary_call(&METHOD_OFFBOARD_SERVICE_IS_ACTIVE, req, opt)
    }

    pub fn is_active(&self, req: &super::offboard::IsActiveRequest) -> ::grpcio::Result<super::offboard::IsActiveResponse> {
        self.is_active_opt(req, ::grpcio::CallOption::default())
    }

    pub fn is_active_async_opt(&self, req: &super::offboard::IsActiveRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::offboard::IsActiveResponse>> {
        self.client.unary_call_async(&METHOD_OFFBOARD_SERVICE_IS_ACTIVE, req, opt)
    }

    pub fn is_active_async(&self, req: &super::offboard::IsActiveRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::offboard::IsActiveResponse>> {
        self.is_active_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn set_attitude_opt(&self, req: &super::offboard::SetAttitudeRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::offboard::SetAttitudeResponse> {
        self.client.unary_call(&METHOD_OFFBOARD_SERVICE_SET_ATTITUDE, req, opt)
    }

    pub fn set_attitude(&self, req: &super::offboard::SetAttitudeRequest) -> ::grpcio::Result<super::offboard::SetAttitudeResponse> {
        self.set_attitude_opt(req, ::grpcio::CallOption::default())
    }

    pub fn set_attitude_async_opt(&self, req: &super::offboard::SetAttitudeRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::offboard::SetAttitudeResponse>> {
        self.client.unary_call_async(&METHOD_OFFBOARD_SERVICE_SET_ATTITUDE, req, opt)
    }

    pub fn set_attitude_async(&self, req: &super::offboard::SetAttitudeRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::offboard::SetAttitudeResponse>> {
        self.set_attitude_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn set_actuator_control_opt(&self, req: &super::offboard::SetActuatorControlRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::offboard::SetActuatorControlResponse> {
        self.client.unary_call(&METHOD_OFFBOARD_SERVICE_SET_ACTUATOR_CONTROL, req, opt)
    }

    pub fn set_actuator_control(&self, req: &super::offboard::SetActuatorControlRequest) -> ::grpcio::Result<super::offboard::SetActuatorControlResponse> {
        self.set_actuator_control_opt(req, ::grpcio::CallOption::default())
    }

    pub fn set_actuator_control_async_opt(&self, req: &super::offboard::SetActuatorControlRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::offboard::SetActuatorControlResponse>> {
        self.client.unary_call_async(&METHOD_OFFBOARD_SERVICE_SET_ACTUATOR_CONTROL, req, opt)
    }

    pub fn set_actuator_control_async(&self, req: &super::offboard::SetActuatorControlRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::offboard::SetActuatorControlResponse>> {
        self.set_actuator_control_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn set_attitude_rate_opt(&self, req: &super::offboard::SetAttitudeRateRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::offboard::SetAttitudeRateResponse> {
        self.client.unary_call(&METHOD_OFFBOARD_SERVICE_SET_ATTITUDE_RATE, req, opt)
    }

    pub fn set_attitude_rate(&self, req: &super::offboard::SetAttitudeRateRequest) -> ::grpcio::Result<super::offboard::SetAttitudeRateResponse> {
        self.set_attitude_rate_opt(req, ::grpcio::CallOption::default())
    }

    pub fn set_attitude_rate_async_opt(&self, req: &super::offboard::SetAttitudeRateRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::offboard::SetAttitudeRateResponse>> {
        self.client.unary_call_async(&METHOD_OFFBOARD_SERVICE_SET_ATTITUDE_RATE, req, opt)
    }

    pub fn set_attitude_rate_async(&self, req: &super::offboard::SetAttitudeRateRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::offboard::SetAttitudeRateResponse>> {
        self.set_attitude_rate_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn set_position_ned_opt(&self, req: &super::offboard::SetPositionNedRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::offboard::SetPositionNedResponse> {
        self.client.unary_call(&METHOD_OFFBOARD_SERVICE_SET_POSITION_NED, req, opt)
    }

    pub fn set_position_ned(&self, req: &super::offboard::SetPositionNedRequest) -> ::grpcio::Result<super::offboard::SetPositionNedResponse> {
        self.set_position_ned_opt(req, ::grpcio::CallOption::default())
    }

    pub fn set_position_ned_async_opt(&self, req: &super::offboard::SetPositionNedRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::offboard::SetPositionNedResponse>> {
        self.client.unary_call_async(&METHOD_OFFBOARD_SERVICE_SET_POSITION_NED, req, opt)
    }

    pub fn set_position_ned_async(&self, req: &super::offboard::SetPositionNedRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::offboard::SetPositionNedResponse>> {
        self.set_position_ned_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn set_velocity_body_opt(&self, req: &super::offboard::SetVelocityBodyRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::offboard::SetVelocityBodyResponse> {
        self.client.unary_call(&METHOD_OFFBOARD_SERVICE_SET_VELOCITY_BODY, req, opt)
    }

    pub fn set_velocity_body(&self, req: &super::offboard::SetVelocityBodyRequest) -> ::grpcio::Result<super::offboard::SetVelocityBodyResponse> {
        self.set_velocity_body_opt(req, ::grpcio::CallOption::default())
    }

    pub fn set_velocity_body_async_opt(&self, req: &super::offboard::SetVelocityBodyRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::offboard::SetVelocityBodyResponse>> {
        self.client.unary_call_async(&METHOD_OFFBOARD_SERVICE_SET_VELOCITY_BODY, req, opt)
    }

    pub fn set_velocity_body_async(&self, req: &super::offboard::SetVelocityBodyRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::offboard::SetVelocityBodyResponse>> {
        self.set_velocity_body_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn set_velocity_ned_opt(&self, req: &super::offboard::SetVelocityNedRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::offboard::SetVelocityNedResponse> {
        self.client.unary_call(&METHOD_OFFBOARD_SERVICE_SET_VELOCITY_NED, req, opt)
    }

    pub fn set_velocity_ned(&self, req: &super::offboard::SetVelocityNedRequest) -> ::grpcio::Result<super::offboard::SetVelocityNedResponse> {
        self.set_velocity_ned_opt(req, ::grpcio::CallOption::default())
    }

    pub fn set_velocity_ned_async_opt(&self, req: &super::offboard::SetVelocityNedRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::offboard::SetVelocityNedResponse>> {
        self.client.unary_call_async(&METHOD_OFFBOARD_SERVICE_SET_VELOCITY_NED, req, opt)
    }

    pub fn set_velocity_ned_async(&self, req: &super::offboard::SetVelocityNedRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::offboard::SetVelocityNedResponse>> {
        self.set_velocity_ned_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait OffboardService {
    fn start(&mut self, ctx: ::grpcio::RpcContext, req: super::offboard::StartRequest, sink: ::grpcio::UnarySink<super::offboard::StartResponse>);
    fn stop(&mut self, ctx: ::grpcio::RpcContext, req: super::offboard::StopRequest, sink: ::grpcio::UnarySink<super::offboard::StopResponse>);
    fn is_active(&mut self, ctx: ::grpcio::RpcContext, req: super::offboard::IsActiveRequest, sink: ::grpcio::UnarySink<super::offboard::IsActiveResponse>);
    fn set_attitude(&mut self, ctx: ::grpcio::RpcContext, req: super::offboard::SetAttitudeRequest, sink: ::grpcio::UnarySink<super::offboard::SetAttitudeResponse>);
    fn set_actuator_control(&mut self, ctx: ::grpcio::RpcContext, req: super::offboard::SetActuatorControlRequest, sink: ::grpcio::UnarySink<super::offboard::SetActuatorControlResponse>);
    fn set_attitude_rate(&mut self, ctx: ::grpcio::RpcContext, req: super::offboard::SetAttitudeRateRequest, sink: ::grpcio::UnarySink<super::offboard::SetAttitudeRateResponse>);
    fn set_position_ned(&mut self, ctx: ::grpcio::RpcContext, req: super::offboard::SetPositionNedRequest, sink: ::grpcio::UnarySink<super::offboard::SetPositionNedResponse>);
    fn set_velocity_body(&mut self, ctx: ::grpcio::RpcContext, req: super::offboard::SetVelocityBodyRequest, sink: ::grpcio::UnarySink<super::offboard::SetVelocityBodyResponse>);
    fn set_velocity_ned(&mut self, ctx: ::grpcio::RpcContext, req: super::offboard::SetVelocityNedRequest, sink: ::grpcio::UnarySink<super::offboard::SetVelocityNedResponse>);
}

pub fn create_offboard_service<S: OffboardService + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_OFFBOARD_SERVICE_START, move |ctx, req, resp| {
        instance.start(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_OFFBOARD_SERVICE_STOP, move |ctx, req, resp| {
        instance.stop(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_OFFBOARD_SERVICE_IS_ACTIVE, move |ctx, req, resp| {
        instance.is_active(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_OFFBOARD_SERVICE_SET_ATTITUDE, move |ctx, req, resp| {
        instance.set_attitude(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_OFFBOARD_SERVICE_SET_ACTUATOR_CONTROL, move |ctx, req, resp| {
        instance.set_actuator_control(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_OFFBOARD_SERVICE_SET_ATTITUDE_RATE, move |ctx, req, resp| {
        instance.set_attitude_rate(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_OFFBOARD_SERVICE_SET_POSITION_NED, move |ctx, req, resp| {
        instance.set_position_ned(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_OFFBOARD_SERVICE_SET_VELOCITY_BODY, move |ctx, req, resp| {
        instance.set_velocity_body(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_OFFBOARD_SERVICE_SET_VELOCITY_NED, move |ctx, req, resp| {
        instance.set_velocity_ned(ctx, req, resp)
    });
    builder.build()
}
