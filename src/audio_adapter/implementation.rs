use crate::enums::{AudioCodecType, AudioControllerType, AudioDriverType};
use crate::utility::macros::macros::{
    get_function_result_bool, get_function_result_number, get_function_result_pointer,
    get_function_result_str, get_function_result_unit,
};
use crate::utility::{c_u64_str_to_string, string_to_c_u64_str};
use crate::{AudioAdapter, VboxError};

impl AudioAdapter {
    /// Flag whether the audio adapter is present in the guest system.
    ///
    /// # Returns
    ///
    /// Returns bool on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::VirtualBox;
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let audio_settings = machine.get_audio_settings().unwrap();
    /// let adapter = audio_settings.get_adapter().unwrap();
    /// let enabled = adapter.get_enabled().unwrap();

    pub fn get_enabled(&self) -> Result<bool, VboxError> {
        get_function_result_bool!(self.object, GetEnabled)
    }

    /// Flag whether the audio adapter is present in the guest system.
    ///
    ///
    /// # Arguments
    ///
    /// * `enabled` - bool. Flag whether the audio adapter is present in the guest system.
    ///
    /// # Returns
    ///
    /// Returns () on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{Session, VirtualBox};
    /// use virtualbox_rs::enums::SessionType;
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let mut session = Session::init().unwrap();
    /// machine.lock_machine(&mut  session, SessionType::Shared ).unwrap();
    /// let machine_mut = session.get_machine().unwrap();
    /// let audio_settings = machine_mut.get_audio_settings().unwrap();
    /// let adapter = audio_settings.get_adapter().unwrap();
    /// adapter.set_enabled(true).unwrap();
    /// machine_mut.save_settings().unwrap();

    pub fn set_enabled(&self, enabled: bool) -> Result<(), VboxError> {
        let enabled_bool: i32 = if enabled { 1 } else { 0 };
        get_function_result_unit!(self.object, SetEnabled, enabled_bool)
    }
    /// Flag whether the audio adapter is enabled for audio input.
    ///
    /// # Returns
    ///
    /// Returns bool on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{Session, VirtualBox};
    /// use virtualbox_rs::enums::SessionType;
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let mut session = Session::init().unwrap();
    /// machine.lock_machine(&mut  session, SessionType::Shared ).unwrap();
    /// let machine_mut = session.get_machine().unwrap();
    /// let audio_settings = machine_mut.get_audio_settings().unwrap();
    /// let adapter = audio_settings.get_adapter().unwrap();
    /// let enabled = adapter.get_enabled_in().unwrap();
    pub fn get_enabled_in(&self) -> Result<bool, VboxError> {
        get_function_result_bool!(self.object, GetEnabledIn)
    }

    /// Flag whether the audio adapter is enabled for audio input.
    ///
    ///
    /// # Arguments
    ///
    /// * `enabled` - bool. Flag whether the audio adapter is enabled for audio input.
    ///
    /// # Returns
    ///
    /// Returns () on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{Session, VirtualBox};
    /// use virtualbox_rs::enums::SessionType;
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let mut session = Session::init().unwrap();
    /// machine.lock_machine(&mut  session, SessionType::Shared ).unwrap();
    /// let machine_mut = session.get_machine().unwrap();
    /// let audio_settings = machine_mut.get_audio_settings().unwrap();
    /// let adapter = audio_settings.get_adapter().unwrap();
    /// adapter.set_enabled_in(true).unwrap();
    /// machine_mut.save_settings().unwrap();
    pub fn set_enabled_in(&self, enabled: bool) -> Result<(), VboxError> {
        let enabled_bool: i32 = if enabled { 1 } else { 0 };
        get_function_result_unit!(self.object, SetEnabledIn, enabled_bool)
    }

    /// Flag whether the audio adapter is enabled for audio output.
    ///
    /// # Returns
    ///
    /// Returns bool on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{Session, VirtualBox};
    /// use virtualbox_rs::enums::SessionType;
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let mut session = Session::init().unwrap();
    /// machine.lock_machine(&mut  session, SessionType::Shared ).unwrap();
    /// let machine_mut = session.get_machine().unwrap();
    /// let audio_settings = machine_mut.get_audio_settings().unwrap();
    /// let adapter = audio_settings.get_adapter().unwrap();
    /// let enabled = adapter.get_enabled_out().unwrap();
    pub fn get_enabled_out(&self) -> Result<bool, VboxError> {
        get_function_result_bool!(self.object, GetEnabledOut)
    }

