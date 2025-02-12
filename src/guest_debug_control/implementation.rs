use crate::enums::{GuestDebugIoProvider, GuestDebugProvider};
use crate::utility::macros::macros::{
    get_function_result_number, get_function_result_str, get_function_result_unit,
};
use crate::utility::string_to_c_u64_str;
use crate::{GuestDebugControl, VboxError};

impl GuestDebugControl {
    /// The currently active debug provider.
    ///
    /// # Returns
    ///
    /// Returns a [`GuestDebugProvider`] success, or a [`VboxError`] on failure.
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
    /// let guest_debug_control = machine.get_guest_debug_control().unwrap();
    /// let debug_provider = guest_debug_control.get_debug_provider().unwrap();
    pub fn get_debug_provider(&self) -> Result<GuestDebugProvider, VboxError> {
        let debug_provider = get_function_result_number!(self.object, GetDebugProvider, u32)?;
        Ok(GuestDebugProvider::from(debug_provider))
    }

    /// The currently active debug provider.
    ///
    /// # Arguments
    ///
    /// * `debug_provider` - [`GuestDebugProvider`].
    ///
    /// # Returns
    ///
    /// Returns a () success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{Session, VirtualBox};
    /// use virtualbox_rs::enums::{GuestDebugProvider, SessionType};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let mut session = Session::init().unwrap();
    /// machine.lock_machine(&mut  session, SessionType::Shared ).unwrap();
    /// let machine_mut = session.get_machine().unwrap();
    /// let guest_debug_control = machine_mut.get_guest_debug_control().unwrap();
    /// guest_debug_control.set_debug_provider(GuestDebugProvider::Native).unwrap();
    pub fn set_debug_provider(&self, debug_provider: GuestDebugProvider) -> Result<(), VboxError> {
        let debug_provider: u32 = debug_provider.into();
        get_function_result_unit!(self.object, SetDebugProvider, debug_provider)
    }

    /// The I/O backend for the selected debug provider.
    ///
    /// # Returns
    ///
    /// Returns a [`GuestDebugIoProvider`] success, or a [`VboxError`] on failure.
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
    /// let guest_debug_control = machine.get_guest_debug_control().unwrap();
    /// let debug_io_provider = guest_debug_control.get_debug_io_provider().unwrap();
    pub fn get_debug_io_provider(&self) -> Result<GuestDebugIoProvider, VboxError> {
        let debug_io_provider = get_function_result_number!(self.object, GetDebugIoProvider, u32)?;
        Ok(GuestDebugIoProvider::from(debug_io_provider))
    }

    /// The I/O backend for the selected debug provider.
    ///
    /// # Arguments
    ///
    /// * `debug_io_provider` - [`GuestDebugIoProvider`].
    ///
    /// # Returns
    ///
    /// Returns a () success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{Session, VirtualBox};
    /// use virtualbox_rs::enums::{GuestDebugIoProvider, SessionType};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let mut session = Session::init().unwrap();
    /// machine.lock_machine(&mut  session, SessionType::Shared ).unwrap();
    /// let machine_mut = session.get_machine().unwrap();
    /// let guest_debug_control = machine_mut.get_guest_debug_control().unwrap();
    /// guest_debug_control.set_debug_io_provider(GuestDebugIoProvider::UDP).unwrap();
    /// machine_mut.save_settings().unwrap();
    pub fn set_debug_io_provider(
        &self,
        debug_io_provider: GuestDebugIoProvider,
    ) -> Result<(), VboxError> {
        let debug_io_provider: u32 = debug_io_provider.into();
        get_function_result_unit!(self.object, SetDebugIoProvider, debug_io_provider)
    }

    /// The address to connect to or listen on, depending on the type.
    ///
    /// # Returns
    ///
    /// Returns &str success, or a [`VboxError`] on failure.
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
    /// let guest_debug_control = machine.get_guest_debug_control().unwrap();
    /// let debug_address = guest_debug_control.get_debug_address().unwrap();
    pub fn get_debug_address(&self) -> Result<&'static str, VboxError> {
        get_function_result_str!(self.object, GetDebugAddress)
    }

    /// The address to connect to or listen on, depending on the type.
    ///
    /// # Arguments
    ///
    /// * `debug_address` - &str.
    ///
    /// # Returns
    ///
    /// Returns a () success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{ Session, VirtualBox};
    /// use virtualbox_rs::enums::SessionType;
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let mut session = Session::init().unwrap();
    /// machine.lock_machine(&mut  session, SessionType::Shared ).unwrap();
    /// let machine_mut = session.get_machine().unwrap();
    /// let guest_debug_control = machine_mut.get_guest_debug_control().unwrap();
    /// guest_debug_control.set_debug_address("127.0.0.1").unwrap();
    /// machine_mut.save_settings().unwrap();
    pub fn set_debug_address(&self, debug_address: &str) -> Result<(), VboxError> {
        let debug_address_ptr = string_to_c_u64_str(debug_address)?;
        get_function_result_unit!(self.object, SetDebugAddress, debug_address_ptr)
    }

    /// The port to listen on or connect to, depending on the selected I/O provider.
    ///
    /// # Returns
    ///
    /// Returns u32 success, or a [`VboxError`] on failure.
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
    /// let guest_debug_control = machine.get_guest_debug_control().unwrap();
    /// let debug_port = guest_debug_control.get_debug_port().unwrap();
    pub fn get_debug_port(&self) -> Result<u32, VboxError> {
        get_function_result_number!(self.object, GetDebugPort, u32)
    }

    /// The port to listen on or connect to, depending on the selected I/O provider.
    ///
    /// # Arguments
    ///
    /// * `debug_port` - u32.
    ///
    /// # Returns
    ///
    /// Returns a () success, or a [`VboxError`] on failure.
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
    /// let guest_debug_control = machine_mut.get_guest_debug_control().unwrap();
    /// guest_debug_control.set_debug_port(12345).unwrap();
    /// machine_mut.save_settings().unwrap();
    pub fn set_debug_port(&self, debug_port: u32) -> Result<(), VboxError> {
        get_function_result_unit!(self.object, SetDebugPort, debug_port)
    }
}
