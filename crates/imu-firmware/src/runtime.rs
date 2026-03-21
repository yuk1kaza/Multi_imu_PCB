use imu_core::{BusProfile, ImuBus, ImuDriver, ImuError, ImuTargetId};
use imu_drivers::CandidateDriver;

pub fn probe_first_matching(
    bus: &mut dyn ImuBus,
    target: ImuTargetId,
    candidates: &[CandidateDriver],
) -> Result<Option<(&'static dyn ImuDriver, BusProfile)>, ImuError> {
    for candidate in candidates {
        for profile in candidate.profiles {
            bus.apply_profile(target, *profile)?;
            bus.delay_ms(1);

            match candidate.descriptor.driver.probe(bus, target) {
                Ok(true) => return Ok(Some((candidate.descriptor.driver, *profile))),
                Ok(false) => continue,
                Err(ImuError::CommunicationError) => continue,
                Err(error) => return Err(error),
            }
        }
    }

    Ok(None)
}
