use crate::event_source::EventSource;
use crate::progress::Progress;
use crate::utility::macros::macros::{
    get_function_result_bool, get_function_result_number, get_function_result_pointer,
    get_function_result_str, get_function_result_unit,
};
use crate::virtualbox_error_info::VirtualBoxErrorInfo;
use crate::VboxError;
use log::debug;
use vbox_raw::sys_lib::{IEventSource, IVirtualBoxErrorInfo};

impl Progress {
    ///
    /// Waits until the task is done (including all sub-operations) with a given timeout in milliseconds; specify -1 for an indefinite wait.
    ///
    /// # Arguments
    ///
    /// * `timeout` - Maximum time in milliseconds to wait or -1 to wait indefinitely.
    ///
    /// # Returns
    ///
    /// Returns Ok(()) on success, or a `VboxError` on failure.
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
    ///
    /// let progress = machine_mut.save_state().unwrap();
    /// progress.wait_for_completion(-1).unwrap();

    pub fn wait_for_completion(&self, timeout: i32) -> Result<(), VboxError> {
        let timeout = if timeout < 0 { i32::MAX } else { timeout };
        get_function_result_unit!(self.object, WaitForCompletion, timeout)
    }

    /// ID of the task.
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
    /// let machine_mut = session.get_machine().unwrap();
    ///
    /// let progress = machine_mut.save_state().unwrap();
    /// progress.wait_for_completion(1).unwrap();
    /// let id = progress.get_id().unwrap();
    pub fn get_id(&self) -> Result<&'static str, VboxError> {
        get_function_result_str!(self.object, GetId)
    }

    /// Description of the task.
    ///
    /// # Returns
    ///
    /// Returns &str on success, or a `VboxError` on failure.
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
    ///
    /// let progress = machine_mut.save_state().unwrap();
    /// progress.wait_for_completion(1).unwrap();
    /// let description = progress.get_description().unwrap();
    pub fn get_description(&self) -> Result<&'static str, VboxError> {
        get_function_result_str!(self.object, GetDescription)
    }

    /// Whether the task can be interrupted.
    ///
    /// # Returns
    ///
    /// Returns bool on success, or a `VboxError` on failure.
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
    ///
    /// let progress = machine_mut.save_state().unwrap();
    /// progress.wait_for_completion(1).unwrap();
    /// let cancelable = progress.get_cancelable().unwrap();

    pub fn get_cancelable(&self) -> Result<bool, VboxError> {
        get_function_result_bool!(self.object, GetCancelable)
    }

    /// Current progress value of the task as a whole, in percent.
    ///
    /// # Returns
    ///
    /// Returns u32 on success, or a `VboxError` on failure.
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
    ///
    /// let progress = machine_mut.save_state().unwrap();
    /// progress.wait_for_completion(1).unwrap();
    /// let percent = progress.get_percent().unwrap();

    pub fn get_percent(&self) -> Result<u32, VboxError> {
        let percent = get_function_result_number!(self.object, GetPercent, u32)?;
        Ok(percent)
    }

    /// Estimated remaining time until the task completes, in seconds.
    ///
    /// # Returns
    ///
    /// Returns i32 on success, or a `VboxError` on failure.
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
    ///
    /// let progress = machine_mut.save_state().unwrap();
    /// progress.wait_for_completion(1).unwrap();
    /// let time_remaining = progress.get_time_remaining().unwrap();

    pub fn get_time_remaining(&self) -> Result<i32, VboxError> {
        let time_remaining = get_function_result_number!(self.object, GetTimeRemaining, i32)?;
        Ok(time_remaining)
    }

    /// Whether the task has been completed.
    ///
    /// # Returns
    ///
    /// Returns bool on success, or a `VboxError` on failure.
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
    ///
    /// let progress = machine_mut.save_state().unwrap();
    /// progress.wait_for_completion(1).unwrap();
    /// let completed = progress.get_completed().unwrap();

    pub fn get_completed(&self) -> Result<bool, VboxError> {
        get_function_result_bool!(self.object, GetCompleted)
    }
    /// Whether the task has been canceled.
    ///
    /// # Returns
    ///
    /// Returns bool on success, or a `VboxError` on failure.
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
    ///
    /// let progress = machine_mut.save_state().unwrap();
    /// progress.wait_for_completion(1).unwrap();
    /// let canceled = progress.get_canceled().unwrap();

    pub fn get_canceled(&self) -> Result<bool, VboxError> {
        get_function_result_bool!(self.object, GetCanceled)
    }

    /// Result code of the progress task.
    ///
    /// # Returns
    ///
    /// Returns i32 on success, or a `VboxError` on failure.
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
    ///
    /// let progress = machine_mut.save_state().unwrap();
    /// progress.wait_for_completion(-1).unwrap();
    /// let result_code = progress.get_result_code().unwrap();
    pub fn get_result_code(&self) -> Result<i32, VboxError> {
        let result_code = get_function_result_number!(self.object, GetResultCode, i32)?;
        Ok(result_code)
    }

    /// Extended information about the unsuccessful result of the progress operation.
    ///
    /// # Returns
    ///
    /// Returns `VirtualBoxErrorInfo` on success, or a `VboxError` on failure.
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
    ///
    /// let progress = machine_mut.save_state().unwrap();
    /// progress.wait_for_completion(-1).unwrap();
    /// let error_info = progress.get_error_info().unwrap();
    pub fn get_error_info(&self) -> Result<VirtualBoxErrorInfo, VboxError> {
        let virtual_box_error_info =
            get_function_result_pointer!(self.object, GetErrorInfo, *mut IVirtualBoxErrorInfo)?;
        Ok(VirtualBoxErrorInfo::new(virtual_box_error_info))
    }
    /// Number of sub-operations this task is divided into.
    ///
    /// # Returns
    ///
    /// Returns u32 on success, or a `VboxError` on failure.
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
    ///
    /// let progress = machine_mut.save_state().unwrap();
    /// progress.wait_for_completion(-1).unwrap();
    /// let operation_count = progress.get_operation_count().unwrap();
    pub fn get_operation_count(&self) -> Result<u32, VboxError> {
        let operation_count = get_function_result_number!(self.object, GetOperationCount, u32)?;
        Ok(operation_count)
    }

    /// Number of the sub-operation being currently executed.
    ///
    /// # Returns
    ///
    /// Returns u32 on success, or a `VboxError` on failure.
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
    ///
    /// let progress = machine_mut.save_state().unwrap();
    /// progress.wait_for_completion(-1).unwrap();
    /// let operation = progress.get_operation().unwrap();
    pub fn get_operation(&self) -> Result<u32, VboxError> {
        let operation = get_function_result_number!(self.object, GetOperation, u32)?;
        Ok(operation)
    }

    /// Description of the sub-operation being currently executed.
    ///
    /// # Returns
    ///
    /// Returns &str on success, or a `VboxError` on failure.
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
    ///
    /// let progress = machine_mut.save_state().unwrap();
    /// progress.wait_for_completion(-1).unwrap();
    /// let operation_description = progress.get_operation_description().unwrap();
    pub fn get_operation_description(&self) -> Result<&'static str, VboxError> {
        get_function_result_str!(self.object, GetOperationDescription)
    }

    /// Progress value of the current sub-operation only, in percent.
    ///
    /// # Returns
    ///
    /// Returns u32 on success, or a `VboxError` on failure.
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
    ///
    /// let progress = machine_mut.save_state().unwrap();
    /// progress.wait_for_completion(-1).unwrap();
    /// let operation_percent = progress.get_operation_percent().unwrap();

    pub fn get_operation_percent(&self) -> Result<u32, VboxError> {
        let operation_percent = get_function_result_number!(self.object, GetOperationPercent, u32)?;
        Ok(operation_percent)
    }

    /// Weight value of the current sub-operation only.
    ///
    /// # Returns
    ///
    /// Returns u32 on success, or a `VboxError` on failure.
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
    ///
    /// let progress = machine_mut.save_state().unwrap();
    /// progress.wait_for_completion(-1).unwrap();
    /// let operation_weight = progress.get_operation_weight().unwrap();

    pub fn get_operation_weight(&self) -> Result<u32, VboxError> {
        let operation_weight = get_function_result_number!(self.object, GetOperationWeight, u32)?;
        Ok(operation_weight)
    }
    /// When non-zero, this specifies the number of milliseconds after which the operation will automatically be canceled.
    ///
    /// # Returns
    ///
    /// Returns u32 on success, or a `VboxError` on failure.
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
    ///
    /// let progress = machine_mut.save_state().unwrap();
    /// progress.wait_for_completion(-1).unwrap();
    /// let timeout = progress.get_timeout().unwrap();

    pub fn get_timeout(&self) -> Result<u32, VboxError> {
        let timeout = get_function_result_number!(self.object, GetTimeout, u32)?;
        Ok(timeout)
    }

    /// EventSource
    ///
    /// # Returns
    ///
    /// Returns `EventSource` on success, or a `VboxError` on failure.
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
    ///
    /// let progress = machine_mut.save_state().unwrap();
    /// progress.wait_for_completion(-1).unwrap();
    /// let event_source = progress.get_event_source().unwrap();

    pub fn get_event_source(&self) -> Result<EventSource, VboxError> {
        let event_source =
            get_function_result_pointer!(self.object, GetEventSource, *mut IEventSource)?;
        Ok(EventSource::new(event_source))
    }
    ///
    /// Waits until the given operation is done with a given timeout in milliseconds; specify -1 for an indefinite wait.
    ///
    /// # Arguments
    ///
    /// * `timeout` - Maximum time in milliseconds to wait or -1 to wait indefinitely.
    ///
    ///
    /// * `operation` - Number of the operation to wait for. Must be less than operationCount.
    ///
    /// # Returns
    ///
    /// Returns () on success, or a `VboxError` on failure.
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
    ///
    /// let progress = machine_mut.save_state().unwrap();
    /// progress.wait_for_operation_completion(-1, 1).unwrap();

    pub fn wait_for_operation_completion(
        &self,
        timeout: i32,
        operation: u32,
    ) -> Result<(), VboxError> {
        let timeout = if timeout < 0 { i32::MAX } else { timeout };
        get_function_result_unit!(self.object, WaitForOperationCompletion, operation, timeout)
    }
    ///
    /// Cancels the task.
    ///
    /// # Returns
    ///
    /// Returns u32 on success, or a `VboxError` on failure.
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
    ///
    /// let progress = machine_mut.save_state().unwrap();
    /// progress.cancel().unwrap();

    pub fn cancel(&self) -> Result<(), VboxError> {
        get_function_result_unit!(self.object, Cancel)
    }
}
