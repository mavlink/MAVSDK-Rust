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

const METHOD_CAMERA_SERVICE_TAKE_PHOTO: ::grpcio::Method<super::camera::TakePhotoRequest, super::camera::TakePhotoResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/mavsdk.rpc.camera.CameraService/TakePhoto",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_CAMERA_SERVICE_START_PHOTO_INTERVAL: ::grpcio::Method<super::camera::StartPhotoIntervalRequest, super::camera::StartPhotoIntervalResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/mavsdk.rpc.camera.CameraService/StartPhotoInterval",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_CAMERA_SERVICE_STOP_PHOTO_INTERVAL: ::grpcio::Method<super::camera::StopPhotoIntervalRequest, super::camera::StopPhotoIntervalResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/mavsdk.rpc.camera.CameraService/StopPhotoInterval",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_CAMERA_SERVICE_START_VIDEO: ::grpcio::Method<super::camera::StartVideoRequest, super::camera::StartVideoResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/mavsdk.rpc.camera.CameraService/StartVideo",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_CAMERA_SERVICE_STOP_VIDEO: ::grpcio::Method<super::camera::StopVideoRequest, super::camera::StopVideoResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/mavsdk.rpc.camera.CameraService/StopVideo",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_CAMERA_SERVICE_START_VIDEO_STREAMING: ::grpcio::Method<super::camera::StartVideoStreamingRequest, super::camera::StartVideoStreamingResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/mavsdk.rpc.camera.CameraService/StartVideoStreaming",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_CAMERA_SERVICE_STOP_VIDEO_STREAMING: ::grpcio::Method<super::camera::StopVideoStreamingRequest, super::camera::StopVideoStreamingResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/mavsdk.rpc.camera.CameraService/StopVideoStreaming",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_CAMERA_SERVICE_SET_MODE: ::grpcio::Method<super::camera::SetModeRequest, super::camera::SetModeResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/mavsdk.rpc.camera.CameraService/SetMode",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_CAMERA_SERVICE_SUBSCRIBE_MODE: ::grpcio::Method<super::camera::SubscribeModeRequest, super::camera::ModeResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ServerStreaming,
    name: "/mavsdk.rpc.camera.CameraService/SubscribeMode",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_CAMERA_SERVICE_SUBSCRIBE_VIDEO_STREAM_INFO: ::grpcio::Method<super::camera::SubscribeVideoStreamInfoRequest, super::camera::VideoStreamInfoResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ServerStreaming,
    name: "/mavsdk.rpc.camera.CameraService/SubscribeVideoStreamInfo",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_CAMERA_SERVICE_SUBSCRIBE_CAPTURE_INFO: ::grpcio::Method<super::camera::SubscribeCaptureInfoRequest, super::camera::CaptureInfoResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ServerStreaming,
    name: "/mavsdk.rpc.camera.CameraService/SubscribeCaptureInfo",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_CAMERA_SERVICE_SUBSCRIBE_CAMERA_STATUS: ::grpcio::Method<super::camera::SubscribeCameraStatusRequest, super::camera::CameraStatusResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ServerStreaming,
    name: "/mavsdk.rpc.camera.CameraService/SubscribeCameraStatus",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_CAMERA_SERVICE_SUBSCRIBE_CURRENT_SETTINGS: ::grpcio::Method<super::camera::SubscribeCurrentSettingsRequest, super::camera::CurrentSettingsResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ServerStreaming,
    name: "/mavsdk.rpc.camera.CameraService/SubscribeCurrentSettings",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_CAMERA_SERVICE_SUBSCRIBE_POSSIBLE_SETTING_OPTIONS: ::grpcio::Method<super::camera::SubscribePossibleSettingOptionsRequest, super::camera::PossibleSettingOptionsResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ServerStreaming,
    name: "/mavsdk.rpc.camera.CameraService/SubscribePossibleSettingOptions",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_CAMERA_SERVICE_SET_SETTING: ::grpcio::Method<super::camera::SetSettingRequest, super::camera::SetSettingResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/mavsdk.rpc.camera.CameraService/SetSetting",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct CameraServiceClient {
    client: ::grpcio::Client,
}

