use crate::console::Console;
use crate::enums::{SessionState, SessionType};
use crate::machine::Machine;
use crate::session::Session;
use crate::utility::macros::macros::{
    get_function_result_number, get_function_result_pointer, get_function_result_str,
    get_function_result_unit,
};
use crate::VboxError;
use vbox_raw::sys_lib::{IConsole, IMachine};

impl Session {
    /// Unlocks a machine that was previously locked for the current session.
    ///
    /// # Returns
    ///
    /// Returns Ok(()) on success, or a [`VboxError`] on failure.
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
    /// session.unlock_machine().unwrap();

    pub fn unlock_machine(&self) -> Result<(), VboxError> {
        get_function_result_unit!(self.object, UnlockMachine)
    }

    /// Current state of this session.
    ///
    /// # Returns
    ///
    /// Returns [`SessionState`] on success, or a [`VboxError`] on failure.
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
    /// let state = session.get_state().unwrap();
    pub fn get_state(&self) -> Result<SessionState, VboxError> {
        let session_type = get_function_result_number!(self.object, GetState, u32)?;
        Ok(SessionState::from(session_type))
    }

    /// Type of this session.
    ///
    /// # Returns
    ///
    /// Returns [`SessionType`] on success, or a [`VboxError`] on failure.
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
    /// let session_type = session.get_type().unwrap();
    pub fn get_type(&self) -> Result<SessionType, VboxError> {
        let session_type = get_function_result_number!(self.object, GetType, u32)?;
        Ok(SessionType::from(session_type))
    }

    /// Name of this session.
    ///
    /// # Returns
    ///
    /// Returns [&str] on success, or a [`VboxError`] on failure.
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
    /// let name = session.get_name().unwrap();
    pub fn get_name(&self) -> Result<&'static str, VboxError> {
        get_function_result_str!(self.object, GetName)
    }
    /// Machine object associated with this session.
    ///
    /// # Returns
    ///
    /// Returns [`Machine`] on success, or a [`VboxError`] on failure.
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
    /// let machine_mut = session.get_machine().unwrap();

    pub fn get_machine(&self) -> Result<Machine, VboxError> {
        let machine = get_function_result_pointer!(self.object, GetMachine, *mut IMachine)?;
        Ok(Machine::new(machine))
    }

    /// Console object associated with this session.
    ///
    /// # Returns
    ///
    /// Returns [`Console`] on success, or a [`VboxError`] on failure.
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
    pub fn get_console(&self) -> Result<Console, VboxError> {
        let console = get_function_result_pointer!(self.object, GetConsole, *mut IConsole)?;
        Ok(Console::new(console))
    }
}
