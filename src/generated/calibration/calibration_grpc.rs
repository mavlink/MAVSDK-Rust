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

const METHOD_CALIBRATION_SERVICE_SUBSCRIBE_CALIBRATE_GYRO: ::grpcio::Method<super::calibration::SubscribeCalibrateGyroRequest, super::calibration::CalibrateGyroResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ServerStreaming,
    name: "/mavsdk.rpc.calibration.CalibrationService/SubscribeCalibrateGyro",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_CALIBRATION_SERVICE_SUBSCRIBE_CALIBRATE_ACCELEROMETER: ::grpcio::Method<super::calibration::SubscribeCalibrateAccelerometerRequest, super::calibration::CalibrateAccelerometerResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ServerStreaming,
    name: "/mavsdk.rpc.calibration.CalibrationService/SubscribeCalibrateAccelerometer",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_CALIBRATION_SERVICE_SUBSCRIBE_CALIBRATE_MAGNETOMETER: ::grpcio::Method<super::calibration::SubscribeCalibrateMagnetometerRequest, super::calibration::CalibrateMagnetometerResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ServerStreaming,
    name: "/mavsdk.rpc.calibration.CalibrationService/SubscribeCalibrateMagnetometer",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_CALIBRATION_SERVICE_SUBSCRIBE_CALIBRATE_GIMBAL_ACCELEROMETER: ::grpcio::Method<super::calibration::SubscribeCalibrateGimbalAccelerometerRequest, super::calibration::CalibrateGimbalAccelerometerResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ServerStreaming,
    name: "/mavsdk.rpc.calibration.CalibrationService/SubscribeCalibrateGimbalAccelerometer",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_CALIBRATION_SERVICE_CANCEL: ::grpcio::Method<super::calibration::CancelRequest, super::calibration::CancelResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/mavsdk.rpc.calibration.CalibrationService/Cancel",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct CalibrationServiceClient {
    client: ::grpcio::Client,
}

