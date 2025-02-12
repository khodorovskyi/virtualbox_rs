use std::collections::BTreeMap;
use std::fmt::{Debug, Display, Formatter};
use crate::utility::macros::macros::call_function;
use crate::{VboxError};
use log::{debug, error};
use vbox_raw::sys_lib::IFirmwareSettings;
use crate::enums::{APICMode, FirmwareBootMenuMode, FirmwareType};

mod implementation;

/// Interface for managing VirtualBox Extension Packs.
/// **Reference to the official documentation:**
///
/// [https://www.virtualbox.org/sdkref/interface_i_firmware_settings.html](https://www.virtualbox.org/sdkref/interface_i_firmware_settings.html)
pub struct FirmwareSettings {
    object: *mut IFirmwareSettings,
}

impl FirmwareSettings {
    pub(crate) fn new(object: *mut IFirmwareSettings) -> Self {
        Self { object }
    }

    fn release(&self) -> Result<i32, VboxError> {
        call_function!(self.object, Release)
    }
}

impl Drop for FirmwareSettings {
    fn drop(&mut self) {
        match self.release() {
            Ok(count) => {
                debug!("FirmwareSettings refcount: {}", count)
            }
            Err(err) => {
                error!("Failed drop FirmwareSettings. Error: {:?}", err)
            }
        }
    }
}

impl Display for FirmwareSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut map = BTreeMap::new();
        map.insert("firmware_type", self.get_firmware_type().unwrap_or(FirmwareType::BIOS).to_string());
        map.insert("logo_image_path", self.get_logo_image_path().unwrap_or("").to_string());
        map.insert("boot_menu_mode", self.get_boot_menu_mode().unwrap_or(FirmwareBootMenuMode::Disabled).to_string());
        map.insert("acpi_enabled", self.get_acpi_enabled().unwrap_or(false).to_string());
        map.insert("apic_mode", self.get_apic_mode().unwrap_or(APICMode::Disabled).to_string());
        map.insert("time_offset", self.get_time_offset().unwrap_or(0).to_string());
        map.insert("pxe_debug_enabled", self.get_pxe_debug_enabled().unwrap_or(false).to_string());
        map.insert("sm_bios_uuid_little_endian", self.get_sm_bios_uuid_little_endian().unwrap_or(false).to_string());
        map.insert("auto_serial_num_gen", self.get_auto_serial_num_gen().unwrap_or(false).to_string());

        if f.alternate() {
            write!(f, "{}", format!("{:#?}", map))
        } else {
            write!(f, "{}", format!("{:?}", map))
        }
    }
}

impl Debug for FirmwareSettings {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}