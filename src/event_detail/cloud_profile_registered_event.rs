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
use vbox_raw::sys_lib::{ICloudProfileRegisteredEvent, ICLOUDPROFILEREGISTEREDEVENT_IID_STR};

use crate::event_detail::DetailEvent;
use std::fmt::Display;
use vbox_raw::sys_lib::IEvent;

/// A cloud provider was installed or uninstalled
#[derive(Debug)]
pub struct CloudProfileRegisteredEvent {
    pub provider_id: &'static str,
    pub name: &'static str,
    pub registered: bool,
}

#[cfg(not(is_v_6_1))]
impl CloudProfileRegisteredEvent {
    pub fn new(object: *mut IEvent) -> DetailEvent {
        match Self::create(object) {
            Ok(detail) => DetailEvent::CloudProfileRegisteredEvent(detail),
            Err(err) => {
                error!("CloudProfileRegisteredEvent error:{}", err);
                DetailEvent::Null
            }
        }
    }
    fn create(object: *mut IEvent) -> Result<Self, VboxError> {
        let obj1 = cast_event(object, ICLOUDPROFILEREGISTEREDEVENT_IID_STR)?;
        let registered = Self::get_registered(obj1)?;
        let provider_id = Self::get_provider_id(obj1)?;
        let name = Self::get_name(obj1)?;
        Ok(Self {
            provider_id,
            name,
            registered,
        })
    }

    fn get_provider_id(
        object: *mut ICloudProfileRegisteredEvent,
    ) -> Result<&'static str, VboxError> {
        get_function_result_str!(object, GetProviderId)
    }
    fn get_name(object: *mut ICloudProfileRegisteredEvent) -> Result<&'static str, VboxError> {
        get_function_result_str!(object, GetName)
    }
    fn get_registered(object: *mut ICloudProfileRegisteredEvent) -> Result<bool, VboxError> {
        get_function_result_bool!(object, GetRegistered)
    }
}

#[cfg(is_v_6_1)]
impl CloudProfileRegisteredEvent {
    pub fn new(_object: *mut IEvent) -> DetailEvent {
        DetailEvent::Null
    }
}
impl Display for CloudProfileRegisteredEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = format!("{:?}", self);
        write!(f, "{}", format!("{}", s))
    }
}