    /// Flag whether the audio adapter is enabled for audio output.
    ///
    ///
    /// # Arguments
    ///
    /// * `enabled` - bool. Flag whether the audio adapter is enabled for audio output.
    ///
    /// # Returns
    ///
    /// Returns () on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{Session, VirtualBox};
    /// use virtualbox_rs::enums::SessionType;
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let mut session = Session::init().unwrap();
    /// machine.lock_machine(&mut  session, SessionType::Shared ).unwrap();
    /// let machine_mut = session.get_machine().unwrap();
    /// let audio_settings = machine_mut.get_audio_settings().unwrap();
    /// let adapter = audio_settings.get_adapter().unwrap();
    /// adapter.set_enabled_out(true).unwrap();
    /// machine_mut.save_settings().unwrap();
    pub fn set_enabled_out(&self, enabled: bool) -> Result<(), VboxError> {
        let enabled_bool: i32 = if enabled { 1 } else { 0 };
        get_function_result_unit!(self.object, SetEnabledOut, enabled_bool)
    }

    /// The emulated audio controller.
    ///
    /// # Returns
    ///
    /// Returns [`AudioControllerType`] on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::VirtualBox;
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let audio_settings = machine.get_audio_settings().unwrap();
    /// let adapter = audio_settings.get_adapter().unwrap();
    /// let audio_controller = adapter.get_audio_controller().unwrap();
    pub fn get_audio_controller(&self) -> Result<AudioControllerType, VboxError> {
        let audio_controller = get_function_result_number!(self.object, GetAudioController, u32)?;
        Ok(AudioControllerType::from(audio_controller))
    }

    /// The emulated audio controller.
    ///
    ///
    /// # Arguments
    ///
    /// * `audio_controller` - [`AudioControllerType`].
    ///
    /// # Returns
    ///
    /// Returns () on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{Session, VirtualBox};
    /// use virtualbox_rs::enums::{AudioControllerType, SessionType};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let mut session = Session::init().unwrap();
    /// machine.lock_machine(&mut  session, SessionType::Shared ).unwrap();
    /// let machine_mut = session.get_machine().unwrap();
    /// let audio_settings = machine_mut.get_audio_settings().unwrap();
    /// let adapter = audio_settings.get_adapter().unwrap();
    /// adapter.set_audio_controller(AudioControllerType::AC97).unwrap();
    /// machine_mut.save_settings().unwrap();

    pub fn set_audio_controller(
        &self,
        audio_controller: AudioControllerType,
    ) -> Result<(), VboxError> {
        let audio_controller: u32 = audio_controller.into();
        get_function_result_unit!(self.object, SetAudioController, audio_controller)
    }

    /// The exact variant of audio codec hardware presented to the guest.
    ///
    /// For HDA and SB16, only one variant is available, but for AC'97, there are several.
    ///
    /// # Returns
    ///
    /// Returns [`AudioCodecType`] on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::VirtualBox;
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let audio_settings = machine.get_audio_settings().unwrap();
    /// let adapter = audio_settings.get_adapter().unwrap();
    /// let audio_codec = adapter.get_audio_codec().unwrap();
    pub fn get_audio_codec(&self) -> Result<AudioCodecType, VboxError> {
        let audio_codec = get_function_result_number!(self.object, GetAudioCodec, u32)?;
        Ok(AudioCodecType::from(audio_codec))
    }

    /// The exact variant of audio codec hardware presented to the guest.
    ///
    /// For HDA and SB16, only one variant is available, but for AC'97, there are several.
    ///
    /// # Arguments
    ///
    /// * `audio_codec` - [`AudioCodecType`].
    ///
    /// # Returns
    ///
    /// Returns () on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{Session, VirtualBox};
    /// use virtualbox_rs::enums::{AudioCodecType, SessionType};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let mut session = Session::init().unwrap();
    /// machine.lock_machine(&mut  session, SessionType::Shared ).unwrap();
    /// let machine_mut = session.get_machine().unwrap();
    /// let audio_settings = machine_mut.get_audio_settings().unwrap();
    /// let adapter = audio_settings.get_adapter().unwrap();
    /// adapter.set_audio_codec(AudioCodecType::AD1980).unwrap();
    /// machine_mut.save_settings().unwrap();
    pub fn set_audio_codec(&self, audio_codec: AudioCodecType) -> Result<(), VboxError> {
        let audio_codec: u32 = audio_codec.into();
        get_function_result_unit!(self.object, SetAudioCodec, audio_codec)
    }

