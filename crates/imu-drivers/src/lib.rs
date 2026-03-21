#![no_std]

pub mod bmi270;
pub mod hxy42688;
pub mod icm42688;
pub mod lsm6;
pub mod qmi8658;

use imu_core::{BusProfile, ImuDriver};

pub struct DriverDescriptor {
    pub name: &'static str,
    pub driver: &'static dyn ImuDriver,
}

#[derive(Clone, Copy)]
pub struct CandidateDriver {
    pub descriptor: &'static DriverDescriptor,
    pub profiles: &'static [BusProfile],
}
