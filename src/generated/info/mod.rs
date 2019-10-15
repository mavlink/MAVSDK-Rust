mod info_grpc;
mod info;

pub use info::Version;
pub use info::InfoResult_Result;

#[derive(Clone)]
pub struct Info {
    service_client: info_grpc::InfoServiceClient
}

#[derive(Clone)]
#[derive(Debug)]
pub struct Response {
    pub result: info::InfoResult_Result,
    pub result_str: ::std::string::String,
}

pub type InfoResult = Result<Response, grpcio::Error>;

impl Info {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        Info {
            service_client: info_grpc::InfoServiceClient::new(channel)
        }
    }

    pub fn get_version(&self) -> InfoResult {
        let req = info::GetVersionRequest::new();
        let response = self.service_client.get_version_opt(&req, ::grpcio::CallOption::default());
        match response {
            Ok(res) => Ok(
                Response {
                    result: res.info_result.clone().unwrap().result,
                    result_str: res.info_result.unwrap().result_str
                }),
            Err(e) => Err(e)
        }
    }
}
