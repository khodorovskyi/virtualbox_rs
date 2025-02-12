use crate::enums::{
    AdditionsFacilityStatus, AdditionsFacilityType, AdditionsRunLevelType, AdditionsUpdateFlag,
    GuestShutdownFlag,
};
use crate::utility::macros::macros::{
    get_function_result_bool, get_function_result_number, get_function_result_pointer,
    get_function_result_str, get_function_result_unit,
};
use crate::utility::string_to_c_u64_str;
use crate::{EventSource, Guest, GuestInternalGetStatistics, GuestSession, Progress, VboxError};
use vbox_raw::sys_lib::{IEventSource, IGuestSession, IProgress, PRUint32};

impl Guest {
    /// Identifier of the Guest OS type as reported by the Guest Additions.
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
    /// let os_type_id = guest.get_os_type_id().unwrap();
    pub fn get_os_type_id(&self) -> Result<&'static str, VboxError> {
        get_function_result_str!(self.object, GetOSTypeId)
    }

    /// Current run level of the installed Guest Additions.
    ///
    /// # Returns
    ///
    /// Returns a [`AdditionsRunLevelType`] on success, or a [`VboxError`] on failure.
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
    /// let additions_run_level = guest.get_additions_run_level().unwrap();
    pub fn get_additions_run_level(&self) -> Result<AdditionsRunLevelType, VboxError> {
        let additions_run_level =
            get_function_result_number!(self.object, GetAdditionsRunLevel, u32)?;
        Ok(AdditionsRunLevelType::from(additions_run_level))
    }

    /// Version of the installed Guest Additions in the same format as IVirtualBox::version.
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
    /// let additions_version = guest.get_additions_version().unwrap();
    pub fn get_additions_version(&self) -> Result<&'static str, VboxError> {
        get_function_result_str!(self.object, GetAdditionsVersion)
    }

    /// The internal build revision number of the installed Guest Additions.
    ///
    /// # Returns
    ///
    /// Returns u32 on success, or a [`VboxError`] on failure.
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
    /// let additions_revision = guest.get_additions_revision().unwrap();
    pub fn get_additions_revision(&self) -> Result<u32, VboxError> {
        get_function_result_number!(self.object, GetAdditionsRevision, u32)
    }
    // TODO GetDnDTarget

    /// Event source for guest events.
    ///
    /// # Returns
    ///
    /// Returns [`EventSource`] class on success, or a [`VboxError`] on failure.
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
    /// let event_source = guest.get_event_source().unwrap();
    pub fn get_event_source(&self) -> Result<EventSource, VboxError> {
        let event_source =
            get_function_result_pointer!(self.object, GetEventSource, *mut IEventSource)?;
        Ok(EventSource::new(event_source))
    }

    /// Returns a collection of all opened guest sessions.
    ///
    /// # Returns
    ///
    /// Returns [`Vec<GuestSession>`]  on success, or a [`VboxError`] on failure.
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
    /// let guest_sessions = guest.get_sessions().unwrap();
    pub fn get_sessions(&self) -> Result<Vec<GuestSession>, VboxError> {
        let mut count = 0;
        let sessions_ptr = get_function_result_pointer!(
            self.object,
            GetSessions,
            *mut *mut IGuestSession,
            &mut count
        )?;
        let sessions_ptr_vec =
            unsafe { Vec::from_raw_parts(sessions_ptr, count as usize, count as usize) };

        let mut sessions = Vec::new();
        for session_ptr in sessions_ptr_vec {
            sessions.push(GuestSession::new(session_ptr));
        }
        Ok(sessions)
    }

    // TODO GetFacilities
    // TODO GetDnDSource

    /// Guest system memory balloon size in megabytes (transient property).
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
    /// let memory_balloon_size = guest.get_memory_balloon_size().unwrap();
    pub fn get_memory_balloon_size(&self) -> Result<u32, VboxError> {
        get_function_result_number!(self.object, GetMemoryBalloonSize, u32)
    }

    /// Guest system memory balloon size in megabytes (transient property).
    ///
    /// # Arguments
    ///
    /// * `memory_balloon_size` - u32
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
    /// guest.set_memory_balloon_size(16).unwrap();
    pub fn set_memory_balloon_size(&self, memory_balloon_size: u32) -> Result<(), VboxError> {
        get_function_result_unit!(self.object, SetMemoryBalloonSize, memory_balloon_size)
    }

    /// Interval to update guest statistics in seconds.
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
    /// let statistics_update_interval = guest.get_statistics_update_interval().unwrap();
    pub fn get_statistics_update_interval(&self) -> Result<u32, VboxError> {
        get_function_result_number!(self.object, GetStatisticsUpdateInterval, u32)
    }

    /// Interval to update guest statistics in seconds.
    ///
    /// # Arguments
    ///
    /// * `statistics_update_interval` - u32
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
    /// guest.set_statistics_update_interval(1).unwrap();
    pub fn set_statistics_update_interval(
        &self,
        statistics_update_interval: u32,
    ) -> Result<(), VboxError> {
        get_function_result_unit!(
            self.object,
            SetStatisticsUpdateInterval,
            statistics_update_interval
        )
    }

    /// <div class="warning">
    ///     Internal method; do not use as it might change at any time.
    /// </div>
    ///
    /// # Returns
    ///
    /// Returns [`GuestInternalGetStatistics`]  on success, or a [`VboxError`] on failure.
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
    /// let internal_get_statistics = guest.internal_get_statistics().unwrap();
    pub fn internal_get_statistics(&self) -> Result<GuestInternalGetStatistics, VboxError> {
        let mut cpu_user: u32 = 0;
        let mut cpu_kernel: u32 = 0;
        let mut cpu_idle: u32 = 0;
        let mut mem_total: u32 = 0;
        let mut mem_free: u32 = 0;
        let mut mem_balloon: u32 = 0;
        let mut mem_shared: u32 = 0;
        let mut mem_cache: u32 = 0;
        let mut paged_total: u32 = 0;
        let mut mem_alloc_total: u32 = 0;
        let mut mem_free_total: u32 = 0;
        let mut mem_balloon_total: u32 = 0;
        let mut mem_shared_total: u32 = 0;
        get_function_result_unit!(
            self.object,
            InternalGetStatistics,
            &mut cpu_user,
            &mut cpu_kernel,
            &mut cpu_idle,
            &mut mem_total,
            &mut mem_free,
            &mut mem_balloon,
            &mut mem_shared,
            &mut mem_cache,
            &mut paged_total,
            &mut mem_alloc_total,
            &mut mem_free_total,
            &mut mem_balloon_total,
            &mut mem_shared_total
        )?;
        Ok(GuestInternalGetStatistics {
            cpu_user,
            cpu_kernel,
            cpu_idle,
            mem_total,
            mem_free,
            mem_balloon,
            mem_shared,
            mem_cache,
            paged_total,
            mem_alloc_total,
            mem_free_total,
            mem_balloon_total,
            mem_shared_total,
        })
    }

    /// Get the current status of a Guest Additions facility.
    ///
    ///
    /// # Arguments
    ///
    /// * `facility` - AdditionsFacilityType. Facility to check status for.
    ///
    /// # Returns
    ///
    /// Returns (i64, [`AdditionsFacilityStatus`]) ([Timestamp (in ms) of last status update seen by the host.], [The current (latest) facility status.]) on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{Session, VirtualBox};
    /// use virtualbox_rs::enums::{AdditionsFacilityType, SessionType};
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
    /// let facility_status = guest.get_facility_status(AdditionsFacilityType::All).unwrap();
    pub fn get_facility_status(
        &self,
        facility: AdditionsFacilityType,
    ) -> Result<(i64, AdditionsFacilityStatus), VboxError> {
        let mut timestamp: i64 = 0;
        let mut status: u32 = 0;
        let facility = facility.into();
        get_function_result_unit!(
            self.object,
            GetFacilityStatus,
            facility,
            &mut timestamp,
            &mut status
        )?;
        let facility_status = AdditionsFacilityStatus::from(facility);
        Ok((timestamp, facility_status))
    }

    /// Retrieve the current status of a certain Guest Additions run level.
    ///
    ///
    /// # Arguments
    ///
    /// * `level` - AdditionsRunLevelType. Status level to check
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
    /// use virtualbox_rs::enums::{AdditionsRunLevelType, SessionType};
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
    /// let additions_status = guest.get_additions_status(AdditionsRunLevelType::Desktop).unwrap();

    pub fn get_additions_status(&self, level: AdditionsRunLevelType) -> Result<bool, VboxError> {
        let level = level.into();
        get_function_result_bool!(self.object, GetAdditionsStatus, level)
    }

    /// Store login credentials that can be queried by guest operating systems with Additions installed.
    ///
    /// The credentials are transient to the session and the guest may also choose to erase them. Note that the caller cannot determine whether the guest operating system has queried or made use of the credentials.    ///
    /// # Arguments
    ///
    /// * `user_name` -  &str. User name string, can be empty.
    /// * `password` -  &str. Password string, can be empty
    /// * `domain` -  &str. Domain name (guest logon scheme specific), can be empty
    /// * `allow_interactive_logon` - bool. Flag whether the guest should alternatively allow the user to interactively specify different credentials. This flag might not be supported by all versions of the Additions.
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
    /// guest.set_credentials(
    ///     "user",
    ///     "pass",
    ///     "domain",
    ///     true).unwrap();
    pub fn set_credentials(
        &self,
        user_name: &str,
        password: &str,
        domain: &str,
        allow_interactive_logon: bool,
    ) -> Result<(), VboxError> {
        let user_name_ptr = string_to_c_u64_str(user_name)?;
        let password_ptr = string_to_c_u64_str(password)?;
        let domain_ptr = string_to_c_u64_str(domain)?;
        let allow_interactive_logon = if allow_interactive_logon { 1 } else { 0 };
        get_function_result_unit!(
            self.object,
            SetCredentials,
            user_name_ptr,
            password_ptr,
            domain_ptr,
            allow_interactive_logon
        )
    }

    /// Creates a new guest session for controlling the guest.
    ///
    /// The new session will be started asynchronously, meaning on return of this function it is not guaranteed that the guest session is in a started and/or usable state. To wait for successful startup, use the [`GuestSession::wait_for`] call.
    ///
    /// A guest session represents one impersonated user account in the guest, so every operation will use the same credentials specified when creating the session object via [`Guest::create_session`]. Anonymous sessions, that is, sessions without specifying a valid user account in the guest are not allowed reasons of security.
    ///
    /// There can be a maximum of 32 sessions at once per VM. An error will be returned if this has been reached.
    ///
    /// For more information please consult [`GuestSession`]
    ///
    /// # Arguments
    ///
    /// * `user_name` -  &str. User name this session will be using to control the guest; has to exist and have the appropriate rights to execute programs in the VM. Must not be empty.
    /// * `password` -  &str. Password of the user account to be used. Empty passwords are allowed.
    /// * `domain` -  &str. Domain name of the user account to be used if the guest is part of a domain. Optional. This feature is not implemented yet.
    /// * `session_name` - &str. The session's friendly name. Optional, can be empty.
    ///
    /// # Returns
    ///
    /// Returns GuestSession  on success, or a [`VboxError`] on failure.
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
    /// let session = guest.create_session(
    ///     "user",
    ///     "pass",
    ///     "domain",
    ///     "s1").unwrap();
    pub fn create_session(
        &self,
        user_name: &str,
        password: &str,
        domain: &str,
        session_name: &str,
    ) -> Result<GuestSession, VboxError> {
        let user_name_ptr = string_to_c_u64_str(user_name)?;
        let password_ptr = string_to_c_u64_str(password)?;
        let domain_ptr = string_to_c_u64_str(domain)?;
        let session_name_ptr = string_to_c_u64_str(session_name)?;

        let session = get_function_result_pointer!(
            self.object,
            CreateSession,
            *mut IGuestSession,
            user_name_ptr,
            password_ptr,
            domain_ptr,
            session_name_ptr
        )?;
        Ok(GuestSession::new(session))
    }
    /// Finds guest sessions by their friendly name and returns an interface array with all found guest sessions.
    ///
    /// # Arguments
    ///
    /// * `session_name` -  &str. The session's friendly name to find. Wildcards like ? and * are allowed.
    ///
    /// # Returns
    ///
    /// Returns [`Vec<GuestSession>`]  on success, or a [`VboxError`] on failure.
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
    /// let session = guest.create_session(
    ///     "user",
    ///     "pass",
    ///     "domain",
    ///     "s1").unwrap();
    pub fn find_session(&self, session_name: &str) -> Result<Vec<GuestSession>, VboxError> {
        let session_name_ptr = string_to_c_u64_str(session_name)?;
        let mut count = 0;
        let sessions_ptr = get_function_result_pointer!(
            self.object,
            FindSession,
            *mut *mut IGuestSession,
            session_name_ptr,
            &mut count
        )?;
        let sessions_ptr_vec =
            unsafe { Vec::from_raw_parts(sessions_ptr, count as usize, count as usize) };

        let mut sessions = Vec::new();
        for session_ptr in sessions_ptr_vec {
            sessions.push(GuestSession::new(session_ptr));
        }
        Ok(sessions)
    }

    #[cfg(not(is_v_6_1))]
    /// Shuts down (and optionally halts and/or reboots) the guest.
    ///
    /// Needs supported Guest Additions installed.
    ///
    /// # Returns
    ///
    /// Returns bool  on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use std::vec;
    /// use virtualbox_rs::{ Session, VirtualBox};
    /// use virtualbox_rs::enums::{GuestShutdownFlag, SessionType};
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
    /// guest.shutdown(vec![GuestShutdownFlag::PowerOff]).unwrap();
    pub fn shutdown(&self, flags: Vec<GuestShutdownFlag>) -> Result<(), VboxError> {
        let mut flags: Vec<u32> = flags.iter().map(|flag| (*flag).into()).collect();
        let flags_ptr = flags.as_mut_ptr();
        let flags_size = flags.len() as PRUint32;
        get_function_result_unit!(self.object, Shutdown, flags_size, flags_ptr)
    }

    ///Automatically updates already installed Guest Additions in a VM.
    ///
    /// At the moment only Windows and Linux guests are supported.
    ///
    /// Because the VirtualBox Guest Additions drivers are not WHQL-certified yet there might be warning dialogs during the actual Guest Additions update. These need to be confirmed manually in order to continue the installation process. This applies to Windows 2000 and Windows XP guests and therefore these guests can't be updated in a fully automated fashion without user interaction. However, to start a Guest Additions update for the mentioned Windows versions anyway, the flag [`AdditionsUpdateFlag::WaitForUpdateStartOnly`] can be specified. See [`AdditionsUpdateFlag`] for more information.
    ///
    /// The guest needs to be restarted in order to make use of the updated Guest Additions.
    ///
    ///
    /// # Arguments
    ///
    /// * `source` -  &str.
    /// * `arguments` -  Vec<&str>.
    /// * `flags` -  [`Vec<AdditionsUpdateFlag>`]
    ///
    /// # Returns
    ///
    /// Progress object to track the operation completion, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
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
    /// let progress = guest.update_guest_additions(
    ///     "https://example.com",
    ///     Vec::new(),
    ///     Vec::new()
    /// ).unwrap();

    pub fn update_guest_additions(
        &self,
        source: &str,
        arguments: Vec<&str>,
        flags: Vec<AdditionsUpdateFlag>,
    ) -> Result<Progress, VboxError> {
        let source_ptr = string_to_c_u64_str(source)?;

        let mut arguments_ptr_vec = Vec::new();
        for argument in &arguments {
            arguments_ptr_vec.push(string_to_c_u64_str(argument)?);
        }
        let arguments_ptr = arguments_ptr_vec.as_mut_ptr();
        let arguments_size = arguments.len() as PRUint32;

        let mut flags: Vec<u32> = flags.iter().map(|flag| (*flag).into()).collect();
        let flags_ptr = flags.as_mut_ptr();
        let flags_size = flags.len() as PRUint32;

        let progress = get_function_result_pointer!(
            self.object,
            UpdateGuestAdditions,
            *mut IProgress,
            source_ptr,
            arguments_size,
            arguments_ptr,
            flags_size,
            flags_ptr
        )?;
        Ok(Progress::new(progress))
    }
}

#[cfg(is_v_6_1)]
impl Guest {
    /// Placeholder Method
    ///
    /// This method serves as a placeholder for versions of the API where the actual method is not available.
    /// Attempting to call this method will result in an `UnsupportedInCurrentApiVersion` error.
    /// This method is intended to ensure that the codebase can be compiled against multiple API versions
    /// without modification.
    ///
    /// Supported from API version: v7_0
    pub fn shutdown(&self, _flags: Vec<GuestShutdownFlag>) -> Result<(), VboxError> {
        Err(VboxError::unsupported_in_current_api_version(
            "Guest::shutdown",
            "v7_0",
        ))
    }
}