impl CalibrationServiceClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        CalibrationServiceClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn subscribe_calibrate_gyro_opt(&self, req: &super::calibration::SubscribeCalibrateGyroRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::calibration::CalibrateGyroResponse>> {
        self.client.server_streaming(&METHOD_CALIBRATION_SERVICE_SUBSCRIBE_CALIBRATE_GYRO, req, opt)
    }

    pub fn subscribe_calibrate_gyro(&self, req: &super::calibration::SubscribeCalibrateGyroRequest) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::calibration::CalibrateGyroResponse>> {
        self.subscribe_calibrate_gyro_opt(req, ::grpcio::CallOption::default())
    }

    pub fn subscribe_calibrate_accelerometer_opt(&self, req: &super::calibration::SubscribeCalibrateAccelerometerRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::calibration::CalibrateAccelerometerResponse>> {
        self.client.server_streaming(&METHOD_CALIBRATION_SERVICE_SUBSCRIBE_CALIBRATE_ACCELEROMETER, req, opt)
    }

    pub fn subscribe_calibrate_accelerometer(&self, req: &super::calibration::SubscribeCalibrateAccelerometerRequest) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::calibration::CalibrateAccelerometerResponse>> {
        self.subscribe_calibrate_accelerometer_opt(req, ::grpcio::CallOption::default())
    }

    pub fn subscribe_calibrate_magnetometer_opt(&self, req: &super::calibration::SubscribeCalibrateMagnetometerRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::calibration::CalibrateMagnetometerResponse>> {
        self.client.server_streaming(&METHOD_CALIBRATION_SERVICE_SUBSCRIBE_CALIBRATE_MAGNETOMETER, req, opt)
    }

    pub fn subscribe_calibrate_magnetometer(&self, req: &super::calibration::SubscribeCalibrateMagnetometerRequest) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::calibration::CalibrateMagnetometerResponse>> {
        self.subscribe_calibrate_magnetometer_opt(req, ::grpcio::CallOption::default())
    }

    pub fn subscribe_calibrate_gimbal_accelerometer_opt(&self, req: &super::calibration::SubscribeCalibrateGimbalAccelerometerRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::calibration::CalibrateGimbalAccelerometerResponse>> {
        self.client.server_streaming(&METHOD_CALIBRATION_SERVICE_SUBSCRIBE_CALIBRATE_GIMBAL_ACCELEROMETER, req, opt)
    }

    pub fn subscribe_calibrate_gimbal_accelerometer(&self, req: &super::calibration::SubscribeCalibrateGimbalAccelerometerRequest) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::calibration::CalibrateGimbalAccelerometerResponse>> {
        self.subscribe_calibrate_gimbal_accelerometer_opt(req, ::grpcio::CallOption::default())
    }

    pub fn cancel_opt(&self, req: &super::calibration::CancelRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::calibration::CancelResponse> {
        self.client.unary_call(&METHOD_CALIBRATION_SERVICE_CANCEL, req, opt)
    }

    pub fn cancel(&self, req: &super::calibration::CancelRequest) -> ::grpcio::Result<super::calibration::CancelResponse> {
        self.cancel_opt(req, ::grpcio::CallOption::default())
    }

    pub fn cancel_async_opt(&self, req: &super::calibration::CancelRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::calibration::CancelResponse>> {
        self.client.unary_call_async(&METHOD_CALIBRATION_SERVICE_CANCEL, req, opt)
    }

    pub fn cancel_async(&self, req: &super::calibration::CancelRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::calibration::CancelResponse>> {
        self.cancel_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait CalibrationService {
    fn subscribe_calibrate_gyro(&mut self, ctx: ::grpcio::RpcContext, req: super::calibration::SubscribeCalibrateGyroRequest, sink: ::grpcio::ServerStreamingSink<super::calibration::CalibrateGyroResponse>);
    fn subscribe_calibrate_accelerometer(&mut self, ctx: ::grpcio::RpcContext, req: super::calibration::SubscribeCalibrateAccelerometerRequest, sink: ::grpcio::ServerStreamingSink<super::calibration::CalibrateAccelerometerResponse>);
    fn subscribe_calibrate_magnetometer(&mut self, ctx: ::grpcio::RpcContext, req: super::calibration::SubscribeCalibrateMagnetometerRequest, sink: ::grpcio::ServerStreamingSink<super::calibration::CalibrateMagnetometerResponse>);
    fn subscribe_calibrate_gimbal_accelerometer(&mut self, ctx: ::grpcio::RpcContext, req: super::calibration::SubscribeCalibrateGimbalAccelerometerRequest, sink: ::grpcio::ServerStreamingSink<super::calibration::CalibrateGimbalAccelerometerResponse>);
    fn cancel(&mut self, ctx: ::grpcio::RpcContext, req: super::calibration::CancelRequest, sink: ::grpcio::UnarySink<super::calibration::CancelResponse>);
}

pub fn create_calibration_service<S: CalibrationService + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_server_streaming_handler(&METHOD_CALIBRATION_SERVICE_SUBSCRIBE_CALIBRATE_GYRO, move |ctx, req, resp| {
        instance.subscribe_calibrate_gyro(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_server_streaming_handler(&METHOD_CALIBRATION_SERVICE_SUBSCRIBE_CALIBRATE_ACCELEROMETER, move |ctx, req, resp| {
        instance.subscribe_calibrate_accelerometer(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_server_streaming_handler(&METHOD_CALIBRATION_SERVICE_SUBSCRIBE_CALIBRATE_MAGNETOMETER, move |ctx, req, resp| {
        instance.subscribe_calibrate_magnetometer(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_server_streaming_handler(&METHOD_CALIBRATION_SERVICE_SUBSCRIBE_CALIBRATE_GIMBAL_ACCELEROMETER, move |ctx, req, resp| {
        instance.subscribe_calibrate_gimbal_accelerometer(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_CALIBRATION_SERVICE_CANCEL, move |ctx, req, resp| {
        instance.cancel(ctx, req, resp)
    });
    builder.build()
}
