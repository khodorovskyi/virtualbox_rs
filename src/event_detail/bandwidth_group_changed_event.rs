use crate::event_detail::utility::cast_event;
use crate::event_detail::DetailEvent;
use crate::utility::macros::macros::get_function_result_pointer;
use crate::{BandwidthGroup, VboxError};
use log::error;
use std::fmt::Display;
use vbox_raw::sys_lib::{
    IBandwidthGroup, IBandwidthGroupChangedEvent, IEvent, IBANDWIDTHGROUPCHANGEDEVENT_IID_STR,
};

/// Notification when VBoxSVC becomes unavailable (due to a crash or similar unexpected circumstances) or available again
#[derive(Debug)]
pub struct BandwidthGroupChangedEvent {
    /// The changed bandwidth group [`BandwidthGroup`].
    pub bandwidth_group: BandwidthGroup,
}

impl BandwidthGroupChangedEvent {
    pub fn new(object: *mut IEvent) -> DetailEvent {
        match Self::create(object) {
            Ok(detail) => DetailEvent::BandwidthGroupChangedEvent(detail),
            Err(err) => {
                error!("BandwidthGroupChangedEvent error:{}", err);
                DetailEvent::Null
            }
        }
    }
    fn create(object: *mut IEvent) -> Result<Self, VboxError> {
        let obj1 = cast_event(object, IBANDWIDTHGROUPCHANGEDEVENT_IID_STR)?;
        let bandwidth_group = Self::get_bandwidth_group(obj1)?;
        Ok(Self { bandwidth_group })
    }

    fn get_bandwidth_group(
        new_obj: *mut IBandwidthGroupChangedEvent,
    ) -> Result<BandwidthGroup, VboxError> {
        let bandwidth_group =
            get_function_result_pointer!(new_obj, GetBandwidthGroup, *mut IBandwidthGroup)?;
        Ok(BandwidthGroup::new(bandwidth_group))
    }
}

impl Display for BandwidthGroupChangedEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = format!("{:?}", self);
        write!(f, "{}", format!("{}", s))
    }
}
