use crate::bandwidth_control::BandwidthControl;
use crate::enums::BandwidthGroupType;
use crate::utility::macros::macros::{
    get_function_result_number, get_function_result_pointer, get_function_result_unit,
};
use crate::utility::string_to_c_u64_str;
use crate::{BandwidthGroup, VboxError};
use vbox_raw::sys_lib::IBandwidthGroup;

impl BandwidthControl {
    /// The current number of existing bandwidth groups managed.
    ///
    /// # Returns
    ///
    /// Returns u32 on success, or a [`VboxError`] on failure.
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
    /// let bandwidth_control = machine.get_bandwidth_control().unwrap();
    /// let num_groups = bandwidth_control.get_num_groups().unwrap();
    pub fn get_num_groups(&self) -> Result<u32, VboxError> {
        get_function_result_number!(self.object, GetNumGroups, u32)
    }

    /// Creates a new bandwidth group.
    ///
    ///
    /// # Arguments
    ///
    /// * `name` - &str. Name of the bandwidth group.
    /// * `group_type` - [`BandwidthGroupType`]. The type of the bandwidth group (network or disk).
    /// * `max_bytes_per_sec` - i64. The maximum number of bytes which can be transfered by all entities attached to this group during one second.
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
    /// use virtualbox_rs::enums::{BandwidthGroupType, SessionType};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let mut session = Session::init().unwrap();
    /// machine.lock_machine(&mut  session, SessionType::Shared ).unwrap();
    /// let machine_mut = session.get_machine().unwrap();
    /// let bandwidth_control = machine_mut.get_bandwidth_control().unwrap();
    /// bandwidth_control.create_bandwidth_group(
    ///     "g1",
    ///     BandwidthGroupType::Network,
    ///     1000000
    /// ).unwrap();
    /// machine_mut.save_settings().unwrap();
    pub fn create_bandwidth_group(
        &self,
        name: &str,
        group_type: BandwidthGroupType,
        max_bytes_per_sec: i64,
    ) -> Result<(), VboxError> {
        let name_ptr = string_to_c_u64_str(name)?;
        let group_type: u32 = group_type.into();
        get_function_result_unit!(
            self.object,
            CreateBandwidthGroup,
            name_ptr,
            group_type,
            max_bytes_per_sec
        )
    }

    /// Deletes a new bandwidth group.
    ///
    ///
    /// # Arguments
    ///
    /// * `name` - &str. Name of the bandwidth group.
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
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let mut session = Session::init().unwrap();
    /// machine.lock_machine(&mut  session, SessionType::Shared ).unwrap();
    /// let machine_mut = session.get_machine().unwrap();
    /// let bandwidth_control = machine_mut.get_bandwidth_control().unwrap();
    /// bandwidth_control.delete_bandwidth_group("g1").unwrap();
    /// machine_mut.save_settings().unwrap();
    pub fn delete_bandwidth_group(&self, name: &str) -> Result<(), VboxError> {
        let name_ptr = string_to_c_u64_str(name)?;
        get_function_result_unit!(self.object, DeleteBandwidthGroup, name_ptr)
    }

    /// Get a bandwidth group by name.
    ///
    ///
    /// # Arguments
    ///
    /// * `name` - &str. Name of the bandwidth group.
    ///
    /// # Returns
    ///
    /// Returns [`BandwidthGroup`] on success, or a [`VboxError`] on failure.
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

    pub fn get_bandwidth_group(&self, name: &str) -> Result<BandwidthGroup, VboxError> {
        let name_ptr = string_to_c_u64_str(name)?;
        let bandwidth_group = get_function_result_pointer!(
            self.object,
            GetBandwidthGroup,
            *mut IBandwidthGroup,
            name_ptr
        )?;
        Ok(BandwidthGroup::new(bandwidth_group))
    }
    /// Get all managed bandwidth groups.
    ///
    /// # Returns
    ///
    /// Returns [`Vec<BandwidthGroup>`] on success, or a [`VboxError`] on failure.
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
    /// let all_bandwidth_groups = bandwidth_control.get_all_bandwidth_groups().unwrap();
    pub fn get_all_bandwidth_groups(&self) -> Result<Vec<BandwidthGroup>, VboxError> {
        let mut count = 0;
        let bandwidth_groups_ptr = get_function_result_pointer!(
            self.object,
            GetAllBandwidthGroups,
            *mut *mut IBandwidthGroup,
            &mut count
        )?;
        let vec_bandwidth_groups_ptr =
            unsafe { Vec::from_raw_parts(bandwidth_groups_ptr, count as usize, count as usize) };
        let mut bandwidth_groups = Vec::new();
        for bandwidth_group_ptr in vec_bandwidth_groups_ptr {
            bandwidth_groups.push(BandwidthGroup::new(bandwidth_group_ptr.clone()))
        }

        Ok(bandwidth_groups)
    }
}
