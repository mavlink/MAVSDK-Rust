/// Point type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Point {
    /// Latitude in degrees (range: -90 to +90)
    #[prost(double, tag = "1")]
    pub latitude_deg: f64,
    /// Longitude in degrees (range: -180 to +180)
    #[prost(double, tag = "2")]
    pub longitude_deg: f64,
}
/// Polygon type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Polygon {
    /// Points defining the polygon
    #[prost(message, repeated, tag = "1")]
    pub points: ::std::vec::Vec<Point>,
    /// Fence type
    #[prost(enumeration = "polygon::Type", tag = "2")]
    pub r#type: i32,
}
pub mod polygon {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Type {
        /// Type representing an inclusion fence
        Inclusion = 0,
        /// Type representing an exclusion fence
        Exclusion = 1,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UploadGeofenceRequest {
    /// Polygon(s) representing the geofence(s)
    #[prost(message, repeated, tag = "1")]
    pub polygons: ::std::vec::Vec<Polygon>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UploadGeofenceResponse {
    #[prost(message, optional, tag = "1")]
    pub geofence_result: ::std::option::Option<GeofenceResult>,
}
/// Result type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GeofenceResult {
    /// Result enum value
    #[prost(enumeration = "geofence_result::Result", tag = "1")]
    pub result: i32,
    /// Human-readable English string describing the result
    #[prost(string, tag = "2")]
    pub result_str: std::string::String,
}
pub mod geofence_result {
    /// Possible results returned for geofence requests.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Result {
        /// Unknown error
        Unknown = 0,
        /// Request succeeded
        Success = 1,
        /// Error
        Error = 2,
        /// Too many Polygon objects in the geofence
        TooManyGeofenceItems = 3,
        /// Vehicle is busy
        Busy = 4,
        /// Request timed out
        Timeout = 5,
        /// Invalid argument
        InvalidArgument = 6,
    }
}
#[doc = r" Generated server implementations."]
pub mod geofence_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Enable setting a geofence."]
    pub struct GeofenceServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl GeofenceServiceClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> GeofenceServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + HttpBody + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as HttpBody>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        #[doc = ""]
        #[doc = " Upload a geofence."]
        #[doc = ""]
        #[doc = " Polygons are uploaded to a drone. Once uploaded, the geofence will remain"]
        #[doc = " on the drone even if a connection is lost."]
        pub async fn upload_geofence(
            &mut self,
            request: impl tonic::IntoRequest<super::UploadGeofenceRequest>,
        ) -> Result<tonic::Response<super::UploadGeofenceResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.geofence.GeofenceService/UploadGeofence",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for GeofenceServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
}
