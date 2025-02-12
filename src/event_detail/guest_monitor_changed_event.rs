use crate::enums::GuestMonitorChangedEventType;
use crate::event_detail::utility::cast_event;
use crate::event_detail::DetailEvent;
use crate::utility::macros::macros::get_function_result_number;
use crate::VboxError;
use log::error;
use std::fmt::Display;
use vbox_raw::sys_lib::{IEvent, IGuestMonitorChangedEvent, IGUESTMONITORCHANGEDEVENT_IID_STR};

/// Notification when the guest enables one of its monitors.
#[derive(Debug)]
pub struct GuestMonitorChangedEvent {
    /// What was changed for this guest monitor.
    pub change_type: GuestMonitorChangedEventType,
    /// The monitor which was changed.
    pub screen_id: u32,
    /// Physical X origin relative to the primary screen.
    ///
    /// Valid for Enabled and NewOrigin
    pub origin_x: u32,
    /// Physical Y origin relative to the primary screen.
    ///
    /// Valid for Enabled and NewOrigin
    pub origin_y: u32,
    /// Width of the screen.
    ///
    /// Valid for Enabled.
    pub width: u32,
    ///Height of the screen.
    ///
    /// Valid for Enabled.
    pub height: u32,
}

impl GuestMonitorChangedEvent {
    pub fn new(object: *mut IEvent) -> DetailEvent {
        match Self::create(object) {
            Ok(detail) => DetailEvent::GuestMonitorChangedEvent(detail),
            Err(err) => {
                error!("GuestMonitorChangedEvent error:{}", err);
                DetailEvent::Null
            }
        }
    }
    fn create(object: *mut IEvent) -> Result<Self, VboxError> {
        let obj1 = cast_event(object, IGUESTMONITORCHANGEDEVENT_IID_STR)?;
        let change_type = Self::get_change_type(obj1)?;
        let screen_id = Self::get_screen_id(obj1)?;
        let origin_x = Self::get_origin_x(obj1)?;
        let origin_y = Self::get_origin_y(obj1)?;
        let width = Self::get_width(obj1)?;
        let height = Self::get_height(obj1)?;
        Ok(Self {
            change_type,
            screen_id,
            origin_x,
            origin_y,
            width,
            height,
        })
    }

    fn get_change_type(
        object: *mut IGuestMonitorChangedEvent,
    ) -> Result<GuestMonitorChangedEventType, VboxError> {
        let change_type = get_function_result_number!(object, GetChangeType, u32)?;
        Ok(GuestMonitorChangedEventType::from(change_type))
    }
    fn get_screen_id(object: *mut IGuestMonitorChangedEvent) -> Result<u32, VboxError> {
        get_function_result_number!(object, GetScreenId, u32)
    }
    fn get_origin_x(object: *mut IGuestMonitorChangedEvent) -> Result<u32, VboxError> {
        get_function_result_number!(object, GetOriginX, u32)
    }
    fn get_origin_y(object: *mut IGuestMonitorChangedEvent) -> Result<u32, VboxError> {
        get_function_result_number!(object, GetOriginY, u32)
    }
    fn get_width(object: *mut IGuestMonitorChangedEvent) -> Result<u32, VboxError> {
        get_function_result_number!(object, GetWidth, u32)
    }
    fn get_height(object: *mut IGuestMonitorChangedEvent) -> Result<u32, VboxError> {
        get_function_result_number!(object, GetHeight, u32)
    }
}

impl Display for GuestMonitorChangedEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = format!("{:?}", self);
        write!(f, "{}", format!("{}", s))
    }
}
