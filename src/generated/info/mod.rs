mod info_grpc;
mod info;

use super::super::MavsdkError;
use super::super::MavsdkResult;

#[derive(PartialEq, Clone, Default, Debug)]
pub struct Version {
    pub flight_sw_major: i32,
    pub flight_sw_minor: i32,
    pub flight_sw_patch: i32,
    pub flight_sw_vendor_major: i32,
    pub flight_sw_vendor_minor: i32,
    pub flight_sw_vendor_patch: i32,
    pub os_sw_major: i32,
    pub os_sw_minor: i32,
    pub os_sw_patch: i32,
}

impl From<&info::Version> for Version {
    fn from(grpc_version: &info::Version) -> Self {
        Version {
            flight_sw_major: grpc_version.flight_sw_major,
            flight_sw_minor: grpc_version.flight_sw_minor,
            flight_sw_patch: grpc_version.flight_sw_patch,
            flight_sw_vendor_major: grpc_version.flight_sw_vendor_major,
            flight_sw_vendor_minor: grpc_version.flight_sw_vendor_minor,
            flight_sw_vendor_patch: grpc_version.flight_sw_vendor_patch,
            os_sw_major: grpc_version.os_sw_major,
            os_sw_minor: grpc_version.os_sw_minor,
            os_sw_patch: grpc_version.os_sw_patch,
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub enum Result {
    Unknown,
    Success,
    InformationNotReceivedYet,
}

impl From<info::InfoResult_Result> for Result {
    fn from(grpc_result: info::InfoResult_Result) -> Self {
        match grpc_result {
            info::InfoResult_Result::UNKNOWN => Result::Unknown,
            info::InfoResult_Result::SUCCESS => Result::Success,
            info::InfoResult_Result::INFORMATION_NOT_RECEIVED_YET => {
                Result::InformationNotReceivedYet
            }
        }
    }
}

impl Into<info::InfoResult_Result> for Result {
    fn into(self) -> info::InfoResult_Result {
        match self {
            Result::Unknown => info::InfoResult_Result::UNKNOWN,
            Result::Success => info::InfoResult_Result::SUCCESS,
            Result::InformationNotReceivedYet => {
                info::InfoResult_Result::INFORMATION_NOT_RECEIVED_YET
            }
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct InfoResult {
    result: Result,
    result_str: String,
}

impl From<&info::InfoResult> for InfoResult {
    fn from(grpc_info_result: &info::InfoResult) -> Self {
        Self {
            result: Result::from(grpc_info_result.get_result()),
            result_str: String::from(grpc_info_result.get_result_str()),
        }
    }
}

impl Into<info::InfoResult> for InfoResult {
    fn into(self) -> info::InfoResult {
        let mut rpc_info_result = info::InfoResult::default();
        rpc_info_result.set_result(self.result.into());
        rpc_info_result.set_result_str(self.result_str);
        rpc_info_result
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct GetVersionResponse {
    info_result: InfoResult,
    version: Version,
}

impl From<info::GetVersionResponse> for GetVersionResponse {
    fn from(rpc_get_version_response: info::GetVersionResponse) -> Self {
        Self {
            info_result: InfoResult::from(rpc_get_version_response.get_info_result()),
            version: Version::from(rpc_get_version_response.get_version()),
        }
    }
}

#[derive(Clone)]
pub struct Info {
    service_client: info_grpc::InfoServiceClient,
}

impl Info {
    pub fn get_version(&self) -> MavsdkResult<GetVersionResponse, Result> {
        let req = info::GetVersionRequest::new();
        let response = self
            .service_client
            .get_version_opt(&req, ::grpcio::CallOption::default());
        match response {
            Ok(res) => match Result::from(res.get_info_result().get_result()) {
                Result::Success => MavsdkResult::Ok(GetVersionResponse::from(res)),
                Result::Unknown => MavsdkResult::Err(MavsdkError {
                    result: Result::from(res.get_info_result().get_result()),
                    result_str: String::from(res.get_info_result().get_result_str()),
                }),
                Result::InformationNotReceivedYet => MavsdkResult::Err(MavsdkError {
                    result: Result::from(res.get_info_result().get_result()),
                    result_str: String::from(res.get_info_result().get_result_str()),
                }),
            },
            Err(e) => MavsdkResult::RpcErr(e),
        }
    }
}

impl super::super::FromChannel for Info {
    fn new(channel: &::grpcio::Channel) -> Self {
        Info {
            service_client: info_grpc::InfoServiceClient::new(channel.clone()),
        }
    }
}
