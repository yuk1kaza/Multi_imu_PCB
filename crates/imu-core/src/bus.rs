use serde::{Deserialize, Serialize};

use crate::types::ImuError;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BusId(pub u8);

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ImuTargetId {
    pub bus_id: BusId,
    pub target_index: u8,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum BusMode {
    Mode0,
    Mode1,
    Mode2,
    Mode3,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct BusProfile {
    pub id: u8,
    pub mode: BusMode,
    pub frequency_khz: u32,
}

impl BusProfile {
    pub const fn new(id: u8, mode: BusMode, frequency_khz: u32) -> Self {
        Self {
            id,
            mode,
            frequency_khz,
        }
    }
}

pub trait ImuBus {
    fn apply_profile(
        &mut self,
        target: ImuTargetId,
        profile: BusProfile,
    ) -> Result<(), ImuError>;
    fn write_regs(
        &mut self,
        target: ImuTargetId,
        reg: u8,
        data: &[u8],
    ) -> Result<(), ImuError>;
    fn read_regs(
        &mut self,
        target: ImuTargetId,
        reg: u8,
        dummy_bytes: usize,
        data: &mut [u8],
    ) -> Result<(), ImuError>;
    fn delay_ms(&mut self, ms: u64);

    fn write_reg(&mut self, target: ImuTargetId, reg: u8, value: u8) -> Result<(), ImuError> {
        self.write_regs(target, reg, &[value])
    }

    fn read_reg(
        &mut self,
        target: ImuTargetId,
        reg: u8,
        dummy_bytes: usize,
    ) -> Result<u8, ImuError> {
        let mut data = [0u8; 1];
        self.read_regs(target, reg, dummy_bytes, &mut data)?;
        Ok(data[0])
    }
}
