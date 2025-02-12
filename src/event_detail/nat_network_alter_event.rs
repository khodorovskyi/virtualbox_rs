use crate::event_detail::utility::cast_event;
use crate::event_detail::DetailEvent;
use crate::utility::macros::macros::{get_function_result_bool, get_function_result_str};
use crate::VboxError;
use log::error;
use std::fmt::Display;
use vbox_raw::sys_lib::{IEvent, INATNetworkAlterEvent, INATNETWORKALTEREVENT_IID_STR};

/// Notification when NAT redirect rule added or removed.
#[derive(Debug)]
pub struct NATNetworkAlterEvent {
    /// Name of network
    pub network_name: &'static str,
    pub midl_does_not_like_empty_interfaces: bool,
}

impl NATNetworkAlterEvent {
    pub fn new(object: *mut IEvent) -> DetailEvent {
        match Self::create(object) {
            Ok(detail) => DetailEvent::NATNetworkAlterEvent(detail),
            Err(err) => {
                error!("NATNetworkAlterEvent error:{}", err);
                DetailEvent::Null
            }
        }
    }
    fn create(object: *mut IEvent) -> Result<Self, VboxError> {
        let obj1 = cast_event(object, INATNETWORKALTEREVENT_IID_STR)?;
        let network_name = Self::get_network_name(obj1)?;
        let midl_does_not_like_empty_interfaces =
            Self::get_midl_does_not_like_empty_interfaces(obj1)?;
        Ok(Self {
            network_name,
            midl_does_not_like_empty_interfaces,
        })
    }

    fn get_network_name(object: *mut INATNetworkAlterEvent) -> Result<&'static str, VboxError> {
        get_function_result_str!(object, GetNetworkName)
    }

    fn get_midl_does_not_like_empty_interfaces(
        new_obj: *mut INATNetworkAlterEvent,
    ) -> Result<bool, VboxError> {
        get_function_result_bool!(new_obj, GetMidlDoesNotLikeEmptyInterfaces)
    }
}

impl Display for NATNetworkAlterEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = format!("{:?}", self);
        write!(f, "{}", format!("{}", s))
    }
}
