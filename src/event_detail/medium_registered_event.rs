use crate::enums::DeviceType;
use crate::event_detail::utility::cast_event;
use crate::event_detail::DetailEvent;
use crate::utility::macros::macros::{
    get_function_result_bool, get_function_result_number, get_function_result_str,
};
use crate::VboxError;
use log::error;
use std::fmt::Display;
use vbox_raw::sys_lib::{IEvent, IMediumRegisteredEvent, IMEDIUMREGISTEREDEVENT_IID_STR};

/// The given medium was registered or unregistered within this VirtualBox installation
#[derive(Debug)]
pub struct MediumRegisteredEvent {
    /// ID of the medium this event relates to.
    pub medium_id: &'static str,
    /// Type of the medium this event relates to. [`DeviceType`]
    pub medium_type: DeviceType,
    ///If true, the medium was registered, otherwise it was unregistered.
    pub registered: bool,
}

impl MediumRegisteredEvent {
    pub fn new(object: *mut IEvent) -> DetailEvent {
        match Self::create(object) {
            Ok(detail) => DetailEvent::MediumRegisteredEvent(detail),
            Err(err) => {
                error!("MediumRegisteredEvent error:{}", err);
                DetailEvent::Null
            }
        }
    }
    fn create(object: *mut IEvent) -> Result<Self, VboxError> {
        let obj1 = cast_event(object, IMEDIUMREGISTEREDEVENT_IID_STR)?;
        let medium_id = Self::get_medium_id(obj1)?;
        let medium_type = Self::get_medium_type(obj1)?;
        let registered = Self::get_registered(obj1)?;
        Ok(Self {
            medium_id,
            medium_type,
            registered,
        })
    }

    fn get_medium_id(object: *mut IMediumRegisteredEvent) -> Result<&'static str, VboxError> {
        get_function_result_str!(object, GetMediumId)
    }

    fn get_registered(new_obj: *mut IMediumRegisteredEvent) -> Result<bool, VboxError> {
        get_function_result_bool!(new_obj, GetRegistered)
    }

    fn get_medium_type(new_obj: *mut IMediumRegisteredEvent) -> Result<DeviceType, VboxError> {
        let medium_type = get_function_result_number!(new_obj, GetMediumType, u32)?;
        Ok(DeviceType::from(medium_type))
    }
}

impl Display for MediumRegisteredEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = format!("{:?}", self);
        write!(f, "{}", format!("{}", s))
    }
}
