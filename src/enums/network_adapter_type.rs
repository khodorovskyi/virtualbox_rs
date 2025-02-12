use log::error;
use std::fmt::Display;
use vbox_raw::sys_lib as raw;

/// Network adapter type.
#[derive(Debug)]
pub enum NetworkAdapterType {
    /// Null value (never used by the API).
    Null,
    /// AMD PCNet-PCI II network card (Am79C970A).
    Am79C970A,
    /// AMD PCNet-FAST III network card (Am79C973).
    Am79C973,
    /// Intel PRO/1000 MT Desktop network card (82540EM).
    I82540EM,
    /// Intel PRO/1000 T Server network card (82543GC).
    I82543GC,
    /// Intel PRO/1000 MT Server network card (82545EM).
    I82545EM,
    /// Virtio network device.
    Virtio,
    /// AMD PCnet-ISA/NE2100 network card (Am79C960).
    Am79C960,
    /// Novell NE2000 network card (NE2000).
    NE2000,
    /// Novell NE1000 network card (NE1000).
    NE1000,
    /// WD/SMC EtherCard Plus 16 network card (WD8013EBT).
    WD8013,
    /// WD/SMC EtherCard Plus network card (WD8003E).
    WD8003,
    /// 3Com EtherLink II network card (3C503).
    ELNK2,
    /// 3Com EtherLink network card (3C501/3C500).
    ELNK1,
}

impl From<u32> for NetworkAdapterType {
    fn from(value: u32) -> Self {
        match value {
            raw::NetworkAdapterType_NetworkAdapterType_Null => NetworkAdapterType::Null,
            raw::NetworkAdapterType_NetworkAdapterType_Am79C970A => NetworkAdapterType::Am79C970A,
            raw::NetworkAdapterType_NetworkAdapterType_Am79C973 => NetworkAdapterType::Am79C973,
            raw::NetworkAdapterType_NetworkAdapterType_I82540EM => NetworkAdapterType::I82540EM,
            raw::NetworkAdapterType_NetworkAdapterType_I82543GC => NetworkAdapterType::I82543GC,
            raw::NetworkAdapterType_NetworkAdapterType_I82545EM => NetworkAdapterType::I82545EM,
            raw::NetworkAdapterType_NetworkAdapterType_Virtio => NetworkAdapterType::Virtio,
            raw::NetworkAdapterType_NetworkAdapterType_Am79C960 => NetworkAdapterType::Am79C960,
            #[cfg(not(is_v_6_1))]
            raw::NetworkAdapterType_NetworkAdapterType_NE2000 => NetworkAdapterType::NE2000,
            #[cfg(not(is_v_6_1))]
            raw::NetworkAdapterType_NetworkAdapterType_NE1000 => NetworkAdapterType::NE1000,
            #[cfg(not(is_v_6_1))]
            raw::NetworkAdapterType_NetworkAdapterType_WD8013 => NetworkAdapterType::WD8013,
            #[cfg(not(is_v_6_1))]
            raw::NetworkAdapterType_NetworkAdapterType_WD8003 => NetworkAdapterType::WD8003,
            #[cfg(not(is_v_6_1))]
            raw::NetworkAdapterType_NetworkAdapterType_ELNK2 => NetworkAdapterType::ELNK2,
            #[cfg(not(is_v_6_1))]
            raw::NetworkAdapterType_NetworkAdapterType_ELNK1 => NetworkAdapterType::ELNK1,
            _ => {
                error!("Unknown NetworkAdapterType. Type: {}", value);
                NetworkAdapterType::Null
            }
        }
    }
}

impl Into<u32> for NetworkAdapterType {
    fn into(self) -> u32 {
        match self {
            NetworkAdapterType::Null => raw::NetworkAdapterType_NetworkAdapterType_Null,
            NetworkAdapterType::Am79C970A => raw::NetworkAdapterType_NetworkAdapterType_Am79C970A,
            NetworkAdapterType::Am79C973 => raw::NetworkAdapterType_NetworkAdapterType_Am79C973,
            NetworkAdapterType::I82540EM => raw::NetworkAdapterType_NetworkAdapterType_I82540EM,
            NetworkAdapterType::I82543GC => raw::NetworkAdapterType_NetworkAdapterType_I82543GC,
            NetworkAdapterType::I82545EM => raw::NetworkAdapterType_NetworkAdapterType_I82545EM,
            NetworkAdapterType::Virtio => raw::NetworkAdapterType_NetworkAdapterType_Virtio,
            NetworkAdapterType::Am79C960 => raw::NetworkAdapterType_NetworkAdapterType_Am79C960,
            #[cfg(not(is_v_6_1))]
            NetworkAdapterType::NE2000 => raw::NetworkAdapterType_NetworkAdapterType_NE2000,
            #[cfg(not(is_v_6_1))]
            NetworkAdapterType::NE1000 => raw::NetworkAdapterType_NetworkAdapterType_NE1000,
            #[cfg(not(is_v_6_1))]
            NetworkAdapterType::WD8013 => raw::NetworkAdapterType_NetworkAdapterType_WD8013,
            #[cfg(not(is_v_6_1))]
            NetworkAdapterType::WD8003 => raw::NetworkAdapterType_NetworkAdapterType_WD8003,
            #[cfg(not(is_v_6_1))]
            NetworkAdapterType::ELNK2 => raw::NetworkAdapterType_NetworkAdapterType_ELNK2,
            #[cfg(not(is_v_6_1))]
            NetworkAdapterType::ELNK1 => raw::NetworkAdapterType_NetworkAdapterType_ELNK1,
            #[cfg(is_v_6_1)]
            _ => raw::NetworkAdapterType_NetworkAdapterType_Null,
        }
    }
}

impl Display for NetworkAdapterType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self))
    }
}
