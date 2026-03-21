use imu_core::{
    DriverResources, ImuBus, ImuCapabilities, ImuConfig, ImuDriver, ImuError, ImuKind,
    ImuTargetId, RangeDps, RangeG, RawSample, ScaleProfile,
};

const CHIP_ID: u8 = 0x6A;
const COM_CFG_DEFAULT: u8 = 0x50;

const REG_WHO_AM_I: u8 = 0x01;
const REG_COM_CFG: u8 = 0x05;
const REG_DATA_STAT: u8 = 0x0B;
const REG_ACC_XH: u8 = 0x0C;
const REG_ACC_CONF: u8 = 0x40;
const REG_ACC_RANGE: u8 = 0x41;
const REG_GYR_CONF: u8 = 0x42;
const REG_GYR_RANGE: u8 = 0x43;
const REG_PWR_CTRL: u8 = 0x7D;

pub static DRIVER: Hxy42688Driver = Hxy42688Driver;
pub static DESCRIPTOR: crate::DriverDescriptor = crate::DriverDescriptor {
    name: "ICM-42688-HXY",
    driver: &DRIVER,
};

pub struct Hxy42688Driver;

impl ImuDriver for Hxy42688Driver {
    fn kind(&self) -> ImuKind {
        ImuKind::Icm42688Hxy
    }

    fn probe(&self, bus: &mut dyn ImuBus, target: ImuTargetId) -> Result<bool, ImuError> {
        for _ in 0..3 {
            let id = bus.read_reg(target, REG_WHO_AM_I, 0)?;
            let com_cfg = bus.read_reg(target, REG_COM_CFG, 0)?;
            if id == CHIP_ID && com_cfg == COM_CFG_DEFAULT {
                return Ok(true);
            }
            bus.delay_ms(5);
        }
        Ok(false)
    }

    fn reset(&self, _bus: &mut dyn ImuBus, _target: ImuTargetId) -> Result<(), ImuError> {
        Ok(())
    }

    fn configure(
        &self,
        bus: &mut dyn ImuBus,
        target: ImuTargetId,
        _config: &ImuConfig,
        _resources: &dyn DriverResources,
    ) -> Result<(), ImuError> {
        bus.write_reg(target, REG_PWR_CTRL, 0x0E)?;
        bus.delay_ms(10);
        bus.write_reg(target, REG_ACC_CONF, 0xA8)?;
        bus.write_reg(target, REG_ACC_RANGE, 0x02)?;
        bus.write_reg(target, REG_GYR_CONF, 0xA9)?;
        bus.write_reg(target, REG_GYR_RANGE, 0x00)?;
        bus.delay_ms(5);
        Ok(())
    }

    fn read_raw(&self, bus: &mut dyn ImuBus, target: ImuTargetId) -> Result<RawSample, ImuError> {
        let status = bus.read_reg(target, REG_DATA_STAT, 0)?;
        if status & 0x03 == 0 {
            return Err(ImuError::DataNotReady);
        }

        let mut buf = [0u8; 12];
        bus.read_regs(target, REG_ACC_XH, 0, &mut buf)?;
        Ok(RawSample {
            accel: [
                i16::from_be_bytes([buf[0], buf[1]]),
                i16::from_be_bytes([buf[2], buf[3]]),
                i16::from_be_bytes([buf[4], buf[5]]),
            ],
            gyro: [
                i16::from_be_bytes([buf[6], buf[7]]),
                i16::from_be_bytes([buf[8], buf[9]]),
                i16::from_be_bytes([buf[10], buf[11]]),
            ],
            temp: None,
        })
    }

    fn scale_profile(&self) -> ScaleProfile {
        ScaleProfile {
            accel_g_per_lsb: 1.0 / 4096.0,
            gyro_dps_per_lsb: 1.0 / 16.4,
            temp_c_per_lsb: None,
            temp_offset_c: 0.0,
        }
    }

    fn capabilities(&self) -> ImuCapabilities {
        ImuCapabilities {
            has_temp: false,
            supports_fifo: false,
            supports_data_ready_interrupt: false,
            supported_accel_ranges: [Some(RangeG(4)), Some(RangeG(8)), Some(RangeG(16)), None],
            supported_gyro_ranges: [Some(RangeDps(250)), Some(RangeDps(500)), Some(RangeDps(1000)), Some(RangeDps(2000))],
        }
    }
}
