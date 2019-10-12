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

const METHOD_INFO_SERVICE_GET_VERSION: ::grpcio::Method<super::info::GetVersionRequest, super::info::GetVersionResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/mavsdk.rpc.info.InfoService/GetVersion",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct InfoServiceClient {
    client: ::grpcio::Client,
}

impl InfoServiceClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        InfoServiceClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn get_version_opt(&self, req: &super::info::GetVersionRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::info::GetVersionResponse> {
        self.client.unary_call(&METHOD_INFO_SERVICE_GET_VERSION, req, opt)
    }

    pub fn get_version(&self) -> ::grpcio::Result<super::info::GetVersionResponse> {
        let req = super::info::GetVersionRequest::new();
        self.get_version_opt(&req, ::grpcio::CallOption::default())
    }

    pub fn get_version_async_opt(&self, req: &super::info::GetVersionRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::info::GetVersionResponse>> {
        self.client.unary_call_async(&METHOD_INFO_SERVICE_GET_VERSION, req, opt)
    }

    pub fn get_version_async(&self, req: &super::info::GetVersionRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::info::GetVersionResponse>> {
        self.get_version_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait InfoService {
    fn get_version(&mut self, ctx: ::grpcio::RpcContext, req: super::info::GetVersionRequest, sink: ::grpcio::UnarySink<super::info::GetVersionResponse>);
}

pub fn create_info_service<S: InfoService + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_INFO_SERVICE_GET_VERSION, move |ctx, req, resp| {
        instance.get_version(ctx, req, resp)
    });
    builder.build()
}
