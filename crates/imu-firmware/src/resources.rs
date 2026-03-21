use imu_core::{DriverResourceKey, DriverResources};

pub struct EmptyResources;

impl DriverResources for EmptyResources {
    fn bytes(&self, _key: DriverResourceKey) -> Option<&[u8]> {
        None
    }
}
