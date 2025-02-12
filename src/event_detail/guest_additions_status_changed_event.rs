use crate::enums::{AdditionsFacilityStatus, AdditionsFacilityType, AdditionsRunLevelType};
use crate::event_detail::utility::cast_event;
use crate::event_detail::DetailEvent;
use crate::utility::macros::macros::get_function_result_number;
use crate::VboxError;
use log::error;
use std::fmt::Display;
use vbox_raw::sys_lib::{
    IEvent, IGuestAdditionsStatusChangedEvent, IGUESTADDITIONSSTATUSCHANGEDEVENT_IID_STR,
};

/// The guest addition status changed
#[derive(Debug)]
pub struct GuestAdditionsStatusChangedEvent {
    /// Facility this event relates to. [`AdditionsFacilityType`]
    pub facility: AdditionsFacilityType,
    /// The new facility status. [`AdditionsFacilityStatus`]
    pub status: AdditionsFacilityStatus,
    /// The new run level. [`AdditionsRunLevelType`]
    pub run_level: AdditionsRunLevelType,
    /// The millisecond timestamp associated with the event.
    pub timestamp: i64,
}

impl GuestAdditionsStatusChangedEvent {
    pub fn new(object: *mut IEvent) -> DetailEvent {
        match Self::create(object) {
            Ok(detail) => DetailEvent::GuestAdditionsStatusChangedEvent(detail),
            Err(err) => {
                error!("GuestAdditionsStatusChangedEvent error:{}", err);
                DetailEvent::Null
            }
        }
    }
    fn create(object: *mut IEvent) -> Result<Self, VboxError> {
        let obj1 = cast_event(object, IGUESTADDITIONSSTATUSCHANGEDEVENT_IID_STR)?;
        let facility = Self::get_facility(obj1)?;
        let status = Self::get_status(obj1)?;
        let run_level = Self::get_run_level(obj1)?;
        let timestamp = Self::get_timestamp(obj1)?;
        Ok(Self {
            facility,
            status,
            run_level,
            timestamp,
        })
    }

    fn get_facility(
        object: *mut IGuestAdditionsStatusChangedEvent,
    ) -> Result<AdditionsFacilityType, VboxError> {
        let facility = get_function_result_number!(object, GetFacility, u32)?;
        Ok(AdditionsFacilityType::from(facility))
    }

    fn get_status(
        object: *mut IGuestAdditionsStatusChangedEvent,
    ) -> Result<AdditionsFacilityStatus, VboxError> {
        let status = get_function_result_number!(object, GetStatus, u32)?;
        Ok(AdditionsFacilityStatus::from(status))
    }
    fn get_run_level(
        object: *mut IGuestAdditionsStatusChangedEvent,
    ) -> Result<AdditionsRunLevelType, VboxError> {
        let run_level = get_function_result_number!(object, GetRunLevel, u32)?;
        Ok(AdditionsRunLevelType::from(run_level))
    }
    fn get_timestamp(object: *mut IGuestAdditionsStatusChangedEvent) -> Result<i64, VboxError> {
        get_function_result_number!(object, GetTimestamp, i64)
    }
}

impl Display for GuestAdditionsStatusChangedEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = format!("{:?}", self);
        write!(f, "{}", format!("{}", s))
    }
}
