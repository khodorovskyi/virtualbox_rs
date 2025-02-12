#[cfg(not(is_v_6_1))]
use crate::event_detail::utility::cast_event;
#[cfg(not(is_v_6_1))]
use crate::utility::macros::macros::get_function_result_bool;
#[cfg(not(is_v_6_1))]
use crate::utility::macros::macros::get_function_result_str;
#[cfg(not(is_v_6_1))]
use crate::VboxError;
#[cfg(not(is_v_6_1))]
use log::error;
#[cfg(not(is_v_6_1))]
use vbox_raw::sys_lib::{ICloudProviderRegisteredEvent, ICLOUDPROVIDERREGISTEREDEVENT_IID_STR};

use crate::event_detail::DetailEvent;
use std::fmt::Display;
use vbox_raw::sys_lib::IEvent;

/// A cloud provider was installed or uninstalled
#[derive(Debug)]
pub struct CloudProviderRegisteredEvent {
    pub id: &'static str,

    pub registered: bool,
}

#[cfg(not(is_v_6_1))]
impl CloudProviderRegisteredEvent {
    pub fn new(object: *mut IEvent) -> DetailEvent {
        match Self::create(object) {
            Ok(detail) => DetailEvent::CloudProviderRegisteredEvent(detail),
            Err(err) => {
                error!("CloudProviderRegisteredEvent error:{}", err);
                DetailEvent::Null
            }
        }
    }
    fn create(object: *mut IEvent) -> Result<Self, VboxError> {
        let obj1 = cast_event(object, ICLOUDPROVIDERREGISTEREDEVENT_IID_STR)?;
        let registered = Self::get_registered(obj1)?;
        let id = Self::get_id(obj1)?;
        Ok(Self { id, registered })
    }

    fn get_id(object: *mut ICloudProviderRegisteredEvent) -> Result<&'static str, VboxError> {
        get_function_result_str!(object, GetId)
    }
    fn get_registered(object: *mut ICloudProviderRegisteredEvent) -> Result<bool, VboxError> {
        get_function_result_bool!(object, GetRegistered)
    }
}

#[cfg(is_v_6_1)]
impl CloudProviderRegisteredEvent {
    pub fn new(_object: *mut IEvent) -> DetailEvent {
        DetailEvent::Null
    }
}
impl Display for CloudProviderRegisteredEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = format!("{:?}", self);
        write!(f, "{}", format!("{}", s))
    }
}
