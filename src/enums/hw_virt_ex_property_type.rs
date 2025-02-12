#[cfg(doc)]
use crate::Machine;
use log::error;
use std::fmt::Display;
use vbox_raw::sys_lib as raw;

/// Hardware virtualization property type.
///
/// This enumeration represents possible values for the [`Machine::get_hw_virt_ex_property`] and [`Machine::set_hw_virt_ex_property`] methods.
#[derive(Debug, Eq, PartialEq)]
pub enum HWVirtExPropertyType {
    /// Null value (never used by the API).
    Null,
    /// Whether hardware virtualization (VT-x/AMD-V) is enabled at all.
    ///
    /// If such extensions are not available, they will not be used.
    Enabled,
    /// Whether VT-x VPID is enabled.
    ///
    /// If this extension is not available, it will not be used.
    VPID,
    /// Whether Nested Paging is enabled.
    ///
    /// If this extension is not available, it will not be used.
    NestedPaging,
    /// Whether VT-x unrestricted execution is enabled.
    ///
    /// If this feature is not available, it will not be used.
    UnrestrictedExecution,
    /// Whether large page allocation is enabled; requires nested paging and a 64-bit host.
    LargePages,
    /// Whether the VM should fail to start if hardware virtualization (VT-x/AMD-V) cannot be used.
    ///
    /// If not set, there will be an automatic fallback to software virtualization.
    Force,
    /// Use the native hypervisor API instead of the VirtualBox one (HM) for VT-X/AMD-V.
    ///
    /// This is ignored if [`HWVirtExPropertyType::Enabled`] isn't set.
    UseNativeApi,
    /// Whether AMD-V Virtualized VMSAVE/VMLOAD is enabled.
    ///
    /// If this feature is not available, it will not be used.
    VirtVmsaveVmload,
}
impl Into<u32> for HWVirtExPropertyType {
    fn into(self) -> u32 {
        match self {
            HWVirtExPropertyType::Enabled => raw::HWVirtExPropertyType_HWVirtExPropertyType_Enabled,
            HWVirtExPropertyType::VPID => raw::HWVirtExPropertyType_HWVirtExPropertyType_VPID,
            HWVirtExPropertyType::NestedPaging => {
                raw::HWVirtExPropertyType_HWVirtExPropertyType_NestedPaging
            }
            HWVirtExPropertyType::UnrestrictedExecution => {
                raw::HWVirtExPropertyType_HWVirtExPropertyType_UnrestrictedExecution
            }
            HWVirtExPropertyType::LargePages => {
                raw::HWVirtExPropertyType_HWVirtExPropertyType_LargePages
            }
            HWVirtExPropertyType::Force => raw::HWVirtExPropertyType_HWVirtExPropertyType_Force,
            HWVirtExPropertyType::UseNativeApi => {
                raw::HWVirtExPropertyType_HWVirtExPropertyType_UseNativeApi
            }
            #[cfg(not(is_v_6_1))]
            HWVirtExPropertyType::VirtVmsaveVmload => {
                raw::HWVirtExPropertyType_HWVirtExPropertyType_VirtVmsaveVmload
            }
            _ => raw::HWVirtExPropertyType_HWVirtExPropertyType_Null,
        }
    }
}

impl From<u32> for HWVirtExPropertyType {
    fn from(value: u32) -> Self {
        match value {
            raw::HWVirtExPropertyType_HWVirtExPropertyType_Null => HWVirtExPropertyType::Null,
            raw::HWVirtExPropertyType_HWVirtExPropertyType_Enabled => HWVirtExPropertyType::Enabled,
            raw::HWVirtExPropertyType_HWVirtExPropertyType_VPID => HWVirtExPropertyType::VPID,
            raw::HWVirtExPropertyType_HWVirtExPropertyType_NestedPaging => {
                HWVirtExPropertyType::NestedPaging
            }
            raw::HWVirtExPropertyType_HWVirtExPropertyType_UnrestrictedExecution => {
                HWVirtExPropertyType::UnrestrictedExecution
            }
            raw::HWVirtExPropertyType_HWVirtExPropertyType_LargePages => {
                HWVirtExPropertyType::LargePages
            }
            raw::HWVirtExPropertyType_HWVirtExPropertyType_Force => HWVirtExPropertyType::Force,
            raw::HWVirtExPropertyType_HWVirtExPropertyType_UseNativeApi => {
                HWVirtExPropertyType::UseNativeApi
            }
            #[cfg(not(is_v_6_1))]
            raw::HWVirtExPropertyType_HWVirtExPropertyType_VirtVmsaveVmload => {
                HWVirtExPropertyType::VirtVmsaveVmload
            }
            _ => {
                error!("HWVirtExPropertyType::from. Unknown type: {}", value);
                HWVirtExPropertyType::Null
            }
        }
    }
}

impl Display for HWVirtExPropertyType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self))
    }
}
