use crate::event_detail::utility::cast_event;
use crate::event_detail::DetailEvent;
use crate::utility::macros::macros::get_function_result_pointer;
use crate::{ParallelPort, VboxError};
use log::error;
use std::fmt::Display;
use vbox_raw::sys_lib::{
    IEvent, IParallelPort, IParallelPortChangedEvent, IPARALLELPORTCHANGEDEVENT_IID_STR,
};

/// Notification when a property of one of the virtual parallel ports changes.
#[derive(Debug)]
pub struct ParallelPortChangedEvent {
    /// Parallel port that is subject to change.
    pub parallel_port: ParallelPort,
}

impl ParallelPortChangedEvent {
    pub fn new(object: *mut IEvent) -> DetailEvent {
        match Self::create(object) {
            Ok(detail) => DetailEvent::ParallelPortChangedEvent(detail),
            Err(err) => {
                error!("ParallelPortChangedEvent error:{}", err);
                DetailEvent::Null
            }
        }
    }
    fn create(object: *mut IEvent) -> Result<Self, VboxError> {
        let obj1 = cast_event(object, IPARALLELPORTCHANGEDEVENT_IID_STR)?;
        let parallel_port = Self::get_parallel_port(obj1)?;
        Ok(Self { parallel_port })
    }

    fn get_parallel_port(
        new_obj: *mut IParallelPortChangedEvent,
    ) -> Result<ParallelPort, VboxError> {
        let parallel_port =
            get_function_result_pointer!(new_obj, GetParallelPort, *mut IParallelPort)?;
        Ok(ParallelPort::new(parallel_port))
    }
}

impl Display for ParallelPortChangedEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = format!("{:?}", self);
        write!(f, "{}", format!("{}", s))
    }
}
