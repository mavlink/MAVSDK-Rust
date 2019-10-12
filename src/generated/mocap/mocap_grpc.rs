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

const METHOD_MOCAP_SERVICE_SET_VISION_POSITION_ESTIMATE: ::grpcio::Method<super::mocap::SetVisionPositionEstimateRequest, super::mocap::SetVisionPositionEstimateResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/mavsdk.rpc.mocap.MocapService/SetVisionPositionEstimate",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_MOCAP_SERVICE_SET_ATTITUDE_POSITION_MOCAP: ::grpcio::Method<super::mocap::SetAttitudePositionMocapRequest, super::mocap::SetAttitudePositionMocapResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/mavsdk.rpc.mocap.MocapService/SetAttitudePositionMocap",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_MOCAP_SERVICE_SET_ODOMETRY: ::grpcio::Method<super::mocap::SetOdometryRequest, super::mocap::SetOdometryResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/mavsdk.rpc.mocap.MocapService/SetOdometry",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct MocapServiceClient {
    client: ::grpcio::Client,
}

impl MocapServiceClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        MocapServiceClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn set_vision_position_estimate_opt(&self, req: &super::mocap::SetVisionPositionEstimateRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::mocap::SetVisionPositionEstimateResponse> {
        self.client.unary_call(&METHOD_MOCAP_SERVICE_SET_VISION_POSITION_ESTIMATE, req, opt)
    }

    pub fn set_vision_position_estimate(&self, req: &super::mocap::SetVisionPositionEstimateRequest) -> ::grpcio::Result<super::mocap::SetVisionPositionEstimateResponse> {
        self.set_vision_position_estimate_opt(req, ::grpcio::CallOption::default())
    }

    pub fn set_vision_position_estimate_async_opt(&self, req: &super::mocap::SetVisionPositionEstimateRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::mocap::SetVisionPositionEstimateResponse>> {
        self.client.unary_call_async(&METHOD_MOCAP_SERVICE_SET_VISION_POSITION_ESTIMATE, req, opt)
    }

    pub fn set_vision_position_estimate_async(&self, req: &super::mocap::SetVisionPositionEstimateRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::mocap::SetVisionPositionEstimateResponse>> {
        self.set_vision_position_estimate_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn set_attitude_position_mocap_opt(&self, req: &super::mocap::SetAttitudePositionMocapRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::mocap::SetAttitudePositionMocapResponse> {
        self.client.unary_call(&METHOD_MOCAP_SERVICE_SET_ATTITUDE_POSITION_MOCAP, req, opt)
    }

    pub fn set_attitude_position_mocap(&self, req: &super::mocap::SetAttitudePositionMocapRequest) -> ::grpcio::Result<super::mocap::SetAttitudePositionMocapResponse> {
        self.set_attitude_position_mocap_opt(req, ::grpcio::CallOption::default())
    }

    pub fn set_attitude_position_mocap_async_opt(&self, req: &super::mocap::SetAttitudePositionMocapRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::mocap::SetAttitudePositionMocapResponse>> {
        self.client.unary_call_async(&METHOD_MOCAP_SERVICE_SET_ATTITUDE_POSITION_MOCAP, req, opt)
    }

    pub fn set_attitude_position_mocap_async(&self, req: &super::mocap::SetAttitudePositionMocapRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::mocap::SetAttitudePositionMocapResponse>> {
        self.set_attitude_position_mocap_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn set_odometry_opt(&self, req: &super::mocap::SetOdometryRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::mocap::SetOdometryResponse> {
        self.client.unary_call(&METHOD_MOCAP_SERVICE_SET_ODOMETRY, req, opt)
    }

    pub fn set_odometry(&self, req: &super::mocap::SetOdometryRequest) -> ::grpcio::Result<super::mocap::SetOdometryResponse> {
        self.set_odometry_opt(req, ::grpcio::CallOption::default())
    }

    pub fn set_odometry_async_opt(&self, req: &super::mocap::SetOdometryRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::mocap::SetOdometryResponse>> {
        self.client.unary_call_async(&METHOD_MOCAP_SERVICE_SET_ODOMETRY, req, opt)
    }

    pub fn set_odometry_async(&self, req: &super::mocap::SetOdometryRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::mocap::SetOdometryResponse>> {
        self.set_odometry_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait MocapService {
    fn set_vision_position_estimate(&mut self, ctx: ::grpcio::RpcContext, req: super::mocap::SetVisionPositionEstimateRequest, sink: ::grpcio::UnarySink<super::mocap::SetVisionPositionEstimateResponse>);
    fn set_attitude_position_mocap(&mut self, ctx: ::grpcio::RpcContext, req: super::mocap::SetAttitudePositionMocapRequest, sink: ::grpcio::UnarySink<super::mocap::SetAttitudePositionMocapResponse>);
    fn set_odometry(&mut self, ctx: ::grpcio::RpcContext, req: super::mocap::SetOdometryRequest, sink: ::grpcio::UnarySink<super::mocap::SetOdometryResponse>);
}

pub fn create_mocap_service<S: MocapService + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_MOCAP_SERVICE_SET_VISION_POSITION_ESTIMATE, move |ctx, req, resp| {
        instance.set_vision_position_estimate(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_MOCAP_SERVICE_SET_ATTITUDE_POSITION_MOCAP, move |ctx, req, resp| {
        instance.set_attitude_position_mocap(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_MOCAP_SERVICE_SET_ODOMETRY, move |ctx, req, resp| {
        instance.set_odometry(ctx, req, resp)
    });
    builder.build()
}
