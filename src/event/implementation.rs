use crate::enums::VBoxEventType;
use crate::event::Event;
use crate::utility::macros::macros::get_function_result_number;
use crate::VboxError;

impl Event {
    /// Event type.
    ///
    /// # Returns
    ///
    /// Returns [`VBoxEventType`] on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{Session, VirtualBox};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let event_source = vbox.get_event_source().unwrap();
    /// let listener = event_source.create_listener().unwrap();
    /// let event = event_source.get_event(&listener, 20).unwrap();
    /// let event_type = event.get_type().unwrap();
    pub fn get_type(&self) -> Result<VBoxEventType, VboxError> {
        let event_type = get_function_result_number!(self.object, GetType, u32)?;
        Ok(VBoxEventType::from(event_type))
    }
}
