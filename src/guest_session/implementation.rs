use crate::enums::{
    DirectoryCopyFlag, DirectoryRemoveRecFlag, FileCopyFlag, FsObjMoveFlag, FsObjRenameFlag,
    GuestSessionStatus, GuestSessionWaitForFlag, GuestSessionWaitResult, PathStyle,
    SymlinkReadFlag, SymlinkType,
};
use crate::utility::macros::macros::{
    get_function_result_bool, get_function_result_number, get_function_result_pointer,
    get_function_result_str, get_function_result_unit,
};
use crate::utility::{c_u64_str_to_string, str_vec_to_ptr, string_to_c_u64_str};
use crate::{EventSource, GuestSession, Progress, VboxError};
use vbox_raw::sys_lib::{IEventSource, IProgress, PRUint32};

impl GuestSession {
    /// Returns the user name used by this session to impersonate users in the guest.
    ///
    /// # Returns
    ///
    /// Returns &str  on success, or a [`VboxError`] on failure.
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
    /// let guest = console.get_guest().unwrap();
    /// let guest_session = guest.create_session(
    ///     "user",
    ///     "pass",
    ///     "domain",
    ///     "s1").unwrap();
    /// let user = guest_session.get_user().unwrap();
    /// guest_session.close().unwrap();
    pub fn get_user(&self) -> Result<&'static str, VboxError> {
        get_function_result_str!(self.object, GetUser)
    }
    /// Returns the domain name used by this session to impersonate users in the guest.
    ///
    /// # Returns
    ///
    /// Returns &str  on success, or a [`VboxError`] on failure.
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
    /// let guest = console.get_guest().unwrap();
    /// let guest_session = guest.create_session(
    ///     "user",
    ///     "pass",
    ///     "domain",
    ///     "s1").unwrap();
    /// let domain = guest_session.get_domain().unwrap();
    /// guest_session.close().unwrap();
    pub fn get_domain(&self) -> Result<&'static str, VboxError> {
        get_function_result_str!(self.object, GetDomain)
    }

    /// Returns the session's friendly name.
    ///
    /// # Returns
    ///
    /// Returns &str  on success, or a [`VboxError`] on failure.
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
    /// let guest = console.get_guest().unwrap();
    /// let guest_session = guest.create_session(
    ///     "user",
    ///     "pass",
    ///     "domain",
    ///     "s1").unwrap();
    /// let name = guest_session.get_name().unwrap();
    /// guest_session.close().unwrap();
    pub fn get_name(&self) -> Result<&'static str, VboxError> {
        get_function_result_str!(self.object, GetName)
    }

    /// Returns the internal session ID.
    ///
    /// # Returns
    ///
    /// Returns u32  on success, or a [`VboxError`] on failure.
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
    /// let guest = console.get_guest().unwrap();
    /// let guest_session = guest.create_session(
    ///     "user",
    ///     "pass",
    ///     "domain",
    ///     "s1").unwrap();
    /// let id = guest_session.get_id().unwrap();
    /// guest_session.close().unwrap();
    pub fn get_id(&self) -> Result<u32, VboxError> {
        get_function_result_number!(self.object, GetId, u32)
    }

    /// Returns the session timeout (in ms).
    ///
    /// # Returns
    ///
    /// Returns u32  on success, or a [`VboxError`] on failure.
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
    /// let guest = console.get_guest().unwrap();
    /// let guest_session = guest.create_session(
    ///     "user",
    ///     "pass",
    ///     "domain",
    ///     "s1").unwrap();
    /// let timeout = guest_session.get_timeout().unwrap();
    /// guest_session.close().unwrap();
    pub fn get_timeout(&self) -> Result<u32, VboxError> {
        get_function_result_number!(self.object, GetTimeout, u32)
    }

    /// Returns the session timeout (in ms).
    ///
    /// # Arguments
    ///
    /// * `timeout` -  u32
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
    /// let mut session = Session::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    ///
    /// let console = session.get_console().unwrap();
    ///
    /// let guest = console.get_guest().unwrap();
    /// let guest_session = guest.create_session(
    ///     "user",
    ///     "pass",
    ///     "domain",
    ///     "s1").unwrap();
    /// guest_session.set_timeout(1000).unwrap();
    pub fn set_timeout(&self, timeout: u32) -> Result<(), VboxError> {
        get_function_result_unit!(self.object, SetTimeout, timeout)
    }

    /// Returns the protocol version which is used by this session to communicate with the guest.
    ///
    /// # Returns
    ///
    /// Returns u32  on success, or a [`VboxError`] on failure.
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
    /// let guest = console.get_guest().unwrap();
    /// let guest_session = guest.create_session(
    ///     "user",
    ///     "pass",
    ///     "domain",
    ///     "s1").unwrap();
    /// let protocol_version = guest_session.get_protocol_version().unwrap();
    /// guest_session.close().unwrap();
    pub fn get_protocol_version(&self) -> Result<u32, VboxError> {
        get_function_result_number!(self.object, GetProtocolVersion, u32)
    }

    /// Returns the current session status.
    ///
    /// # Returns
    ///
    /// Returns [`GuestSessionStatus`]  on success, or a [`VboxError`] on failure.
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
    /// let guest = console.get_guest().unwrap();
    /// let guest_session = guest.create_session(
    ///     "user",
    ///     "pass",
    ///     "domain",
    ///     "s1").unwrap();
    /// let status = guest_session.get_status().unwrap();
    /// guest_session.close().unwrap();
    pub fn get_status(&self) -> Result<GuestSessionStatus, VboxError> {
        let status = get_function_result_number!(self.object, GetStatus, u32)?;
        Ok(GuestSessionStatus::from(status))
    }

    /// The get of scheduled environment changes to the base environment of the session.
    ///
    /// # Returns
    ///
    /// Returns Vec<&str>  on success, or a [`VboxError`] on failure.
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
    /// let guest = console.get_guest().unwrap();
    /// let guest_session = guest.create_session(
    ///     "user",
    ///     "pass",
    ///     "domain",
    ///     "s1").unwrap();
    /// let environment_changes = guest_session.get_environment_changes().unwrap();
    /// guest_session.close().unwrap();
    pub fn get_environment_changes(&self) -> Result<Vec<&'static str>, VboxError> {
        let mut count = 0;
        let environment_changes_ptr = get_function_result_pointer!(
            self.object,
            GetEnvironmentChanges,
            *mut *mut u16,
            &mut count
        )?;
        let environment_changes_ptr_vec =
            unsafe { Vec::from_raw_parts(environment_changes_ptr, count as usize, count as usize) };
        let environment_changes = environment_changes_ptr_vec
            .iter()
            .map(|s| c_u64_str_to_string(s.clone()))
            .collect();
        environment_changes
    }

    /// The set of scheduled environment changes to the base environment of the session.
    ///
    /// They are in putenv format, i.e. "VAR=VALUE" for setting and "VAR" for unsetting. One entry per variable (change). The changes are applied when creating new guest processes.
    ///
    /// This is writable, so to undo all the scheduled changes, assign it an empty array.
    ///
    /// # Arguments
    ///
    /// * `environment_changes` -  Vec<&str>
    ///
    /// # Returns
    ///
    /// Returns ()  on success, or a [`VboxError`] on failure.
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
    /// let guest = console.get_guest().unwrap();
    /// let guest_session = guest.create_session(
    ///     "user",
    ///     "pass",
    ///     "domain",
    ///     "s1").unwrap();
    /// guest_session.set_environment_changes(vec!["VAR=VALUE", "VAR1=VALUE1"]).unwrap();
    pub fn set_environment_changes(&self, environment_changes: Vec<&str>) -> Result<(), VboxError> {
        let mut environment_changes_ptr_vec = Vec::new();
        for argument in &environment_changes {
            environment_changes_ptr_vec.push(string_to_c_u64_str(argument)?);
        }
        let environment_changes_ptr = environment_changes_ptr_vec.as_mut_ptr();
        let environment_changes_size = environment_changes.len() as PRUint32;

        get_function_result_unit!(
            self.object,
            SetEnvironmentChanges,
            environment_changes_size,
            environment_changes_ptr
        )
    }

    /// The base environment of the session.
    ///
    /// They are on the "VAR=VALUE" form, one array entry per variable.
    ///
    /// Access fails with VBOX_E_NOT_SUPPORTED if the Guest Additions does not support the session base environment feature. Support for this was introduced with protocol version XXXX.
    ///
    /// Access fails with VBOX_E_INVALID_OBJECT_STATE if the Guest Additions has yet to report the session base environment.
    ///
    /// # Returns
    ///
    /// Returns Vec<&str>  on success, or a [`VboxError`] on failure.
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
    /// let guest = console.get_guest().unwrap();
    /// let guest_session = guest.create_session(
    ///     "user",
    ///     "pass",
    ///     "domain",
    ///     "s1").unwrap();
    /// let environment_base = guest_session.get_environment_base().unwrap();
    /// guest_session.close().unwrap();
    pub fn get_environment_base(&self) -> Result<Vec<&'static str>, VboxError> {
        let mut count = 0;
        let environment_base_ptr = get_function_result_pointer!(
            self.object,
            GetEnvironmentBase,
            *mut *mut u16,
            &mut count
        )?;
        let environment_base_ptr_vec =
            unsafe { Vec::from_raw_parts(environment_base_ptr, count as usize, count as usize) };
        let environment_base = environment_base_ptr_vec
            .iter()
            .map(|s| c_u64_str_to_string(s.clone()))
            .collect();
        environment_base
    }
    // TODO GetProcesses

    /// The style of paths used by the guest.
    ///
    /// # Returns
    ///
    /// Returns [`PathStyle`]  on success, or a [`VboxError`] on failure.
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
    /// let guest = console.get_guest().unwrap();
    /// let guest_session = guest.create_session(
    ///     "user",
    ///     "pass",
    ///     "domain",
    ///     "s1").unwrap();
    /// let path_style = guest_session.get_path_style().unwrap();
    /// guest_session.close().unwrap();
    pub fn get_path_style(&self) -> Result<PathStyle, VboxError> {
        let path_style = get_function_result_number!(self.object, GetPathStyle, u32)?;
        Ok(PathStyle::from(path_style))
    }

    /// Gets the current directory of the session.
    ///
    /// Guest path style.
    ///
    /// # Returns
    ///
    /// Returns &str  on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{Session, VirtualBox};
    /// use virtualbox_rs::enums::{GuestSessionWaitForFlag, SessionType};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let mut session = Session::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    ///
    /// let console = session.get_console().unwrap();
    ///
    /// let guest = console.get_guest().unwrap();
    /// let guest_session = guest.create_session(
    ///     "user",
    ///     "pass",
    ///     "domain",
    ///     "s1").unwrap();
    /// guest_session.wait_for(GuestSessionWaitForFlag::Start, u32::MAX).unwrap();
    /// let current_directory = guest_session.get_current_directory().unwrap();
    pub fn get_current_directory(&self) -> Result<&'static str, VboxError> {
        get_function_result_str!(self.object, GetCurrentDirectory)
    }

    /// Sets the current directory of the session.
    ///
    /// Guest path style.
    ///
    /// # Arguments
    ///
    /// * `current_directory` -  &str
    ///
    /// # Returns
    ///
    /// Returns ()  on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{Session, VirtualBox};
    /// use virtualbox_rs::enums::{GuestSessionWaitForFlag, SessionType};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let mut session = Session::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    ///
    /// let console = session.get_console().unwrap();
    ///
    /// let guest = console.get_guest().unwrap();
    /// let guest_session = guest.create_session(
    ///     "user",
    ///     "pass",
    ///     "domain",
    ///     "s1").unwrap();
    /// guest_session.wait_for(GuestSessionWaitForFlag::Start, u32::MAX).unwrap();
    /// guest_session.set_current_directory("/home/user/").unwrap();
    pub fn set_current_directory(&self, current_directory: &str) -> Result<(), VboxError> {
        let current_directory = string_to_c_u64_str(current_directory)?;
        get_function_result_unit!(self.object, SetCurrentDirectory, current_directory)
    }

    /// Returns the user's home / profile directory.
    ///
    /// Guest path style.
    ///
    /// # Returns
    ///
    /// Returns &str  on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{Session, VirtualBox};
    /// use virtualbox_rs::enums::{GuestSessionWaitForFlag, SessionType};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let mut session = Session::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    ///
    /// let console = session.get_console().unwrap();
    ///
    /// let guest = console.get_guest().unwrap();
    /// let guest_session = guest.create_session(
    ///     "user",
    ///     "pass",
    ///     "domain",
    ///     "s1").unwrap();
    /// guest_session.wait_for(GuestSessionWaitForFlag::Start, u32::MAX).unwrap();
    /// let user_home = guest_session.get_user_home().unwrap();
    /// guest_session.close().unwrap();
    pub fn get_user_home(&self) -> Result<&'static str, VboxError> {
        get_function_result_str!(self.object, GetUserHome)
    }

    /// Returns the user's documents directory.
    ///
    /// Guest path style.
    ///
    /// # Returns
    ///
    /// Returns &str  on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{Session,  VirtualBox};
    /// use virtualbox_rs::enums::{GuestSessionWaitForFlag, SessionType};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let mut session = Session::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    ///
    /// let console = session.get_console().unwrap();
    ///
    /// let guest = console.get_guest().unwrap();
    /// let guest_session = guest.create_session(
    ///     "user",
    ///     "pass",
    ///     "domain",
    ///     "s1").unwrap();
    /// guest_session.wait_for(GuestSessionWaitForFlag::Start, u32::MAX).unwrap();
    /// let user_documents = guest_session.get_user_documents().unwrap();
    pub fn get_user_documents(&self) -> Result<&'static str, VboxError> {
        get_function_result_str!(self.object, GetUserDocuments)
    }
    // TODO GetFiles

    /// Event source for guest session events.
    ///
    /// # Returns
    ///
    /// Returns [`EventSource`]  on success, or a [`VboxError`] on failure.
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
    /// let guest = console.get_guest().unwrap();
    /// let guest_session = guest.create_session(
    ///     "user",
    ///     "pass",
    ///     "domain",
    ///     "s1").unwrap();
    /// let event_source = guest_session.get_event_source().unwrap();
    pub fn get_event_source(&self) -> Result<EventSource, VboxError> {
        let event_source =
            get_function_result_pointer!(self.object, GetEventSource, *mut IEventSource)?;
        Ok(EventSource::new(event_source))
    }

    // TODO GetDirectories

    /// Closes this session.
    ///
    /// All opened guest directories, files and processes which are not referenced by clients anymore will be closed. Guest processes which fall into this category and still are running in the guest will be terminated automatically.
    ///
    /// # Returns
    ///
    /// Returns ()  on success, or a [`VboxError`] on failure.
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
    /// let guest = console.get_guest().unwrap();
    /// let guest_session = guest.create_session(
    ///     "user",
    ///     "pass",
    ///     "domain",
    ///     "s1").unwrap();
    /// guest_session.close().unwrap();
    pub fn close(&self) -> Result<(), VboxError> {
        get_function_result_unit!(self.object, Close)
    }

    /// Copies directories and/or files from guest to the host.
    ///
    /// This function requires several parallel arrays to be supplied, one set for each source.
    /// <div class="warning">
    /// The lengths of all arrays must match, otherwise an NS_ERROR_ILLEGAL_VALUE error will be returned.
    /// </div>
    ///
    /// # Arguments
    ///
    /// * `source` - Vec<&str>. Paths to directories and/or files on the guest side that should be copied to the host. If the path ends with a path delimiter, only the directory's content is being copied. Guest path style.
    /// * `filters` - Vec<&str>. Array of source filters. This uses the DOS/NT style wildcard characters '?' and '*'.
    /// * `flags` - Vec<&str>. 	Array of comma-separated list of source flags.
    /// * `destination` - &str. Where to put the sources on the host. Host path style.
    ///
    /// The following flags are available:
    ///
    /// ***CopyIntoExisting*** - Allow copying into an existing destination directory.
    ///
    /// ***NoReplace*** - Do not replace any existing destination files on the destination.
    ///
    /// ***FollowLinks*** - Follows (and handles) (symbolic) links.
    ///
    /// ***Update*** - Only copy when the source file is newer than the destination file or when the destination file is missing.
    ///
    /// # Returns
    ///
    /// Returns [`Progress`] on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{Session, VirtualBox};
    /// use virtualbox_rs::enums::{GuestSessionWaitForFlag, SessionType};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let mut session = Session::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    ///
    /// let console = session.get_console().unwrap();
    ///
    /// let guest = console.get_guest().unwrap();
    /// let guest_session = guest.create_session(
    ///     "user",
    ///     "pass",
    ///     "domain",
    ///     "s1").unwrap();
    /// guest_session.wait_for(GuestSessionWaitForFlag::Start, u32::MAX).unwrap();
    /// let progress = guest_session.copy_from_guest(
    ///             vec![
    ///                 "/home/guest_user/d1",
    ///                 "/home/guest_user/d2"
    ///             ],
    ///             vec!["*", "*"],
    ///             vec!["", ""],
    ///             "/home/host_user/dir/"
    ///
    ///     ).unwrap();
    /// progress.wait_for_completion(-1).unwrap();
    /// guest_session.close().unwrap();
    pub fn copy_from_guest(
        &self,
        sources: Vec<&str>,
        filters: Vec<&str>,
        flags: Vec<&str>,
        destination: &str,
    ) -> Result<Progress, VboxError> {
        let (sources_size, sources_ptr) = str_vec_to_ptr(sources)?;
        let (filters_size, filters_ptr) = str_vec_to_ptr(filters)?;
        let (flags_size, flags_ptr) = str_vec_to_ptr(flags)?;
        let destination_ptr = string_to_c_u64_str(destination)?;
        let progress = get_function_result_pointer!(
            self.object,
            CopyFromGuest,
            *mut IProgress,
            sources_size,
            sources_ptr,
            filters_size,
            filters_ptr,
            flags_size,
            flags_ptr,
            destination_ptr
        )?;
        Ok(Progress::new(progress))
    }

    /// Copies directories and/or files from host to the guest.
    ///
    /// This function requires several parallel arrays to be supplied, one set for each source.
    /// <div class="warning">
    /// The lengths of all arrays must match, otherwise an NS_ERROR_ILLEGAL_VALUE error will be returned.
    /// </div>
    ///
    /// # Arguments
    ///
    /// * `source` - Vec<&str>. Paths to directories and/or files on the host side that should be copied to the guest. If the path ends with a path delimiter, only the directory's content is being copied. Host path style.
    /// * `filters` - Vec<&str>. Array of source filters. This uses the DOS/NT style wildcard characters '?' and '*'.
    /// * `flags` - Vec<&str>. 	Array of comma-separated list of source flags.
    /// * `destination` - &str. Where to put the sources on the guest. Guest path style.
    ///
    /// The following flags are available:
    ///
    /// ***CopyIntoExisting*** - Allow copying into an existing destination directory.
    ///
    /// ***NoReplace*** - Do not replace any existing destination files on the destination.
    ///
    /// ***FollowLinks*** - Follows (and handles) (symbolic) links.
    ///
    /// ***Update*** - Only copy when the source file is newer than the destination file or when the destination file is missing.
    ///
    /// # Returns
    ///
    /// Returns [`Progress`] on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{Session, VirtualBox};
    /// use virtualbox_rs::enums::{GuestSessionWaitForFlag, SessionType};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let mut session = Session::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    ///
    /// let console = session.get_console().unwrap();
    ///
    /// let guest = console.get_guest().unwrap();
    /// let guest_session = guest.create_session(
    ///     "user",
    ///     "pass",
    ///     "domain",
    ///     "s1").unwrap();
    /// guest_session.wait_for(GuestSessionWaitForFlag::Start, u32::MAX).unwrap();
    /// let progress = guest_session.copy_to_guest(
    ///             vec![
    ///                 "/home/host_user/d1",
    ///                 "/home/host_user/d2"
    ///             ],
    ///             vec!["*", "*"],
    ///             vec!["", ""],
    ///             "/home/guest_user/dir/"
    ///
    ///     ).unwrap();
    /// progress.wait_for_completion(-1).unwrap();
    /// guest_session.close().unwrap();
    pub fn copy_to_guest(
        &self,
        sources: Vec<&str>,
        filters: Vec<&str>,
        flags: Vec<&str>,
        destination: &str,
    ) -> Result<Progress, VboxError> {
        let (sources_size, sources_ptr) = str_vec_to_ptr(sources)?;
        let (filters_size, filters_ptr) = str_vec_to_ptr(filters)?;
        let (flags_size, flags_ptr) = str_vec_to_ptr(flags)?;
        let destination_ptr = string_to_c_u64_str(destination)?;
        let progress = get_function_result_pointer!(
            self.object,
            CopyToGuest,
            *mut IProgress,
            sources_size,
            sources_ptr,
            filters_size,
            filters_ptr,
            flags_size,
            flags_ptr,
            destination_ptr
        )?;
        Ok(Progress::new(progress))
    }

    ///  Recursively copies a directory from one guest location to another.
    ///
    ///
    /// # Arguments
    ///
    /// * `source` - &str. The path to the directory to copy (in the guest). Guest path style.
    /// * `destination` - &str. The path to the target directory (in the guest). Unless the [`DirectoryCopyFlag::CopyIntoExisting`] flag is given, the directory shall not already exist. Guest path style.
    /// * `flags` - [`Vec<DirectoryCopyFlag>`]. Zero or more [`DirectoryCopyFlag`] values.
    ///
    /// # Returns
    ///
    /// Returns [`Progress`]  on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{Session, VirtualBox};
    /// use virtualbox_rs::enums::{DirectoryCopyFlag, GuestSessionWaitForFlag, SessionType};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let mut session = Session::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    ///
    /// let console = session.get_console().unwrap();
    ///
    /// let guest = console.get_guest().unwrap();
    /// let guest_session = guest.create_session(
    ///     "user",
    ///     "pass",
    ///     "domain",
    ///     "s1").unwrap();
    /// guest_session.wait_for(GuestSessionWaitForFlag::Start, u32::MAX).unwrap();
    /// let progress = guest_session.directory_copy(
    ///         "/home/guest_user/old_dir",
    ///         "/home/guest_user/new_dir",
    ///         vec![DirectoryCopyFlag::CopyIntoExisting]
    ///     ).unwrap();
    /// progress.wait_for_completion(-1).unwrap();
    /// guest_session.close().unwrap();
    pub fn directory_copy(
        &self,
        source: &str,
        destination: &str,
        flags: Vec<DirectoryCopyFlag>,
    ) -> Result<Progress, VboxError> {
        let mut flags: Vec<u32> = flags.iter().map(|f| (*f).into()).collect();
        let source_ptr = string_to_c_u64_str(source)?;
        let destination_ptr = string_to_c_u64_str(destination)?;
        let flags_size = flags.len() as u32;
        let flags_ptr = flags.as_mut_ptr();
        let progress = get_function_result_pointer!(
            self.object,
            DirectoryCopy,
            *mut IProgress,
            source_ptr,
            destination_ptr,
            flags_size,
            flags_ptr
        )?;
        Ok(Progress::new(progress))
    }

    ///  Recursively copies a directory from the guest to the host.
    ///
    ///
    /// # Arguments
    ///
    /// * `source` - &str. Path to the directory on the guest side that should be copied to the host. Guest path style.
    /// * `destination` - &str. Where to put the directory on the host. Unless the [`DirectoryCopyFlag::CopyIntoExisting`] flag is given, the directory shall not already exist. Host path style.
    /// * `flags` - [`Vec<DirectoryCopyFlag>`]. Zero or more [`DirectoryCopyFlag`] values.
    ///
    /// # Returns
    ///
    /// Returns [`Progress`]  on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{Session, VirtualBox};
    /// use virtualbox_rs::enums::{DirectoryCopyFlag, GuestSessionWaitForFlag, SessionType};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let mut session = Session::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    ///
    /// let console = session.get_console().unwrap();
    ///
    /// let guest = console.get_guest().unwrap();
    /// let guest_session = guest.create_session(
    ///     "user",
    ///     "pass",
    ///     "domain",
    ///     "s1").unwrap();
    /// guest_session.wait_for(GuestSessionWaitForFlag::Start, u32::MAX).unwrap();
    /// let progress = guest_session.directory_copy_from_guest(
    ///         "/home/guest_user/dir",
    ///         "/home/host_user/dir",
    ///         vec![DirectoryCopyFlag::CopyIntoExisting]
    ///     ).unwrap();
    /// progress.wait_for_completion(-1).unwrap();
    /// guest_session.close().unwrap();
    pub fn directory_copy_from_guest(
        &self,
        source: &str,
        destination: &str,
        flags: Vec<DirectoryCopyFlag>,
    ) -> Result<Progress, VboxError> {
        let mut flags: Vec<u32> = flags.iter().map(|f| (*f).into()).collect();
        let source_ptr = string_to_c_u64_str(source)?;
        let destination_ptr = string_to_c_u64_str(destination)?;
        let flags_size = flags.len() as u32;
        let flags_ptr = flags.as_mut_ptr();
        let progress = get_function_result_pointer!(
            self.object,
            DirectoryCopyFromGuest,
            *mut IProgress,
            source_ptr,
            destination_ptr,
            flags_size,
            flags_ptr
        )?;
        Ok(Progress::new(progress))
    }

    ///  Recursively copies a directory from the host to the guest.
    ///
    ///
    /// # Arguments
    ///
    /// * `source` - &str. Path to the directory on the host side that should be copied to the guest. Host path style.
    /// * `destination` - &str. Where to put the file in the guest. Unless the [`DirectoryCopyFlag::CopyIntoExisting`] flag is given, the directory shall not already exist. Guest style path.
    /// * `flags` - [`Vec<DirectoryCopyFlag>`]. Zero or more [`DirectoryCopyFlag`] values.
    ///
    /// # Returns
    ///
    /// Returns [`Progress`]  on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{Session, VirtualBox};
    /// use virtualbox_rs::enums::{DirectoryCopyFlag, GuestSessionWaitForFlag, SessionType};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let mut session = Session::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    ///
    /// let console = session.get_console().unwrap();
    ///
    /// let guest = console.get_guest().unwrap();
    /// let guest_session = guest.create_session(
    ///     "user",
    ///     "pass",
    ///     "domain",
    ///     "s1").unwrap();
    /// guest_session.wait_for(GuestSessionWaitForFlag::Start, u32::MAX).unwrap();
    /// let progress = guest_session.directory_copy_to_guest(
    ///         "/home/host_user/dir",
    ///         "/home/guest_user/dir",
    ///         vec![DirectoryCopyFlag::CopyIntoExisting]
    ///     ).unwrap();
    /// progress.wait_for_completion(-1).unwrap();
    /// guest_session.close().unwrap();

    pub fn directory_copy_to_guest(
        &self,
        source: &str,
        destination: &str,
        flags: Vec<DirectoryCopyFlag>,
    ) -> Result<Progress, VboxError> {
        let mut flags: Vec<u32> = flags.iter().map(|f| (*f).into()).collect();
        let source_ptr = string_to_c_u64_str(source)?;
        let destination_ptr = string_to_c_u64_str(destination)?;
        let flags_size = flags.len() as u32;
        let flags_ptr = flags.as_mut_ptr();
        let progress = get_function_result_pointer!(
            self.object,
            DirectoryCopyToGuest,
            *mut IProgress,
            source_ptr,
            destination_ptr,
            flags_size,
            flags_ptr
        )?;
        Ok(Progress::new(progress))
    }

    ///  Creates a directory in the guest.
    ///
    ///
    /// # Arguments
    ///
    /// * `path` - &str. Path to the directory directory to be created. Guest path style.
    /// * `mode` - u32. The UNIX-style access mode mask to create the directory with. Whether/how all three access groups and associated access rights are realized is guest OS dependent. The API does the best it can on each OS.
    /// * `flags` - [`Vec<DirectoryCopyFlag>`]. Zero or more [`DirectoryCopyFlag`] values.
    ///
    /// # Returns
    ///
    /// Returns ()  on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{Session, VirtualBox};
    /// use virtualbox_rs::enums::{DirectoryCopyFlag, GuestSessionWaitForFlag, SessionType};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let mut session = Session::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    ///
    /// let console = session.get_console().unwrap();
    ///
    /// let guest = console.get_guest().unwrap();
    /// let guest_session = guest.create_session(
    ///     "user",
    ///     "pass",
    ///     "domain",
    ///     "s1").unwrap();
    /// guest_session.wait_for(GuestSessionWaitForFlag::Start, u32::MAX).unwrap();
    /// guest_session.directory_create(
    ///         "/home/guest_user/dir",
    ///         777,
    ///         vec![DirectoryCopyFlag::CopyIntoExisting]
    ///     ).unwrap();
    /// guest_session.close().unwrap();
    pub fn directory_create(
        &self,
        path: &str,
        mode: u32,
        flags: Vec<DirectoryCopyFlag>,
    ) -> Result<(), VboxError> {
        let mut flags: Vec<u32> = flags.iter().map(|f| (*f).into()).collect();
        let path_ptr = string_to_c_u64_str(path)?;
        let flags_size = flags.len() as u32;
        let flags_ptr = flags.as_mut_ptr();
        get_function_result_unit!(
            self.object,
            DirectoryCreate,
            path_ptr,
            mode,
            flags_size,
            flags_ptr
        )
    }

    ///  Creates a temporary directory in the guest.
    ///
    ///
    /// # Arguments
    ///
    /// * `template_name` - &str. Template for the name of the directory to create. This must contain at least one 'X' character. The first group of consecutive 'X' characters in the template will be replaced by a random alphanumeric string to produce a unique name.
    /// * `mode` - u32. The UNIX-style access mode mask to create the directory with. Whether/how all three access groups and associated access rights are realized is guest OS dependent. The API does the best it can on each OS.
    /// * `path` - &str. The path to the directory in which the temporary directory should be created. Guest path style.
    /// * `secure` - bool. Whether to fail if the directory can not be securely created. Currently this means that another unprivileged user cannot manipulate the path specified or remove the temporary directory after it has been created. Also causes the mode specified to be ignored. May not be supported on all guest types.
    ///
    /// # Returns
    ///
    /// Returns the full path to the created directory  on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{Session, VirtualBox};
    /// use virtualbox_rs::enums::{GuestSessionWaitForFlag, SessionType};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let mut session = Session::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    ///
    /// let console = session.get_console().unwrap();
    ///
    /// let guest = console.get_guest().unwrap();
    /// let guest_session = guest.create_session(
    ///     "user",
    ///     "pass",
    ///     "domain",
    ///     "s1").unwrap();
    /// guest_session.wait_for(GuestSessionWaitForFlag::Start, u32::MAX).unwrap();
    /// let dir_name = guest_session.directory_create_temp(
    ///             "tmpXX",
    ///             755,
    ///             "/tmp",
    ///             false
    ///         ).unwrap();
    /// guest_session.close().unwrap();
    pub fn directory_create_temp(
        &self,
        template_name: &str,
        mode: u32,
        path: &str,
        secure: bool,
    ) -> Result<&'static str, VboxError> {
        let template_name_ptr = string_to_c_u64_str(template_name)?;
        let path_ptr = string_to_c_u64_str(path)?;
        let secure = if secure { 1 } else { 0 };
        get_function_result_str!(
            self.object,
            DirectoryCreateTemp,
            template_name_ptr,
            mode,
            path_ptr,
            secure
        )
    }

    ///  Checks whether a directory exists in the guest or not.
    ///
    ///
    /// # Arguments
    ///
    /// * `path` - &str. Path to the directory to check if exists. Guest path style.
    /// * `follow_sym_links` - bool. If true, symbolic links in the final component will be followed and the existance of the symlink target made the question for this method. If false, a symbolic link in the final component will make the method return false (because a symlink isn't a directory).
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
    /// use virtualbox_rs::enums::{GuestSessionWaitForFlag, SessionType};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let mut session = Session::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    ///
    /// let console = session.get_console().unwrap();
    ///
    /// let guest = console.get_guest().unwrap();
    /// let guest_session = guest.create_session(
    ///     "user",
    ///     "pass",
    ///     "domain",
    ///     "s1").unwrap();
    /// guest_session.wait_for(GuestSessionWaitForFlag::Start, u32::MAX).unwrap();
    /// let exists = guest_session.directory_exists(
    ///             "/home/guest_user/dir",
    ///             false
    ///         ).unwrap();
    /// guest_session.close().unwrap();
    pub fn directory_exists(&self, path: &str, follow_sym_links: bool) -> Result<bool, VboxError> {
        let path_ptr = string_to_c_u64_str(path)?;
        let follow_sym_links = if follow_sym_links { 1 } else { 0 };
        get_function_result_bool!(self.object, DirectoryExists, path_ptr, follow_sym_links)
    }
    // TODO DirectoryOpen

    ///  Removes a guest directory if empty.
    ///
    ///
    /// # Arguments
    ///
    /// * `path` - &str. Path to the directory that should be removed. Guest path style.
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
    /// use virtualbox_rs::enums::{GuestSessionWaitForFlag, SessionType};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let mut session = Session::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    ///
    /// let console = session.get_console().unwrap();
    ///
    /// let guest = console.get_guest().unwrap();
    /// let guest_session = guest.create_session(
    ///     "user",
    ///     "pass",
    ///     "domain",
    ///     "s1").unwrap();
    /// guest_session.wait_for(GuestSessionWaitForFlag::Start, u32::MAX).unwrap();
    /// guest_session.directory_remove(
    ///             "/home/guest_user/dir"
    ///         ).unwrap();
    /// guest_session.close().unwrap();
    pub fn directory_remove(&self, path: &str) -> Result<(), VboxError> {
        let path_ptr = string_to_c_u64_str(path)?;
        get_function_result_unit!(self.object, DirectoryRemove, path_ptr)
    }

    ///  Removes a guest directory recursively.
    ///
    ///
    /// # Arguments
    ///
    /// * `path` - &str. Path of the directory that is to be removed recursively. Guest path style.
    /// * `flags` - [`Vec<DirectoryRemoveRecFlag>`]. Zero or more [`DirectoryRemoveRecFlag`] flags.
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
    /// use virtualbox_rs::enums::{DirectoryRemoveRecFlag, GuestSessionWaitForFlag, SessionType};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let mut session = Session::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    ///
    /// let console = session.get_console().unwrap();
    ///
    /// let guest = console.get_guest().unwrap();
    /// let guest_session = guest.create_session(
    ///     "user",
    ///     "pass",
    ///     "domain",
    ///     "s1").unwrap();
    /// guest_session.wait_for(GuestSessionWaitForFlag::Start, u32::MAX).unwrap();
    /// let progress = guest_session.directory_remove_recursive(
    ///             "/home/guest_user/dir",
    ///             vec![DirectoryRemoveRecFlag::ContentAndDir]
    ///         ).unwrap();
    /// progress.wait_for_completion(-1).unwrap();
    /// guest_session.close().unwrap();
    pub fn directory_remove_recursive(
        &self,
        path: &str,
        flags: Vec<DirectoryRemoveRecFlag>,
    ) -> Result<Progress, VboxError> {
        let mut flags: Vec<u32> = flags.iter().map(|f| (*f).into()).collect();
        let path_ptr = string_to_c_u64_str(path)?;
        let flags_size = flags.len() as u32;
        let flags_ptr = flags.as_mut_ptr();
        let progress = get_function_result_pointer!(
            self.object,
            DirectoryRemoveRecursive,
            *mut IProgress,
            path_ptr,
            flags_size,
            flags_ptr
        )?;
        Ok(Progress::new(progress))
    }

    ///  Schedules setting an environment variable when creating the next guest process.
    ///
    /// This affects the [`GuestSession::get_environment_changes`] attribute.
    ///
    ///
    /// # Arguments
    ///
    /// * `name` - &str. Name of the environment variable to set. This cannot be empty nor can it contain any equal signs.
    /// * `value` - &str. Value to set the session environment variable to.
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
    /// use virtualbox_rs::enums::{GuestSessionWaitForFlag, SessionType};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let mut session = Session::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    ///
    /// let console = session.get_console().unwrap();
    ///
    /// let guest = console.get_guest().unwrap();
    /// let guest_session = guest.create_session(
    ///     "user",
    ///     "pass",
    ///     "domain",
    ///     "s1").unwrap();
    /// guest_session.wait_for(GuestSessionWaitForFlag::Start, u32::MAX).unwrap();
    /// guest_session.environment_schedule_set(
    ///             "name",
    ///             "value"
    ///         ).unwrap();
    /// guest_session.close().unwrap();
    pub fn environment_schedule_set(&self, name: &str, value: &str) -> Result<(), VboxError> {
        let name_ptr = string_to_c_u64_str(name)?;
        let value_ptr = string_to_c_u64_str(value)?;
        get_function_result_unit!(self.object, EnvironmentScheduleSet, name_ptr, value_ptr)
    }

    /// Schedules unsetting (removing) an environment variable when creating the next guest process.
    ///
    /// This affects the [`GuestSession::get_environment_changes`] attribute.
    ///
    /// # Arguments
    ///
    /// * `name` - &str. Name of the environment variable to unset. This cannot be empty nor can it contain any equal signs.
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
    /// use virtualbox_rs::enums::{GuestSessionWaitForFlag, SessionType};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let mut session = Session::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    ///
    /// let console = session.get_console().unwrap();
    ///
    /// let guest = console.get_guest().unwrap();
    /// let guest_session = guest.create_session(
    ///     "user",
    ///     "pass",
    ///     "domain",
    ///     "s1").unwrap();
    /// guest_session.wait_for(GuestSessionWaitForFlag::Start, u32::MAX).unwrap();
    /// guest_session.environment_schedule_unset(
    ///             "name"
    ///         ).unwrap();
    /// guest_session.close().unwrap();
    pub fn environment_schedule_unset(&self, name: &str) -> Result<(), VboxError> {
        let name_ptr = string_to_c_u64_str(name)?;
        get_function_result_unit!(self.object, EnvironmentScheduleUnset, name_ptr)
    }

    /// Gets an environment variable from the session's base environment ([`GuestSession::get_environment_base`]).
    ///
    /// # Arguments
    ///
    /// * `name` - &str. Name of the environment variable to get.This cannot be empty nor can it contain any equal signs.
    ///
    /// # Returns
    ///
    /// Returns &str on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{Session, VirtualBox};
    /// use virtualbox_rs::enums::{GuestSessionWaitForFlag, SessionType};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let mut session = Session::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    ///
    /// let console = session.get_console().unwrap();
    ///
    /// let guest = console.get_guest().unwrap();
    /// let guest_session = guest.create_session(
    ///     "user",
    ///     "pass",
    ///     "domain",
    ///     "s1").unwrap();
    /// guest_session.wait_for(GuestSessionWaitForFlag::Start, u32::MAX).unwrap();
    /// let name = guest_session.environment_get_base_variable(
    ///             "name"
    ///         ).unwrap();
    /// guest_session.close().unwrap();
    pub fn environment_get_base_variable(&self, name: &str) -> Result<&'static str, VboxError> {
        let name_ptr = string_to_c_u64_str(name)?;
        get_function_result_str!(self.object, EnvironmentGetBaseVariable, name_ptr)
    }

    /// Checks if the given environment variable exists in the session's base environment ([`GuestSession::get_environment_base`]).
    ///
    /// # Arguments
    ///
    /// * `name` - &str. Name of the environment variable to look for. This cannot be empty nor can it contain any equal signs.
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
    /// use virtualbox_rs::enums::{GuestSessionWaitForFlag, SessionType};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let mut session = Session::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    ///
    /// let console = session.get_console().unwrap();
    ///
    /// let guest = console.get_guest().unwrap();
    /// let guest_session = guest.create_session(
    ///     "user",
    ///     "pass",
    ///     "domain",
    ///     "s1").unwrap();
    /// guest_session.wait_for(GuestSessionWaitForFlag::Start, u32::MAX).unwrap();
    /// let exist = guest_session.environment_does_base_variable_exist(
    ///             "name"
    ///         ).unwrap();
    /// guest_session.close().unwrap();
    pub fn environment_does_base_variable_exist(&self, name: &str) -> Result<bool, VboxError> {
        let exist_ptr = string_to_c_u64_str(name)?;
        get_function_result_bool!(self.object, EnvironmentDoesBaseVariableExist, exist_ptr)
    }

    /// Copies a file from one guest location to another.
    ///
    /// # Arguments
    ///
    /// * `source` - &str. The path to the file to copy (in the guest). Guest path style.
    /// * `destination` - &str. The path to the target file (in the guest). This cannot be a directory. Guest path style.
    /// * `flags` - [`Vec<FileCopyFlag>`]. Zero or more [`FileCopyFlag`] values.
    ///
    /// # Returns
    ///
    /// Returns [`Progress`] on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{Session, VirtualBox};
    /// use virtualbox_rs::enums::{GuestSessionWaitForFlag, SessionType};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let mut session = Session::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    ///
    /// let console = session.get_console().unwrap();
    ///
    /// let guest = console.get_guest().unwrap();
    /// let guest_session = guest.create_session(
    ///     "user",
    ///     "pass",
    ///     "domain",
    ///     "s1").unwrap();
    /// guest_session.wait_for(GuestSessionWaitForFlag::Start, u32::MAX).unwrap();
    /// let progress = guest_session.file_copy(
    ///             "/home/guest_user/1.txt",
    ///             "/home/guest_user/new_dir/1.txt",
    ///             vec![]
    ///         ).unwrap();
    /// progress.wait_for_completion(-1).unwrap();
    /// guest_session.close().unwrap();
    pub fn file_copy(
        &self,
        source: &str,
        destination: &str,
        flags: Vec<FileCopyFlag>,
    ) -> Result<Progress, VboxError> {
        let mut flags: Vec<u32> = flags.iter().map(|f| (*f).into()).collect();
        let source_ptr = string_to_c_u64_str(source)?;
        let destination_ptr = string_to_c_u64_str(destination)?;
        let flags_size = flags.len() as u32;
        let flags_ptr = flags.as_mut_ptr();
        let progress = get_function_result_pointer!(
            self.object,
            FileCopy,
            *mut IProgress,
            source_ptr,
            destination_ptr,
            flags_size,
            flags_ptr
        )?;
        Ok(Progress::new(progress))
    }

    ///  Copies a file from the guest to the host.
    ///
    ///
    /// # Arguments
    ///
    /// * `source` - &str. Path to the file on the guest side that should be copied to the host. Guest path style.
    /// * `destination` - &str. Where to put the file on the host (file, not directory). Host path style.
    /// * `flags` - [`Vec<FileCopyFlag>`]. Zero or more [`FileCopyFlag`] values.
    ///
    /// # Returns
    ///
    /// Returns [`Progress`]  on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{Session, VirtualBox};
    /// use virtualbox_rs::enums::{FileCopyFlag, GuestSessionWaitForFlag, SessionType};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let mut session = Session::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    ///
    /// let console = session.get_console().unwrap();
    ///
    /// let guest = console.get_guest().unwrap();
    /// let guest_session = guest.create_session(
    ///     "user",
    ///     "pass",
    ///     "domain",
    ///     "s1").unwrap();
    /// guest_session.wait_for(GuestSessionWaitForFlag::Start, u32::MAX).unwrap();
    /// let progress = guest_session.file_copy_from_guest(
    ///         "/home/guest_user/1.txt",
    ///         "/home/host_user/1.txt",
    ///         vec![FileCopyFlag::NoReplace]
    ///     ).unwrap();
    /// progress.wait_for_completion(-1).unwrap();
    /// guest_session.close().unwrap();
    pub fn file_copy_from_guest(
        &self,
        source: &str,
        destination: &str,
        flags: Vec<FileCopyFlag>,
    ) -> Result<Progress, VboxError> {
        let mut flags: Vec<u32> = flags.iter().map(|f| (*f).into()).collect();
        let source_ptr = string_to_c_u64_str(source)?;
        let destination_ptr = string_to_c_u64_str(destination)?;
        let flags_size = flags.len() as u32;
        let flags_ptr = flags.as_mut_ptr();
        let progress = get_function_result_pointer!(
            self.object,
            FileCopyFromGuest,
            *mut IProgress,
            source_ptr,
            destination_ptr,
            flags_size,
            flags_ptr
        )?;
        Ok(Progress::new(progress))
    }

    ///  Copies a file from the host to the guest.
    ///
    ///
    /// # Arguments
    ///
    /// * `source` - &str. Path to the file on the host side that should be copied to the guest. Host path style.
    /// * `destination` - &str. Where to put the file in the guest (file, not directory). Guest style path.
    /// * `flags` - [`Vec<FileCopyFlag>`]. Zero or more [`FileCopyFlag`] values.
    ///
    /// # Returns
    ///
    /// Returns [`Progress`]  on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{Session, VirtualBox};
    /// use virtualbox_rs::enums::{FileCopyFlag, GuestSessionWaitForFlag, SessionType};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let mut session = Session::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    ///
    /// let console = session.get_console().unwrap();
    ///
    /// let guest = console.get_guest().unwrap();
    /// let guest_session = guest.create_session(
    ///     "user",
    ///     "pass",
    ///     "domain",
    ///     "s1").unwrap();
    /// guest_session.wait_for(GuestSessionWaitForFlag::Start, u32::MAX).unwrap();
    /// let progress = guest_session.file_copy_to_guest(
    ///         "/home/host_user/1.txt",
    ///         "/home/guest_user/1.txt",
    ///         vec![FileCopyFlag::NoReplace]
    ///     ).unwrap();
    /// progress.wait_for_completion(-1).unwrap();
    /// guest_session.close().unwrap();
    pub fn file_copy_to_guest(
        &self,
        source: &str,
        destination: &str,
        flags: Vec<FileCopyFlag>,
    ) -> Result<Progress, VboxError> {
        let mut flags: Vec<u32> = flags.iter().map(|f| (*f).into()).collect();
        let source_ptr = string_to_c_u64_str(source)?;
        let destination_ptr = string_to_c_u64_str(destination)?;
        let flags_size = flags.len() as u32;
        let flags_ptr = flags.as_mut_ptr();
        let progress = get_function_result_pointer!(
            self.object,
            FileCopyToGuest,
            *mut IProgress,
            source_ptr,
            destination_ptr,
            flags_size,
            flags_ptr
        )?;
        Ok(Progress::new(progress))
    }

    // TODO FileCreateTemp

    ///  Checks whether a regular file exists in the guest or not.
    ///
    ///
    /// # Arguments
    ///
    /// * `path` - &str. Path to the alleged regular file. Guest path style.
    /// * `follow_sym_links` - bool. If true, symbolic links in the final component will be followed and the existance of the symlink target made the question for this method. If false, a symbolic link in the final component will make the method return false (because a symlink isn't a regular file).
    ///
    /// # Returns
    ///
    /// Returns bool  on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{Session, VirtualBox};
    /// use virtualbox_rs::enums::{GuestSessionWaitForFlag, SessionType};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let mut session = Session::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    ///
    /// let console = session.get_console().unwrap();
    ///
    /// let guest = console.get_guest().unwrap();
    /// let guest_session = guest.create_session(
    ///     "user",
    ///     "pass",
    ///     "domain",
    ///     "s1").unwrap();
    /// guest_session.wait_for(GuestSessionWaitForFlag::Start, u32::MAX).unwrap();
    /// let exists = guest_session.file_exists(
    ///         "/home/host_user/1.txt",
    ///         false
    ///     ).unwrap();
    /// guest_session.close().unwrap();
    pub fn file_exists(&self, path: &str, follow_sym_links: bool) -> Result<bool, VboxError> {
        let path_ptr = string_to_c_u64_str(path)?;
        let follow_sym_links = if follow_sym_links { 1 } else { 0 };
        get_function_result_bool!(self.object, FileExists, path_ptr, follow_sym_links)
    }
    // TODO FileOpen
    // TODO FileOpenEx

    ///  Queries the size of a regular file in the guest.
    ///
    ///
    /// # Arguments
    ///
    /// * `path` - &str. Path to the file which size is requested. Guest path style.
    /// * `follow_sym_links` - bool. If true, symbolic links in the final component will be followed and the existance of the symlink target made the question for this method. If false, a symbolic link in the final component will make the method return false (because a symlink isn't a regular file).
    ///
    /// # Returns
    ///
    /// Returns i64  on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{Session, VirtualBox};
    /// use virtualbox_rs::enums::{GuestSessionWaitForFlag, SessionType};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let mut session = Session::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    ///
    /// let console = session.get_console().unwrap();
    ///
    /// let guest = console.get_guest().unwrap();
    /// let guest_session = guest.create_session(
    ///     "user",
    ///     "pass",
    ///     "domain",
    ///     "s1").unwrap();
    /// guest_session.wait_for(GuestSessionWaitForFlag::Start, u32::MAX).unwrap();
    /// let size = guest_session.file_query_size(
    ///         "/home/host_user/1.txt",
    ///         false
    ///     ).unwrap();
    /// guest_session.close().unwrap();
    pub fn file_query_size(&self, path: &str, follow_sym_links: bool) -> Result<i64, VboxError> {
        let path_ptr = string_to_c_u64_str(path)?;
        let follow_sym_links = if follow_sym_links { 1 } else { 0 };
        get_function_result_number!(self.object, FileQuerySize, i64, path_ptr, follow_sym_links)
    }

    ///  Checks whether a file system object (file, directory, etc) exists in the guest or not.
    ///
    ///
    /// # Arguments
    ///
    /// * `path` - &str. Path to the file system object to check the existance of. Guest path style.
    /// * `follow_sym_links` - bool. If true, symbolic links in the final component will be followed and the existance of the symlink target made the question for this method. If false, a symbolic link in the final component will make the method return false (because a symlink isn't a regular file).
    ///
    /// # Returns
    ///
    /// Returns bool  on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{Session, VirtualBox};
    /// use virtualbox_rs::enums::{GuestSessionWaitForFlag, SessionType};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let mut session = Session::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    ///
    /// let console = session.get_console().unwrap();
    ///
    /// let guest = console.get_guest().unwrap();
    /// let guest_session = guest.create_session(
    ///     "user",
    ///     "pass",
    ///     "domain",
    ///     "s1").unwrap();
    /// guest_session.wait_for(GuestSessionWaitForFlag::Start, u32::MAX).unwrap();
    /// let exists = guest_session.fs_obj_exists(
    ///         "/home/host_user/1.txt",
    ///         false
    ///     ).unwrap();
    /// guest_session.close().unwrap();
    pub fn fs_obj_exists(&self, path: &str, follow_sym_links: bool) -> Result<bool, VboxError> {
        let path_ptr = string_to_c_u64_str(path)?;
        let follow_sym_links = if follow_sym_links { 1 } else { 0 };
        get_function_result_bool!(self.object, FsObjExists, path_ptr, follow_sym_links)
    }

    // TODO FsObjQueryInfo

    ///  Removes a file system object (file, symlink, etc) in the guest.
    ///
    /// Will not work on directories, use [`GuestSession::directory_remove`] to remove directories.
    ///
    ///
    /// # Arguments
    ///
    /// * `path` - &str. Path to the file system object to remove. Guest style path.
    ///
    /// # Returns
    ///
    /// Returns ()  on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{Session, VirtualBox};
    /// use virtualbox_rs::enums::{GuestSessionWaitForFlag, SessionType};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let mut session = Session::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    ///
    /// let console = session.get_console().unwrap();
    ///
    /// let guest = console.get_guest().unwrap();
    /// let guest_session = guest.create_session(
    ///     "user",
    ///     "pass",
    ///     "domain",
    ///     "s1").unwrap();
    /// guest_session.wait_for(GuestSessionWaitForFlag::Start, u32::MAX).unwrap();
    /// guest_session.fs_obj_remove(
    ///         "/home/host_user/1.txt"
    ///     ).unwrap();
    /// guest_session.close().unwrap();
    pub fn fs_obj_remove(&self, path: &str) -> Result<(), VboxError> {
        let path_ptr = string_to_c_u64_str(path)?;
        get_function_result_unit!(self.object, FsObjRemove, path_ptr)
    }

    ///  Removes multiple file system objects (files, directories, symlinks, etc) in the guest.
    ///
    /// # Arguments
    ///
    /// * `path` - Vec<&str>. Array of paths to the file system objects to remove. Guest style path.
    ///
    /// # Returns
    ///
    /// Returns [`Progress`] on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{Session, VirtualBox};
    /// use virtualbox_rs::enums::{GuestSessionWaitForFlag, SessionType};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let mut session = Session::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    ///
    /// let console = session.get_console().unwrap();
    ///
    /// let guest = console.get_guest().unwrap();
    /// let guest_session = guest.create_session(
    ///     "user",
    ///     "pass",
    ///     "domain",
    ///     "s1").unwrap();
    /// guest_session.wait_for(GuestSessionWaitForFlag::Start, u32::MAX).unwrap();
    /// let progress = guest_session.fs_obj_remove_array(
    ///         vec![
    ///                 "/home/host_user/1.txt",
    ///                 "/home/host_user/2.txt"
    ///             ]
    ///     ).unwrap();
    /// progress.wait_for_completion(-1).unwrap();
    /// guest_session.close().unwrap();
    pub fn fs_obj_remove_array(&self, path: Vec<&str>) -> Result<Progress, VboxError> {
        let (path_size, path_ptr) = str_vec_to_ptr(path)?;
        let progress = get_function_result_pointer!(
            self.object,
            FsObjRemoveArray,
            *mut IProgress,
            path_size,
            path_ptr
        )?;
        Ok(Progress::new(progress))
    }

    ///  Renames a file system object (file, directory, symlink, etc) in the guest.
    ///
    /// # Arguments
    ///
    /// * `old_path` - &str. The current path to the object. Guest path style.
    /// * `new_path` - &str. The new path to the object. Guest path style.
    /// * `flags` - [`Vec<FsObjRenameFlag>`]. Zero or more [`FsObjRenameFlag`] values.
    ///
    /// # Returns
    ///
    /// Returns ()  on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{Session, VirtualBox};
    /// use virtualbox_rs::enums::{GuestSessionWaitForFlag, SessionType};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let mut session = Session::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    ///
    /// let console = session.get_console().unwrap();
    ///
    /// let guest = console.get_guest().unwrap();
    /// let guest_session = guest.create_session(
    ///     "user",
    ///     "pass",
    ///     "domain",
    ///     "s1").unwrap();
    /// guest_session.wait_for(GuestSessionWaitForFlag::Start, u32::MAX).unwrap();
    /// guest_session.fs_obj_rename(
    ///         "/home/host_user/old.txt",
    ///         "/home/host_user/new.txt",
    ///         vec![]
    ///     ).unwrap();
    /// guest_session.close().unwrap();
    pub fn fs_obj_rename(
        &self,
        old_path: &str,
        new_path: &str,
        flags: Vec<FsObjRenameFlag>,
    ) -> Result<(), VboxError> {
        let mut flags: Vec<u32> = flags.iter().map(|f| (*f).into()).collect();
        let old_path_ptr = string_to_c_u64_str(old_path)?;
        let new_path_ptr = string_to_c_u64_str(new_path)?;
        let flags_size = flags.len() as u32;
        let flags_ptr = flags.as_mut_ptr();
        get_function_result_unit!(
            self.object,
            FsObjRename,
            old_path_ptr,
            new_path_ptr,
            flags_size,
            flags_ptr
        )
    }

    ///  Moves a file system object (file, directory, symlink, etc) from one guest location to another.
    ///
    /// This differs from [`GuestSession::fs_obj_rename`] in that it can move accross file system boundraries. In that case it will perform a copy and then delete the original. For directories, this can take a while and is subject to races.
    ///
    /// # Arguments
    ///
    /// * `source` - &str. Path to the file to move. Guest path style.
    /// * `destination` - &str. Where to move the file to (file, not directory). Guest path style.
    /// * `flags` - [`Vec<FsObjMoveFlag>`]. Zero or more [`FsObjMoveFlag`] values.
    ///
    /// # Returns
    ///
    /// Returns [`Progress`] on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{Session, VirtualBox};
    /// use virtualbox_rs::enums::{GuestSessionWaitForFlag, SessionType};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let mut session = Session::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    ///
    /// let console = session.get_console().unwrap();
    ///
    /// let guest = console.get_guest().unwrap();
    /// let guest_session = guest.create_session(
    ///     "user",
    ///     "pass",
    ///     "domain",
    ///     "s1").unwrap();
    /// guest_session.wait_for(GuestSessionWaitForFlag::Start, u32::MAX).unwrap();
    /// guest_session.fs_obj_move(
    ///         "/home/host_user/old.txt",
    ///         "/home/host_user/new.txt",
    ///         vec![]
    ///     ).unwrap();
    /// guest_session.close().unwrap();
    pub fn fs_obj_move(
        &self,
        source: &str,
        destination: &str,
        flags: Vec<FsObjMoveFlag>,
    ) -> Result<Progress, VboxError> {
        let mut flags: Vec<u32> = flags.iter().map(|f| (*f).into()).collect();
        let source_ptr = string_to_c_u64_str(source)?;
        let destination_ptr = string_to_c_u64_str(destination)?;
        let flags_size = flags.len() as u32;
        let flags_ptr = flags.as_mut_ptr();
        let progress = get_function_result_pointer!(
            self.object,
            FsObjMove,
            *mut IProgress,
            source_ptr,
            destination_ptr,
            flags_size,
            flags_ptr
        )?;
        Ok(Progress::new(progress))
    }

    /// Moves file system objects (files, directories, symlinks, etc) from one guest location to another.
    ///
    /// # Arguments
    ///
    /// * `source` - Vec<&str>. Array of paths to the file system objects to move. Guest style path.
    /// * `destination` - &str. Where to move the file system objects to (directory). Guest path style.
    /// * `flags` - [`Vec<FsObjMoveFlag>`]. Zero or more [`FsObjMoveFlag`] values.
    ///
    /// # Returns
    ///
    /// Returns [`Progress`] on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{Session, VirtualBox};
    /// use virtualbox_rs::enums::{GuestSessionWaitForFlag, SessionType};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let mut session = Session::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    ///
    /// let console = session.get_console().unwrap();
    ///
    /// let guest = console.get_guest().unwrap();
    /// let guest_session = guest.create_session(
    ///     "user",
    ///     "pass",
    ///     "domain",
    ///     "s1").unwrap();
    /// guest_session.wait_for(GuestSessionWaitForFlag::Start, u32::MAX).unwrap();
    /// guest_session.fs_obj_move_array(
    ///         vec!["/home/host_user/old.txt"],
    ///         "/home/host_user/dir/",
    ///         vec![]
    ///     ).unwrap();
    /// guest_session.close().unwrap();
    pub fn fs_obj_move_array(
        &self,
        sources: Vec<&str>,
        destination: &str,
        flags: Vec<FsObjMoveFlag>,
    ) -> Result<Progress, VboxError> {
        let mut flags: Vec<u32> = flags.iter().map(|f| (*f).into()).collect();
        let (sources_size, sources_ptr) = str_vec_to_ptr(sources)?;
        let flags_size = flags.len() as u32;
        let flags_ptr = flags.as_mut_ptr();
        let destination_ptr = string_to_c_u64_str(destination)?;
        let progress = get_function_result_pointer!(
            self.object,
            FsObjMoveArray,
            *mut IProgress,
            sources_size,
            sources_ptr,
            destination_ptr,
            flags_size,
            flags_ptr
        )?;
        Ok(Progress::new(progress))
    }

    /// Copies file system objects (files, directories, symlinks, etc) from one guest location to another.
    ///
    /// # Arguments
    ///
    /// * `source` - Vec<&str>. Array of paths to the file system objects to copy. Guest style path.
    /// * `destination` - &str. Where to copy the file system objects to (directory). Guest path style.
    /// * `flags` - [`Vec<FileCopyFlag>`]. 	Zero or more [`FileCopyFlag`] values.
    ///
    /// # Returns
    ///
    /// Returns [`Progress`] on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{Session, VirtualBox};
    /// use virtualbox_rs::enums::{GuestSessionWaitForFlag, SessionType};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let mut session = Session::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    ///
    /// let console = session.get_console().unwrap();
    ///
    /// let guest = console.get_guest().unwrap();
    /// let guest_session = guest.create_session(
    ///     "user",
    ///     "pass",
    ///     "domain",
    ///     "s1").unwrap();
    /// guest_session.wait_for(GuestSessionWaitForFlag::Start, u32::MAX).unwrap();
    /// guest_session.fs_obj_copy_array(
    ///         vec!["/home/host_user/old.txt"],
    ///         "/home/host_user/dir/",
    ///         vec![]
    ///     ).unwrap();
    /// guest_session.close().unwrap();
    pub fn fs_obj_copy_array(
        &self,
        sources: Vec<&str>,
        destination: &str,
        flags: Vec<FileCopyFlag>,
    ) -> Result<Progress, VboxError> {
        let mut flags: Vec<u32> = flags.iter().map(|f| (*f).into()).collect();
        let (sources_size, sources_ptr) = str_vec_to_ptr(sources)?;
        let flags_size = flags.len() as u32;
        let flags_ptr = flags.as_mut_ptr();
        let destination_ptr = string_to_c_u64_str(destination)?;
        let progress = get_function_result_pointer!(
            self.object,
            FsObjCopyArray,
            *mut IProgress,
            sources_size,
            sources_ptr,
            destination_ptr,
            flags_size,
            flags_ptr
        )?;
        Ok(Progress::new(progress))
    }

    /// Sets the access control list (ACL) of a file system object (file, directory, etc) in the guest.
    ///
    /// # Arguments
    ///
    /// * `path` - &str. Full path of the file system object which ACL to set.
    /// * `follow_sym_links` - bool. If true symbolic links in the final component will be followed, otherwise, if false, the method will work directly on a symbolic link in the final component.
    /// * `acl` - &str. The ACL specification string. To-be-defined.
    /// * `mode` - u32. UNIX-style mode mask to use if acl is empty. As mention in [`GuestSession::directory_create`] this is realized on a best effort basis and the exact behavior depends on the Guest OS.
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
    /// use virtualbox_rs::enums::{GuestSessionWaitForFlag, SessionType};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let mut session = Session::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    ///
    /// let console = session.get_console().unwrap();
    ///
    /// let guest = console.get_guest().unwrap();
    /// let guest_session = guest.create_session(
    ///     "user",
    ///     "pass",
    ///     "domain",
    ///     "s1").unwrap();
    /// guest_session.wait_for(GuestSessionWaitForFlag::Start, u32::MAX).unwrap();
    /// guest_session.fs_obj_set_acl(
    ///         "/home/host_user/dir/",
    ///         false,
    ///         "",
    ///         777
    ///     ).unwrap();
    /// guest_session.close().unwrap();
    pub fn fs_obj_set_acl(
        &self,
        path: &str,
        follow_sym_links: bool,
        acl: &str,
        mode: u32,
    ) -> Result<(), VboxError> {
        let path_ptr = string_to_c_u64_str(path)?;
        let follow_sym_links = if follow_sym_links { 1 } else { 0 };
        let acl_ptr = string_to_c_u64_str(acl)?;
        get_function_result_unit!(
            self.object,
            FsObjSetACL,
            path_ptr,
            follow_sym_links,
            acl_ptr,
            mode
        )
    }

    #[cfg(not(is_v_6_1))]
    /// Returns the free space (in bytes) of a given path.
    ///
    /// # Arguments
    ///
    /// * `path` - &str. Full path to return the free space for.
    ///
    /// # Returns
    ///
    /// Returns i64 on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{Session, VirtualBox};
    /// use virtualbox_rs::enums::{GuestSessionWaitForFlag, SessionType};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let mut session = Session::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    ///
    /// let console = session.get_console().unwrap();
    ///
    /// let guest = console.get_guest().unwrap();
    /// let guest_session = guest.create_session(
    ///     "user",
    ///     "pass",
    ///     "domain",
    ///     "s1").unwrap();
    /// guest_session.wait_for(GuestSessionWaitForFlag::Start, u32::MAX).unwrap();
    /// let free_space = guest_session.fs_query_free_space(
    ///         "/home/host_user/"
    ///     ).unwrap();
    /// guest_session.close().unwrap();
    pub fn fs_query_free_space(&self, path: &str) -> Result<i64, VboxError> {
        let path_ptr = string_to_c_u64_str(path)?;
        get_function_result_number!(self.object, FsQueryFreeSpace, i64, path_ptr)
    }

    // TODO FsQueryInfo
    // TODO ProcessCreate
    // TODO ProcessCreateEx
    // TODO ProcessGet

    /// Returns file system information for a given path.
    ///
    /// # Arguments
    ///
    /// * `symlink` - &str. Path to the symbolic link that should be created. Guest path style.
    /// * `target` - &str. The path to the symbolic link target. If not an absolute, this will be relative to the symlink location at access time. Guest path style.
    /// * `type_` - [`SymlinkType`]. The symbolic link type (mainly for Windows). See [`SymlinkType`] for more information.
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
    /// use virtualbox_rs::enums::{GuestSessionWaitForFlag, SessionType, SymlinkType};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let mut session = Session::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    ///
    /// let console = session.get_console().unwrap();
    ///
    /// let guest = console.get_guest().unwrap();
    /// let guest_session = guest.create_session(
    ///     "user",
    ///     "pass",
    ///     "domain",
    ///     "s1").unwrap();
    /// guest_session.wait_for(GuestSessionWaitForFlag::Start, u32::MAX).unwrap();
    /// guest_session.symlink_create(
    ///         "/home/host_user/dir/1.txt",
    ///         "/home/host_user/dir/s_1.txt",
    ///         SymlinkType::File
    ///     ).unwrap();
    /// guest_session.close().unwrap();

    pub fn symlink_create(
        &self,
        symlink: &str,
        target: &str,
        type_: SymlinkType,
    ) -> Result<(), VboxError> {
        let type_ = type_.into();
        let symlink_ptr = string_to_c_u64_str(symlink)?;
        let target_ptr = string_to_c_u64_str(target)?;
        get_function_result_unit!(self.object, SymlinkCreate, symlink_ptr, target_ptr, type_)
    }

    /// Checks whether a symbolic link exists in the guest.
    ///
    /// # Arguments
    ///
    /// * `symlink` - &str. Path to the alleged symbolic link. Guest path style.
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
    /// use virtualbox_rs::enums::{GuestSessionWaitForFlag, SessionType};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let mut session = Session::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    ///
    /// let console = session.get_console().unwrap();
    ///
    /// let guest = console.get_guest().unwrap();
    /// let guest_session = guest.create_session(
    ///     "user",
    ///     "pass",
    ///     "domain",
    ///     "s1").unwrap();
    /// guest_session.wait_for(GuestSessionWaitForFlag::Start, u32::MAX).unwrap();
    /// let exists = guest_session.symlink_exists(
    ///         "/home/host_user/dir/1.txt"
    ///     ).unwrap();
    /// guest_session.close().unwrap();
    pub fn symlink_exists(&self, symlink: &str) -> Result<bool, VboxError> {
        let symlink_ptr = string_to_c_u64_str(symlink)?;
        get_function_result_bool!(self.object, SymlinkExists, symlink_ptr)
    }

    /// Reads the target value of a symbolic link in the guest.
    ///
    /// # Arguments
    ///
    /// * `symlink` - &str. Path to the symbolic link to read.
    /// * `flags` - [`Vec<SymlinkReadFlag>`]. Zero or more [`SymlinkReadFlag`] values.
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
    /// use virtualbox_rs::enums::{GuestSessionWaitForFlag, SessionType};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let mut session = Session::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    ///
    /// let console = session.get_console().unwrap();
    ///
    /// let guest = console.get_guest().unwrap();
    /// let guest_session = guest.create_session(
    ///     "user",
    ///     "pass",
    ///     "domain",
    ///     "s1").unwrap();
    /// guest_session.wait_for(GuestSessionWaitForFlag::Start, u32::MAX).unwrap();
    /// let symlink = guest_session.symlink_read(
    ///         "/home/host_user/dir/1.txt",
    ///         vec![]
    ///     ).unwrap();
    /// guest_session.close().unwrap();
    pub fn symlink_read(
        &self,
        symlink: &str,
        flags: Vec<SymlinkReadFlag>,
    ) -> Result<&'static str, VboxError> {
        let mut flags: Vec<u32> = flags.iter().map(|f| (*f).into()).collect();
        let symlink_ptr = string_to_c_u64_str(symlink)?;
        let flags_size = flags.len() as u32;
        let flags_ptr = flags.as_mut_ptr();
        get_function_result_str!(self.object, SymlinkRead, symlink_ptr, flags_size, flags_ptr)
    }

    /// Waits for event to happen.
    ///
    /// # Arguments
    ///
    /// * `wait_for_` - [`GuestSessionWaitForFlag`]. Specifies what to wait for; see [`GuestSessionWaitForFlag`] for more information.
    /// * `timeout_ms` - u32. Timeout (in ms) to wait for the operation to complete. Pass 0 for an infinite timeout..
    ///
    /// # Returns
    ///
    /// Returns [`GuestSessionWaitResult`] on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{Session, VirtualBox};
    /// use virtualbox_rs::enums::{GuestSessionWaitForFlag, SessionType};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let mut session = Session::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    ///
    /// let console = session.get_console().unwrap();
    ///
    /// let guest = console.get_guest().unwrap();
    /// let guest_session = guest.create_session(
    ///     "user",
    ///     "pass",
    ///     "domain",
    ///     "s1").unwrap();
    /// let reason = guest_session.wait_for(GuestSessionWaitForFlag::Start, 500).unwrap();
    /// guest_session.close().unwrap();
    pub fn wait_for(
        &self,
        wait_for_: GuestSessionWaitForFlag,
        timeout_ms: u32,
    ) -> Result<GuestSessionWaitResult, VboxError> {
        let wait_for_ = wait_for_.into();
        let reason = get_function_result_number!(self.object, WaitFor, u32, wait_for_, timeout_ms)?;
        Ok(GuestSessionWaitResult::from(reason))
    }

    /// Waits for one or more events to happen.
    ///
    /// Scriptable version of [`GuestSession::wait_for`].
    ///
    /// # Arguments
    ///
    /// * `wait_for_` - [`Vec<GuestSessionWaitForFlag>`]. Specifies what to wait for; see [`GuestSessionWaitForFlag`] for more information.
    /// * `timeout_ms` - u32. Timeout (in ms) to wait for the operation to complete. Pass 0 for an infinite timeout..
    ///
    /// # Returns
    ///
    /// Returns [`GuestSessionWaitResult`] on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{Session, VirtualBox};
    /// use virtualbox_rs::enums::{GuestSessionWaitForFlag, SessionType};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let mut session = Session::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    ///
    /// let console = session.get_console().unwrap();
    ///
    /// let guest = console.get_guest().unwrap();
    /// let guest_session = guest.create_session(
    ///     "user",
    ///     "pass",
    ///     "domain",
    ///     "s1").unwrap();
    /// let reason = guest_session.wait_for(GuestSessionWaitForFlag::Start, 500).unwrap();
    /// guest_session.close().unwrap();
    pub fn wait_for_array(
        &self,
        wait_for_: Vec<GuestSessionWaitForFlag>,
        timeout_ms: u32,
    ) -> Result<GuestSessionWaitResult, VboxError> {
        let mut wait_for_: Vec<u32> = wait_for_.iter().map(|f| (*f).into()).collect();
        let wait_for_size = wait_for_.len() as u32;
        let wait_for_ptr = wait_for_.as_mut_ptr();
        let reason = get_function_result_number!(
            self.object,
            WaitForArray,
            u32,
            wait_for_size,
            wait_for_ptr,
            timeout_ms
        )?;
        Ok(GuestSessionWaitResult::from(reason))
    }
}

#[cfg(is_v_6_1)]
impl GuestSession {
    /// Placeholder Method
    ///
    /// This method serves as a placeholder for versions of the API where the actual method is not available.
    /// Attempting to call this method will result in an `UnsupportedInCurrentApiVersion` error.
    /// This method is intended to ensure that the codebase can be compiled against multiple API versions
    /// without modification.
    ///
    /// Supported from API version: v7_0
    pub fn fs_query_free_space(&self, _path: &str) -> Result<i64, VboxError> {
        Err(VboxError::unsupported_in_current_api_version(
            "GuestSession::fs_query_free_space",
            "v7_0",
        ))
    }
}
