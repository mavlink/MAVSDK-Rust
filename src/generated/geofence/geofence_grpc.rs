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

const METHOD_GEOFENCE_SERVICE_UPLOAD_GEOFENCE: ::grpcio::Method<super::geofence::UploadGeofenceRequest, super::geofence::UploadGeofenceResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/mavsdk.rpc.geofence.GeofenceService/UploadGeofence",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct GeofenceServiceClient {
    client: ::grpcio::Client,
}

impl GeofenceServiceClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        GeofenceServiceClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn upload_geofence_opt(&self, req: &super::geofence::UploadGeofenceRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::geofence::UploadGeofenceResponse> {
        self.client.unary_call(&METHOD_GEOFENCE_SERVICE_UPLOAD_GEOFENCE, req, opt)
    }

    pub fn upload_geofence(&self, req: &super::geofence::UploadGeofenceRequest) -> ::grpcio::Result<super::geofence::UploadGeofenceResponse> {
        self.upload_geofence_opt(req, ::grpcio::CallOption::default())
    }

    pub fn upload_geofence_async_opt(&self, req: &super::geofence::UploadGeofenceRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::geofence::UploadGeofenceResponse>> {
        self.client.unary_call_async(&METHOD_GEOFENCE_SERVICE_UPLOAD_GEOFENCE, req, opt)
    }

    pub fn upload_geofence_async(&self, req: &super::geofence::UploadGeofenceRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::geofence::UploadGeofenceResponse>> {
        self.upload_geofence_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait GeofenceService {
    fn upload_geofence(&mut self, ctx: ::grpcio::RpcContext, req: super::geofence::UploadGeofenceRequest, sink: ::grpcio::UnarySink<super::geofence::UploadGeofenceResponse>);
}

pub fn create_geofence_service<S: GeofenceService + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_GEOFENCE_SERVICE_UPLOAD_GEOFENCE, move |ctx, req, resp| {
        instance.upload_geofence(ctx, req, resp)
    });
    builder.build()
}
