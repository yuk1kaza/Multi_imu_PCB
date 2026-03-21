use crate::bus::{ImuBus, ImuTargetId};
use crate::resource::DriverResources;
use crate::sample::{RawSample, ScaleProfile};
use crate::types::{ImuCapabilities, ImuConfig, ImuError, ImuKind};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ImuTargetInfo {
    pub id: crate::types::ImuId,
    pub target: ImuTargetId,
}

pub trait ImuDriver: Sync {
    fn kind(&self) -> ImuKind;
    fn probe(&self, bus: &mut dyn ImuBus, target: ImuTargetId) -> Result<bool, ImuError>;
    fn reset(&self, bus: &mut dyn ImuBus, target: ImuTargetId) -> Result<(), ImuError>;
    fn configure(
        &self,
        bus: &mut dyn ImuBus,
        target: ImuTargetId,
        config: &ImuConfig,
        resources: &dyn DriverResources,
    ) -> Result<(), ImuError>;
    fn read_raw(&self, bus: &mut dyn ImuBus, target: ImuTargetId) -> Result<RawSample, ImuError>;
    fn scale_profile(&self) -> ScaleProfile;
    fn capabilities(&self) -> ImuCapabilities;
}
