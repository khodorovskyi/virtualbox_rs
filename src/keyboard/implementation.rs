use crate::utility::macros::macros::{
    get_function_result_number, get_function_result_pointer, get_function_result_unit,
};
use crate::{EventSource, Keyboard, VboxError};
use vbox_raw::sys_lib::{IEventSource, PRUint32};

impl Keyboard {
    /// Sends a scancode to the keyboard.
    ///
    /// # Returns
    ///
    /// Returns a Ok(()) success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{Session, VirtualBox};
    /// use virtualbox_rs::enums::SessionType;
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let mut session = Session::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    ///
    /// let console = session.get_console().unwrap();
    ///
    /// let keyboard = console.get_keyboard().unwrap();
    /// keyboard.put_scancode(21).unwrap();
    pub fn put_scancode(&self, scancode: i32) -> Result<(), VboxError> {
        get_function_result_unit!(self.object, PutScancode, scancode)
    }

    /// Sends an array of scancodes to the keyboard.
    ///
    /// # Returns
    ///
    /// Returns u32 success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{Session, VirtualBox};
    /// use virtualbox_rs::enums::SessionType;
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let mut session = Session::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    ///
    /// let console = session.get_console().unwrap();
    ///
    /// let keyboard = console.get_keyboard().unwrap();
    /// let codes_stored = keyboard.put_scancodes([1,2,3].to_vec()).unwrap();
    pub fn put_scancodes(&self, mut scancodes: Vec<i32>) -> Result<u32, VboxError> {
        let scancodes_ptr = scancodes.as_mut_ptr();
        let scancodes_size = scancodes.len() as PRUint32;
        let codes_stored = get_function_result_number!(
            self.object,
            PutScancodes,
            u32,
            scancodes_size,
            scancodes_ptr
        )?;
        Ok(codes_stored)
    }

    /// Sends an array of scancodes to the keyboard.
    ///
    /// # Returns
    ///
    /// Returns a Ok(()) success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{Session, VirtualBox};
    /// use virtualbox_rs::enums::SessionType;
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let mut session = Session::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    ///
    /// let console = session.get_console().unwrap();
    ///
    /// let keyboard = console.get_keyboard().unwrap();
    /// keyboard.put_scancode(21).unwrap();
    pub fn put_cad(&self) -> Result<(), VboxError> {
        get_function_result_unit!(self.object, PutCAD)
    }

    /// Causes the virtual keyboard to release any keys which are currently pressed.
    ///
    /// # Returns
    ///
    /// Returns Ok(()) success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{Session, VirtualBox};
    /// use virtualbox_rs::enums::SessionType;
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let mut session = Session::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    ///
    /// let console = session.get_console().unwrap();
    ///
    /// let keyboard = console.get_keyboard().unwrap();
    /// keyboard.release_keys().unwrap();
    pub fn release_keys(&self) -> Result<(), VboxError> {
        get_function_result_unit!(self.object, ReleaseKeys)
    }

    /// Sends a USB HID usage code and page to the keyboard.
    ///
    /// # Returns
    ///
    /// Returns Ok(()) success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{Session, VirtualBox};
    /// use virtualbox_rs::enums::SessionType;
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let mut session = Session::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    ///
    /// let console = session.get_console().unwrap();
    ///
    /// let keyboard = console.get_keyboard().unwrap();
    /// keyboard.put_usage_code(1, 2, true).unwrap();
    pub fn put_usage_code(
        &self,
        usage_code: i32,
        usage_page: i32,
        key_release: bool,
    ) -> Result<(), VboxError> {
        let key_release = if key_release { 1 } else { 0 };
        get_function_result_unit!(
            self.object,
            PutUsageCode,
            usage_code,
            usage_page,
            key_release
        )
    }

    /// Event source for keyboard events.
    ///
    /// # Returns
    ///
    /// Returns a new instance of the [`EventSource`] class on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{Session, VirtualBox};
    /// use virtualbox_rs::enums::SessionType;
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let mut session = Session::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    ///
    /// let console = session.get_console().unwrap();
    /// let keyboard = console.get_keyboard().unwrap();
    ///
    /// let event_source = keyboard.get_event_source().unwrap();
    pub fn get_event_source(&self) -> Result<EventSource, VboxError> {
        let event_source =
            get_function_result_pointer!(self.object, GetEventSource, *mut IEventSource)?;
        Ok(EventSource::new(event_source))
    }
}
