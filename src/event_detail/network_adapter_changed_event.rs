use crate::event_detail::utility::cast_event;
use crate::event_detail::DetailEvent;
use crate::utility::macros::macros::get_function_result_pointer;
use crate::{NetworkAdapter, VboxError};
use log::error;
use std::fmt::Display;
use vbox_raw::sys_lib::{
    IEvent, INetworkAdapter, INetworkAdapterChangedEvent, INETWORKADAPTERCHANGEDEVENT_IID_STR,
};

/// Notification when a Guest Additions property changes.
#[derive(Debug)]
pub struct NetworkAdapterChangedEvent {
    /// Network adapter that is subject to change.
    pub network_adapter: NetworkAdapter,
}

impl NetworkAdapterChangedEvent {
    pub fn new(object: *mut IEvent) -> DetailEvent {
        match Self::create(object) {
            Ok(detail) => DetailEvent::NetworkAdapterChangedEvent(detail),
            Err(err) => {
                error!("NetworkAdapterChangedEvent error:{}", err);
                DetailEvent::Null
            }
        }
    }
    fn create(object: *mut IEvent) -> Result<Self, VboxError> {
        let obj1 = cast_event(object, INETWORKADAPTERCHANGEDEVENT_IID_STR)?;
        let network_adapter = Self::get_network_adapter(obj1)?;
        Ok(Self { network_adapter })
    }

    fn get_network_adapter(
        new_obj: *mut INetworkAdapterChangedEvent,
    ) -> Result<NetworkAdapter, VboxError> {
        let network_adapter =
            get_function_result_pointer!(new_obj, GetNetworkAdapter, *mut INetworkAdapter)?;
        Ok(NetworkAdapter::new(network_adapter))
    }
}

impl Display for NetworkAdapterChangedEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = format!("{:?}", self);
        write!(f, "{}", format!("{}", s))
    }
}
