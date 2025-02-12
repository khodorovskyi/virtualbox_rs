#[cfg(doc)]
use crate::enums::HWVirtExPropertyType;
#[cfg(doc)]
use crate::Machine;
use crate::{PlatformProperties, VboxError};
use crate::utility::macros::macros::{get_function_result_bool, get_function_result_number, get_function_result_unit};

impl PlatformProperties {
    /// Indicates whether VirtualBox was built with raw-mode support.
    ///
    /// When this reads as False, the [`HWVirtExPropertyType::Enabled`] setting will be ignored and assumed to be True.
    ///
    /// # Returns
    ///
    /// Returns bool on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::PlatformProperties;
    ///
    /// let platform_properties = PlatformProperties::init().unwrap();
    /// let raw_mode_supported = platform_properties.get_raw_mode_supported().unwrap();
    ///```
    pub fn get_raw_mode_supported(&self) -> Result<bool, VboxError> {
        get_function_result_bool!(self.object, GetRawModeSupported)
    }

    /// Exclusive use of hardware virtualization by VirtualBox.
    ///
    /// When enabled, VirtualBox assumes it can obtain full and exclusive access to the VT-x or AMD-V feature of the host. To share hardware virtualization with other hypervisors, this property must be disabled.
    ///
    /// # Returns
    ///
    /// Returns bool on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::PlatformProperties;
    ///
    /// let platform_properties = PlatformProperties::init().unwrap();
    /// let exclusive_hw_virt = platform_properties.get_exclusive_hw_virt().unwrap();
    ///```
    pub fn get_exclusive_hw_virt(&self) -> Result<bool, VboxError> {
        get_function_result_bool!(self.object, GetExclusiveHwVirt)
    }

    /// Exclusive use of hardware virtualization by VirtualBox.
    ///
    /// When enabled, VirtualBox assumes it can obtain full and exclusive access to the VT-x or AMD-V feature of the host. To share hardware virtualization with other hypervisors, this property must be disabled.
    ///
    /// # Arguments
    ///
    /// * `exclusive_hw_virt` - bool
    ///
    /// # Returns
    ///
    /// Returns () on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::PlatformProperties;
    ///
    /// let platform_properties = PlatformProperties::init().unwrap();
    /// platform_properties.set_exclusive_hw_virt(true).unwrap();
    ///```
    pub fn set_exclusive_hw_virt(&self, exclusive_hw_virt: bool) -> Result<(), VboxError> {
        let exclusive_hw_virt = if exclusive_hw_virt { 1 } else { 0 };
        get_function_result_unit!(self.object, SetExclusiveHwVirt, exclusive_hw_virt)
    }

    /// Maximum number of serial ports associated with every [`Machine`] instance.
    ///
    /// # Returns
    ///
    /// Returns u32 on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::PlatformProperties;
    ///
    /// let platform_properties = PlatformProperties::init().unwrap();
    /// let serial_port_count = platform_properties.get_serial_port_count().unwrap();
    ///```
    pub fn get_serial_port_count(&self) -> Result<u32, VboxError> {
        get_function_result_number!(self.object, GetSerialPortCount, u32)
    }

    /// Maximum number of parallel ports associated with every [`Machine`] instance.
    ///
    /// # Returns
    ///
    /// Returns u32 on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::PlatformProperties;
    ///
    /// let platform_properties = PlatformProperties::init().unwrap();
    /// let parallel_port_count = platform_properties.get_parallel_port_count().unwrap();
    ///```
    pub fn get_parallel_port_count(&self) -> Result<u32, VboxError> {
        get_function_result_number!(self.object, GetParallelPortCount, u32)
    }

    /// Maximum device position in the boot order.
    ///
    /// This value corresponds to the total number of devices a machine can boot from, to make it possible to include all possible devices to the boot list.
    ///
    /// # Returns
    ///
    /// Returns u32 on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::PlatformProperties;
    ///
    /// let platform_properties = PlatformProperties::init().unwrap();
    /// let max_boot_position =  platform_properties.get_max_boot_position().unwrap();
    ///```
    pub fn get_max_boot_position(&self) -> Result<u32, VboxError> {
        get_function_result_number!(self.object, GetMaxBootPosition, u32)
    }
}