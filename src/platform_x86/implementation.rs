use crate::{PlatformX86, VboxError};
use crate::enums::{CPUPropertyType, HWVirtExPropertyType};
use crate::utility::macros::macros::{get_function_result_bool, get_function_result_unit};

impl PlatformX86 {
    /// This attribute controls if High Precision Event Timer (HPET) is enabled in this VM.
    ///
    /// # Returns
    ///
    /// Returns bool on success, or a [`VboxError`] on failure.
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
    /// let platform = machine.get_platform().unwrap();
    /// let platform_x86 = platform.get_x86().unwrap();
    /// let hpet_enabled = platform_x86.get_hpet_enabled().unwrap();
    pub fn get_hpet_enabled(&self) -> Result<bool, VboxError> {
        get_function_result_bool!(self.object, GetHPETEnabled)
    }


    /// This attribute controls if High Precision Event Timer (HPET) is enabled in this VM.
    ///
    /// # Arguments
    ///
    /// * `hpet_enabled` -  This attribute controls if High Precision Event Timer (HPET) is enabled in this VM.
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
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    /// let machine_mut = session.get_machine().unwrap();
    /// let platform = machine_mut.get_platform().unwrap();
    /// let platform_x86 = platform.get_x86().unwrap();
    /// platform_x86.set_hpet_enabled(true).unwrap();
    pub fn set_hpet_enabled(&self, hpet_enabled: bool) -> Result<(), VboxError> {
        let hpet_enabled = if hpet_enabled { 1 } else { 0 };
        get_function_result_unit!(self.object, SetHPETEnabled, hpet_enabled)
    }


    /// Returns the virtual CPU boolean value of the specified property.
    ///
    /// # Arguments
    ///
    /// * `property` - [`CPUPropertyType`]. Property type to query.
    ///
    /// # Returns
    ///
    /// Returns bool success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{VirtualBox};
    /// use virtualbox_rs::enums::CPUPropertyType;
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let platform = machine.get_platform().unwrap();
    /// let platform_x86 = platform.get_x86().unwrap();
    /// let cpu_property = platform_x86.get_cpu_property(CPUPropertyType::APIC).unwrap();
    pub fn get_cpu_property(&self, property: CPUPropertyType) -> Result<bool, VboxError> {
        let property = property.into();
        get_function_result_bool!(self.object, GetCPUProperty, property)
    }

    ///  Sets the virtual CPU boolean value of the specified property.
    ///
    /// # Arguments
    ///
    /// * `property` - [`CPUPropertyType`]. Property type to query.
    /// * `value` - bool. Property value.
    ///
    /// # Returns
    ///
    /// Returns () success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{Session, VirtualBox};
    /// use virtualbox_rs::enums::{CPUPropertyType, SessionType};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let mut session = Session::init().unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    /// let machine_mut = session.get_machine().unwrap();
    /// let platform = machine_mut.get_platform().unwrap();
    /// let platform_x86 = platform.get_x86().unwrap();
    /// platform_x86.set_cpu_property(CPUPropertyType::APIC, true).unwrap();
    pub fn set_cpu_property(
        &self,
        property: CPUPropertyType,
        value: bool,
    ) -> Result<(), VboxError> {
        let property = property.into();
        let value = if value { 1 } else { 0 };
        get_function_result_unit!(self.object, SetCPUProperty, property, value)
    }


    /// Returns the value of the specified hardware virtualization boolean property.
    ///
    /// # Arguments
    ///
    /// * `property` - [`HWVirtExPropertyType`]. Property type to query.
    ///
    /// # Returns
    ///
    /// Returns bool success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{VirtualBox};
    /// use virtualbox_rs::enums::HWVirtExPropertyType;
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let platform = machine.get_platform().unwrap();
    /// let platform_x86 = platform.get_x86().unwrap();
    /// let property = platform_x86.get_hw_virt_ex_property(HWVirtExPropertyType::NestedPaging).unwrap();
    pub fn get_hw_virt_ex_property(
        &self,
        property: HWVirtExPropertyType,
    ) -> Result<bool, VboxError> {
        let property = property.into();
        get_function_result_bool!(self.object, GetHWVirtExProperty, property)
    }

    ///  Sets a new value for the specified hardware virtualization boolean property.
    ///
    /// # Arguments
    ///
    /// * `property` - [`HWVirtExPropertyType`]. Property type to set.
    /// * `value` - bool. New property value.
    ///
    /// # Returns
    ///
    /// Returns () success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{Session, VirtualBox};
    /// use virtualbox_rs::enums::{HWVirtExPropertyType, SessionType};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let mut session = Session::init().unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    /// let machine_mut = session.get_machine().unwrap();
    /// let platform = machine_mut.get_platform().unwrap();
    /// let platform_x86 = platform.get_x86().unwrap();
    /// platform_x86.set_hw_virt_ex_property(HWVirtExPropertyType::NestedPaging, true).unwrap();
    pub fn set_hw_virt_ex_property(
        &self,
        property: HWVirtExPropertyType,
        value: bool,
    ) -> Result<(), VboxError> {
        let property = property.into();
        let value = if value { 1 } else { 0 };
        get_function_result_unit!(self.object, SetHWVirtExProperty, property, value)
    }
}