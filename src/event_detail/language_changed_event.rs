#[cfg(not(is_v_6_1))]
use crate::event_detail::utility::cast_event;
use crate::event_detail::DetailEvent;
#[cfg(not(is_v_6_1))]
use crate::utility::macros::macros::get_function_result_str;
#[cfg(not(is_v_6_1))]
use crate::VboxError;
#[cfg(not(is_v_6_1))]
use log::error;
use std::fmt::Display;
use vbox_raw::sys_lib::IEvent;
#[cfg(not(is_v_6_1))]
use vbox_raw::sys_lib::{ILanguageChangedEvent, ILANGUAGECHANGEDEVENT_IID_STR};

#[derive(Debug)]
pub struct LanguageChangedEvent {
    /// GUID of the progress this event relates to.
    pub language_id: &'static str,
}

#[cfg(not(is_v_6_1))]
impl LanguageChangedEvent {
    pub fn new(object: *mut IEvent) -> DetailEvent {
        match Self::create(object) {
            Ok(detail) => DetailEvent::LanguageChangedEvent(detail),
            Err(err) => {
                error!("LanguageChangedEvent error:{}", err);
                DetailEvent::Null
            }
        }
    }
    fn create(object: *mut IEvent) -> Result<Self, VboxError> {
        let obj1 = cast_event(object, ILANGUAGECHANGEDEVENT_IID_STR)?;
        let language_id = Self::get_language_id(obj1)?;
        Ok(Self { language_id })
    }

    fn get_language_id(new_obj: *mut ILanguageChangedEvent) -> Result<&'static str, VboxError> {
        get_function_result_str!(new_obj, GetLanguageId)
    }
}
#[cfg(is_v_6_1)]
impl LanguageChangedEvent {
    pub fn new(_object: *mut IEvent) -> DetailEvent {
        DetailEvent::Null
    }
}
impl Display for LanguageChangedEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = format!("{:?}", self);
        write!(f, "{}", format!("{}", s))
    }
}
