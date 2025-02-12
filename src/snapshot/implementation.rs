use crate::snapshot::Snapshot;
use crate::utility::macros::macros::{
    get_function_result_bool, get_function_result_number, get_function_result_pointer,
    get_function_result_pointer_vec, get_function_result_str, get_function_result_unit,
};
use crate::utility::string_to_c_u64_str;
use crate::{Machine, VboxError, VboxErrorType};
use vbox_raw::sys_lib::{IMachine, ISnapshot};

impl Snapshot {
    /// UUID of the snapshot.
    ///
    /// # Returns
    ///
    /// Returns [&str] on success, or a [`VboxError`] on failure.
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
    /// let snapshot = machine.get_current_snapshot().unwrap().unwrap();
    ///
    /// let id = snapshot.get_id().unwrap();
    ///
    pub fn get_id(&self) -> Result<&'static str, VboxError> {
        get_function_result_str!(self.object, GetId)
    }

    /// Short name of the snapshot.
    ///
    /// # Returns
    ///
    /// Returns [&str] on success, or a [`VboxError`] on failure.
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
    /// let snapshot = machine.get_current_snapshot().unwrap().unwrap();
    ///
    /// let name = snapshot.get_name().unwrap();
    ///
    pub fn get_name(&self) -> Result<&'static str, VboxError> {
        get_function_result_str!(self.object, GetName)
    }

    /// Short name of the snapshot.
    ///
    /// # Arguments
    ///
    /// * `name` - &str. Short name of the snapshot.
    ///
    /// # Returns
    ///
    /// Returns () success, or a [`VboxError`] on failure.
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
    /// let snapshot = machine.get_current_snapshot().unwrap().unwrap();
    ///
    /// snapshot.set_name("Snapshot_1").unwrap();
    pub fn set_name(&self, name: &str) -> Result<(), VboxError> {
        let name_ptr = string_to_c_u64_str(name)?;
        get_function_result_unit!(self.object, SetName, name_ptr)
    }
    /// Optional description of the snapshot.
    ///
    /// # Returns
    ///
    /// Returns [&str] on success, or a [`VboxError`] on failure.
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
    /// let snapshot = machine.get_current_snapshot().unwrap().unwrap();
    ///
    /// let description = snapshot.get_description().unwrap();
    ///
    pub fn get_description(&self) -> Result<&'static str, VboxError> {
        get_function_result_str!(self.object, GetDescription)
    }

    /// Optional description of the snapshot.
    ///
    /// # Arguments
    ///
    /// * `description` - &str. Optional description of the snapshot.
    ///
    /// # Returns
    ///
    /// Returns () success, or a [`VboxError`] on failure.
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
    /// let snapshot = machine.get_current_snapshot().unwrap().unwrap();
    ///
    /// snapshot.set_description("description").unwrap();
    pub fn set_description(&self, name: &str) -> Result<(), VboxError> {
        let description = string_to_c_u64_str(name)?;
        get_function_result_unit!(self.object, SetDescription, description)
    }

    /// Timestamp of the snapshot, in milliseconds since 1970-01-01 UTC.
    ///
    /// # Returns
    ///
    /// Returns i64 success, or a [`VboxError`] on failure.
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
    /// let snapshot = machine.get_current_snapshot().unwrap().unwrap();
    ///
    /// let time_stamp = snapshot.get_time_stamp().unwrap();
    pub fn get_time_stamp(&self) -> Result<i64, VboxError> {
        get_function_result_number!(self.object, GetTimeStamp, i64)
    }

    /// true if this snapshot is an online snapshot and false otherwise.
    ///
    /// When this attribute is true, the [`Machine::get_state_file_path`] attribute of the machine object associated with this snapshot will point to the saved state file. Otherwise, it will be an empty string.
    ///
    /// # Returns
    ///
    /// Returns bool success, or a [`VboxError`] on failure.
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
    /// let snapshot = machine.get_current_snapshot().unwrap().unwrap();
    ///
    /// let time_stamp = snapshot.get_time_stamp().unwrap();
    pub fn get_online(&self) -> Result<bool, VboxError> {
        get_function_result_bool!(self.object, GetOnline)
    }

    /// Virtual machine this snapshot is taken on.
    ///
    /// This object stores all settings the machine had when taking this snapshot.
    ///
    /// <div class="warning">
    ///  The returned machine object is immutable, i.e. no any settings can be changed.
    /// </div>
    ///
    /// # Returns
    ///
    /// Returns [`Machine`] success, or a [`VboxError`] on failure.
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
    /// let snapshot = machine.get_current_snapshot().unwrap().unwrap();
    ///
    /// let machine = snapshot.get_machine().unwrap();
    pub fn get_machine(&self) -> Result<Machine, VboxError> {
        let machine = get_function_result_pointer!(self.object, GetMachine, *mut IMachine)?;
        Ok(Machine::new(machine))
    }

    /// Parent snapshot (a snapshot this one is based on), or None if the snapshot has no parent (i.e. is the first snapshot).
    ///
    /// # Returns
    ///
    /// Returns [`Option<Snapshot>`] success, or a [`VboxError`] on failure.
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
    /// let snapshot = machine.find_snapshot("Snapshot_2").unwrap();
    ///
    /// let machine = snapshot.get_machine().unwrap();
    pub fn get_parent(&self) -> Result<Option<Snapshot>, VboxError> {
        let parent = get_function_result_pointer!(self.object, GetParent, *mut ISnapshot);
        match parent {
            Ok(parent) => Ok(Some(Snapshot::new(parent))),
            Err(error) => {
                if error.error_type == VboxErrorType::NullPointerError {
                    Ok(None)
                } else {
                    Err(error)
                }
            }
        }
    }

    /// Child snapshots (all snapshots having this one as a parent).
    ///
    /// By inspecting this attribute starting with a machine's root snapshot (which can be obtained by calling [`Machine::find_snapshot`] with empty UUID), a machine's snapshots tree can be iterated over.
    ///
    /// # Returns
    ///
    /// Returns [`Vec<Snapshot>`] success, or a [`VboxError`] on failure.
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
    /// let snapshot = machine.find_snapshot("Snapshot_2").unwrap();
    ///
    /// let children = snapshot.get_children().unwrap();
    pub fn get_children(&self) -> Result<Vec<Snapshot>, VboxError> {
        let children = get_function_result_pointer_vec!(self.object, GetChildren, *mut ISnapshot)?;
        Ok(children
            .iter()
            .map(|object| Snapshot::new(object.clone()))
            .collect())
    }

    /// Returns the number of direct children of this snapshot.
    ///
    /// # Returns
    ///
    /// Returns [`Vec<Snapshot>`] success, or a [`VboxError`] on failure.
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
    /// let snapshot = machine.find_snapshot("Snapshot_2").unwrap();
    ///
    /// let count = snapshot.get_children_count().unwrap();
    pub fn get_children_count(&self) -> Result<u32, VboxError> {
        get_function_result_number!(self.object, GetChildrenCount, u32)
    }
}
