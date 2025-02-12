use crate::event_detail::utility::cast_event;
use crate::event_detail::DetailEvent;
use crate::utility::macros::macros::get_function_result_number;
use crate::VboxError;
use log::error;
use std::fmt::Display;
use vbox_raw::sys_lib::{
    ICPUExecutionCapChangedEvent, IEvent, ICPUEXECUTIONCAPCHANGEDEVENT_IID_STR,
};

/// Notification when the CPU execution cap changes.
#[derive(Debug)]
pub struct CPUExecutionCapChangedEvent {
    /// The new CPU execution cap value.
    pub execution_cap: u32,
}

impl CPUExecutionCapChangedEvent {
    pub fn new(object: *mut IEvent) -> DetailEvent {
        match Self::create(object) {
            Ok(detail) => DetailEvent::CPUExecutionCapChangedEvent(detail),
            Err(err) => {
                error!("CPUExecutionCapChangedEvent error:{}", err);
                DetailEvent::Null
            }
        }
    }
    fn create(object: *mut IEvent) -> Result<Self, VboxError> {
        let obj1 = cast_event(object, ICPUEXECUTIONCAPCHANGEDEVENT_IID_STR)?;
        let execution_cap = Self::get_execution_cap(obj1)?;
        Ok(Self { execution_cap })
    }

    fn get_execution_cap(new_obj: *mut ICPUExecutionCapChangedEvent) -> Result<u32, VboxError> {
        get_function_result_number!(new_obj, GetExecutionCap, u32)
    }
}

impl Display for CPUExecutionCapChangedEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = format!("{:?}", self);
        write!(f, "{}", format!("{}", s))
    }
}
