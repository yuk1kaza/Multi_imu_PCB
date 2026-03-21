use heapless::{String, Vec};
use imu_core::{BusDescriptor, BusProfile, ImuDescriptor, ImuTargetId};
use imu_drivers::CandidateDriver;

pub const MAX_DEVICE_IMUS: usize = 16;

#[derive(Clone)]
pub struct ImuInstanceProfile {
    pub descriptor: ImuDescriptor,
    pub target: ImuTargetId,
    pub candidates: &'static [CandidateDriver],
    pub default_profiles: &'static [BusProfile],
}

pub struct DeviceProfile {
    pub system_id: u16,
    pub system_label: String<32>,
    pub buses: Vec<BusDescriptor, 8>,
    pub imus: Vec<ImuInstanceProfile, MAX_DEVICE_IMUS>,
}
