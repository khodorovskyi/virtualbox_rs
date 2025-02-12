use crate::event_detail::utility::cast_event;
use crate::event_detail::DetailEvent;
use crate::utility::macros::macros::get_function_result_pointer;
use crate::{AudioAdapter, VboxError};
use log::error;
use std::fmt::Display;
use vbox_raw::sys_lib::{
    IAudioAdapter, IAudioAdapterChangedEvent, IEvent, IAUDIOADAPTERCHANGEDEVENT_IID_STR,
};

/// Notification when a property of the audio adapter changes
#[derive(Debug)]
pub struct AudioAdapterChangedEvent {
    /// Audio adapter that is subject to change.
    pub audio_adapter: AudioAdapter,
}

impl AudioAdapterChangedEvent {
    pub fn new(object: *mut IEvent) -> DetailEvent {
        match Self::create(object) {
            Ok(detail) => DetailEvent::AudioAdapterChangedEvent(detail),
            Err(err) => {
                error!("AudioAdapterChangedEvent error:{}", err);
                DetailEvent::Null
            }
        }
    }
    fn create(object: *mut IEvent) -> Result<Self, VboxError> {
        let obj1 = cast_event(object, IAUDIOADAPTERCHANGEDEVENT_IID_STR)?;
        let audio_adapter = Self::get_audio_adapter(obj1)?;
        Ok(Self { audio_adapter })
    }

    fn get_audio_adapter(
        new_obj: *mut IAudioAdapterChangedEvent,
    ) -> Result<AudioAdapter, VboxError> {
        let audio_adapter_ptr =
            get_function_result_pointer!(new_obj, GetAudioAdapter, *mut IAudioAdapter)?;
        Ok(AudioAdapter::new(audio_adapter_ptr))
    }
}

impl Display for AudioAdapterChangedEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = format!("{:?}", self);
        write!(f, "{}", format!("{}", s))
    }
}
