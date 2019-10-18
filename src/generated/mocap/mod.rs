mod mocap;
mod mocap_grpc;

pub use mocap::AngleBody;
pub use mocap::AngularVelocityBody;
pub use mocap::AttitudePositionMocap;
pub use mocap::Covariance;
pub use mocap::MocapResult_Result;
pub use mocap::Odometry;
pub use mocap::Odometry_MavFrame;
pub use mocap::PositionBody;
pub use mocap::Quaternion;
pub use mocap::SpeedBody;
pub use mocap::VisionPositionEstimate;

#[derive(Clone)]
pub struct Mocap {
    service_client: mocap_grpc::MocapServiceClient,
}

#[derive(Clone, Debug)]
pub struct Response {
    pub result: mocap::MocapResult_Result,
    pub result_str: ::std::string::String,
}

pub type MocapResult = Result<Response, grpcio::Error>;

impl Mocap {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        Mocap {
            service_client: mocap_grpc::MocapServiceClient::new(channel),
        }
    }

    pub fn set_vision_position_estimate(
        &self,
        vision_position_estimate: mocap::VisionPositionEstimate,
    ) -> MocapResult {
        let mut req = mocap::SetVisionPositionEstimateRequest::new();
        req.set_vision_position_estimate(vision_position_estimate);
        let response = self
            .service_client
            .set_vision_position_estimate_opt(&req, ::grpcio::CallOption::default());
        match response {
            Ok(res) => Ok(Response {
                result: res.mocap_result.clone().unwrap().result,
                result_str: res.mocap_result.unwrap().result_str,
            }),
            Err(e) => Err(e),
        }
    }

    pub fn set_attitude_position_mocap(
        &self,
        attitude_position_mocap: mocap::AttitudePositionMocap,
    ) -> MocapResult {
        let mut req = mocap::SetAttitudePositionMocapRequest::new();
        req.set_attitude_position_mocap(attitude_position_mocap);
        let response = self
            .service_client
            .set_attitude_position_mocap_opt(&req, ::grpcio::CallOption::default());
        match response {
            Ok(res) => Ok(Response {
                result: res.mocap_result.clone().unwrap().result,
                result_str: res.mocap_result.unwrap().result_str,
            }),
            Err(e) => Err(e),
        }
    }

    pub fn set_odometry(&self, odometry: mocap::Odometry) -> MocapResult {
        let mut req = mocap::SetOdometryRequest::new();
        req.set_odometry(odometry);
        let response = self
            .service_client
            .set_odometry_opt(&req, ::grpcio::CallOption::default());
        match response {
            Ok(res) => Ok(Response {
                result: res.mocap_result.clone().unwrap().result,
                result_str: res.mocap_result.unwrap().result_str,
            }),
            Err(e) => Err(e),
        }
    }
}
