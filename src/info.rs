use crate::FromRpcResponse;
use crate::RequestError;
use crate::RequestResult;
use crate::TonicResult;

mod pb {
    tonic::include_proto!("mavsdk.rpc.info");
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

#[derive(PartialEq, Clone, Debug, thiserror::Error)]
pub enum Error {
    #[error("Unknown error: {0}")]
    Unknown(String),
    #[error("Information not yet received: {0}")]
    InformationNotReceivedYet(String),
}

impl From<Error> for RequestError<Error> {
    fn from(e: Error) -> Self {
        Self::Mav(e)
    }
}

pub type GetVersionResult = RequestResult<Version, Error>;

impl FromRpcResponse<pb::GetVersionResponse> for GetVersionResult {
    fn from_rpc_response(rpc_get_version_response: TonicResult<pb::GetVersionResponse>) -> Self {
        let get_version_response = rpc_get_version_response?.into_inner();

        let rpc_info_result = get_version_response
            .info_result
            .ok_or_else(|| Error::Unknown("InfoResult does not received".into()))?;

        let info_result = pb::info_result::Result::from_i32(rpc_info_result.result)
            .ok_or_else(|| Error::Unknown("Unsupported InfoResult.result value".into()))?;

        match info_result {
            pb::info_result::Result::Success => match get_version_response.version {
                Some(ref rpc_version) => Ok(Version::from(rpc_version)),
                None => Err(Error::Unknown("Version does not received".into()).into()),
            },
            pb::info_result::Result::Unknown => {
                Err(Error::Unknown(rpc_info_result.result_str).into())
            }
            pb::info_result::Result::InformationNotReceivedYet => {
                Err(Error::InformationNotReceivedYet(rpc_info_result.result_str).into())
            }
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
impl crate::Connect for Info {
    async fn connect(url: String) -> std::result::Result<Info, tonic::transport::Error> {
        Ok(Info {
            service_client: pb::info_service_client::InfoServiceClient::connect(url).await?,
        })
    }
}
