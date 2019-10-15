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

const METHOD_MISSION_SERVICE_UPLOAD_MISSION: ::grpcio::Method<super::mission::UploadMissionRequest, super::mission::UploadMissionResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/mavsdk.rpc.mission.MissionService/UploadMission",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_MISSION_SERVICE_CANCEL_MISSION_UPLOAD: ::grpcio::Method<super::mission::CancelMissionUploadRequest, super::mission::CancelMissionUploadResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/mavsdk.rpc.mission.MissionService/CancelMissionUpload",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_MISSION_SERVICE_DOWNLOAD_MISSION: ::grpcio::Method<super::mission::DownloadMissionRequest, super::mission::DownloadMissionResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/mavsdk.rpc.mission.MissionService/DownloadMission",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_MISSION_SERVICE_CANCEL_MISSION_DOWNLOAD: ::grpcio::Method<super::mission::CancelMissionDownloadRequest, super::mission::CancelMissionDownloadResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/mavsdk.rpc.mission.MissionService/CancelMissionDownload",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_MISSION_SERVICE_START_MISSION: ::grpcio::Method<super::mission::StartMissionRequest, super::mission::StartMissionResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/mavsdk.rpc.mission.MissionService/StartMission",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_MISSION_SERVICE_PAUSE_MISSION: ::grpcio::Method<super::mission::PauseMissionRequest, super::mission::PauseMissionResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/mavsdk.rpc.mission.MissionService/PauseMission",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_MISSION_SERVICE_CLEAR_MISSION: ::grpcio::Method<super::mission::ClearMissionRequest, super::mission::ClearMissionResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/mavsdk.rpc.mission.MissionService/ClearMission",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_MISSION_SERVICE_SET_CURRENT_MISSION_ITEM_INDEX: ::grpcio::Method<super::mission::SetCurrentMissionItemIndexRequest, super::mission::SetCurrentMissionItemIndexResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/mavsdk.rpc.mission.MissionService/SetCurrentMissionItemIndex",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_MISSION_SERVICE_IS_MISSION_FINISHED: ::grpcio::Method<super::mission::IsMissionFinishedRequest, super::mission::IsMissionFinishedResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/mavsdk.rpc.mission.MissionService/IsMissionFinished",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_MISSION_SERVICE_SUBSCRIBE_MISSION_PROGRESS: ::grpcio::Method<super::mission::SubscribeMissionProgressRequest, super::mission::MissionProgressResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ServerStreaming,
    name: "/mavsdk.rpc.mission.MissionService/SubscribeMissionProgress",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_MISSION_SERVICE_GET_RETURN_TO_LAUNCH_AFTER_MISSION: ::grpcio::Method<super::mission::GetReturnToLaunchAfterMissionRequest, super::mission::GetReturnToLaunchAfterMissionResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/mavsdk.rpc.mission.MissionService/GetReturnToLaunchAfterMission",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_MISSION_SERVICE_SET_RETURN_TO_LAUNCH_AFTER_MISSION: ::grpcio::Method<super::mission::SetReturnToLaunchAfterMissionRequest, super::mission::SetReturnToLaunchAfterMissionResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/mavsdk.rpc.mission.MissionService/SetReturnToLaunchAfterMission",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct MissionServiceClient {
    client: ::grpcio::Client,
}

