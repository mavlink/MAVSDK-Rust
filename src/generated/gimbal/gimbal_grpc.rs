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

const METHOD_GIMBAL_SERVICE_SET_PITCH_AND_YAW: ::grpcio::Method<super::gimbal::SetPitchAndYawRequest, super::gimbal::SetPitchAndYawResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/mavsdk.rpc.gimbal.GimbalService/SetPitchAndYaw",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_GIMBAL_SERVICE_SET_MODE: ::grpcio::Method<super::gimbal::SetModeRequest, super::gimbal::SetModeResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/mavsdk.rpc.gimbal.GimbalService/SetMode",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct GimbalServiceClient {
    client: ::grpcio::Client,
}

impl GimbalServiceClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        GimbalServiceClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn set_pitch_and_yaw_opt(&self, req: &super::gimbal::SetPitchAndYawRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::gimbal::SetPitchAndYawResponse> {
        self.client.unary_call(&METHOD_GIMBAL_SERVICE_SET_PITCH_AND_YAW, req, opt)
    }

    pub fn set_pitch_and_yaw(&self, req: &super::gimbal::SetPitchAndYawRequest) -> ::grpcio::Result<super::gimbal::SetPitchAndYawResponse> {
        self.set_pitch_and_yaw_opt(req, ::grpcio::CallOption::default())
    }

    pub fn set_pitch_and_yaw_async_opt(&self, req: &super::gimbal::SetPitchAndYawRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::gimbal::SetPitchAndYawResponse>> {
        self.client.unary_call_async(&METHOD_GIMBAL_SERVICE_SET_PITCH_AND_YAW, req, opt)
    }

    pub fn set_pitch_and_yaw_async(&self, req: &super::gimbal::SetPitchAndYawRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::gimbal::SetPitchAndYawResponse>> {
        self.set_pitch_and_yaw_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn set_mode_opt(&self, req: &super::gimbal::SetModeRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::gimbal::SetModeResponse> {
        self.client.unary_call(&METHOD_GIMBAL_SERVICE_SET_MODE, req, opt)
    }

    pub fn set_mode(&self, req: &super::gimbal::SetModeRequest) -> ::grpcio::Result<super::gimbal::SetModeResponse> {
        self.set_mode_opt(req, ::grpcio::CallOption::default())
    }

    pub fn set_mode_async_opt(&self, req: &super::gimbal::SetModeRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::gimbal::SetModeResponse>> {
        self.client.unary_call_async(&METHOD_GIMBAL_SERVICE_SET_MODE, req, opt)
    }

    pub fn set_mode_async(&self, req: &super::gimbal::SetModeRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::gimbal::SetModeResponse>> {
        self.set_mode_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait GimbalService {
    fn set_pitch_and_yaw(&mut self, ctx: ::grpcio::RpcContext, req: super::gimbal::SetPitchAndYawRequest, sink: ::grpcio::UnarySink<super::gimbal::SetPitchAndYawResponse>);
    fn set_mode(&mut self, ctx: ::grpcio::RpcContext, req: super::gimbal::SetModeRequest, sink: ::grpcio::UnarySink<super::gimbal::SetModeResponse>);
}

pub fn create_gimbal_service<S: GimbalService + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_GIMBAL_SERVICE_SET_PITCH_AND_YAW, move |ctx, req, resp| {
        instance.set_pitch_and_yaw(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_GIMBAL_SERVICE_SET_MODE, move |ctx, req, resp| {
        instance.set_mode(ctx, req, resp)
    });
    builder.build()
}
