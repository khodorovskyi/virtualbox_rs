use crate::enums::GuestUserState;
use crate::event_detail::utility::cast_event;
use crate::event_detail::DetailEvent;
use crate::utility::macros::macros::{get_function_result_number, get_function_result_str};
use crate::VboxError;
use log::error;
use std::fmt::Display;
use vbox_raw::sys_lib::{IEvent, IGuestUserStateChangedEvent, IGUESTUSERSTATECHANGEDEVENT_IID_STR};

/// Notification when a guest user changed its state.
#[derive(Debug)]
pub struct GuestUserStateChangedEvent {
    /// Name of the guest user whose state changed.
    pub name: &'static str,
    /// Name of the FQDN (fully qualified domain name) this user is bound to.
    pub domain: &'static str,
    /// What was changed for this guest user. [`GuestUserState`]
    pub state: GuestUserState,
    /// Optional state details, depending on the state attribute.
    pub state_details: &'static str,
}

impl GuestUserStateChangedEvent {
    pub fn new(object: *mut IEvent) -> DetailEvent {
        match Self::create(object) {
            Ok(detail) => DetailEvent::GuestUserStateChangedEvent(detail),
            Err(err) => {
                error!("GuestUserStateChangedEvent error:{}", err);
                DetailEvent::Null
            }
        }
    }
    fn create(object: *mut IEvent) -> Result<Self, VboxError> {
        let obj1 = cast_event(object, IGUESTUSERSTATECHANGEDEVENT_IID_STR)?;
        let name = Self::get_name(obj1)?;
        let domain = Self::get_domain(obj1)?;
        let state = Self::get_state(obj1)?;
        let state_details = Self::get_state_details(obj1)?;
        Ok(Self {
            name,
            domain,
            state,
            state_details,
        })
    }

    fn get_name(object: *mut IGuestUserStateChangedEvent) -> Result<&'static str, VboxError> {
        get_function_result_str!(object, GetName)
    }
    fn get_domain(object: *mut IGuestUserStateChangedEvent) -> Result<&'static str, VboxError> {
        get_function_result_str!(object, GetDomain)
    }
    fn get_state(object: *mut IGuestUserStateChangedEvent) -> Result<GuestUserState, VboxError> {
        let state = get_function_result_number!(object, GetState, u32)?;
        Ok(GuestUserState::from(state))
    }
    fn get_state_details(
        object: *mut IGuestUserStateChangedEvent,
    ) -> Result<&'static str, VboxError> {
        get_function_result_str!(object, GetStateDetails)
    }
}

impl Display for GuestUserStateChangedEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = format!("{:?}", self);
        write!(f, "{}", format!("{}", s))
    }
}
