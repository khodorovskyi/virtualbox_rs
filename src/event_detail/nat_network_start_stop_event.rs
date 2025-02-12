use crate::event_detail::utility::cast_event;
use crate::event_detail::DetailEvent;
use crate::utility::macros::macros::{get_function_result_bool, get_function_result_str};
use crate::VboxError;
use log::error;
use std::fmt::Display;
use vbox_raw::sys_lib::{IEvent, INATNetworkStartStopEvent, INATNETWORKSTARTSTOPEVENT_IID_STR};

/// Notification when NAT redirect rule added or removed.
#[derive(Debug)]
pub struct NATNetworkStartStopEvent {
    /// Name of network
    pub network_name: &'static str,
    /// IsStartEvent is true when NAT network is started and false on stopping.
    pub start_event: bool,
}

impl NATNetworkStartStopEvent {
    pub fn new(object: *mut IEvent) -> DetailEvent {
        match Self::create(object) {
            Ok(detail) => DetailEvent::NATNetworkStartStopEvent(detail),
            Err(err) => {
                error!("NATNetworkStartStopEvent error:{}", err);
                DetailEvent::Null
            }
        }
    }
    fn create(object: *mut IEvent) -> Result<Self, VboxError> {
        let obj1 = cast_event(object, INATNETWORKSTARTSTOPEVENT_IID_STR)?;
        let network_name = Self::get_network_name(obj1)?;
        let start_event = Self::get_start_event(obj1)?;
        Ok(Self {
            network_name,
            start_event,
        })
    }

    fn get_network_name(object: *mut INATNetworkStartStopEvent) -> Result<&'static str, VboxError> {
        get_function_result_str!(object, GetNetworkName)
    }
    fn get_start_event(object: *mut INATNetworkStartStopEvent) -> Result<bool, VboxError> {
        get_function_result_bool!(object, GetStartEvent)
    }
}

impl Display for NATNetworkStartStopEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = format!("{:?}", self);
        write!(f, "{}", format!("{}", s))
    }
}
