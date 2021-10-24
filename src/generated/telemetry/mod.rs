use super::super::RequestError::{MavErr, RpcErr};
use super::super::RequestResult;
use futures::stream::{Stream, StreamExt};
use futures::task::{Context, Poll};
use std::convert::From;
use std::pin::Pin;

mod pb {
    include!("mavsdk.rpc.telemetry.rs");
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
pub enum MavFrame {
    Undef = 0,
    /// Setpoint in body NED frame. This makes sense if all position control is
    /// externalized - e.g. useful to command 2 m/s^2 acceleration to the right.
    BodyNed = 8,
    /// Odometry local coordinate frame of data given by a vision estimation system,
    /// Z-down (x: north, y: east, z: down).
    VisionNed = 16,
    /// Odometry local coordinate frame of data given by an estimator running
    /// onboard the vehicle, Z-down (x: north, y: east, z: down).
    EstimNed = 18,
}

/// Odometry message type.
#[derive(Clone, PartialEq, Debug, Default)]
pub struct Odometry {
    /// Timestamp (0 to use Backend timestamp).
    pub time_usec: u64,
    /// Coordinate frame of reference for the pose data.
    pub frame_id: MavFrame,
    /// Coordinate frame of reference for the velocity in free space (twist) data.
    pub child_frame_id: MavFrame,
    /// Position.
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

/// Body position type
#[derive(Clone, PartialEq, Debug, Default)]
pub struct PositionBody {
    /// X position in metres.
    pub x_m: f32,
    /// Y position in metres.
    pub y_m: f32,
    /// Z position in metres.
    pub z_m: f32,
}

impl From<&pb::PositionBody> for PositionBody {
    fn from(rpc_position_body: &pb::PositionBody) -> Self {
        PositionBody {
            x_m: rpc_position_body.x_m,
            y_m: rpc_position_body.y_m,
            z_m: rpc_position_body.z_m,
        }
    }
}

/// Speed type, represented in the Body (X Y Z) frame and in metres/second.
#[derive(Clone, PartialEq, Debug, Default)]
pub struct SpeedBody {
    /// Velocity in X in metres/second.
    pub velocity_x_m_s: f32,
    /// Velocity in Y in metres/second.
    pub velocity_y_m_s: f32,
    /// Velocity in Z in metres/second.
    pub velocity_z_m_s: f32,
}

impl From<&pb::SpeedBody> for SpeedBody {
    fn from(rpc_speed_body: &pb::SpeedBody) -> Self {
        SpeedBody {
            velocity_x_m_s: rpc_speed_body.velocity_x_m_s,
            velocity_y_m_s: rpc_speed_body.velocity_y_m_s,
            velocity_z_m_s: rpc_speed_body.velocity_z_m_s,
        }
    }
}

impl Into<pb::SpeedBody> for SpeedBody {
    fn into(self) -> pb::SpeedBody {
        pb::SpeedBody {
            velocity_x_m_s: self.velocity_x_m_s,
            velocity_y_m_s: self.velocity_y_m_s,
            velocity_z_m_s: self.velocity_z_m_s,
        }
    }
}

/// Angular velocity type
#[derive(Clone, PartialEq, Debug, Default)]
pub struct AngularVelocityBody {
    /// Roll angular velocity in radians/second.
    pub roll_rad_s: f32,
    /// Pitch angular velocity in radians/second.
    pub pitch_rad_s: f32,
    /// Yaw angular velocity in radians/second.
    pub yaw_rad_s: f32,
}

impl From<&pb::AngularVelocityBody> for AngularVelocityBody {
    fn from(rpc_angular_velocity_body: &pb::AngularVelocityBody) -> Self {
        AngularVelocityBody {
            roll_rad_s: rpc_angular_velocity_body.roll_rad_s,
            pitch_rad_s: rpc_angular_velocity_body.pitch_rad_s,
            yaw_rad_s: rpc_angular_velocity_body.yaw_rad_s,
        }
    }
}

impl Into<pb::AngularVelocityBody> for AngularVelocityBody {
    fn into(self) -> pb::AngularVelocityBody {
        pb::AngularVelocityBody {
            roll_rad_s: self.roll_rad_s,
            pitch_rad_s: self.pitch_rad_s,
            yaw_rad_s: self.yaw_rad_s,
        }
    }
}

/// Covariance type.
/// Row-major representation of a 6x6 cross-covariance matrix
/// upper right triangle.
/// Set first to NaN if unknown.
#[derive(Clone, PartialEq, Debug, Default)]
pub struct Covariance {
    pub covariance_matrix: ::std::vec::Vec<f32>,
}

impl From<pb::Odometry> for Odometry {
    fn from(rpc_odometry: pb::Odometry) -> Odometry {
        Odometry {
            time_usec: 0,
            frame_id: MavFrame::from_i32(rpc_odometry.frame_id).unwrap(),
            child_frame_id: MavFrame::from_i32(rpc_odometry.child_frame_id).unwrap(),
            position_body: PositionBody::from(&rpc_odometry.position_body.unwrap()),
            q: Quaternion::from(&rpc_odometry.q.unwrap()),
            speed_body: SpeedBody::from(
                &rpc_odometry.speed_body.unwrap_or(pb::SpeedBody::default()),
            ),
            angular_velocity_body: AngularVelocityBody::from(
                &rpc_odometry.angular_velocity_body.unwrap(),
            ),
            pose_covariance: Covariance {
                covariance_matrix: rpc_odometry.pose_covariance.unwrap().covariance_matrix,
            },
            velocity_covariance: Covariance {
                covariance_matrix: rpc_odometry.velocity_covariance.unwrap().covariance_matrix,
            },
        }
    }
}

impl Into<pb::Covariance> for Covariance {
    fn into(self) -> pb::Covariance {
        pb::Covariance {
            covariance_matrix: self.covariance_matrix,
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
/// For more info see: <https://en.wikipedia.org/wiki/Quaternion>
#[derive(Clone, PartialEq, Debug, Default)]
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

impl From<&pb::Quaternion> for Quaternion {
    fn from(rpc_quaternion: &pb::Quaternion) -> Self {
        Quaternion {
            w: rpc_quaternion.w,
            x: rpc_quaternion.x,
            y: rpc_quaternion.y,
            z: rpc_quaternion.z,
        }
    }
}

impl Into<pb::Quaternion> for Quaternion {
    fn into(self) -> pb::Quaternion {
        pb::Quaternion {
            w: self.w,
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }
}

#[derive(Clone, Debug)]
pub enum TelemetryError {
    /// Unknown error
    Unknown(String),
    /// No system is connected
    NoSystem(String),
    /// Connection error
    ConnectionError(String),
    /// Invalid request data
    InvalidRequestData(String),
}

#[doc = ""]
#[doc = " Allow users to get vehicle telemetry and state information"]
#[doc = " (e.g. battery, GPS, RC connection, flight mode etc.) and set telemetry update rates."]
pub struct Telemetry {
    service_client: pb::telemetry_service_client::TelemetryServiceClient<tonic::transport::Channel>,
}

impl Telemetry {
    pub async fn subscribe_odometry(&mut self) -> Result<OdometryStream, tonic::Status> {
        let request = pb::SubscribeOdometryRequest {};
        let response = self.service_client.subscribe_odometry(request).await?;
        Ok(OdometryStream {
            streaming: response.into_inner(),
        })
    }
}

pub struct OdometryStream {
    streaming: tonic::codec::Streaming<pb::OdometryResponse>,
}

pub type OdometryResult = RequestResult<Odometry, TelemetryError>;

impl Stream for OdometryStream {
    type Item = OdometryResult;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        match self.streaming.poll_next_unpin(cx) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Ready(Some(rpc_result)) => match rpc_result {
                Ok(odometry_response) => match odometry_response.odometry {
                    Some(rpc_odometry) => Poll::Ready(Some(Ok(Odometry::from(rpc_odometry)))),
                    None => Poll::Ready(Some(Err(MavErr(TelemetryError::Unknown(
                        "Unexpected value".into(),
                    ))))),
                },
                Err(err) => Poll::Ready(Some(Err(RpcErr(err)))),
            },
        }
    }
}

#[tonic::async_trait]
impl super::super::Connect for Telemetry {
    async fn connect(url: &String) -> std::result::Result<Telemetry, tonic::transport::Error> {
        match pb::telemetry_service_client::TelemetryServiceClient::connect(url.clone()).await {
            Ok(client) => Ok(Telemetry {
                service_client: client,
            }),
            Err(err) => Err(err),
        }
    }
}
