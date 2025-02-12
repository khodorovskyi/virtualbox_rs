#[cfg(not(is_v_6_1))]
use crate::event_detail::utility::cast_event;
#[cfg(not(is_v_6_1))]
use crate::utility::macros::macros::get_function_result_str;
#[cfg(not(is_v_6_1))]
use crate::VboxError;
#[cfg(not(is_v_6_1))]
use log::error;
#[cfg(not(is_v_6_1))]
use vbox_raw::sys_lib::{ICloudProfileChangedEvent, ICLOUDPROFILECHANGEDEVENT_IID_STR};

use crate::event_detail::DetailEvent;
use std::fmt::Display;
use vbox_raw::sys_lib::IEvent;

#[derive(Debug)]
pub struct CloudProfileChangedEvent {
    pub provider_id: &'static str,
    pub name: &'static str,
}

#[cfg(not(is_v_6_1))]
impl CloudProfileChangedEvent {
    pub fn new(object: *mut IEvent) -> DetailEvent {
        match Self::create(object) {
            Ok(detail) => DetailEvent::CloudProfileChangedEvent(detail),
            Err(err) => {
                error!("CloudProfileChangedEvent error:{}", err);
                DetailEvent::Null
            }
        }
    }
    fn create(object: *mut IEvent) -> Result<Self, VboxError> {
        let obj1 = cast_event(object, ICLOUDPROFILECHANGEDEVENT_IID_STR)?;
        let name = Self::get_name(obj1)?;
        let provider_id = Self::get_provider_id(obj1)?;
        Ok(Self { provider_id, name })
    }

    fn get_provider_id(object: *mut ICloudProfileChangedEvent) -> Result<&'static str, VboxError> {
        get_function_result_str!(object, GetProviderId)
    }
    fn get_name(object: *mut ICloudProfileChangedEvent) -> Result<&'static str, VboxError> {
        get_function_result_str!(object, GetName)
    }
}

#[cfg(is_v_6_1)]
impl CloudProfileChangedEvent {
    pub fn new(_object: *mut IEvent) -> DetailEvent {
        DetailEvent::Null
    }
}
impl Display for CloudProfileChangedEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = format!("{:?}", self);
        write!(f, "{}", format!("{}", s))
    }
}
