use heapless::String;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ImuKind {
    Unknown,
    Icm42688Hxy,
    Icm42688Pc,
    Bmi270,
    Qmi8658A,
    Sc7u22,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ImuId {
    pub system_id: u16,
    pub sensor_id: u16,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Quaternion {
    pub w: f32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RangeG(pub u16);

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RangeDps(pub u16);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum FilterProfile {
    Off,
    LowLatency,
    Balanced,
    LowNoise,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ViewMode {
    Raw6Axis,
    Quaternion,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImuConfig {
    pub accel_range: RangeG,
    pub gyro_range: RangeDps,
    pub sample_rate_hz: u16,
    pub filter_profile: FilterProfile,
}

impl Default for ImuConfig {
    fn default() -> Self {
        Self {
            accel_range: RangeG(8),
            gyro_range: RangeDps(2000),
            sample_rate_hz: 100,
            filter_profile: FilterProfile::Balanced,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImuCapabilities {
    pub has_temp: bool,
    pub supports_fifo: bool,
    pub supports_data_ready_interrupt: bool,
    pub supported_accel_ranges: [Option<RangeG>; 4],
    pub supported_gyro_ranges: [Option<RangeDps>; 4],
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ImuLocation {
    Slot(u8),
    Index(u16),
    Named(String<32>),
}

impl Default for ImuLocation {
    fn default() -> Self {
        Self::Index(0)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImuDescriptor {
    pub id: ImuId,
    pub bus_id: crate::bus::BusId,
    pub kind: ImuKind,
    pub location: ImuLocation,
    pub label: String<32>,
    pub capabilities: ImuCapabilities,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ImuError {
    CommunicationError,
    ChipNotFound,
    ConfigError,
    DataNotReady,
    MissingResource,
    UnsupportedConfig,
    InvalidTarget,
}
