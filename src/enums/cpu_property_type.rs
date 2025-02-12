#[cfg(doc)]
use crate::Machine;
use log::error;
use std::fmt::Display;
use vbox_raw::sys_lib as raw;

/// This enumeration represents possible values of the [`Machine::get_cpu_property`]  and [`Machine::get_cpu_property`] methods.
#[derive(Debug, Eq, PartialEq)]
pub enum CPUPropertyType {
    /// Null value (never used by the API).
    Null,
    /// This setting determines whether VirtualBox will expose the Physical Address Extension (PAE) feature of the host CPU to the guest.
    ///
    /// Note that in case PAE is not available, it will not be reported.
    PAE,
    /// This setting determines whether VirtualBox will advertise long mode (i.e.
    ///
    /// 64-bit guest support) and let the guest enter it.
    LongMode,
    /// This setting determines whether a triple fault within a guest will trigger an internal error condition and stop the VM (default) or reset the virtual CPU/VM and continue execution.
    TripleFaultReset,
    /// This setting determines whether an APIC is part of the virtual CPU.
    ///
    /// This feature can only be turned off when the X2APIC feature is off.
    APIC,
    /// This setting determines whether an x2APIC is part of the virtual CPU.
    ///
    /// Since this feature implies that the APIC feature is present, it automatically enables the APIC feature when set.
    X2APIC,
    /// If set, force an indirect branch prediction barrier on VM exits if the host CPU supports it.
    ///
    /// This setting will significantly slow down workloads causing many VM exits, so it is only recommended for situation where there is a real need to be paranoid.
    IBPBOnVMExit,
    /// If set, force an indirect branch prediction barrier on VM entry if the host CPU supports it.
    ///
    /// This setting will significantly slow down workloads causing many VM exits, so it is only recommended for situation where there is a real need to be paranoid.
    IBPBOnVMEntry,
    /// Enabled the hardware virtualization (AMD-V/VT-x) feature on the guest CPU.
    ///
    /// This requires hardware virtualization on the host CPU
    HWVirt,
    ///
    /// If set, the speculation control CPUID bits and MSRs, when available on the host, are exposed to the guest.
    ///
    /// Depending on the host CPU and operating system, this may significantly slow down workloads causing many VM exits.
    SpecCtrl,
    /// If set, the speculation controls are managed by the host.
    ///
    /// This is intended for guests which do not set the speculation controls themselves. Note! This has not yet been implemented beyond leaving everything to the host OS.
    SpecCtrlByHost,
    /// If set and the host is affected by CVE-2018-3646, flushes the level 1 data cache when the EMT is scheduled to do ring-0 guest execution.
    ///
    /// There could be a small performance penalty for certain typs of workloads. For security reasons this setting will be enabled by default.
    L1DFlushOnEMTScheduling,
    /// If set and the host is affected by CVE-2018-3646, flushes the level 1 data on every VM entry.
    ///
    /// This setting may significantly slow down workloads causing many VM exits, so it is only recommended for situation where there is a real need to be paranoid.
    L1DFlushOnVMEntry,
    /// If set and the host is affected by CVE-2018-12126, CVE-2018-12127, or CVE-2018-12130, clears the relevant MDS buffers when the EMT is scheduled to do ring-0 guest execution.
    ///
    /// There could be a small performance penalty for certain typs of workloads. For security reasons this setting will be enabled by default.
    MDSClearOnEMTScheduling,
    ///
    /// If set and the host is affected by CVE-2018-12126, CVE-2018-12127, or CVE-2018-12130, clears the relevant MDS buffers on every VM entry.
    ///
    /// This setting may slow down workloads causing many VM exits, so it is only recommended for situation where there is a real need to be paranoid.
    MDSClearOnVMEntry,
}
#[cfg(not(is_v_7_1))]
impl Into<u32> for CPUPropertyType {
    fn into(self) -> u32 {
        match self {
            CPUPropertyType::PAE => raw::CPUPropertyType_CPUPropertyType_PAE,
            CPUPropertyType::LongMode => raw::CPUPropertyType_CPUPropertyType_LongMode,
            CPUPropertyType::TripleFaultReset => {
                raw::CPUPropertyType_CPUPropertyType_TripleFaultReset
            }
            CPUPropertyType::APIC => raw::CPUPropertyType_CPUPropertyType_APIC,
            CPUPropertyType::X2APIC => raw::CPUPropertyType_CPUPropertyType_X2APIC,
            CPUPropertyType::IBPBOnVMExit => raw::CPUPropertyType_CPUPropertyType_IBPBOnVMExit,
            CPUPropertyType::IBPBOnVMEntry => raw::CPUPropertyType_CPUPropertyType_IBPBOnVMEntry,
            CPUPropertyType::HWVirt => raw::CPUPropertyType_CPUPropertyType_HWVirt,
            CPUPropertyType::SpecCtrl => raw::CPUPropertyType_CPUPropertyType_SpecCtrl,
            CPUPropertyType::SpecCtrlByHost => raw::CPUPropertyType_CPUPropertyType_SpecCtrlByHost,
            CPUPropertyType::L1DFlushOnEMTScheduling => {
                raw::CPUPropertyType_CPUPropertyType_L1DFlushOnEMTScheduling
            }
            CPUPropertyType::L1DFlushOnVMEntry => {
                raw::CPUPropertyType_CPUPropertyType_L1DFlushOnVMEntry
            }
            CPUPropertyType::MDSClearOnEMTScheduling => {
                raw::CPUPropertyType_CPUPropertyType_MDSClearOnEMTScheduling
            }
            CPUPropertyType::MDSClearOnVMEntry => {
                raw::CPUPropertyType_CPUPropertyType_MDSClearOnVMEntry
            }
            _ => raw::CPUPropertyType_CPUPropertyType_Null,
        }
    }
}
#[cfg(not(is_v_7_1))]
impl From<u32> for CPUPropertyType {
    fn from(value: u32) -> Self {
        match value {
            raw::CPUPropertyType_CPUPropertyType_Null => CPUPropertyType::Null,
            raw::CPUPropertyType_CPUPropertyType_PAE => CPUPropertyType::PAE,
            raw::CPUPropertyType_CPUPropertyType_LongMode => CPUPropertyType::LongMode,
            raw::CPUPropertyType_CPUPropertyType_TripleFaultReset => {
                CPUPropertyType::TripleFaultReset
            }
            raw::CPUPropertyType_CPUPropertyType_APIC => CPUPropertyType::APIC,
            raw::CPUPropertyType_CPUPropertyType_X2APIC => CPUPropertyType::X2APIC,
            raw::CPUPropertyType_CPUPropertyType_IBPBOnVMExit => CPUPropertyType::IBPBOnVMExit,
            raw::CPUPropertyType_CPUPropertyType_IBPBOnVMEntry => CPUPropertyType::IBPBOnVMEntry,
            raw::CPUPropertyType_CPUPropertyType_HWVirt => CPUPropertyType::HWVirt,
            raw::CPUPropertyType_CPUPropertyType_SpecCtrl => CPUPropertyType::SpecCtrl,
            raw::CPUPropertyType_CPUPropertyType_SpecCtrlByHost => CPUPropertyType::SpecCtrlByHost,
            raw::CPUPropertyType_CPUPropertyType_L1DFlushOnEMTScheduling => {
                CPUPropertyType::L1DFlushOnEMTScheduling
            }
            raw::CPUPropertyType_CPUPropertyType_L1DFlushOnVMEntry => {
                CPUPropertyType::L1DFlushOnVMEntry
            }
            raw::CPUPropertyType_CPUPropertyType_MDSClearOnEMTScheduling => {
                CPUPropertyType::MDSClearOnEMTScheduling
            }
            raw::CPUPropertyType_CPUPropertyType_MDSClearOnVMEntry => {
                CPUPropertyType::MDSClearOnVMEntry
            }
            _ => {
                error!("CPUPropertyType::from. Unknown type: {}", value);
                CPUPropertyType::Null
            }
        }
    }
}
#[cfg(is_v_7_1)]
impl Into<u32> for CPUPropertyType {
    fn into(self) -> u32 {
        match self {
            CPUPropertyType::PAE => raw::CPUPropertyTypeX86_CPUPropertyTypeX86_PAE,
            CPUPropertyType::LongMode => raw::CPUPropertyTypeX86_CPUPropertyTypeX86_LongMode,
            CPUPropertyType::TripleFaultReset => {
                raw::CPUPropertyTypeX86_CPUPropertyTypeX86_TripleFaultReset
            }
            CPUPropertyType::APIC => raw::CPUPropertyTypeX86_CPUPropertyTypeX86_APIC,
            CPUPropertyType::X2APIC => raw::CPUPropertyTypeX86_CPUPropertyTypeX86_X2APIC,
            CPUPropertyType::IBPBOnVMExit => raw::CPUPropertyTypeX86_CPUPropertyTypeX86_IBPBOnVMExit,
            CPUPropertyType::IBPBOnVMEntry => raw::CPUPropertyTypeX86_CPUPropertyTypeX86_IBPBOnVMEntry,
            CPUPropertyType::HWVirt => raw::CPUPropertyTypeX86_CPUPropertyTypeX86_HWVirt,
            CPUPropertyType::SpecCtrl => raw::CPUPropertyTypeX86_CPUPropertyTypeX86_SpecCtrl,
            CPUPropertyType::SpecCtrlByHost => raw::CPUPropertyTypeX86_CPUPropertyTypeX86_SpecCtrlByHost,
            CPUPropertyType::L1DFlushOnEMTScheduling => {
                raw::CPUPropertyTypeX86_CPUPropertyTypeX86_L1DFlushOnEMTScheduling
            }
            CPUPropertyType::L1DFlushOnVMEntry => {
                raw::CPUPropertyTypeX86_CPUPropertyTypeX86_L1DFlushOnVMEntry
            }
            CPUPropertyType::MDSClearOnEMTScheduling => {
                raw::CPUPropertyTypeX86_CPUPropertyTypeX86_MDSClearOnEMTScheduling
            }
            CPUPropertyType::MDSClearOnVMEntry => {
                raw::CPUPropertyTypeX86_CPUPropertyTypeX86_MDSClearOnVMEntry
            }
            _ => raw::CPUPropertyTypeX86_CPUPropertyTypeX86_Null,
        }
    }
}
#[cfg(is_v_7_1)]
impl From<u32> for CPUPropertyType {
    fn from(value: u32) -> Self {
        match value {
            raw::CPUPropertyTypeX86_CPUPropertyTypeX86_Null => CPUPropertyType::Null,
            raw::CPUPropertyTypeX86_CPUPropertyTypeX86_PAE => CPUPropertyType::PAE,
            raw::CPUPropertyTypeX86_CPUPropertyTypeX86_LongMode => CPUPropertyType::LongMode,
            raw::CPUPropertyTypeX86_CPUPropertyTypeX86_TripleFaultReset => {
                CPUPropertyType::TripleFaultReset
            }
            raw::CPUPropertyTypeX86_CPUPropertyTypeX86_APIC => CPUPropertyType::APIC,
            raw::CPUPropertyTypeX86_CPUPropertyTypeX86_X2APIC => CPUPropertyType::X2APIC,
            raw::CPUPropertyTypeX86_CPUPropertyTypeX86_IBPBOnVMExit => CPUPropertyType::IBPBOnVMExit,
            raw::CPUPropertyTypeX86_CPUPropertyTypeX86_IBPBOnVMEntry => CPUPropertyType::IBPBOnVMEntry,
            raw::CPUPropertyTypeX86_CPUPropertyTypeX86_HWVirt => CPUPropertyType::HWVirt,
            raw::CPUPropertyTypeX86_CPUPropertyTypeX86_SpecCtrl => CPUPropertyType::SpecCtrl,
            raw::CPUPropertyTypeX86_CPUPropertyTypeX86_SpecCtrlByHost => CPUPropertyType::SpecCtrlByHost,
            raw::CPUPropertyTypeX86_CPUPropertyTypeX86_L1DFlushOnEMTScheduling => {
                CPUPropertyType::L1DFlushOnEMTScheduling
            }
            raw::CPUPropertyTypeX86_CPUPropertyTypeX86_L1DFlushOnVMEntry => {
                CPUPropertyType::L1DFlushOnVMEntry
            }
            raw::CPUPropertyTypeX86_CPUPropertyTypeX86_MDSClearOnEMTScheduling => {
                CPUPropertyType::MDSClearOnEMTScheduling
            }
            raw::CPUPropertyTypeX86_CPUPropertyTypeX86_MDSClearOnVMEntry => {
                CPUPropertyType::MDSClearOnVMEntry
            }
            _ => {
                error!("CPUPropertyType::from. Unknown type: {}", value);
                CPUPropertyType::Null
            }
        }
    }
}

impl Display for CPUPropertyType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self))
    }
}
