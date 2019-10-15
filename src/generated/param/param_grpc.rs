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

const METHOD_PARAM_SERVICE_GET_INT_PARAM: ::grpcio::Method<super::param::GetIntParamRequest, super::param::GetIntParamResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/mavsdk.rpc.param.ParamService/GetIntParam",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_PARAM_SERVICE_SET_INT_PARAM: ::grpcio::Method<super::param::SetIntParamRequest, super::param::SetIntParamResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/mavsdk.rpc.param.ParamService/SetIntParam",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_PARAM_SERVICE_GET_FLOAT_PARAM: ::grpcio::Method<super::param::GetFloatParamRequest, super::param::GetFloatParamResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/mavsdk.rpc.param.ParamService/GetFloatParam",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_PARAM_SERVICE_SET_FLOAT_PARAM: ::grpcio::Method<super::param::SetFloatParamRequest, super::param::SetFloatParamResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/mavsdk.rpc.param.ParamService/SetFloatParam",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct ParamServiceClient {
    client: ::grpcio::Client,
}

impl ParamServiceClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        ParamServiceClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn get_int_param_opt(&self, req: &super::param::GetIntParamRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::param::GetIntParamResponse> {
        self.client.unary_call(&METHOD_PARAM_SERVICE_GET_INT_PARAM, req, opt)
    }

    pub fn get_int_param(&self, req: &super::param::GetIntParamRequest) -> ::grpcio::Result<super::param::GetIntParamResponse> {
        self.get_int_param_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_int_param_async_opt(&self, req: &super::param::GetIntParamRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::param::GetIntParamResponse>> {
        self.client.unary_call_async(&METHOD_PARAM_SERVICE_GET_INT_PARAM, req, opt)
    }

    pub fn get_int_param_async(&self, req: &super::param::GetIntParamRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::param::GetIntParamResponse>> {
        self.get_int_param_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn set_int_param_opt(&self, req: &super::param::SetIntParamRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::param::SetIntParamResponse> {
        self.client.unary_call(&METHOD_PARAM_SERVICE_SET_INT_PARAM, req, opt)
    }

    pub fn set_int_param(&self, req: &super::param::SetIntParamRequest) -> ::grpcio::Result<super::param::SetIntParamResponse> {
        self.set_int_param_opt(req, ::grpcio::CallOption::default())
    }

    pub fn set_int_param_async_opt(&self, req: &super::param::SetIntParamRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::param::SetIntParamResponse>> {
        self.client.unary_call_async(&METHOD_PARAM_SERVICE_SET_INT_PARAM, req, opt)
    }

    pub fn set_int_param_async(&self, req: &super::param::SetIntParamRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::param::SetIntParamResponse>> {
        self.set_int_param_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_float_param_opt(&self, req: &super::param::GetFloatParamRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::param::GetFloatParamResponse> {
        self.client.unary_call(&METHOD_PARAM_SERVICE_GET_FLOAT_PARAM, req, opt)
    }

    pub fn get_float_param(&self, req: &super::param::GetFloatParamRequest) -> ::grpcio::Result<super::param::GetFloatParamResponse> {
        self.get_float_param_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_float_param_async_opt(&self, req: &super::param::GetFloatParamRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::param::GetFloatParamResponse>> {
        self.client.unary_call_async(&METHOD_PARAM_SERVICE_GET_FLOAT_PARAM, req, opt)
    }

    pub fn get_float_param_async(&self, req: &super::param::GetFloatParamRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::param::GetFloatParamResponse>> {
        self.get_float_param_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn set_float_param_opt(&self, req: &super::param::SetFloatParamRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::param::SetFloatParamResponse> {
        self.client.unary_call(&METHOD_PARAM_SERVICE_SET_FLOAT_PARAM, req, opt)
    }

    pub fn set_float_param(&self, req: &super::param::SetFloatParamRequest) -> ::grpcio::Result<super::param::SetFloatParamResponse> {
        self.set_float_param_opt(req, ::grpcio::CallOption::default())
    }

    pub fn set_float_param_async_opt(&self, req: &super::param::SetFloatParamRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::param::SetFloatParamResponse>> {
        self.client.unary_call_async(&METHOD_PARAM_SERVICE_SET_FLOAT_PARAM, req, opt)
    }

    pub fn set_float_param_async(&self, req: &super::param::SetFloatParamRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::param::SetFloatParamResponse>> {
        self.set_float_param_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait ParamService {
    fn get_int_param(&mut self, ctx: ::grpcio::RpcContext, req: super::param::GetIntParamRequest, sink: ::grpcio::UnarySink<super::param::GetIntParamResponse>);
    fn set_int_param(&mut self, ctx: ::grpcio::RpcContext, req: super::param::SetIntParamRequest, sink: ::grpcio::UnarySink<super::param::SetIntParamResponse>);
    fn get_float_param(&mut self, ctx: ::grpcio::RpcContext, req: super::param::GetFloatParamRequest, sink: ::grpcio::UnarySink<super::param::GetFloatParamResponse>);
    fn set_float_param(&mut self, ctx: ::grpcio::RpcContext, req: super::param::SetFloatParamRequest, sink: ::grpcio::UnarySink<super::param::SetFloatParamResponse>);
}

pub fn create_param_service<S: ParamService + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_PARAM_SERVICE_GET_INT_PARAM, move |ctx, req, resp| {
        instance.get_int_param(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_PARAM_SERVICE_SET_INT_PARAM, move |ctx, req, resp| {
        instance.set_int_param(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_PARAM_SERVICE_GET_FLOAT_PARAM, move |ctx, req, resp| {
        instance.get_float_param(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_PARAM_SERVICE_SET_FLOAT_PARAM, move |ctx, req, resp| {
        instance.set_float_param(ctx, req, resp)
    });
    builder.build()
}