    /// Audio driver the adapter is connected to.
    ///
    /// # Returns
    ///
    /// Returns [`AudioDriverType`] on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::VirtualBox;
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let audio_settings = machine.get_audio_settings().unwrap();
    /// let adapter = audio_settings.get_adapter().unwrap();
    /// let audio_driver = adapter.get_audio_driver().unwrap();
    pub fn get_audio_driver(&self) -> Result<AudioDriverType, VboxError> {
        let audio_driver = get_function_result_number!(self.object, GetAudioDriver, u32)?;
        Ok(AudioDriverType::from(audio_driver))
    }

    /// Audio driver the adapter is connected to.
    ///
    /// This setting can only be changed when the VM is not running.
    ///
    /// # Arguments
    ///
    /// * `audio_driver` - [`AudioDriverType`].
    ///
    /// # Returns
    ///
    /// Returns () on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{Session, VirtualBox};
    /// use virtualbox_rs::enums::{AudioDriverType, SessionType};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let mut session = Session::init().unwrap();
    /// machine.lock_machine(&mut  session, SessionType::Shared ).unwrap();
    /// let machine_mut = session.get_machine().unwrap();
    /// let audio_settings = machine_mut.get_audio_settings().unwrap();
    /// let adapter = audio_settings.get_adapter().unwrap();
    /// adapter.set_audio_driver(AudioDriverType::Pulse).unwrap();
    /// machine_mut.save_settings().unwrap();
    pub fn set_audio_driver(&self, audio_driver: AudioDriverType) -> Result<(), VboxError> {
        let audio_driver: u32 = audio_driver.into();
        get_function_result_unit!(self.object, SetAudioDriver, audio_driver)
    }

    /// Array of names of tunable properties, which can be supported by audio driver.
    ///
    /// # Returns
    ///
    /// Returns Vec<&str> on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::VirtualBox;
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let audio_settings = machine.get_audio_settings().unwrap();
    /// let adapter = audio_settings.get_adapter().unwrap();
    /// let audio_driver = adapter.get_properties_list().unwrap();
    pub fn get_properties_list(&self) -> Result<Vec<&'static str>, VboxError> {
        let mut count = 0;
        let data_ptr = get_function_result_pointer!(
            self.object,
            GetPropertiesList,
            *mut *mut u16,
            &mut count
        )?;
        let data = unsafe { Vec::from_raw_parts(data_ptr, count as usize, count as usize) };
        let data_str = data
            .iter()
            .map(|s| c_u64_str_to_string(s.clone()))
            .collect();
        data_str
    }

    /// Sets an audio specific property string.
    ///
    /// If you pass empty string as a key value, the given key will be deleted.
    ///
    /// # Arguments
    ///
    /// * `key` - &str.
    /// * `value` - &str.
    ///
    /// # Returns
    ///
    /// Returns () on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{Session, VirtualBox};
    /// use virtualbox_rs::enums::SessionType;
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let mut session = Session::init().unwrap();
    /// machine.lock_machine(&mut  session, SessionType::Shared ).unwrap();
    /// let machine_mut = session.get_machine().unwrap();
    /// let audio_settings = machine_mut.get_audio_settings().unwrap();
    /// let adapter = audio_settings.get_adapter().unwrap();
    /// adapter.set_property("key", "value").unwrap();
    pub fn set_property(&self, key: &str, value: &str) -> Result<(), VboxError> {
        let key_ptr = string_to_c_u64_str(key)?;
        let value_ptr = string_to_c_u64_str(value)?;
        get_function_result_unit!(self.object, SetProperty, key_ptr, value_ptr)
    }

    /// Returns an audio specific property string.
    ///
    /// If the requested data key does not exist, this function will succeed and return an empty string.
    ///
    /// # Returns
    ///
    /// Returns Vec<&str> on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::VirtualBox;
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let audio_settings = machine.get_audio_settings().unwrap();
    /// let adapter = audio_settings.get_adapter().unwrap();
    /// let property = adapter.get_property("key").unwrap();
    pub fn get_property(&self, key: &str) -> Result<&'static str, VboxError> {
        let key_ptr = string_to_c_u64_str(key)?;
        get_function_result_str!(self.object, GetProperty, key_ptr)
    }
}