impl CameraServiceClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        CameraServiceClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn take_photo_opt(&self, req: &super::camera::TakePhotoRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::camera::TakePhotoResponse> {
        self.client.unary_call(&METHOD_CAMERA_SERVICE_TAKE_PHOTO, req, opt)
    }

    pub fn take_photo(&self, req: &super::camera::TakePhotoRequest) -> ::grpcio::Result<super::camera::TakePhotoResponse> {
        self.take_photo_opt(req, ::grpcio::CallOption::default())
    }

    pub fn take_photo_async_opt(&self, req: &super::camera::TakePhotoRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::camera::TakePhotoResponse>> {
        self.client.unary_call_async(&METHOD_CAMERA_SERVICE_TAKE_PHOTO, req, opt)
    }

    pub fn take_photo_async(&self, req: &super::camera::TakePhotoRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::camera::TakePhotoResponse>> {
        self.take_photo_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn start_photo_interval_opt(&self, req: &super::camera::StartPhotoIntervalRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::camera::StartPhotoIntervalResponse> {
        self.client.unary_call(&METHOD_CAMERA_SERVICE_START_PHOTO_INTERVAL, req, opt)
    }

    pub fn start_photo_interval(&self, req: &super::camera::StartPhotoIntervalRequest) -> ::grpcio::Result<super::camera::StartPhotoIntervalResponse> {
        self.start_photo_interval_opt(req, ::grpcio::CallOption::default())
    }

    pub fn start_photo_interval_async_opt(&self, req: &super::camera::StartPhotoIntervalRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::camera::StartPhotoIntervalResponse>> {
        self.client.unary_call_async(&METHOD_CAMERA_SERVICE_START_PHOTO_INTERVAL, req, opt)
    }

    pub fn start_photo_interval_async(&self, req: &super::camera::StartPhotoIntervalRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::camera::StartPhotoIntervalResponse>> {
        self.start_photo_interval_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn stop_photo_interval_opt(&self, req: &super::camera::StopPhotoIntervalRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::camera::StopPhotoIntervalResponse> {
        self.client.unary_call(&METHOD_CAMERA_SERVICE_STOP_PHOTO_INTERVAL, req, opt)
    }

    pub fn stop_photo_interval(&self, req: &super::camera::StopPhotoIntervalRequest) -> ::grpcio::Result<super::camera::StopPhotoIntervalResponse> {
        self.stop_photo_interval_opt(req, ::grpcio::CallOption::default())
    }

    pub fn stop_photo_interval_async_opt(&self, req: &super::camera::StopPhotoIntervalRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::camera::StopPhotoIntervalResponse>> {
        self.client.unary_call_async(&METHOD_CAMERA_SERVICE_STOP_PHOTO_INTERVAL, req, opt)
    }

    pub fn stop_photo_interval_async(&self, req: &super::camera::StopPhotoIntervalRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::camera::StopPhotoIntervalResponse>> {
        self.stop_photo_interval_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn start_video_opt(&self, req: &super::camera::StartVideoRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::camera::StartVideoResponse> {
        self.client.unary_call(&METHOD_CAMERA_SERVICE_START_VIDEO, req, opt)
    }

    pub fn start_video(&self, req: &super::camera::StartVideoRequest) -> ::grpcio::Result<super::camera::StartVideoResponse> {
        self.start_video_opt(req, ::grpcio::CallOption::default())
    }

    pub fn start_video_async_opt(&self, req: &super::camera::StartVideoRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::camera::StartVideoResponse>> {
        self.client.unary_call_async(&METHOD_CAMERA_SERVICE_START_VIDEO, req, opt)
    }

    pub fn start_video_async(&self, req: &super::camera::StartVideoRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::camera::StartVideoResponse>> {
        self.start_video_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn stop_video_opt(&self, req: &super::camera::StopVideoRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::camera::StopVideoResponse> {
        self.client.unary_call(&METHOD_CAMERA_SERVICE_STOP_VIDEO, req, opt)
    }

    pub fn stop_video(&self, req: &super::camera::StopVideoRequest) -> ::grpcio::Result<super::camera::StopVideoResponse> {
        self.stop_video_opt(req, ::grpcio::CallOption::default())
    }

    pub fn stop_video_async_opt(&self, req: &super::camera::StopVideoRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::camera::StopVideoResponse>> {
        self.client.unary_call_async(&METHOD_CAMERA_SERVICE_STOP_VIDEO, req, opt)
    }

    pub fn stop_video_async(&self, req: &super::camera::StopVideoRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::camera::StopVideoResponse>> {
        self.stop_video_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn start_video_streaming_opt(&self, req: &super::camera::StartVideoStreamingRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::camera::StartVideoStreamingResponse> {
        self.client.unary_call(&METHOD_CAMERA_SERVICE_START_VIDEO_STREAMING, req, opt)
    }

    pub fn start_video_streaming(&self, req: &super::camera::StartVideoStreamingRequest) -> ::grpcio::Result<super::camera::StartVideoStreamingResponse> {
        self.start_video_streaming_opt(req, ::grpcio::CallOption::default())
    }

    pub fn start_video_streaming_async_opt(&self, req: &super::camera::StartVideoStreamingRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::camera::StartVideoStreamingResponse>> {
        self.client.unary_call_async(&METHOD_CAMERA_SERVICE_START_VIDEO_STREAMING, req, opt)
    }

    pub fn start_video_streaming_async(&self, req: &super::camera::StartVideoStreamingRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::camera::StartVideoStreamingResponse>> {
        self.start_video_streaming_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn stop_video_streaming_opt(&self, req: &super::camera::StopVideoStreamingRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::camera::StopVideoStreamingResponse> {
        self.client.unary_call(&METHOD_CAMERA_SERVICE_STOP_VIDEO_STREAMING, req, opt)
    }

    pub fn stop_video_streaming(&self, req: &super::camera::StopVideoStreamingRequest) -> ::grpcio::Result<super::camera::StopVideoStreamingResponse> {
        self.stop_video_streaming_opt(req, ::grpcio::CallOption::default())
    }

    pub fn stop_video_streaming_async_opt(&self, req: &super::camera::StopVideoStreamingRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::camera::StopVideoStreamingResponse>> {
        self.client.unary_call_async(&METHOD_CAMERA_SERVICE_STOP_VIDEO_STREAMING, req, opt)
    }

    pub fn stop_video_streaming_async(&self, req: &super::camera::StopVideoStreamingRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::camera::StopVideoStreamingResponse>> {
        self.stop_video_streaming_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn set_mode_opt(&self, req: &super::camera::SetModeRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::camera::SetModeResponse> {
        self.client.unary_call(&METHOD_CAMERA_SERVICE_SET_MODE, req, opt)
    }

    pub fn set_mode(&self, req: &super::camera::SetModeRequest) -> ::grpcio::Result<super::camera::SetModeResponse> {
        self.set_mode_opt(req, ::grpcio::CallOption::default())
    }

    pub fn set_mode_async_opt(&self, req: &super::camera::SetModeRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::camera::SetModeResponse>> {
        self.client.unary_call_async(&METHOD_CAMERA_SERVICE_SET_MODE, req, opt)
    }

    pub fn set_mode_async(&self, req: &super::camera::SetModeRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::camera::SetModeResponse>> {
        self.set_mode_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn subscribe_mode_opt(&self, req: &super::camera::SubscribeModeRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::camera::ModeResponse>> {
        self.client.server_streaming(&METHOD_CAMERA_SERVICE_SUBSCRIBE_MODE, req, opt)
    }

    pub fn subscribe_mode(&self, req: &super::camera::SubscribeModeRequest) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::camera::ModeResponse>> {
        self.subscribe_mode_opt(req, ::grpcio::CallOption::default())
    }

    pub fn subscribe_video_stream_info_opt(&self, req: &super::camera::SubscribeVideoStreamInfoRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::camera::VideoStreamInfoResponse>> {
        self.client.server_streaming(&METHOD_CAMERA_SERVICE_SUBSCRIBE_VIDEO_STREAM_INFO, req, opt)
    }

    pub fn subscribe_video_stream_info(&self, req: &super::camera::SubscribeVideoStreamInfoRequest) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::camera::VideoStreamInfoResponse>> {
        self.subscribe_video_stream_info_opt(req, ::grpcio::CallOption::default())
    }

    pub fn subscribe_capture_info_opt(&self, req: &super::camera::SubscribeCaptureInfoRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::camera::CaptureInfoResponse>> {
        self.client.server_streaming(&METHOD_CAMERA_SERVICE_SUBSCRIBE_CAPTURE_INFO, req, opt)
    }

    pub fn subscribe_capture_info(&self, req: &super::camera::SubscribeCaptureInfoRequest) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::camera::CaptureInfoResponse>> {
        self.subscribe_capture_info_opt(req, ::grpcio::CallOption::default())
    }

    pub fn subscribe_camera_status_opt(&self, req: &super::camera::SubscribeCameraStatusRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::camera::CameraStatusResponse>> {
        self.client.server_streaming(&METHOD_CAMERA_SERVICE_SUBSCRIBE_CAMERA_STATUS, req, opt)
    }

    pub fn subscribe_camera_status(&self, req: &super::camera::SubscribeCameraStatusRequest) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::camera::CameraStatusResponse>> {
        self.subscribe_camera_status_opt(req, ::grpcio::CallOption::default())
    }

    pub fn subscribe_current_settings_opt(&self, req: &super::camera::SubscribeCurrentSettingsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::camera::CurrentSettingsResponse>> {
        self.client.server_streaming(&METHOD_CAMERA_SERVICE_SUBSCRIBE_CURRENT_SETTINGS, req, opt)
    }

    pub fn subscribe_current_settings(&self, req: &super::camera::SubscribeCurrentSettingsRequest) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::camera::CurrentSettingsResponse>> {
        self.subscribe_current_settings_opt(req, ::grpcio::CallOption::default())
    }

    pub fn subscribe_possible_setting_options_opt(&self, req: &super::camera::SubscribePossibleSettingOptionsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::camera::PossibleSettingOptionsResponse>> {
        self.client.server_streaming(&METHOD_CAMERA_SERVICE_SUBSCRIBE_POSSIBLE_SETTING_OPTIONS, req, opt)
    }

    pub fn subscribe_possible_setting_options(&self, req: &super::camera::SubscribePossibleSettingOptionsRequest) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::camera::PossibleSettingOptionsResponse>> {
        self.subscribe_possible_setting_options_opt(req, ::grpcio::CallOption::default())
    }

    pub fn set_setting_opt(&self, req: &super::camera::SetSettingRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::camera::SetSettingResponse> {
        self.client.unary_call(&METHOD_CAMERA_SERVICE_SET_SETTING, req, opt)
    }

    pub fn set_setting(&self, req: &super::camera::SetSettingRequest) -> ::grpcio::Result<super::camera::SetSettingResponse> {
        self.set_setting_opt(req, ::grpcio::CallOption::default())
    }

    pub fn set_setting_async_opt(&self, req: &super::camera::SetSettingRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::camera::SetSettingResponse>> {
        self.client.unary_call_async(&METHOD_CAMERA_SERVICE_SET_SETTING, req, opt)
    }

    pub fn set_setting_async(&self, req: &super::camera::SetSettingRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::camera::SetSettingResponse>> {
        self.set_setting_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait CameraService {
    fn take_photo(&mut self, ctx: ::grpcio::RpcContext, req: super::camera::TakePhotoRequest, sink: ::grpcio::UnarySink<super::camera::TakePhotoResponse>);
    fn start_photo_interval(&mut self, ctx: ::grpcio::RpcContext, req: super::camera::StartPhotoIntervalRequest, sink: ::grpcio::UnarySink<super::camera::StartPhotoIntervalResponse>);
    fn stop_photo_interval(&mut self, ctx: ::grpcio::RpcContext, req: super::camera::StopPhotoIntervalRequest, sink: ::grpcio::UnarySink<super::camera::StopPhotoIntervalResponse>);
    fn start_video(&mut self, ctx: ::grpcio::RpcContext, req: super::camera::StartVideoRequest, sink: ::grpcio::UnarySink<super::camera::StartVideoResponse>);
    fn stop_video(&mut self, ctx: ::grpcio::RpcContext, req: super::camera::StopVideoRequest, sink: ::grpcio::UnarySink<super::camera::StopVideoResponse>);
    fn start_video_streaming(&mut self, ctx: ::grpcio::RpcContext, req: super::camera::StartVideoStreamingRequest, sink: ::grpcio::UnarySink<super::camera::StartVideoStreamingResponse>);
    fn stop_video_streaming(&mut self, ctx: ::grpcio::RpcContext, req: super::camera::StopVideoStreamingRequest, sink: ::grpcio::UnarySink<super::camera::StopVideoStreamingResponse>);
    fn set_mode(&mut self, ctx: ::grpcio::RpcContext, req: super::camera::SetModeRequest, sink: ::grpcio::UnarySink<super::camera::SetModeResponse>);
    fn subscribe_mode(&mut self, ctx: ::grpcio::RpcContext, req: super::camera::SubscribeModeRequest, sink: ::grpcio::ServerStreamingSink<super::camera::ModeResponse>);
    fn subscribe_video_stream_info(&mut self, ctx: ::grpcio::RpcContext, req: super::camera::SubscribeVideoStreamInfoRequest, sink: ::grpcio::ServerStreamingSink<super::camera::VideoStreamInfoResponse>);
    fn subscribe_capture_info(&mut self, ctx: ::grpcio::RpcContext, req: super::camera::SubscribeCaptureInfoRequest, sink: ::grpcio::ServerStreamingSink<super::camera::CaptureInfoResponse>);
    fn subscribe_camera_status(&mut self, ctx: ::grpcio::RpcContext, req: super::camera::SubscribeCameraStatusRequest, sink: ::grpcio::ServerStreamingSink<super::camera::CameraStatusResponse>);
    fn subscribe_current_settings(&mut self, ctx: ::grpcio::RpcContext, req: super::camera::SubscribeCurrentSettingsRequest, sink: ::grpcio::ServerStreamingSink<super::camera::CurrentSettingsResponse>);
    fn subscribe_possible_setting_options(&mut self, ctx: ::grpcio::RpcContext, req: super::camera::SubscribePossibleSettingOptionsRequest, sink: ::grpcio::ServerStreamingSink<super::camera::PossibleSettingOptionsResponse>);
    fn set_setting(&mut self, ctx: ::grpcio::RpcContext, req: super::camera::SetSettingRequest, sink: ::grpcio::UnarySink<super::camera::SetSettingResponse>);
}

pub fn create_camera_service<S: CameraService + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_CAMERA_SERVICE_TAKE_PHOTO, move |ctx, req, resp| {
        instance.take_photo(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_CAMERA_SERVICE_START_PHOTO_INTERVAL, move |ctx, req, resp| {
        instance.start_photo_interval(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_CAMERA_SERVICE_STOP_PHOTO_INTERVAL, move |ctx, req, resp| {
        instance.stop_photo_interval(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_CAMERA_SERVICE_START_VIDEO, move |ctx, req, resp| {
        instance.start_video(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_CAMERA_SERVICE_STOP_VIDEO, move |ctx, req, resp| {
        instance.stop_video(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_CAMERA_SERVICE_START_VIDEO_STREAMING, move |ctx, req, resp| {
        instance.start_video_streaming(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_CAMERA_SERVICE_STOP_VIDEO_STREAMING, move |ctx, req, resp| {
        instance.stop_video_streaming(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_CAMERA_SERVICE_SET_MODE, move |ctx, req, resp| {
        instance.set_mode(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_server_streaming_handler(&METHOD_CAMERA_SERVICE_SUBSCRIBE_MODE, move |ctx, req, resp| {
        instance.subscribe_mode(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_server_streaming_handler(&METHOD_CAMERA_SERVICE_SUBSCRIBE_VIDEO_STREAM_INFO, move |ctx, req, resp| {
        instance.subscribe_video_stream_info(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_server_streaming_handler(&METHOD_CAMERA_SERVICE_SUBSCRIBE_CAPTURE_INFO, move |ctx, req, resp| {
        instance.subscribe_capture_info(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_server_streaming_handler(&METHOD_CAMERA_SERVICE_SUBSCRIBE_CAMERA_STATUS, move |ctx, req, resp| {
        instance.subscribe_camera_status(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_server_streaming_handler(&METHOD_CAMERA_SERVICE_SUBSCRIBE_CURRENT_SETTINGS, move |ctx, req, resp| {
        instance.subscribe_current_settings(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_server_streaming_handler(&METHOD_CAMERA_SERVICE_SUBSCRIBE_POSSIBLE_SETTING_OPTIONS, move |ctx, req, resp| {
        instance.subscribe_possible_setting_options(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_CAMERA_SERVICE_SET_SETTING, move |ctx, req, resp| {
        instance.set_setting(ctx, req, resp)
    });
    builder.build()
}
