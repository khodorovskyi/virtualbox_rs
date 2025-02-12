use log::error;
use std::fmt::Display;
use vbox_raw::sys_lib as raw;
#[cfg(doc)]
use crate::{Medium, MediumAttachment, Snapshot};

/// or each [`Medium`], this defines how the medium is attached to a virtual machine (see [`MediumAttachment`]) and what happens when a snapshot (see [`Snapshot`]) is taken of a virtual machine which has the medium attached. At the moment DVD and floppy media are always of type "writethrough".
#[derive(Debug, Eq, PartialEq)]
pub enum MediumType {
    /// Normal medium (attached directly or indirectly, preserved when taking snapshots).
    Normal,
    /// Immutable medium (attached indirectly, changes are wiped out the next time the virtual machine is started).
    Immutable,
    /// Write through medium (attached directly, ignored when taking snapshots).
    Writethrough,
    /// Allow using this medium concurrently by several machines.
    Shareable,
    /// A readonly medium, which can of course be used by several machines.
    Readonly ,
    /// A medium which is indirectly attached, so that one base medium can be used for several VMs which have their own differencing medium to store their modifications.
    ///
    /// In some sense a variant of Immutable with unset AutoReset flag in each differencing medium.
    MultiAttach,
}
impl Into<u32> for MediumType {
    fn into(self) -> u32 {
        match self {
            MediumType::Normal => raw::MediumType_MediumType_Normal,
            MediumType::Immutable => raw::MediumType_MediumType_Immutable,
            MediumType::Writethrough => raw::MediumType_MediumType_Writethrough,
            MediumType::Shareable => raw::MediumType_MediumType_Shareable,
            MediumType::Readonly  => raw::MediumType_MediumType_Readonly ,
            MediumType::MultiAttach => raw::MediumType_MediumType_MultiAttach,
        }
    }
}

impl From<u32> for MediumType {
    fn from(value: u32) -> Self {
        match value {
            raw::MediumType_MediumType_Normal => MediumType::Normal,
            raw::MediumType_MediumType_Immutable => MediumType::Immutable,
            raw::MediumType_MediumType_Writethrough => MediumType::Writethrough,
            raw::MediumType_MediumType_Shareable => MediumType::Shareable,
            raw::MediumType_MediumType_Readonly  => MediumType::Readonly ,
            raw::MediumType_MediumType_MultiAttach => MediumType::MultiAttach,
            _ => {
                error!("MediumType::from. Unknown type: {}", value);
                MediumType::Normal
            }
        }
    }
}

impl Display for MediumType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self))
    }
}
