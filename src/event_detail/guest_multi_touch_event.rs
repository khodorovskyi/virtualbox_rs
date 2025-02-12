use crate::event_detail::utility::cast_event;
use crate::event_detail::DetailEvent;
#[cfg(not(is_v_6_1))]
use crate::utility::macros::macros::get_function_result_bool;
use crate::utility::macros::macros::{get_function_result_number, get_function_result_pointer};
use crate::VboxError;
use log::error;
use std::fmt::Display;
use vbox_raw::sys_lib::{IEvent, IGuestMultiTouchEvent, IGUESTMULTITOUCHEVENT_IID_STR};

/// Notification when guest touch screen event happens.
#[derive(Debug)]
pub struct GuestMultiTouchEvent {
    /// Number of contacts in the event.
    pub contact_count: i32,
    /// X positions.
    pub x_positions: Vec<i16>,
    /// Y positions.
    pub y_positions: Vec<i16>,
    /// Contact identifiers.
    pub contact_ids: Vec<u16>,
    /// Contact state.
    ///
    /// Bit 0: in contact. Bit 1: in range.
    pub contact_flags: Vec<u16>,
    /// Distinguishes between touchscreen and touchpad events.
    pub is_touch_screen: bool,
    /// Timestamp of the event in milliseconds.
    pub scan_time: u32,
}

impl GuestMultiTouchEvent {
    pub fn new(object: *mut IEvent) -> DetailEvent {
        match Self::create(object) {
            Ok(detail) => DetailEvent::GuestMultiTouchEvent(detail),
            Err(err) => {
                error!("GuestMultiTouchEvent error:{}", err);
                DetailEvent::Null
            }
        }
    }
    fn create(object: *mut IEvent) -> Result<Self, VboxError> {
        let obj1 = cast_event(object, IGUESTMULTITOUCHEVENT_IID_STR)?;
        let contact_count = Self::get_contact_count(obj1)?;
        let x_positions = Self::get_x_positions(obj1)?;
        let y_positions = Self::get_y_positions(obj1)?;
        let contact_ids = Self::get_contact_ids(obj1)?;
        let contact_flags = Self::get_contact_flags(obj1)?;
        let is_touch_screen = Self::get_is_touch_screen(obj1)?;
        let scan_time = Self::get_scan_time(obj1)?;
        Ok(Self {
            contact_count,
            x_positions,
            y_positions,
            contact_ids,
            contact_flags,
            is_touch_screen,
            scan_time,
        })
    }

    fn get_contact_count(object: *mut IGuestMultiTouchEvent) -> Result<i32, VboxError> {
        get_function_result_number!(object, GetContactCount, i32)
    }
    fn get_x_positions(object: *mut IGuestMultiTouchEvent) -> Result<Vec<i16>, VboxError> {
        let mut count = 0;
        let positions_ptr =
            get_function_result_pointer!(object, GetXPositions, *mut i16, &mut count)?;
        let positions =
            unsafe { Vec::from_raw_parts(positions_ptr, count as usize, count as usize) };
        Ok(positions)
    }
    fn get_y_positions(object: *mut IGuestMultiTouchEvent) -> Result<Vec<i16>, VboxError> {
        let mut count = 0;
        let positions_ptr =
            get_function_result_pointer!(object, GetYPositions, *mut i16, &mut count)?;
        let positions =
            unsafe { Vec::from_raw_parts(positions_ptr, count as usize, count as usize) };
        Ok(positions)
    }
    fn get_contact_ids(object: *mut IGuestMultiTouchEvent) -> Result<Vec<u16>, VboxError> {
        let mut count = 0;
        let ids_ptr = get_function_result_pointer!(object, GetContactIds, *mut u16, &mut count)?;
        let ids = unsafe { Vec::from_raw_parts(ids_ptr, count as usize, count as usize) };
        Ok(ids)
    }
    fn get_contact_flags(object: *mut IGuestMultiTouchEvent) -> Result<Vec<u16>, VboxError> {
        let mut count = 0;
        let positions_ptr =
            get_function_result_pointer!(object, GetContactFlags, *mut u16, &mut count)?;
        let positions =
            unsafe { Vec::from_raw_parts(positions_ptr, count as usize, count as usize) };
        Ok(positions)
    }
    #[cfg(not(is_v_6_1))]
    fn get_is_touch_screen(object: *mut IGuestMultiTouchEvent) -> Result<bool, VboxError> {
        get_function_result_bool!(object, GetIsTouchScreen)
    }
    #[cfg(is_v_6_1)]
    fn get_is_touch_screen(_object: *mut IGuestMultiTouchEvent) -> Result<bool, VboxError> {
        Ok(false)
    }
    fn get_scan_time(object: *mut IGuestMultiTouchEvent) -> Result<u32, VboxError> {
        get_function_result_number!(object, GetScanTime, u32)
    }
}

impl Display for GuestMultiTouchEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = format!("{:?}", self);
        write!(f, "{}", format!("{}", s))
    }
}
