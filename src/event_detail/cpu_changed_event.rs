use crate::event_detail::utility::cast_event;
use crate::event_detail::DetailEvent;
use crate::utility::macros::macros::{get_function_result_bool, get_function_result_number};
use crate::VboxError;
use log::error;
use std::fmt::Display;
use vbox_raw::sys_lib::{ICPUChangedEvent, IEvent, ICPUCHANGEDEVENT_IID_STR};

/// Notification when a CPU changes.
#[derive(Debug)]
pub struct CPUChangedEvent {
    /// The CPU which changed.
    pub cpu: u32,
    /// Flag whether the CPU was added or removed.
    pub add: bool,
}

impl CPUChangedEvent {
    pub fn new(object: *mut IEvent) -> DetailEvent {
        match Self::create(object) {
            Ok(detail) => DetailEvent::CPUChangedEvent(detail),
            Err(err) => {
                error!("CPUChangedEvent error:{}", err);
                DetailEvent::Null
            }
        }
    }
    fn create(object: *mut IEvent) -> Result<Self, VboxError> {
        let obj1 = cast_event(object, ICPUCHANGEDEVENT_IID_STR)?;
        let cpu = Self::get_cpu(obj1)?;
        let add = Self::get_add(obj1)?;
        Ok(Self { cpu, add })
    }

    fn get_cpu(new_obj: *mut ICPUChangedEvent) -> Result<u32, VboxError> {
        get_function_result_number!(new_obj, GetCPU, u32)
    }
    fn get_add(new_obj: *mut ICPUChangedEvent) -> Result<bool, VboxError> {
        get_function_result_bool!(new_obj, GetAdd)
    }
}

impl Display for CPUChangedEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = format!("{:?}", self);
        write!(f, "{}", format!("{}", s))
    }
}
