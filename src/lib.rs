mod generated;

pub use generated::mocap;
pub use generated::info;

pub struct System {
    pub mocap : Box<dyn mocap::MocapService>,
    pub info : Box<dyn info::InfoService>,
    channel : ::grpcio::Client
}

impl System {
}