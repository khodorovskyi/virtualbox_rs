#[cfg(not(is_v_6_1))]
use crate::event_detail::utility::cast_event;
use crate::event_detail::DetailEvent;
#[cfg(not(is_v_6_1))]
use crate::utility::macros::macros::get_function_result_bool;
#[cfg(not(is_v_6_1))]
use crate::VboxError;
#[cfg(not(is_v_6_1))]
use log::error;
use std::fmt::Display;
use vbox_raw::sys_lib::IEvent;
#[cfg(not(is_v_6_1))]
use vbox_raw::sys_lib::{ICloudProviderListChangedEvent, ICLOUDPROVIDERLISTCHANGEDEVENT_IID_STR};

/// Each individual provider that is installed or uninstalled is reported as an CloudProviderRegisteredEvent.
#[derive(Debug)]
pub struct CloudProviderListChangedEvent {
    pub registered: bool,
}

#[cfg(not(is_v_6_1))]
impl CloudProviderListChangedEvent {
    pub fn new(object: *mut IEvent) -> DetailEvent {
        match Self::create(object) {
            Ok(detail) => DetailEvent::CloudProviderListChangedEvent(detail),
            Err(err) => {
                error!("CloudProviderListChangedEvent error:{}", err);
                DetailEvent::Null
            }
        }
    }
    fn create(object: *mut IEvent) -> Result<Self, VboxError> {
        let obj1 = cast_event(object, ICLOUDPROVIDERLISTCHANGEDEVENT_IID_STR)?;
        let registered = Self::get_registered(obj1)?;
        Ok(Self { registered })
    }

    fn get_registered(object: *mut ICloudProviderListChangedEvent) -> Result<bool, VboxError> {
        get_function_result_bool!(object, GetRegistered)
    }
}

#[cfg(is_v_6_1)]
impl CloudProviderListChangedEvent {
    pub fn new(_object: *mut IEvent) -> DetailEvent {
        DetailEvent::Null
    }
}
impl Display for CloudProviderListChangedEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = format!("{:?}", self);
        write!(f, "{}", format!("{}", s))
    }
}
