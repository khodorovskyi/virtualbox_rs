use crate::event_detail::utility::cast_event;
use crate::event_detail::DetailEvent;
use crate::utility::macros::macros::{get_function_result_bool, get_function_result_str};
use crate::VboxError;
use log::error;
use std::fmt::Display;
use vbox_raw::sys_lib::{
    IEvent, INATNetworkCreationDeletionEvent, INATNETWORKCREATIONDELETIONEVENT_IID_STR,
};

/// Notification when NAT redirect rule added or removed.
#[derive(Debug)]
pub struct NATNetworkCreationDeletionEvent {
    /// Name of network
    pub network_name: &'static str,
    pub midl_does_not_like_empty_interfaces: bool,
    pub creation_event: bool,
}

impl NATNetworkCreationDeletionEvent {
    pub fn new(object: *mut IEvent) -> DetailEvent {
        match Self::create(object) {
            Ok(detail) => DetailEvent::NATNetworkCreationDeletionEvent(detail),
            Err(err) => {
                error!("NATNetworkCreationDeletionEvent error:{}", err);
                DetailEvent::Null
            }
        }
    }
    fn create(object: *mut IEvent) -> Result<Self, VboxError> {
        let obj1 = cast_event(object, INATNETWORKCREATIONDELETIONEVENT_IID_STR)?;
        let network_name = Self::get_network_name(obj1)?;
        let midl_does_not_like_empty_interfaces =
            Self::get_midl_does_not_like_empty_interfaces(obj1)?;
        let creation_event = Self::get_creation_event(obj1)?;
        Ok(Self {
            network_name,
            midl_does_not_like_empty_interfaces,
            creation_event,
        })
    }

    fn get_network_name(
        object: *mut INATNetworkCreationDeletionEvent,
    ) -> Result<&'static str, VboxError> {
        get_function_result_str!(object, GetNetworkName)
    }

    fn get_midl_does_not_like_empty_interfaces(
        new_obj: *mut INATNetworkCreationDeletionEvent,
    ) -> Result<bool, VboxError> {
        get_function_result_bool!(new_obj, GetMidlDoesNotLikeEmptyInterfaces)
    }
    fn get_creation_event(
        new_obj: *mut INATNetworkCreationDeletionEvent,
    ) -> Result<bool, VboxError> {
        get_function_result_bool!(new_obj, GetCreationEvent)
    }
}

impl Display for NATNetworkCreationDeletionEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = format!("{:?}", self);
        write!(f, "{}", format!("{}", s))
    }
}
