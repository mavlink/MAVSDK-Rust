mod info_grpc;
mod info;

use info::InfoResult_Result;

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

#[derive(PartialEq, Clone, Debug)]
pub struct InfoResult {
    result: Result,
    result_str: String,
}

#[derive(PartialEq, Clone, Debug)]
pub struct GetVersionResponse {
    info_result: InfoResult,
    version: Option<Version>,
}

#[derive(Clone)]
pub struct Info {
    service_client: info_grpc::InfoServiceClient,
}

impl Info {
    pub fn get_version(&self) -> std::result::Result<GetVersionResponse, grpcio::Error> {
        let req = info::GetVersionRequest::new();
        let response = self
            .service_client
            .get_version_opt(&req, ::grpcio::CallOption::default());
        match response {
            Ok(res) => Ok(match res.clone().info_result.unwrap().result {
                InfoResult_Result::SUCCESS => GetVersionResponse {
                    info_result: InfoResult {
                        result: Result::Success,
                        result_str: res.info_result.unwrap().result_str,
                    },
                    version: Some(Version::from_grpc(res.version.unwrap())),
                },
                InfoResult_Result::UNKNOWN => GetVersionResponse {
                    info_result: InfoResult {
                        result: Result::Unknown,
                        result_str: res.info_result.unwrap().result_str,
                    },
                    version: None,
                },
                InfoResult_Result::INFORMATION_NOT_RECEIVED_YET => GetVersionResponse {
                    info_result: InfoResult {
                        result: Result::InformationNotReceivedYet,
                        result_str: res.info_result.unwrap().result_str,
                    },
                    version: None,
                },
            }),
            Err(e) => Err(e),
        }
    }
}

impl super::super::FromChannel for Info {
    fn new(channel: &::grpcio::Channel) -> Self {
        Info {
            service_client: info_grpc::InfoServiceClient::new(channel),
        }
    }
}
