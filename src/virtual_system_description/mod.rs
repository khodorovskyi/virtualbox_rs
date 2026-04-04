pub mod implementation;

use crate::utility::macros::macros::call_function;
use crate::VboxError;
use log::{debug, error};
use vbox_raw::sys_lib::IVirtualSystemDescription;
use crate::enums::VirtualSystemDescriptionType;

/// Represents one virtual system (machine) in an appliance.
///
/// **Reference to the official documentation:**
///
/// [https://www.virtualbox.org/sdkref/interface_i_virtual_system_description.html](https://www.virtualbox.org/sdkref/interface_i_virtual_system_description.html)
#[derive(Debug)]
pub struct VirtualSystemDescription {
    object: *mut IVirtualSystemDescription,
}

impl VirtualSystemDescription {
    pub(crate) fn new(object: *mut IVirtualSystemDescription) -> Self {
        Self { object }
    }

    fn release(&self) -> Result<i32, VboxError> {
        call_function!(self.object, Release)
    }
}

impl Drop for VirtualSystemDescription {
    fn drop(&mut self) {
        match self.release() {
            Ok(count) => {
                debug!("VirtualSystemDescription refcount: {}", count)
            }
            Err(err) => {
                error!("Failed drop VirtualSystemDescription. Error: {:?}", err)
            }
        }
    }
}

#[derive(Debug)]
/// One entry describing an element of a virtual system description (OVF → VirtualBox mapping).
pub struct VirtualSystemDescriptionInfo {
    /// Description type (see [`VirtualSystemDescriptionType`]).
    pub type_: VirtualSystemDescriptionType,

    /// Reference string (e.g. filename or identifier such as "vm-disk001.vmdk").
    /// Empty if not used for this `type_`.
    pub ref_: &'static str,

    /// Original value from the OVF file: the raw value as read from the OVF descriptor.
    /// Informational: it may be unnormalized or unvalidated and should not be used directly
    /// to configure VirtualBox without appropriate processing.
    pub ovf_value: &'static str,

    /// Suggested/normalized value for VirtualBox.
    pub vbox_value: &'static str,

    /// Optional extra configuration (attachment info, network type, etc.).
    pub extra_config_value: &'static str,
}
