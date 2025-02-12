use crate::enums::NATProtocol;
use crate::event_detail::utility::cast_event;
use crate::event_detail::DetailEvent;
use crate::utility::macros::macros::{
    get_function_result_bool, get_function_result_number, get_function_result_str,
};
use crate::VboxError;
use log::error;
use std::fmt::Display;
use vbox_raw::sys_lib::{IEvent, INATRedirectEvent, INATREDIRECTEVENT_IID_STR};

/// Notification when NAT redirect rule added or removed.
#[derive(Debug)]
pub struct NATRedirectEvent {
    /// ID of the machine this event relates to.
    pub machine_id: &'static str,
    /// Adapter which NAT attached to.
    pub slot: u32,
    /// Whether rule remove or add.
    pub remove: bool,
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

impl NATRedirectEvent {
    pub fn new(object: *mut IEvent) -> DetailEvent {
        match Self::create(object) {
            Ok(detail) => DetailEvent::NATRedirectEvent(detail),
            Err(err) => {
                error!("NATRedirectEvent error:{}", err);
                DetailEvent::Null
            }
        }
    }
    fn create(object: *mut IEvent) -> Result<Self, VboxError> {
        let obj1 = cast_event(object, INATREDIRECTEVENT_IID_STR)?;
        let machine_id = Self::get_machine_id(obj1)?;
        let slot = Self::get_slot(obj1)?;
        let remove = Self::get_remove(obj1)?;
        let name = Self::get_name(obj1)?;
        let proto = Self::get_proto(obj1)?;
        let host_ip = Self::get_host_ip(obj1)?;
        let host_port = Self::get_host_port(obj1)?;
        let guest_ip = Self::get_guest_ip(obj1)?;
        let guest_port = Self::get_guest_port(obj1)?;
        Ok(Self {
            machine_id,
            slot,
            remove,
            name,
            proto,
            host_ip,
            host_port,
            guest_ip,
            guest_port,
        })
    }

    fn get_machine_id(object: *mut INATRedirectEvent) -> Result<&'static str, VboxError> {
        get_function_result_str!(object, GetMachineId)
    }
    fn get_slot(object: *mut INATRedirectEvent) -> Result<u32, VboxError> {
        get_function_result_number!(object, GetSlot, u32)
    }
    fn get_remove(object: *mut INATRedirectEvent) -> Result<bool, VboxError> {
        get_function_result_bool!(object, GetRemove)
    }
    fn get_name(object: *mut INATRedirectEvent) -> Result<&'static str, VboxError> {
        get_function_result_str!(object, GetName)
    }
    fn get_proto(object: *mut INATRedirectEvent) -> Result<NATProtocol, VboxError> {
        let proto = get_function_result_number!(object, GetProto, u32)?;
        Ok(NATProtocol::from(proto))
    }
    fn get_host_ip(object: *mut INATRedirectEvent) -> Result<&'static str, VboxError> {
        get_function_result_str!(object, GetHostIP)
    }
    fn get_host_port(object: *mut INATRedirectEvent) -> Result<i32, VboxError> {
        get_function_result_number!(object, GetHostPort, i32)
    }
    fn get_guest_ip(object: *mut INATRedirectEvent) -> Result<&'static str, VboxError> {
        get_function_result_str!(object, GetGuestIP)
    }
    fn get_guest_port(object: *mut INATRedirectEvent) -> Result<i32, VboxError> {
        get_function_result_number!(object, GetGuestPort, i32)
    }
}

impl Display for NATRedirectEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = format!("{:?}", self);
        write!(f, "{}", format!("{}", s))
    }
}
