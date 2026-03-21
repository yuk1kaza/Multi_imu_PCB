use imu_core::{BusId, BusMode, BusProfile, ImuId, ImuKind, ImuTargetId};
use imu_drivers::{bmi270, hxy42688, icm42688, lsm6, qmi8658, CandidateDriver};

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TransportMode {
    Json,
    Binary,
}

pub const SPI_FREQ_KHZ: u32 = 1_000;
pub const STREAM_INTERVAL_MS: u64 = 50;
pub const POWER_UP_DELAY_MS: u64 = 500;
pub const SYSTEM_ID: u16 = 1;
pub const BUS_ID: BusId = BusId(0);
#[cfg(feature = "json-transport")]
pub const TRANSPORT_MODE: TransportMode = TransportMode::Json;
#[cfg(feature = "binary-transport")]
pub const TRANSPORT_MODE: TransportMode = TransportMode::Binary;

pub const PROFILE_MODE0: BusProfile = BusProfile::new(0, BusMode::Mode0, SPI_FREQ_KHZ);
pub const PROFILE_MODE1: BusProfile = BusProfile::new(1, BusMode::Mode1, SPI_FREQ_KHZ);
pub const PROFILE_MODE2: BusProfile = BusProfile::new(2, BusMode::Mode2, SPI_FREQ_KHZ);
pub const PROFILE_MODE3: BusProfile = BusProfile::new(3, BusMode::Mode3, SPI_FREQ_KHZ);
pub const PROFILE_MODE0_500K: BusProfile = BusProfile::new(4, BusMode::Mode0, 500);
pub const PROFILE_MODE3_500K: BusProfile = BusProfile::new(5, BusMode::Mode3, 500);
pub const PROFILE_MODE0_100K: BusProfile = BusProfile::new(6, BusMode::Mode0, 100);
pub const PROFILE_MODE3_100K: BusProfile = BusProfile::new(7, BusMode::Mode3, 100);
pub const SLOT3_OPTIONAL: bool = true;

pub const PROFILES_MODE0: [BusProfile; 1] = [PROFILE_MODE0];
pub const PROFILES_MODE3: [BusProfile; 1] = [PROFILE_MODE3];
pub const PROFILES_MODE0_3: [BusProfile; 2] = [PROFILE_MODE0, PROFILE_MODE3];
pub const PROFILES_ALL: [BusProfile; 4] =
    [PROFILE_MODE0, PROFILE_MODE1, PROFILE_MODE2, PROFILE_MODE3];
pub const PROFILES_BMI: [BusProfile; 8] = [
    PROFILE_MODE3,
    PROFILE_MODE0,
    PROFILE_MODE1,
    PROFILE_MODE2,
    PROFILE_MODE3_500K,
    PROFILE_MODE0_500K,
    PROFILE_MODE3_100K,
    PROFILE_MODE0_100K,
];

#[derive(Clone, Copy)]
pub struct BoardImuConfig {
    pub imu_id: ImuId,
    pub target: ImuTargetId,
    pub label: &'static str,
    pub expected: ImuKind,
    pub candidates: &'static [CandidateDriver],
}

pub static SLOT1_CANDIDATES: [CandidateDriver; 4] = [
    CandidateDriver {
        descriptor: &hxy42688::DESCRIPTOR,
        profiles: &PROFILES_MODE0_3,
    },
    CandidateDriver {
        descriptor: &lsm6::DESCRIPTOR,
        profiles: &PROFILES_MODE3,
    },
    CandidateDriver {
        descriptor: &icm42688::DESCRIPTOR,
        profiles: &PROFILES_MODE0,
    },
    CandidateDriver {
        descriptor: &qmi8658::DESCRIPTOR,
        profiles: &PROFILES_MODE0,
    },
];

pub static SLOT2_CANDIDATES: [CandidateDriver; 3] = [
    CandidateDriver {
        descriptor: &icm42688::DESCRIPTOR,
        profiles: &PROFILES_ALL,
    },
    CandidateDriver {
        descriptor: &qmi8658::DESCRIPTOR,
        profiles: &PROFILES_ALL,
    },
    CandidateDriver {
        descriptor: &lsm6::DESCRIPTOR,
        profiles: &PROFILES_MODE3,
    },
];

pub static SLOT3_CANDIDATES: [CandidateDriver; 2] = [
    CandidateDriver {
        descriptor: &bmi270::DESCRIPTOR,
        profiles: &PROFILES_BMI,
    },
    CandidateDriver {
        descriptor: &bmi270::DESCRIPTOR,
        profiles: &PROFILES_MODE0,
    },
];

pub static SLOT4_CANDIDATES: [CandidateDriver; 2] = [
    CandidateDriver {
        descriptor: &qmi8658::DESCRIPTOR,
        profiles: &PROFILES_MODE0,
    },
    CandidateDriver {
        descriptor: &icm42688::DESCRIPTOR,
        profiles: &PROFILES_MODE0,
    },
];

pub static SLOT5_CANDIDATES: [CandidateDriver; 4] = [
    CandidateDriver {
        descriptor: &lsm6::DESCRIPTOR,
        profiles: &PROFILES_MODE0_3,
    },
    CandidateDriver {
        descriptor: &hxy42688::DESCRIPTOR,
        profiles: &PROFILES_MODE0_3,
    },
    CandidateDriver {
        descriptor: &qmi8658::DESCRIPTOR,
        profiles: &PROFILES_MODE0_3,
    },
    CandidateDriver {
        descriptor: &icm42688::DESCRIPTOR,
        profiles: &PROFILES_MODE0,
    },
];

pub static BOARD_IMUS: [BoardImuConfig; 5] = [
    BoardImuConfig {
        imu_id: ImuId {
            system_id: SYSTEM_ID,
            sensor_id: 1,
        },
        target: ImuTargetId {
            bus_id: BUS_ID,
            target_index: 0,
        },
        label: "slot-1",
        expected: ImuKind::Icm42688Hxy,
        candidates: &SLOT1_CANDIDATES,
    },
    BoardImuConfig {
        imu_id: ImuId {
            system_id: SYSTEM_ID,
            sensor_id: 2,
        },
        target: ImuTargetId {
            bus_id: BUS_ID,
            target_index: 1,
        },
        label: "slot-2",
        expected: ImuKind::Icm42688Pc,
        candidates: &SLOT2_CANDIDATES,
    },
    BoardImuConfig {
        imu_id: ImuId {
            system_id: SYSTEM_ID,
            sensor_id: 3,
        },
        target: ImuTargetId {
            bus_id: BUS_ID,
            target_index: 2,
        },
        label: "slot-3",
        expected: ImuKind::Bmi270,
        candidates: &SLOT3_CANDIDATES,
    },
    BoardImuConfig {
        imu_id: ImuId {
            system_id: SYSTEM_ID,
            sensor_id: 4,
        },
        target: ImuTargetId {
            bus_id: BUS_ID,
            target_index: 3,
        },
        label: "slot-4",
        expected: ImuKind::Qmi8658A,
        candidates: &SLOT4_CANDIDATES,
    },
    BoardImuConfig {
        imu_id: ImuId {
            system_id: SYSTEM_ID,
            sensor_id: 5,
        },
        target: ImuTargetId {
            bus_id: BUS_ID,
            target_index: 4,
        },
        label: "slot-5",
        expected: ImuKind::Sc7u22,
        candidates: &SLOT5_CANDIDATES,
    },
];
