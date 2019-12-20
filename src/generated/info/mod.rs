use super::super::FromRpcResponse;
use super::super::RequestError::{MavErr, RpcErr};
use super::super::RequestResult;
use super::super::TonicResult;

mod pb {
    include!("mavsdk.rpc.info.rs");
}

#[derive(PartialEq, Clone, Default, Debug)]
pub struct Version {
    /// Flight software major version
    pub flight_sw_major: i32,
    /// Flight software minor version
    pub flight_sw_minor: i32,
    /// Flight software patch version
    pub flight_sw_patch: i32,
    /// Flight software vendor major version
    pub flight_sw_vendor_major: i32,
    /// Flight software vendor minor version
    pub flight_sw_vendor_minor: i32,
    /// Flight software vendor patch version
    pub flight_sw_vendor_patch: i32,
    /// Operating system software major version
    pub os_sw_major: i32,
    /// Operating system software minor version
    pub os_sw_minor: i32,
    /// Operating system software patch version
    pub os_sw_patch: i32,
}

impl From<&pb::Version> for Version {
    fn from(rpc_version: &pb::Version) -> Self {
        Version {
            flight_sw_major: rpc_version.flight_sw_major,
            flight_sw_minor: rpc_version.flight_sw_minor,
            flight_sw_patch: rpc_version.flight_sw_patch,
            flight_sw_vendor_major: rpc_version.flight_sw_vendor_major,
            flight_sw_vendor_minor: rpc_version.flight_sw_vendor_minor,
            flight_sw_vendor_patch: rpc_version.flight_sw_vendor_patch,
            os_sw_major: rpc_version.os_sw_major,
            os_sw_minor: rpc_version.os_sw_minor,
            os_sw_patch: rpc_version.os_sw_patch,
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub enum InfoError {
    Unknown(String),
    InformationNotReceivedYet(String),
}

pub type GetVersionResult = RequestResult<Version, InfoError>;

impl FromRpcResponse<pb::GetVersionResponse> for GetVersionResult {
    fn from_rpc_response(rpc_get_version_response: TonicResult<pb::GetVersionResponse>) -> Self {
        match rpc_get_version_response {
            Ok(tonic_response) => {
                let get_version_response = tonic_response.into_inner();
                match &get_version_response.info_result {
                    Some(ref rpc_info_result) => {
                        match pb::info_result::Result::from_i32(rpc_info_result.result) {
                            Some(info_result) => match info_result {
                                pb::info_result::Result::Success => {
                                    match get_version_response.version {
                                        Some(ref rpc_version) => Ok(Version::from(rpc_version)),
                                        None => Err(MavErr(InfoError::Unknown(
                                            "Version does not received".into(),
                                        ))),
                                    }
                                }
                                pb::info_result::Result::Unknown => Err(MavErr(
                                    InfoError::Unknown(rpc_info_result.result_str.clone()),
                                )),
                                pb::info_result::Result::InformationNotReceivedYet => {
                                    Err(MavErr(InfoError::InformationNotReceivedYet(
                                        rpc_info_result.result_str.clone(),
                                    )))
                                }
                            },
                            None => Err(MavErr(InfoError::Unknown(
                                "Unsupported InfoResult.result value".into(),
                            ))),
                        }
                    }
                    None => Err(MavErr(InfoError::Unknown(
                        "InfoResult does not received".into(),
                    ))),
                }
            }
            Err(err) => Err(RpcErr(err)),
        }
    }
}

#[doc = " Provide infomation about the hardware and/or software of a system."]
pub struct Info {
    service_client: pb::info_service_client::InfoServiceClient<tonic::transport::Channel>,
}

impl Info {
    pub async fn get_version(&mut self) -> GetVersionResult {
        let request = pb::GetVersionRequest {};
        let response = self.service_client.get_version(request).await;
        GetVersionResult::from_rpc_response(response)
    }
}

#[tonic::async_trait]
impl super::super::Connect for Info {
    async fn connect(url: &String) -> std::result::Result<Info, tonic::transport::Error> {
        Ok(Info {
            service_client: pb::info_service_client::InfoServiceClient::connect(url.clone())
                .await?,
        })
    }
}
