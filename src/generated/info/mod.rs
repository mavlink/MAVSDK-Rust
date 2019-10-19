mod info_grpc;
mod info;

use super::super::FromRpcResult;

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
pub enum InfoError {
    Unknown(String),
    InformationNotReceivedYet(String),
}

pub type GetVersionResult = super::super::RequestResult<Version, InfoError>;

impl FromRpcResult<info::GetVersionResponse> for GetVersionResult {
    fn from_rpc_result(
        rpc_get_version_response: ::grpcio::Result<info::GetVersionResponse>,
    ) -> Self {
        match rpc_get_version_response {
            Ok(verison_response) => match verison_response.get_info_result().get_result() {
                info::InfoResult_Result::UNKNOWN => Err(super::super::RequestError::MavErr(
                    InfoError::Unknown(verison_response.get_info_result().get_result_str().into()),
                )),
                info::InfoResult_Result::INFORMATION_NOT_RECEIVED_YET => Err(
                    super::super::RequestError::MavErr(InfoError::InformationNotReceivedYet(
                        verison_response.get_info_result().get_result_str().into(),
                    )),
                ),
                info::InfoResult_Result::SUCCESS => {
                    Ok(Version::from(verison_response.get_version()))
                }
            },
            Err(e) => Err(super::super::RequestError::RpcErr(e)),
        }
    }
}

#[derive(Clone)]
pub struct Info {
    service_client: info_grpc::InfoServiceClient,
}

impl Info {
    pub fn get_version(&self) -> GetVersionResult {
        let req = info::GetVersionRequest::new();
        GetVersionResult::from_rpc_result(
            self.service_client
                .get_version_opt(&req, ::grpcio::CallOption::default()),
        )
    }
}

impl super::super::FromChannel for Info {
    fn new(channel: &::grpcio::Channel) -> Self {
        Info {
            service_client: info_grpc::InfoServiceClient::new(channel.clone()),
        }
    }
}
