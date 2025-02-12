use crate::event_detail::utility::cast_event;
use crate::event_detail::DetailEvent;
use crate::utility::macros::macros::{get_function_result_bool, get_function_result_pointer};
use crate::{EventListener, VboxError};
use log::error;
use std::fmt::Display;
use vbox_raw::sys_lib::{
    IEvent, IEventListener, IEventSourceChangedEvent, IEVENTSOURCECHANGEDEVENT_IID_STR,
};

/// Notification when an event source state changes (listener added or removed).
#[derive(Debug)]
pub struct EventSourceChangedEvent {
    /// Event listener which has changed.
    pub listener: EventListener,
    /// Flag whether the CPU was added or removed.
    pub add: bool,
}

impl EventSourceChangedEvent {
    pub fn new(object: *mut IEvent) -> DetailEvent {
        match Self::create(object) {
            Ok(detail) => DetailEvent::EventSourceChangedEvent(detail),
            Err(err) => {
                error!("EventSourceChangedEvent error:{}", err);
                DetailEvent::Null
            }
        }
    }
    fn create(object: *mut IEvent) -> Result<Self, VboxError> {
        let obj1 = cast_event(object, IEVENTSOURCECHANGEDEVENT_IID_STR)?;
        let listener = Self::get_listener(obj1)?;
        let add = Self::get_add(obj1)?;
        Ok(Self { listener, add })
    }

    fn get_listener(new_obj: *mut IEventSourceChangedEvent) -> Result<EventListener, VboxError> {
        let listener = get_function_result_pointer!(new_obj, GetListener, *mut IEventListener)?;
        Ok(EventListener::new(listener))
    }
    fn get_add(new_obj: *mut IEventSourceChangedEvent) -> Result<bool, VboxError> {
        get_function_result_bool!(new_obj, GetAdd)
    }
}

impl Display for EventSourceChangedEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = format!("{:?}", self);
        write!(f, "{}", format!("{}", s))
    }
}