impl MissionServiceClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        MissionServiceClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn upload_mission_opt(&self, req: &super::mission::UploadMissionRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::mission::UploadMissionResponse> {
        self.client.unary_call(&METHOD_MISSION_SERVICE_UPLOAD_MISSION, req, opt)
    }

    pub fn upload_mission(&self, req: &super::mission::UploadMissionRequest) -> ::grpcio::Result<super::mission::UploadMissionResponse> {
        self.upload_mission_opt(req, ::grpcio::CallOption::default())
    }

    pub fn upload_mission_async_opt(&self, req: &super::mission::UploadMissionRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::mission::UploadMissionResponse>> {
        self.client.unary_call_async(&METHOD_MISSION_SERVICE_UPLOAD_MISSION, req, opt)
    }

    pub fn upload_mission_async(&self, req: &super::mission::UploadMissionRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::mission::UploadMissionResponse>> {
        self.upload_mission_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn cancel_mission_upload_opt(&self, req: &super::mission::CancelMissionUploadRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::mission::CancelMissionUploadResponse> {
        self.client.unary_call(&METHOD_MISSION_SERVICE_CANCEL_MISSION_UPLOAD, req, opt)
    }

    pub fn cancel_mission_upload(&self, req: &super::mission::CancelMissionUploadRequest) -> ::grpcio::Result<super::mission::CancelMissionUploadResponse> {
        self.cancel_mission_upload_opt(req, ::grpcio::CallOption::default())
    }

    pub fn cancel_mission_upload_async_opt(&self, req: &super::mission::CancelMissionUploadRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::mission::CancelMissionUploadResponse>> {
        self.client.unary_call_async(&METHOD_MISSION_SERVICE_CANCEL_MISSION_UPLOAD, req, opt)
    }

    pub fn cancel_mission_upload_async(&self, req: &super::mission::CancelMissionUploadRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::mission::CancelMissionUploadResponse>> {
        self.cancel_mission_upload_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn download_mission_opt(&self, req: &super::mission::DownloadMissionRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::mission::DownloadMissionResponse> {
        self.client.unary_call(&METHOD_MISSION_SERVICE_DOWNLOAD_MISSION, req, opt)
    }

    pub fn download_mission(&self, req: &super::mission::DownloadMissionRequest) -> ::grpcio::Result<super::mission::DownloadMissionResponse> {
        self.download_mission_opt(req, ::grpcio::CallOption::default())
    }

    pub fn download_mission_async_opt(&self, req: &super::mission::DownloadMissionRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::mission::DownloadMissionResponse>> {
        self.client.unary_call_async(&METHOD_MISSION_SERVICE_DOWNLOAD_MISSION, req, opt)
    }

    pub fn download_mission_async(&self, req: &super::mission::DownloadMissionRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::mission::DownloadMissionResponse>> {
        self.download_mission_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn cancel_mission_download_opt(&self, req: &super::mission::CancelMissionDownloadRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::mission::CancelMissionDownloadResponse> {
        self.client.unary_call(&METHOD_MISSION_SERVICE_CANCEL_MISSION_DOWNLOAD, req, opt)
    }

    pub fn cancel_mission_download(&self, req: &super::mission::CancelMissionDownloadRequest) -> ::grpcio::Result<super::mission::CancelMissionDownloadResponse> {
        self.cancel_mission_download_opt(req, ::grpcio::CallOption::default())
    }

    pub fn cancel_mission_download_async_opt(&self, req: &super::mission::CancelMissionDownloadRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::mission::CancelMissionDownloadResponse>> {
        self.client.unary_call_async(&METHOD_MISSION_SERVICE_CANCEL_MISSION_DOWNLOAD, req, opt)
    }

    pub fn cancel_mission_download_async(&self, req: &super::mission::CancelMissionDownloadRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::mission::CancelMissionDownloadResponse>> {
        self.cancel_mission_download_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn start_mission_opt(&self, req: &super::mission::StartMissionRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::mission::StartMissionResponse> {
        self.client.unary_call(&METHOD_MISSION_SERVICE_START_MISSION, req, opt)
    }

    pub fn start_mission(&self, req: &super::mission::StartMissionRequest) -> ::grpcio::Result<super::mission::StartMissionResponse> {
        self.start_mission_opt(req, ::grpcio::CallOption::default())
    }

    pub fn start_mission_async_opt(&self, req: &super::mission::StartMissionRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::mission::StartMissionResponse>> {
        self.client.unary_call_async(&METHOD_MISSION_SERVICE_START_MISSION, req, opt)
    }

    pub fn start_mission_async(&self, req: &super::mission::StartMissionRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::mission::StartMissionResponse>> {
        self.start_mission_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn pause_mission_opt(&self, req: &super::mission::PauseMissionRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::mission::PauseMissionResponse> {
        self.client.unary_call(&METHOD_MISSION_SERVICE_PAUSE_MISSION, req, opt)
    }

    pub fn pause_mission(&self, req: &super::mission::PauseMissionRequest) -> ::grpcio::Result<super::mission::PauseMissionResponse> {
        self.pause_mission_opt(req, ::grpcio::CallOption::default())
    }

    pub fn pause_mission_async_opt(&self, req: &super::mission::PauseMissionRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::mission::PauseMissionResponse>> {
        self.client.unary_call_async(&METHOD_MISSION_SERVICE_PAUSE_MISSION, req, opt)
    }

    pub fn pause_mission_async(&self, req: &super::mission::PauseMissionRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::mission::PauseMissionResponse>> {
        self.pause_mission_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn clear_mission_opt(&self, req: &super::mission::ClearMissionRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::mission::ClearMissionResponse> {
        self.client.unary_call(&METHOD_MISSION_SERVICE_CLEAR_MISSION, req, opt)
    }

    pub fn clear_mission(&self, req: &super::mission::ClearMissionRequest) -> ::grpcio::Result<super::mission::ClearMissionResponse> {
        self.clear_mission_opt(req, ::grpcio::CallOption::default())
    }

    pub fn clear_mission_async_opt(&self, req: &super::mission::ClearMissionRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::mission::ClearMissionResponse>> {
        self.client.unary_call_async(&METHOD_MISSION_SERVICE_CLEAR_MISSION, req, opt)
    }

    pub fn clear_mission_async(&self, req: &super::mission::ClearMissionRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::mission::ClearMissionResponse>> {
        self.clear_mission_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn set_current_mission_item_index_opt(&self, req: &super::mission::SetCurrentMissionItemIndexRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::mission::SetCurrentMissionItemIndexResponse> {
        self.client.unary_call(&METHOD_MISSION_SERVICE_SET_CURRENT_MISSION_ITEM_INDEX, req, opt)
    }

    pub fn set_current_mission_item_index(&self, req: &super::mission::SetCurrentMissionItemIndexRequest) -> ::grpcio::Result<super::mission::SetCurrentMissionItemIndexResponse> {
        self.set_current_mission_item_index_opt(req, ::grpcio::CallOption::default())
    }

    pub fn set_current_mission_item_index_async_opt(&self, req: &super::mission::SetCurrentMissionItemIndexRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::mission::SetCurrentMissionItemIndexResponse>> {
        self.client.unary_call_async(&METHOD_MISSION_SERVICE_SET_CURRENT_MISSION_ITEM_INDEX, req, opt)
    }

    pub fn set_current_mission_item_index_async(&self, req: &super::mission::SetCurrentMissionItemIndexRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::mission::SetCurrentMissionItemIndexResponse>> {
        self.set_current_mission_item_index_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn is_mission_finished_opt(&self, req: &super::mission::IsMissionFinishedRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::mission::IsMissionFinishedResponse> {
        self.client.unary_call(&METHOD_MISSION_SERVICE_IS_MISSION_FINISHED, req, opt)
    }

    pub fn is_mission_finished(&self, req: &super::mission::IsMissionFinishedRequest) -> ::grpcio::Result<super::mission::IsMissionFinishedResponse> {
        self.is_mission_finished_opt(req, ::grpcio::CallOption::default())
    }

    pub fn is_mission_finished_async_opt(&self, req: &super::mission::IsMissionFinishedRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::mission::IsMissionFinishedResponse>> {
        self.client.unary_call_async(&METHOD_MISSION_SERVICE_IS_MISSION_FINISHED, req, opt)
    }

    pub fn is_mission_finished_async(&self, req: &super::mission::IsMissionFinishedRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::mission::IsMissionFinishedResponse>> {
        self.is_mission_finished_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn subscribe_mission_progress_opt(&self, req: &super::mission::SubscribeMissionProgressRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::mission::MissionProgressResponse>> {
        self.client.server_streaming(&METHOD_MISSION_SERVICE_SUBSCRIBE_MISSION_PROGRESS, req, opt)
    }

    pub fn subscribe_mission_progress(&self, req: &super::mission::SubscribeMissionProgressRequest) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::mission::MissionProgressResponse>> {
        self.subscribe_mission_progress_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_return_to_launch_after_mission_opt(&self, req: &super::mission::GetReturnToLaunchAfterMissionRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::mission::GetReturnToLaunchAfterMissionResponse> {
        self.client.unary_call(&METHOD_MISSION_SERVICE_GET_RETURN_TO_LAUNCH_AFTER_MISSION, req, opt)
    }

    pub fn get_return_to_launch_after_mission(&self, req: &super::mission::GetReturnToLaunchAfterMissionRequest) -> ::grpcio::Result<super::mission::GetReturnToLaunchAfterMissionResponse> {
        self.get_return_to_launch_after_mission_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_return_to_launch_after_mission_async_opt(&self, req: &super::mission::GetReturnToLaunchAfterMissionRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::mission::GetReturnToLaunchAfterMissionResponse>> {
        self.client.unary_call_async(&METHOD_MISSION_SERVICE_GET_RETURN_TO_LAUNCH_AFTER_MISSION, req, opt)
    }

    pub fn get_return_to_launch_after_mission_async(&self, req: &super::mission::GetReturnToLaunchAfterMissionRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::mission::GetReturnToLaunchAfterMissionResponse>> {
        self.get_return_to_launch_after_mission_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn set_return_to_launch_after_mission_opt(&self, req: &super::mission::SetReturnToLaunchAfterMissionRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::mission::SetReturnToLaunchAfterMissionResponse> {
        self.client.unary_call(&METHOD_MISSION_SERVICE_SET_RETURN_TO_LAUNCH_AFTER_MISSION, req, opt)
    }

    pub fn set_return_to_launch_after_mission(&self, req: &super::mission::SetReturnToLaunchAfterMissionRequest) -> ::grpcio::Result<super::mission::SetReturnToLaunchAfterMissionResponse> {
        self.set_return_to_launch_after_mission_opt(req, ::grpcio::CallOption::default())
    }

    pub fn set_return_to_launch_after_mission_async_opt(&self, req: &super::mission::SetReturnToLaunchAfterMissionRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::mission::SetReturnToLaunchAfterMissionResponse>> {
        self.client.unary_call_async(&METHOD_MISSION_SERVICE_SET_RETURN_TO_LAUNCH_AFTER_MISSION, req, opt)
    }

    pub fn set_return_to_launch_after_mission_async(&self, req: &super::mission::SetReturnToLaunchAfterMissionRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::mission::SetReturnToLaunchAfterMissionResponse>> {
        self.set_return_to_launch_after_mission_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait MissionService {
    fn upload_mission(&mut self, ctx: ::grpcio::RpcContext, req: super::mission::UploadMissionRequest, sink: ::grpcio::UnarySink<super::mission::UploadMissionResponse>);
    fn cancel_mission_upload(&mut self, ctx: ::grpcio::RpcContext, req: super::mission::CancelMissionUploadRequest, sink: ::grpcio::UnarySink<super::mission::CancelMissionUploadResponse>);
    fn download_mission(&mut self, ctx: ::grpcio::RpcContext, req: super::mission::DownloadMissionRequest, sink: ::grpcio::UnarySink<super::mission::DownloadMissionResponse>);
    fn cancel_mission_download(&mut self, ctx: ::grpcio::RpcContext, req: super::mission::CancelMissionDownloadRequest, sink: ::grpcio::UnarySink<super::mission::CancelMissionDownloadResponse>);
    fn start_mission(&mut self, ctx: ::grpcio::RpcContext, req: super::mission::StartMissionRequest, sink: ::grpcio::UnarySink<super::mission::StartMissionResponse>);
    fn pause_mission(&mut self, ctx: ::grpcio::RpcContext, req: super::mission::PauseMissionRequest, sink: ::grpcio::UnarySink<super::mission::PauseMissionResponse>);
    fn clear_mission(&mut self, ctx: ::grpcio::RpcContext, req: super::mission::ClearMissionRequest, sink: ::grpcio::UnarySink<super::mission::ClearMissionResponse>);
    fn set_current_mission_item_index(&mut self, ctx: ::grpcio::RpcContext, req: super::mission::SetCurrentMissionItemIndexRequest, sink: ::grpcio::UnarySink<super::mission::SetCurrentMissionItemIndexResponse>);
    fn is_mission_finished(&mut self, ctx: ::grpcio::RpcContext, req: super::mission::IsMissionFinishedRequest, sink: ::grpcio::UnarySink<super::mission::IsMissionFinishedResponse>);
    fn subscribe_mission_progress(&mut self, ctx: ::grpcio::RpcContext, req: super::mission::SubscribeMissionProgressRequest, sink: ::grpcio::ServerStreamingSink<super::mission::MissionProgressResponse>);
    fn get_return_to_launch_after_mission(&mut self, ctx: ::grpcio::RpcContext, req: super::mission::GetReturnToLaunchAfterMissionRequest, sink: ::grpcio::UnarySink<super::mission::GetReturnToLaunchAfterMissionResponse>);
    fn set_return_to_launch_after_mission(&mut self, ctx: ::grpcio::RpcContext, req: super::mission::SetReturnToLaunchAfterMissionRequest, sink: ::grpcio::UnarySink<super::mission::SetReturnToLaunchAfterMissionResponse>);
}

pub fn create_mission_service<S: MissionService + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_MISSION_SERVICE_UPLOAD_MISSION, move |ctx, req, resp| {
        instance.upload_mission(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_MISSION_SERVICE_CANCEL_MISSION_UPLOAD, move |ctx, req, resp| {
        instance.cancel_mission_upload(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_MISSION_SERVICE_DOWNLOAD_MISSION, move |ctx, req, resp| {
        instance.download_mission(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_MISSION_SERVICE_CANCEL_MISSION_DOWNLOAD, move |ctx, req, resp| {
        instance.cancel_mission_download(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_MISSION_SERVICE_START_MISSION, move |ctx, req, resp| {
        instance.start_mission(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_MISSION_SERVICE_PAUSE_MISSION, move |ctx, req, resp| {
        instance.pause_mission(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_MISSION_SERVICE_CLEAR_MISSION, move |ctx, req, resp| {
        instance.clear_mission(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_MISSION_SERVICE_SET_CURRENT_MISSION_ITEM_INDEX, move |ctx, req, resp| {
        instance.set_current_mission_item_index(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_MISSION_SERVICE_IS_MISSION_FINISHED, move |ctx, req, resp| {
        instance.is_mission_finished(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_server_streaming_handler(&METHOD_MISSION_SERVICE_SUBSCRIBE_MISSION_PROGRESS, move |ctx, req, resp| {
        instance.subscribe_mission_progress(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_MISSION_SERVICE_GET_RETURN_TO_LAUNCH_AFTER_MISSION, move |ctx, req, resp| {
        instance.get_return_to_launch_after_mission(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_MISSION_SERVICE_SET_RETURN_TO_LAUNCH_AFTER_MISSION, move |ctx, req, resp| {
        instance.set_return_to_launch_after_mission(ctx, req, resp)
    });
    builder.build()
}
