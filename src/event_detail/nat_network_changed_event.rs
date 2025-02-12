use crate::event_detail::utility::cast_event;
use crate::event_detail::DetailEvent;
use crate::utility::macros::macros::get_function_result_str;
use crate::VboxError;
use log::error;
use std::fmt::Display;
use vbox_raw::sys_lib::{IEvent, INATNetworkChangedEvent, INATNETWORKCHANGEDEVENT_IID_STR};

/// Notification when NAT redirect rule added or removed.
#[derive(Debug)]
pub struct NATNetworkChangedEvent {
    /// Name of network
    pub network_name: &'static str,
}

impl NATNetworkChangedEvent {
    pub fn new(object: *mut IEvent) -> DetailEvent {
        match Self::create(object) {
            Ok(detail) => DetailEvent::NATNetworkChangedEvent(detail),
            Err(err) => {
                error!("NATNetworkChangedEvent error:{}", err);
                DetailEvent::Null
            }
        }
    }
    fn create(object: *mut IEvent) -> Result<Self, VboxError> {
        let obj1 = cast_event(object, INATNETWORKCHANGEDEVENT_IID_STR)?;
        let network_name = Self::get_network_name(obj1)?;
        Ok(Self { network_name })
    }

    fn get_network_name(object: *mut INATNetworkChangedEvent) -> Result<&'static str, VboxError> {
        get_function_result_str!(object, GetNetworkName)
    }
}

impl Display for NATNetworkChangedEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = format!("{:?}", self);
        write!(f, "{}", format!("{}", s))
    }
}
