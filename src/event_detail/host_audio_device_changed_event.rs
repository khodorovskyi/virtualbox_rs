use crate::enums::AudioDeviceState;
#[cfg(not(is_v_6_1))]
use crate::event_detail::utility::cast_event;
use crate::event_detail::DetailEvent;
#[cfg(not(is_v_6_1))]
use crate::utility::macros::macros::{
    get_function_result_bool, get_function_result_number, get_function_result_pointer,
};
#[cfg(not(is_v_6_1))]
use crate::VboxError;
use crate::{HostAudioDevice, VirtualBoxErrorInfo};
#[cfg(not(is_v_6_1))]
use log::error;
use std::fmt::Display;
use vbox_raw::sys_lib::IEvent;
#[cfg(not(is_v_6_1))]
use vbox_raw::sys_lib::{
    IHostAudioDevice, IHostAudioDeviceChangedEvent, IVirtualBoxErrorInfo,
    IHOSTAUDIODEVICECHANGEDEVENT_IID_STR,
};

/// Notification when a host audio device state has changed.
#[derive(Debug)]
pub struct HostAudioDeviceChangedEvent {
    /// Host audio device that has changed.
    pub device: HostAudioDevice,
    /// true if the host device is a newly detected device, false if not.
    pub new: bool,
    /// New audio device state.
    pub state: AudioDeviceState,
    /// null on success or an error message object on failure.
    pub error: VirtualBoxErrorInfo,
}

#[cfg(not(is_v_6_1))]
impl HostAudioDeviceChangedEvent {
    pub fn new(object: *mut IEvent) -> DetailEvent {
        match Self::create(object) {
            Ok(detail) => DetailEvent::HostAudioDeviceChangedEvent(detail),
            Err(err) => {
                error!("HostAudioDeviceChangedEvent error:{}", err);
                DetailEvent::Null
            }
        }
    }
    fn create(object: *mut IEvent) -> Result<Self, VboxError> {
        let obj1 = cast_event(object, IHOSTAUDIODEVICECHANGEDEVENT_IID_STR)?;
        let device = Self::get_device(obj1)?;
        let new = Self::get_new(obj1)?;
        let state = Self::get_state(obj1)?;
        let error = Self::get_error(obj1)?;
        Ok(Self {
            device,
            new,
            state,
            error,
        })
    }

    fn get_device(
        new_obj: *mut IHostAudioDeviceChangedEvent,
    ) -> Result<HostAudioDevice, VboxError> {
        let device = get_function_result_pointer!(new_obj, GetDevice, *mut IHostAudioDevice)?;
        Ok(HostAudioDevice::new(device))
    }
    fn get_new(new_obj: *mut IHostAudioDeviceChangedEvent) -> Result<bool, VboxError> {
        get_function_result_bool!(new_obj, GetNew)
    }
    fn get_state(
        new_obj: *mut IHostAudioDeviceChangedEvent,
    ) -> Result<AudioDeviceState, VboxError> {
        let state = get_function_result_number!(new_obj, GetState, u32)?;
        Ok(AudioDeviceState::from(state))
    }
    fn get_error(
        new_obj: *mut IHostAudioDeviceChangedEvent,
    ) -> Result<VirtualBoxErrorInfo, VboxError> {
        let error = get_function_result_pointer!(new_obj, GetError, *mut IVirtualBoxErrorInfo)?;
        Ok(VirtualBoxErrorInfo::new(error))
    }
}
#[cfg(is_v_6_1)]
impl HostAudioDeviceChangedEvent {
    pub fn new(_object: *mut IEvent) -> DetailEvent {
        DetailEvent::Null
    }
}
impl Display for HostAudioDeviceChangedEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = format!("{:?}", self);
        write!(f, "{}", format!("{}", s))
    }
}
