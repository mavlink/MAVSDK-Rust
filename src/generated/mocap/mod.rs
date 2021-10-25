use super::super::FromRpcResponse;
use super::super::RequestError;
use super::super::RequestResult;
use super::super::TonicResult;
use std::convert::Into;

mod pb {
    include!("mavsdk.rpc.mocap.rs");
}

/// Global position/attitude estimate from a vision source.
#[derive(Default, Clone, PartialEq, Debug)]
pub struct VisionPositionEstimate {
    /// PositionBody frame timestamp UNIX Epoch time (0 to use Backend timestamp)
    pub time_usec: u64,
    /// Global position (m)
    pub position_body: PositionBody,
    /// Body angle (rad).
    pub angle_body: AngleBody,
    /// Pose cross-covariance matrix.
    pub pose_covariance: Covariance,
}

impl From<VisionPositionEstimate> for pb::VisionPositionEstimate {
    fn from(est: VisionPositionEstimate) -> Self {
        Self {
            time_usec: est.time_usec,
            position_body: Some(est.position_body.into()),
            angle_body: Some(est.angle_body.into()),
            pose_covariance: Some(est.pose_covariance.into()),
        }
    }
}

/// Motion capture attitude and position
#[derive(Clone, PartialEq, Debug)]
pub struct AttitudePositionMocap {
    /// PositionBody frame timestamp UNIX Epoch time (0 to use Backend timestamp)
    pub time_usec: u64,
    /// Attitude quaternion (w, x, y, z order, zero-rotation is 1, 0, 0, 0)
    pub q: Quaternion,
    /// Body Position (NED)
    pub position_body: PositionBody,
    /// Pose cross-covariance matrix.
    pub pose_covariance: Covariance,
}

impl From<AttitudePositionMocap> for pb::AttitudePositionMocap {
    fn from(attitude_position: AttitudePositionMocap) -> Self {
        Self {
            time_usec: attitude_position.time_usec,
            q: Some(attitude_position.q.into()),
            position_body: Some(attitude_position.position_body.into()),
            pose_covariance: Some(attitude_position.pose_covariance.into()),
        }
    }
}

/// Odometry message to communicate odometry mocaprmation with an external interface.
#[derive(Clone, PartialEq, Debug)]
pub struct Odometry {
    /// Timestamp (0 to use Backend timestamp).
    pub time_usec: u64,
    /// Coordinate frame of reference for the pose data.
    pub frame_id: i32,
    /// Body Position.
    pub position_body: PositionBody,
    /// Quaternion components, w, x, y, z (1 0 0 0 is the null-rotation).
    pub q: Quaternion,
    /// Linear speed (m/s).
    pub speed_body: SpeedBody,
    /// Angular speed (rad/s).
    pub angular_velocity_body: AngularVelocityBody,
    /// Pose cross-covariance matrix.
    pub pose_covariance: Covariance,
    /// Velocity cross-covariance matrix.
    pub velocity_covariance: Covariance,
}

impl From<Odometry> for pb::Odometry {
    fn from(odometry: Odometry) -> Self {
        Self {
            time_usec: odometry.time_usec,
            frame_id: odometry.frame_id,
            position_body: Some(odometry.position_body.into()),
            q: Some(odometry.q.into()),
            speed_body: Some(odometry.speed_body.into()),
            angular_velocity_body: Some(odometry.angular_velocity_body.into()),
            pose_covariance: Some(odometry.pose_covariance.into()),
            velocity_covariance: Some(odometry.velocity_covariance.into()),
        }
    }
}

/// Body position type
#[derive(Default, Clone, PartialEq, Debug)]
pub struct PositionBody {
    /// X position in metres.
    pub x_m: f32,
    /// Y position in metres.
    pub y_m: f32,
    /// Z position in metres.
    pub z_m: f32,
}

impl From<PositionBody> for pb::PositionBody {
    fn from(position: PositionBody) -> Self {
        Self {
            x_m: position.x_m,
            y_m: position.y_m,
            z_m: position.z_m,
        }
    }
}

/// Body angle type
#[derive(Default, Clone, PartialEq, Debug)]
pub struct AngleBody {
    /// Roll angle in radians.
    pub roll_rad: f32,
    /// Pitch angle in radians.
    pub pitch_rad: f32,
    /// Yaw angle in radians.
    pub yaw_rad: f32,
}

impl From<AngleBody> for pb::AngleBody {
    fn from(angle: AngleBody) -> Self {
        Self {
            roll_rad: angle.roll_rad,
            pitch_rad: angle.pitch_rad,
            yaw_rad: angle.yaw_rad,
        }
    }
}

/// Speed type, represented in the Body (X Y Z) frame and in metres/second.
#[derive(Default, Clone, PartialEq, Debug)]
pub struct SpeedBody {
    /// Velocity in X in metres/second.
    pub x_m_s: f32,
    /// Velocity in Y in metres/second.
    pub y_m_s: f32,
    /// Velocity in Z in metres/second.
    pub z_m_s: f32,
}

impl From<SpeedBody> for pb::SpeedBody {
    fn from(speed: SpeedBody) -> Self {
        Self {
            x_m_s: speed.x_m_s,
            y_m_s: speed.y_m_s,
            z_m_s: speed.z_m_s,
        }
    }
}

