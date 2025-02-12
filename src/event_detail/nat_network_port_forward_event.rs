use crate::enums::NATProtocol;
use crate::event_detail::utility::cast_event;
use crate::event_detail::DetailEvent;
use crate::utility::macros::macros::{
    get_function_result_bool, get_function_result_number, get_function_result_str,
};
use crate::VboxError;
use log::error;
use std::fmt::Display;
use vbox_raw::sys_lib::{IEvent, INATNetworkPortForwardEvent, INATNETWORKPORTFORWARDEVENT_IID_STR};

/// Notification when NAT redirect rule added or removed.
#[derive(Debug)]
pub struct NATNetworkPortForwardEvent {
    /// Name of network
    pub network_name: &'static str,
    pub midl_does_not_like_empty_interfaces: bool,
    pub create: bool,
    pub ipv6: bool,
    /// Name of the rule.
    pub name: &'static str,
    /// [`NATProtocol`] (TCP or UDP) of the redirect rule.
    pub proto: NATProtocol,
    /// Host ip address to bind socket on.
    pub host_ip: &'static str,
    /// Host port to bind socket on.
    pub host_port: i32,
    /// Guest ip address to redirect to.
    pub guest_ip: &'static str,
    /// Guest port to redirect to.
    pub guest_port: i32,
}

impl NATNetworkPortForwardEvent {
    pub fn new(object: *mut IEvent) -> DetailEvent {
        match Self::create(object) {
            Ok(detail) => DetailEvent::NATNetworkPortForwardEvent(detail),
            Err(err) => {
                error!("NATNetworkPortForwardEvent error:{}", err);
                DetailEvent::Null
            }
        }
    }
    fn create(object: *mut IEvent) -> Result<Self, VboxError> {
        let obj1 = cast_event(object, INATNETWORKPORTFORWARDEVENT_IID_STR)?;
        let network_name = Self::get_network_name(obj1)?;
        let midl_does_not_like_empty_interfaces =
            Self::get_midl_does_not_like_empty_interfaces(obj1)?;
        let create = Self::get_create(obj1)?;
        let ipv6 = Self::get_ipv6(obj1)?;
        let name = Self::get_name(obj1)?;
        let proto = Self::get_proto(obj1)?;
        let host_ip = Self::get_host_ip(obj1)?;
        let host_port = Self::get_host_port(obj1)?;
        let guest_ip = Self::get_guest_ip(obj1)?;
        let guest_port = Self::get_guest_port(obj1)?;
        Ok(Self {
            network_name,
            midl_does_not_like_empty_interfaces,
            create,
            ipv6,
            name,
            proto,
            host_ip,
            host_port,
            guest_ip,
            guest_port,
        })
    }

    fn get_network_name(
        object: *mut INATNetworkPortForwardEvent,
    ) -> Result<&'static str, VboxError> {
        get_function_result_str!(object, GetNetworkName)
    }
    fn get_midl_does_not_like_empty_interfaces(
        new_obj: *mut INATNetworkPortForwardEvent,
    ) -> Result<bool, VboxError> {
        get_function_result_bool!(new_obj, GetMidlDoesNotLikeEmptyInterfaces)
    }

    fn get_create(new_obj: *mut INATNetworkPortForwardEvent) -> Result<bool, VboxError> {
        get_function_result_bool!(new_obj, GetCreate)
    }
    fn get_ipv6(new_obj: *mut INATNetworkPortForwardEvent) -> Result<bool, VboxError> {
        get_function_result_bool!(new_obj, GetIpv6)
    }
    fn get_name(object: *mut INATNetworkPortForwardEvent) -> Result<&'static str, VboxError> {
        get_function_result_str!(object, GetName)
    }
    fn get_proto(object: *mut INATNetworkPortForwardEvent) -> Result<NATProtocol, VboxError> {
        let proto = get_function_result_number!(object, GetProto, u32)?;
        Ok(NATProtocol::from(proto))
    }

    fn get_host_ip(object: *mut INATNetworkPortForwardEvent) -> Result<&'static str, VboxError> {
        get_function_result_str!(object, GetHostIp)
    }
    fn get_host_port(object: *mut INATNetworkPortForwardEvent) -> Result<i32, VboxError> {
        get_function_result_number!(object, GetHostPort, i32)
    }
    fn get_guest_ip(object: *mut INATNetworkPortForwardEvent) -> Result<&'static str, VboxError> {
        get_function_result_str!(object, GetGuestIp)
    }
    fn get_guest_port(object: *mut INATNetworkPortForwardEvent) -> Result<i32, VboxError> {
        get_function_result_number!(object, GetGuestPort, i32)
    }
}

impl Display for NATNetworkPortForwardEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = format!("{:?}", self);
        write!(f, "{}", format!("{}", s))
    }
}
