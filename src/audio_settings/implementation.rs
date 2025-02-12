use crate::utility::macros::macros::get_function_result_pointer;
use crate::{AudioAdapter, AudioSettings, VboxError};
use vbox_raw::sys_lib::IAudioAdapter;

impl AudioSettings {
    /// Associated audio adapter, always present.
    ///
    /// # Returns
    ///
    /// Returns [`AudioAdapter`] on success, or a [`VboxError`] on failure.
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
    ///
    pub fn get_adapter(&self) -> Result<AudioAdapter, VboxError> {
        let adapter_ptr =
            get_function_result_pointer!(self.object, GetAdapter, *mut IAudioAdapter)?;
        Ok(AudioAdapter::new(adapter_ptr))
    }
}