/// Angular velocity type
#[derive(Default, Clone, PartialEq, Debug)]
pub struct AngularVelocityBody {
    /// Roll angular velocity in radians/second.
    pub roll_rad_s: f32,
    /// Pitch angular velocity in radians/second.
    pub pitch_rad_s: f32,
    /// Yaw angular velocity in radians/second.
    pub yaw_rad_s: f32,
}

impl From<AngularVelocityBody> for pb::AngularVelocityBody {
    fn from(angular_velocity: AngularVelocityBody) -> Self {
        Self {
            roll_rad_s: angular_velocity.roll_rad_s,
            pitch_rad_s: angular_velocity.pitch_rad_s,
            yaw_rad_s: angular_velocity.yaw_rad_s,
        }
    }
}

/// Covariance type.
/// Row-major representation of a 6x6 cross-covariance matrix
/// upper right triangle.
/// Set first to NaN if unknown.
#[derive(Default, Clone, PartialEq, Debug)]
pub struct Covariance {
    pub covariance_matrix: ::std::vec::Vec<f32>,
}

impl From<Covariance> for pb::Covariance {
    fn from(covariance: Covariance) -> Self {
        Self {
            covariance_matrix: covariance.covariance_matrix,
        }
    }
}

///
/// Quaternion type.
///
/// All rotations and axis systems follow the right-hand rule.
/// The Hamilton quaternion product definition is used.
/// A zero-rotation quaternion is represented by (1,0,0,0).
/// The quaternion could also be written as w + xi + yj + zk.
///
/// For more info see: https://en.wikipedia.org/wiki/Quaternion
#[derive(Clone, PartialEq, Debug)]
pub struct Quaternion {
    /// Quaternion entry 0, also denoted as a
    pub w: f32,
    /// Quaternion entry 1, also denoted as b
    pub x: f32,
    /// Quaternion entry 2, also denoted as c
    pub y: f32,
    /// Quaternion entry 3, also denoted as d
    pub z: f32,
}

impl From<Quaternion> for pb::Quaternion {
    fn from(quarternion: Quaternion) -> Self {
        Self {
            w: quarternion.w,
            x: quarternion.x,
            y: quarternion.y,
            z: quarternion.z,
        }
    }
}

#[derive(PartialEq, Clone, Debug, thiserror::Error)]
pub enum MocapError {
    /// Unknown error
    #[error("Unknown error: {0}")]
    Unknown(String),
    /// No system is connected
    #[error("No system is connected: {0}")]
    NoSystem(String),
    /// Connection error
    #[error("Connection error: {0}")]
    ConnectionError(String),
    /// Invalid request data
    #[error("Invalid request: {0}")]
    InvalidRequestData(String),
}

impl From<MocapError> for RequestError<MocapError> {
    fn from(e: MocapError) -> Self {
        Self::Mav(e)
    }
}

pub type SetVisionPositionEstimateResult = RequestResult<(), MocapError>;

impl FromRpcResponse<pb::SetVisionPositionEstimateResponse> for SetVisionPositionEstimateResult {
    fn from_rpc_response(
        rpc_set_vision_position_estimate_response: TonicResult<
            pb::SetVisionPositionEstimateResponse,
        >,
    ) -> Self {
        let rpc_mocap_result = rpc_set_vision_position_estimate_response?
            .into_inner()
            .mocap_result
            .ok_or_else(|| MocapError::Unknown("MocapResult does not received".into()))?;

        let mocap_result = pb::mocap_result::Result::from_i32(rpc_mocap_result.result)
            .ok_or_else(|| MocapError::Unknown("Unsupported MocapResult.result value".into()))?;

        match mocap_result {
            pb::mocap_result::Result::Success => Ok(()),
            pb::mocap_result::Result::Unknown => {
                Err(MocapError::Unknown(rpc_mocap_result.result_str).into())
            }
            pb::mocap_result::Result::NoSystem => {
                Err(MocapError::NoSystem(rpc_mocap_result.result_str).into())
            }
            pb::mocap_result::Result::ConnectionError => {
                Err(MocapError::ConnectionError(rpc_mocap_result.result_str).into())
            }
            pb::mocap_result::Result::InvalidRequestData => {
                Err(MocapError::InvalidRequestData(rpc_mocap_result.result_str).into())
            }
        }
    }
}

#[doc = " Motion Capture allow vehicles to navigate when a global"]
#[doc = " position source is unavailable or unreliable"]
#[doc = " (e.g. indoors, or when flying under a bridge. etc.)."]
pub struct Mocap {
    service_client: pb::mocap_service_client::MocapServiceClient<tonic::transport::Channel>,
}

impl Mocap {
    pub async fn set_vision_position_estimate(
        &mut self,
        vision_position_estimate: VisionPositionEstimate,
    ) -> SetVisionPositionEstimateResult {
        let request = pb::SetVisionPositionEstimateRequest {
            vision_position_estimate: Some(vision_position_estimate.into()),
        };
        let response = self
            .service_client
            .set_vision_position_estimate(request)
            .await;
        SetVisionPositionEstimateResult::from_rpc_response(response)
    }
}

#[tonic::async_trait]
impl super::super::Connect for Mocap {
    async fn connect(url: &String) -> std::result::Result<Mocap, tonic::transport::Error> {
        Ok(Mocap {
            service_client: pb::mocap_service_client::MocapServiceClient::connect(url.clone())
                .await?,
        })
    }
}
