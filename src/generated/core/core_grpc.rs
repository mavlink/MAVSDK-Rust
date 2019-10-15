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

const METHOD_CORE_SERVICE_SUBSCRIBE_CONNECTION_STATE: ::grpcio::Method<super::core::SubscribeConnectionStateRequest, super::core::ConnectionStateResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ServerStreaming,
    name: "/mavsdk.rpc.core.CoreService/SubscribeConnectionState",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_CORE_SERVICE_LIST_RUNNING_PLUGINS: ::grpcio::Method<super::core::ListRunningPluginsRequest, super::core::ListRunningPluginsResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/mavsdk.rpc.core.CoreService/ListRunningPlugins",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct CoreServiceClient {
    client: ::grpcio::Client,
}

impl CoreServiceClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        CoreServiceClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn subscribe_connection_state_opt(&self, req: &super::core::SubscribeConnectionStateRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::core::ConnectionStateResponse>> {
        self.client.server_streaming(&METHOD_CORE_SERVICE_SUBSCRIBE_CONNECTION_STATE, req, opt)
    }

    pub fn subscribe_connection_state(&self, req: &super::core::SubscribeConnectionStateRequest) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::core::ConnectionStateResponse>> {
        self.subscribe_connection_state_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_running_plugins_opt(&self, req: &super::core::ListRunningPluginsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::core::ListRunningPluginsResponse> {
        self.client.unary_call(&METHOD_CORE_SERVICE_LIST_RUNNING_PLUGINS, req, opt)
    }

    pub fn list_running_plugins(&self, req: &super::core::ListRunningPluginsRequest) -> ::grpcio::Result<super::core::ListRunningPluginsResponse> {
        self.list_running_plugins_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_running_plugins_async_opt(&self, req: &super::core::ListRunningPluginsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::core::ListRunningPluginsResponse>> {
        self.client.unary_call_async(&METHOD_CORE_SERVICE_LIST_RUNNING_PLUGINS, req, opt)
    }

    pub fn list_running_plugins_async(&self, req: &super::core::ListRunningPluginsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::core::ListRunningPluginsResponse>> {
        self.list_running_plugins_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait CoreService {
    fn subscribe_connection_state(&mut self, ctx: ::grpcio::RpcContext, req: super::core::SubscribeConnectionStateRequest, sink: ::grpcio::ServerStreamingSink<super::core::ConnectionStateResponse>);
    fn list_running_plugins(&mut self, ctx: ::grpcio::RpcContext, req: super::core::ListRunningPluginsRequest, sink: ::grpcio::UnarySink<super::core::ListRunningPluginsResponse>);
}

pub fn create_core_service<S: CoreService + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_server_streaming_handler(&METHOD_CORE_SERVICE_SUBSCRIBE_CONNECTION_STATE, move |ctx, req, resp| {
        instance.subscribe_connection_state(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_CORE_SERVICE_LIST_RUNNING_PLUGINS, move |ctx, req, resp| {
        instance.list_running_plugins(ctx, req, resp)
    });
    builder.build()
}
