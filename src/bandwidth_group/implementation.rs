use crate::enums::BandwidthGroupType;
use crate::utility::macros::macros::{
    get_function_result_number, get_function_result_str, get_function_result_unit,
};
use crate::{BandwidthGroup, VboxError};

impl BandwidthGroup {
    /// Name of the group.
    ///
    /// # Returns
    ///
    /// Returns &str on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{VirtualBox};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let bandwidth_control = machine.get_bandwidth_control().unwrap();
    /// let bandwidth_group = bandwidth_control.get_bandwidth_group("g1").unwrap();
    /// let name = bandwidth_group.get_name().unwrap();
    pub fn get_name(&self) -> Result<&'static str, VboxError> {
        get_function_result_str!(self.object, GetName)
    }

    /// Type of the group.
    ///
    /// # Returns
    ///
    /// Returns &str on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{VirtualBox};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let bandwidth_control = machine.get_bandwidth_control().unwrap();
    /// let bandwidth_group = bandwidth_control.get_bandwidth_group("g1").unwrap();
    /// let group_type = bandwidth_group.get_type().unwrap();
    pub fn get_type(&self) -> Result<BandwidthGroupType, VboxError> {
        let group_type = get_function_result_number!(self.object, GetType, u32)?;
        Ok(BandwidthGroupType::from(group_type))
    }

    /// How many devices/medium attachments use this group.
    ///
    /// # Returns
    ///
    /// Returns u32 on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{VirtualBox};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let bandwidth_control = machine.get_bandwidth_control().unwrap();
    /// let bandwidth_group = bandwidth_control.get_bandwidth_group("g1").unwrap();
    /// let reference = bandwidth_group.get_reference().unwrap();
    pub fn get_reference(&self) -> Result<u32, VboxError> {
        get_function_result_number!(self.object, GetReference, u32)
    }

    /// The maximum number of bytes which can be transfered by all entities attached to this group during one second.
    ///
    /// # Returns
    ///
    /// Returns u32 on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{VirtualBox};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let bandwidth_control = machine.get_bandwidth_control().unwrap();
    /// let bandwidth_group = bandwidth_control.get_bandwidth_group("g1").unwrap();
    /// let max_bytes_per_sec = bandwidth_group.get_max_bytes_per_sec().unwrap();
    pub fn get_max_bytes_per_sec(&self) -> Result<i64, VboxError> {
        get_function_result_number!(self.object, GetMaxBytesPerSec, i64)
    }

    /// The maximum number of bytes which can be transfered by all entities attached to this group during one second.
    ///
    /// # Arguments
    ///
    /// * `max_bytes_per_sec` - u32.
    ///
    /// # Returns
    ///
    /// Returns () on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{VirtualBox};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let bandwidth_control = machine.get_bandwidth_control().unwrap();
    /// let bandwidth_group = bandwidth_control.get_bandwidth_group("g1").unwrap();
    /// bandwidth_group.set_max_bytes_per_sec(100000).unwrap();
    pub fn set_max_bytes_per_sec(&self, max_bytes_per_sec: i64) -> Result<(), VboxError> {
        get_function_result_unit!(self.object, SetMaxBytesPerSec, max_bytes_per_sec)
    }
}
