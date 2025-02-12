#[cfg(doc)]
use crate::Machine;
use log::error;
use std::fmt::Display;
use vbox_raw::sys_lib as raw;

/// The paravirtualized guest interface provider.
///
/// This enumeration represents possible values for the [`Machine::get_paravirt_provider`] attribute.
#[derive(Debug, Eq, PartialEq)]
pub enum ParavirtProvider {
    /// No provider is used.
    None,
    /// A default provider is automatically chosen according to the guest OS type.
    Default,
    /// Used for VMs which didn't used to have any provider settings.
    ///
    /// Usually interpreted as None for most VMs.
    Legacy,
    /// A minimal set of features to expose to the paravirtualized guest.
    Minimal,
    /// Microsoft Hyper-V.
    HyperV,
    /// Linux KVM.
    KVM,
}
impl Into<u32> for ParavirtProvider {
    fn into(self) -> u32 {
        match self {
            ParavirtProvider::Default => raw::ParavirtProvider_ParavirtProvider_Default,
            ParavirtProvider::Legacy => raw::ParavirtProvider_ParavirtProvider_Legacy,
            ParavirtProvider::Minimal => raw::ParavirtProvider_ParavirtProvider_Minimal,
            ParavirtProvider::HyperV => raw::ParavirtProvider_ParavirtProvider_HyperV,
            ParavirtProvider::KVM => raw::ParavirtProvider_ParavirtProvider_KVM,
            _ => raw::ParavirtProvider_ParavirtProvider_None,
        }
    }
}

impl From<u32> for ParavirtProvider {
    fn from(value: u32) -> Self {
        match value {
            raw::ParavirtProvider_ParavirtProvider_None => ParavirtProvider::None,
            raw::ParavirtProvider_ParavirtProvider_Default => ParavirtProvider::Default,
            raw::ParavirtProvider_ParavirtProvider_Legacy => ParavirtProvider::Legacy,
            raw::ParavirtProvider_ParavirtProvider_Minimal => ParavirtProvider::Minimal,
            raw::ParavirtProvider_ParavirtProvider_HyperV => ParavirtProvider::HyperV,
            raw::ParavirtProvider_ParavirtProvider_KVM => ParavirtProvider::KVM,
            _ => {
                error!("ParavirtProvider::from. Unknown type: {}", value);
                ParavirtProvider::None
            }
        }
    }
}

impl Display for ParavirtProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self))
    }
}
