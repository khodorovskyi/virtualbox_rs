use crate::event_detail::utility::cast_event;
use crate::event_detail::DetailEvent;
use crate::utility::macros::macros::{get_function_result_bool, get_function_result_number};
use crate::VboxError;
use log::error;
use std::fmt::Display;
use vbox_raw::sys_lib::{ICursorPositionChangedEvent, IEvent, ICURSORPOSITIONCHANGEDEVENT_IID_STR};

/// The guest reports cursor position data.
#[derive(Debug)]
pub struct CursorPositionChangedEvent {
    /// Event contains valid data.
    pub has_data: bool,
    /// Reported X position.
    pub x: u32,
    /// Reported Y position.
    pub y: u32,
}

impl CursorPositionChangedEvent {
    pub fn new(object: *mut IEvent) -> DetailEvent {
        match Self::create(object) {
            Ok(detail) => DetailEvent::CursorPositionChangedEvent(detail),
            Err(err) => {
                error!("CursorPositionChangedEvent error:{}", err);
                DetailEvent::Null
            }
        }
    }
    fn create(object: *mut IEvent) -> Result<Self, VboxError> {
        let obj1 = cast_event(object, ICURSORPOSITIONCHANGEDEVENT_IID_STR)?;
        let has_data = Self::get_has_data(obj1)?;
        let x = Self::get_x(obj1)?;
        let y = Self::get_y(obj1)?;
        Ok(Self { has_data, x, y })
    }

    fn get_has_data(object: *mut ICursorPositionChangedEvent) -> Result<bool, VboxError> {
        get_function_result_bool!(object, GetHasData)
    }
    fn get_x(object: *mut ICursorPositionChangedEvent) -> Result<u32, VboxError> {
        get_function_result_number!(object, GetX, u32)
    }
    fn get_y(object: *mut ICursorPositionChangedEvent) -> Result<u32, VboxError> {
        get_function_result_number!(object, GetY, u32)
    }
}

impl Display for CursorPositionChangedEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = format!("{:?}", self);
        write!(f, "{}", format!("{}", s))
    }
}
