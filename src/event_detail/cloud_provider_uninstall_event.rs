#[cfg(not(is_v_6_1))]
use crate::event_detail::utility::cast_event;
#[cfg(not(is_v_6_1))]
use crate::utility::macros::macros::get_function_result_str;
#[cfg(not(is_v_6_1))]
use crate::VboxError;
#[cfg(not(is_v_6_1))]
use log::error;
#[cfg(not(is_v_6_1))]
use vbox_raw::sys_lib::{ICloudProviderUninstallEvent, ICLOUDPROVIDERUNINSTALLEVENT_IID_STR};

use crate::event_detail::DetailEvent;
use std::fmt::Display;
use vbox_raw::sys_lib::IEvent;

/// A cloud provider was installed or uninstalled
#[derive(Debug)]
pub struct CloudProviderUninstallEvent {
    pub id: &'static str,
}

#[cfg(not(is_v_6_1))]
impl CloudProviderUninstallEvent {
    pub fn new(object: *mut IEvent) -> DetailEvent {
        match Self::create(object) {
            Ok(detail) => DetailEvent::CloudProviderUninstallEvent(detail),
            Err(err) => {
                error!("CloudProviderUninstallEvent error:{}", err);
                DetailEvent::Null
            }
        }
    }
    fn create(object: *mut IEvent) -> Result<Self, VboxError> {
        let obj1 = cast_event(object, ICLOUDPROVIDERUNINSTALLEVENT_IID_STR)?;
        let id = Self::get_id(obj1)?;
        Ok(Self { id })
    }

    fn get_id(object: *mut ICloudProviderUninstallEvent) -> Result<&'static str, VboxError> {
        get_function_result_str!(object, GetId)
    }
}

#[cfg(is_v_6_1)]
impl CloudProviderUninstallEvent {
    pub fn new(_object: *mut IEvent) -> DetailEvent {
        DetailEvent::Null
    }
}
impl Display for CloudProviderUninstallEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = format!("{:?}", self);
        write!(f, "{}", format!("{}", s))
    }
}
