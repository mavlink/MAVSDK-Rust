mod mocap;
mod mocap_grpc;

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub enum Result {
    Unknown = 0,
    Success = 1,
    NoSystem = 2,
    ConnectionError = 3,
    InvalidRequestData = 4,
}

impl From<&mocap::MocapResult_Result> for Result {
    fn from(grpc_result: &mocap::MocapResult_Result) -> Self {
        match grpc_result {
            mocap::MocapResult_Result::UNKNOWN => Result::Unknown,
            mocap::MocapResult_Result::SUCCESS => Result::Success,
            mocap::MocapResult_Result::NO_SYSTEM => Result::NoSystem,
            mocap::MocapResult_Result::CONNECTION_ERROR => Result::ConnectionError,
            mocap::MocapResult_Result::INVALID_REQUEST_DATA => Result::InvalidRequestData,
        }
    }
}

impl Into<mocap::MocapResult_Result> for Result {
    fn into(self) -> mocap::MocapResult_Result {
        match self {
            Result::Unknown => mocap::MocapResult_Result::UNKNOWN,
            Result::Success => mocap::MocapResult_Result::SUCCESS,
            Result::NoSystem => mocap::MocapResult_Result::NO_SYSTEM,
            Result::ConnectionError => mocap::MocapResult_Result::CONNECTION_ERROR,
            Result::InvalidRequestData => mocap::MocapResult_Result::INVALID_REQUEST_DATA,
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub enum MavFrame {
    MocapNed = 14,
    LocalFrd = 20,
}

impl Default for MavFrame {
    fn default() -> Self {
        Self::MocapNed
    }
}

impl Into<mocap::Odometry_MavFrame> for MavFrame {
    fn into(self) -> mocap::Odometry_MavFrame {
        match self {
            MavFrame::MocapNed => mocap::Odometry_MavFrame::MOCAP_NED,
            MavFrame::LocalFrd => mocap::Odometry_MavFrame::LOCAL_FRD,
        }
    }
}

impl From<mocap::Odometry_MavFrame> for MavFrame {
    fn from(rpc_mav_frame: mocap::Odometry_MavFrame) -> Self {
        match rpc_mav_frame {
            mocap::Odometry_MavFrame::MOCAP_NED => MavFrame::MocapNed,
            mocap::Odometry_MavFrame::LOCAL_FRD => MavFrame::LocalFrd,
            _ => Self::default(),
        }
    }
}

#[derive(PartialEq, Clone, Default)]
pub struct Quaternion {
    pub w: f32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Into<mocap::Quaternion> for Quaternion {
    fn into(self) -> mocap::Quaternion {
        let mut rpc_quaternion = mocap::Quaternion::new();
        rpc_quaternion.set_w(self.w);
        rpc_quaternion.set_x(self.x);
        rpc_quaternion.set_y(self.y);
        rpc_quaternion.set_z(self.z);
        rpc_quaternion
    }
}

impl From<&mocap::Quaternion> for Quaternion {
    fn from(rpc_quaternion: &mocap::Quaternion) -> Self {
        Self {
            w: rpc_quaternion.get_w(),
            x: rpc_quaternion.get_x(),
            y: rpc_quaternion.get_y(),
            z: rpc_quaternion.get_z(),
        }
    }
}

#[derive(PartialEq, Clone)]
pub struct Covariance {
    pub covariance_matrix: ::std::vec::Vec<f32>,
}

impl Default for Covariance {
    fn default() -> Self {
        Self {
            covariance_matrix: vec![std::f32::NAN],
        }
    }
}

impl Into<mocap::Covariance> for Covariance {
    fn into(self) -> mocap::Covariance {
        let mut rpc_covariance = mocap::Covariance::new();
        rpc_covariance.set_covariance_matrix(self.covariance_matrix);
        rpc_covariance
    }
}

impl From<&mocap::Covariance> for Covariance {
    fn from(rpc_covariance: &mocap::Covariance) -> Self {
        Self {
            covariance_matrix: rpc_covariance.get_covariance_matrix().to_vec(),
        }
    }
}

#[derive(PartialEq, Clone, Default)]
pub struct AngularVelocityBody {
    pub roll_rad_s: f32,
    pub pitch_rad_s: f32,
    pub yaw_rad_s: f32,
}

impl Into<mocap::AngularVelocityBody> for AngularVelocityBody {
    fn into(self) -> mocap::AngularVelocityBody {
        let mut rpc_angular_velocity_body = mocap::AngularVelocityBody::new();
        rpc_angular_velocity_body.set_roll_rad_s(self.roll_rad_s);
        rpc_angular_velocity_body.set_pitch_rad_s(self.pitch_rad_s);
        rpc_angular_velocity_body.set_yaw_rad_s(self.yaw_rad_s);
        rpc_angular_velocity_body
    }
}

impl From<&mocap::AngularVelocityBody> for AngularVelocityBody {
    fn from(rpc_angular_velocity_body: &mocap::AngularVelocityBody) -> Self {
        Self {
            roll_rad_s: rpc_angular_velocity_body.get_roll_rad_s(),
            pitch_rad_s: rpc_angular_velocity_body.get_pitch_rad_s(),
            yaw_rad_s: rpc_angular_velocity_body.get_yaw_rad_s(),
        }
    }
}

#[derive(PartialEq, Clone, Default)]
pub struct SpeedBody {
    pub x_m_s: f32,
    pub y_m_s: f32,
    pub z_m_s: f32,
}

impl Into<mocap::SpeedBody> for SpeedBody {
    fn into(self) -> mocap::SpeedBody {
        let mut rpc_speed_body = mocap::SpeedBody::new();
        rpc_speed_body.set_x_m_s(self.x_m_s);
        rpc_speed_body.set_y_m_s(self.y_m_s);
        rpc_speed_body.set_z_m_s(self.z_m_s);
        rpc_speed_body
    }
}

impl From<&mocap::SpeedBody> for SpeedBody {
    fn from(rpc_speed_body: &mocap::SpeedBody) -> Self {
        Self {
            x_m_s: rpc_speed_body.get_x_m_s(),
            y_m_s: rpc_speed_body.get_y_m_s(),
            z_m_s: rpc_speed_body.get_z_m_s(),
        }
    }
}

#[derive(PartialEq, Clone, Default)]
pub struct AngleBody {
    pub roll_rad: f32,
    pub pitch_rad: f32,
    pub yaw_rad: f32,
}

impl Into<mocap::AngleBody> for AngleBody {
    fn into(self) -> mocap::AngleBody {
        let mut rpc_angle_body = mocap::AngleBody::new();
        rpc_angle_body.set_roll_rad(self.roll_rad);
        rpc_angle_body.set_pitch_rad(self.pitch_rad);
        rpc_angle_body.set_yaw_rad(self.yaw_rad);
        rpc_angle_body
    }
}

impl From<&mocap::AngleBody> for AngleBody {
    fn from(rpc_angle_body: &mocap::AngleBody) -> Self {
        Self {
            roll_rad: rpc_angle_body.get_roll_rad(),
            pitch_rad: rpc_angle_body.get_pitch_rad(),
            yaw_rad: rpc_angle_body.get_yaw_rad(),
        }
    }
}

#[derive(PartialEq, Clone, Default)]
pub struct PositionBody {
    pub x_m: f32,
    pub y_m: f32,
    pub z_m: f32,
}

impl Into<mocap::PositionBody> for PositionBody {
    fn into(self) -> mocap::PositionBody {
        let mut rpc_position_body = mocap::PositionBody::new();
        rpc_position_body.set_x_m(self.x_m);
        rpc_position_body.set_y_m(self.y_m);
        rpc_position_body.set_z_m(self.z_m);
        rpc_position_body
    }
}

impl From<mocap::PositionBody> for PositionBody {
    fn from(rpc_position_body: mocap::PositionBody) -> Self {
        Self {
            x_m: rpc_position_body.get_x_m(),
            y_m: rpc_position_body.get_y_m(),
            z_m: rpc_position_body.get_z_m(),
        }
    }
}

impl From<&mocap::PositionBody> for PositionBody {
    fn from(rpc_position_body: &mocap::PositionBody) -> Self {
        Self {
            x_m: rpc_position_body.get_x_m(),
            y_m: rpc_position_body.get_y_m(),
            z_m: rpc_position_body.get_z_m(),
        }
    }
}

#[derive(PartialEq, Clone, Default)]
pub struct AttitudePositionMocap {
    pub time_usec: u64,
    pub q: Quaternion,
    pub position_body: PositionBody,
    pub pose_covariance: Covariance,
}

impl Into<mocap::AttitudePositionMocap> for AttitudePositionMocap {
    fn into(self) -> mocap::AttitudePositionMocap {
        let mut rpc_attitude_position_mocap = mocap::AttitudePositionMocap::new();
        rpc_attitude_position_mocap.set_time_usec(self.time_usec);
        rpc_attitude_position_mocap.set_q(self.q.into());
        rpc_attitude_position_mocap.set_position_body(self.position_body.into());
        rpc_attitude_position_mocap.set_pose_covariance(self.pose_covariance.into());
        rpc_attitude_position_mocap
    }
}

impl From<&mocap::AttitudePositionMocap> for AttitudePositionMocap {
    fn from(rpc_attitude_position_mocap: &mocap::AttitudePositionMocap) -> Self {
        Self {
            time_usec: rpc_attitude_position_mocap.get_time_usec(),
            q: Quaternion::from(rpc_attitude_position_mocap.get_q()),
            position_body: PositionBody::from(rpc_attitude_position_mocap.get_position_body()),
            pose_covariance: Covariance::from(rpc_attitude_position_mocap.get_pose_covariance()),
        }
    }
}

#[derive(PartialEq, Clone, Default)]
pub struct Odometry {
    pub time_usec: u64,
    pub frame_id: MavFrame,
    pub position_body: PositionBody,
    pub q: Quaternion,
    pub speed_body: SpeedBody,
    pub angular_velocity_body: AngularVelocityBody,
    pub pose_covariance: Covariance,
    pub velocity_covariance: Covariance,
}

impl Into<mocap::Odometry> for Odometry {
    fn into(self) -> mocap::Odometry {
        let mut rpc_odometry = mocap::Odometry::new();
        rpc_odometry.set_time_usec(self.time_usec);
        rpc_odometry.set_position_body(self.position_body.into());
        rpc_odometry.set_q(self.q.into());
        rpc_odometry.set_speed_body(self.speed_body.into());
        rpc_odometry.set_angular_velocity_body(self.angular_velocity_body.into());
        rpc_odometry.set_pose_covariance(self.pose_covariance.into());
        rpc_odometry.set_velocity_covariance(self.velocity_covariance.into());
        rpc_odometry
    }
}

impl From<&mocap::Odometry> for Odometry {
    fn from(rpc_odometry: &mocap::Odometry) -> Self {
        Self {
            time_usec: rpc_odometry.get_time_usec(),
            frame_id: MavFrame::from(rpc_odometry.get_frame_id()),
            position_body: PositionBody::from(rpc_odometry.get_position_body()),
            q: Quaternion::from(rpc_odometry.get_q()),
            speed_body: SpeedBody::from(rpc_odometry.get_speed_body()),
            angular_velocity_body: AngularVelocityBody::from(
                rpc_odometry.get_angular_velocity_body(),
            ),
            pose_covariance: Covariance::from(rpc_odometry.get_pose_covariance()),
            velocity_covariance: Covariance::from(rpc_odometry.get_velocity_covariance()),
        }
    }
}

#[derive(PartialEq, Clone, Default)]
pub struct VisionPositionEstimate {
    pub time_usec: u64,
    pub position_body: PositionBody,
    pub angle_body: AngleBody,
    pub pose_covariance: Covariance,
}

impl Into<mocap::VisionPositionEstimate> for VisionPositionEstimate {
    fn into(self) -> mocap::VisionPositionEstimate {
        let mut rpc_vision_position_estimate = mocap::VisionPositionEstimate::new();
        rpc_vision_position_estimate.set_time_usec(self.time_usec);
        rpc_vision_position_estimate.set_position_body(self.position_body.into());
        rpc_vision_position_estimate.set_angle_body(self.angle_body.into());
        rpc_vision_position_estimate.set_pose_covariance(self.pose_covariance.into());
        rpc_vision_position_estimate
    }
}

impl From<&mocap::VisionPositionEstimate> for VisionPositionEstimate {
    fn from(vision_position_estimate: &mocap::VisionPositionEstimate) -> Self {
        Self {
            time_usec: vision_position_estimate.get_time_usec(),
            position_body: PositionBody::from(vision_position_estimate.get_position_body()),
            angle_body: AngleBody::from(vision_position_estimate.get_angle_body()),
            pose_covariance: Covariance::from(vision_position_estimate.get_pose_covariance()),
        }
    }
}

#[derive(Clone)]
pub struct Mocap {
    service_client: mocap_grpc::MocapServiceClient,
}

#[derive(Clone, Debug)]
pub struct Response {
    pub result: mocap::MocapResult_Result,
    pub result_str: ::std::string::String,
}

pub type MocapResult = std::result::Result<Response, grpcio::Error>;

impl Mocap {
    pub fn set_vision_position_estimate(
        &self,
        vision_position_estimate: VisionPositionEstimate,
    ) -> MocapResult {
        let mut req = mocap::SetVisionPositionEstimateRequest::new();
        req.set_vision_position_estimate(vision_position_estimate.into());
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
        attitude_position_mocap: AttitudePositionMocap,
    ) -> MocapResult {
        let mut req = mocap::SetAttitudePositionMocapRequest::new();
        req.set_attitude_position_mocap(attitude_position_mocap.into());
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

    pub fn set_odometry(&self, odometry: Odometry) -> MocapResult {
        let mut req = mocap::SetOdometryRequest::new();
        req.set_odometry(odometry.into());
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

impl super::super::FromChannel for Mocap {
    fn new(channel: &::grpcio::Channel) -> Self {
        Mocap {
            service_client: mocap_grpc::MocapServiceClient::new(channel.clone()),
        }
    }
}

impl super::super::FromChannel for Mocap {
    fn new(channel: &::grpcio::Channel) -> Self {
        Mocap {
            service_client: mocap_grpc::MocapServiceClient::new(channel.clone())
        }
    }
}
