#![no_std]

pub mod bus;
pub mod driver;
pub mod protocol;
pub mod resource;
pub mod sample;
pub mod types;

pub use bus::{BusId, BusMode, BusProfile, ImuBus, ImuTargetId};
pub use driver::{ImuDriver, ImuTargetInfo};
pub use protocol::*;
pub use resource::{DriverResourceKey, DriverResources};
pub use sample::{default_scale_profile_for_kind, PhysicalSample, RawSample, ScaleProfile};
pub use types::{
    FilterProfile, ImuCapabilities, ImuConfig, ImuDescriptor, ImuError, ImuId, ImuKind,
    ImuLocation, Quaternion, RangeDps, RangeG, ViewMode,
};
