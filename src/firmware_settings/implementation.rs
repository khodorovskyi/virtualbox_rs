use crate::enums::{APICMode, FirmwareBootMenuMode, FirmwareType};
use crate::{FirmwareSettings, VboxError};
use crate::utility::macros::macros::{get_function_result_bool, get_function_result_number, get_function_result_str, get_function_result_unit};
use crate::utility::string_to_c_u64_str;

impl FirmwareSettings {
    /// Type of firmware (such as legacy BIOS or EFI), used for initial bootstrap in this VM.
    ///
    /// # Returns
    ///
    /// Returns [`FirmwareType`] on success, or a [`VboxError`] on failure.
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
    ///
    /// let firmware_settings = machine.get_firmware_settings().unwrap();
    /// let firmware_type = firmware_settings.get_firmware_type().unwrap();
    /// ```
    pub fn get_firmware_type(&self) -> Result<FirmwareType, VboxError> {
        let firmware_type = get_function_result_number!(self.object, GetFirmwareType, u32)?;
        Ok(FirmwareType::from(firmware_type))
    }
    /// Type of firmware (such as legacy BIOS or EFI), used for initial bootstrap in this VM.
    ///
    /// # Arguments
    ///
    /// * `firmware_type` - [`FirmwareType`] - Type of firmware (such as legacy BIOS or EFI), used for initial bootstrap in this VM.
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
    /// use virtualbox_rs::enums::{FirmwareType, SessionType};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let mut session = Session::init().unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    /// let machine_mut = session.get_machine().unwrap();
    ///
    /// let firmware_settings = machine_mut.get_firmware_settings().unwrap();
    /// firmware_settings.set_firmware_type(FirmwareType::EFI).unwrap();
    /// ```
    pub fn set_firmware_type(&self, firmware_type: FirmwareType) -> Result<(), VboxError> {
        let firmware_type: u32 = firmware_type.into();
        get_function_result_unit!(self.object, SetFirmwareType, firmware_type)
    }

    /// Fade in flag for BIOS logo animation.
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
    ///
    /// let firmware_settings = machine.get_firmware_settings().unwrap();
    /// let logo_fade_in = firmware_settings.get_logo_fade_in().unwrap();
    /// ```
    pub fn get_logo_fade_in(&self) -> Result<bool, VboxError> {
        get_function_result_bool!(self.object, GetLogoFadeIn)
    }

    /// Fade in flag for BIOS logo animation.
    ///
    /// # Arguments
    ///
    /// * `logo_fade_in` - bool - Fade in flag for BIOS logo animation.
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
    /// use virtualbox_rs::enums::{FirmwareType, SessionType};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let mut session = Session::init().unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    /// let machine_mut = session.get_machine().unwrap();
    ///
    /// let firmware_settings = machine_mut.get_firmware_settings().unwrap();
    /// firmware_settings.set_logo_fade_in(true).unwrap();
    /// ```
    pub fn set_logo_fade_in(&self, logo_fade_in: bool) -> Result<(), VboxError> {
        let logo_fade_in = if logo_fade_in { 1 } else { 0 };
        get_function_result_unit!(self.object, SetLogoFadeIn, logo_fade_in)
    }
    /// Fade out flag for BIOS logo animation.
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
    ///
    /// let firmware_settings = machine.get_firmware_settings().unwrap();
    /// let logo_fade_out = firmware_settings.get_logo_fade_out().unwrap();
    /// ```
    pub fn get_logo_fade_out(&self) -> Result<bool, VboxError> {
        get_function_result_bool!(self.object, GetLogoFadeOut)
    }

    /// Fade out flag for BIOS logo animation.
    ///
    /// # Arguments
    ///
    /// * `logo_fade_out` - bool - Fade out flag for BIOS logo animation.
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
    /// use virtualbox_rs::enums::{FirmwareType, SessionType};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let mut session = Session::init().unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    /// let machine_mut = session.get_machine().unwrap();
    ///
    /// let firmware_settings = machine_mut.get_firmware_settings().unwrap();
    /// firmware_settings.set_logo_fade_out(true).unwrap();
    /// ```
    pub fn set_logo_fade_out(&self, logo_fade_out: bool) -> Result<(), VboxError> {
        let logo_fade_out = if logo_fade_out { 1 } else { 0 };
        get_function_result_unit!(self.object, SetLogoFadeIn, logo_fade_out)
    }

    /// BIOS logo display time in milliseconds (0 = default).
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
    ///
    /// let firmware_settings = machine.get_firmware_settings().unwrap();
    /// let logo_display_time = firmware_settings.get_logo_display_time().unwrap();
    /// ```
    pub fn get_logo_display_time(&self) -> Result<u32, VboxError> {
        get_function_result_number!(self.object, GetLogoDisplayTime, u32)
    }

    /// BIOS logo display time in milliseconds (0 = default).
    ///
    /// # Arguments
    ///
    /// * `logo_display_time` - u32 - BIOS logo display time in milliseconds (0 = default).
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
    /// use virtualbox_rs::enums::{FirmwareType, SessionType};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let mut session = Session::init().unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    /// let machine_mut = session.get_machine().unwrap();
    ///
    /// let firmware_settings = machine_mut.get_firmware_settings().unwrap();
    /// firmware_settings.set_logo_display_time(1000).unwrap();
    /// ```
    pub fn set_logo_display_time(&self, logo_display_time: u32) -> Result<(), VboxError> {
        get_function_result_unit!(self.object, SetLogoDisplayTime, logo_display_time)
    }

    /// Local file system path for external BIOS splash image.
    ///
    /// # Returns
    ///
    /// Returns &str on success, or a [`VboxError`] on failure.
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
    ///
    /// let firmware_settings = machine.get_firmware_settings().unwrap();
    /// let logo_image_path = firmware_settings.get_logo_image_path().unwrap();
    /// ```
    pub fn get_logo_image_path(&self) -> Result<&'static str, VboxError> {
        get_function_result_str!(self.object, GetLogoImagePath)
    }

    /// Local file system path for external BIOS splash image.
    ///
    /// # Arguments
    ///
    /// * `logo_image_path` - &str - BIOS logo display time in milliseconds (0 = default).
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
    /// use virtualbox_rs::enums::{FirmwareType, SessionType};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let mut session = Session::init().unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    /// let machine_mut = session.get_machine().unwrap();
    ///
    /// let firmware_settings = machine_mut.get_firmware_settings().unwrap();
    /// firmware_settings.set_logo_image_path("/home/host_user/logo.png").unwrap();
    /// ```
    pub fn set_logo_image_path(&self, logo_image_path: &str) -> Result<(), VboxError> {
        let  logo_image_path = string_to_c_u64_str(logo_image_path)?;
        get_function_result_unit!(self.object, SetLogoImagePath, logo_image_path)
    }


    /// Mode of the firmware boot device menu.
    ///
    /// # Returns
    ///
    /// Returns [`FirmwareBootMenuMode`] on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    /// use virtualbox_rs::VirtualBox;
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    ///
    /// let firmware_settings = machine.get_firmware_settings().unwrap();
    /// let boot_menu_mode = firmware_settings.get_boot_menu_mode().unwrap();
    /// ```
    pub fn get_boot_menu_mode(&self) -> Result<FirmwareBootMenuMode, VboxError> {
        let boot_menu_mode = get_function_result_number!(self.object, GetBootMenuMode, u32)?;
        Ok(FirmwareBootMenuMode::from(boot_menu_mode))
    }

    /// Mode of the firmware boot device menu.
    ///
    /// # Arguments
    ///
    /// * `boot_menu_mode` - [`FirmwareBootMenuMode`] - Mode of the firmware boot device menu.
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
    /// use virtualbox_rs::enums::{FirmwareBootMenuMode, FirmwareType, SessionType};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let mut session = Session::init().unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    /// let machine_mut = session.get_machine().unwrap();
    ///
    /// let firmware_settings = machine_mut.get_firmware_settings().unwrap();
    /// firmware_settings.set_boot_menu_mode(FirmwareBootMenuMode::MenuOnly).unwrap();
    /// ```
    pub fn set_boot_menu_mode(&self, boot_menu_mode: FirmwareBootMenuMode) -> Result<(), VboxError> {
        let boot_menu_mode = boot_menu_mode.into();
        get_function_result_unit!(self.object, SetBootMenuMode, boot_menu_mode)
    }

    /// ACPI support flag.
    ///
    /// # Returns
    ///
    /// Returns bool on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    /// use virtualbox_rs::VirtualBox;
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    ///
    /// let firmware_settings = machine.get_firmware_settings().unwrap();
    /// let acpi_enabled = firmware_settings.get_acpi_enabled().unwrap();
    /// ```
    pub fn get_acpi_enabled(&self) -> Result<bool, VboxError> {
        get_function_result_bool!(self.object, GetACPIEnabled)
    }

    /// ACPI support flag.
    ///
    /// # Arguments
    ///
    /// * `logo_fade_out` - bool - ACPI support flag.
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
    /// use virtualbox_rs::enums::{FirmwareType, SessionType};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let mut session = Session::init().unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    /// let machine_mut = session.get_machine().unwrap();
    ///
    /// let firmware_settings = machine_mut.get_firmware_settings().unwrap();
    /// firmware_settings.set_acpi_enabled(true).unwrap();
    /// ```
    pub fn set_acpi_enabled(&self, acpi_enabled: bool) -> Result<(), VboxError> {
        let acpi_enabled = if acpi_enabled { 1 } else { 0 };
        get_function_result_unit!(self.object, SetACPIEnabled, acpi_enabled)
    }

    /// I/O-APIC support flag.
    ///
    /// If set, VirtualBox will provide an I/O-APIC and support IRQs above 15.
    ///
    /// # Returns
    ///
    /// Returns bool on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    /// use virtualbox_rs::VirtualBox;
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    ///
    /// let firmware_settings = machine.get_firmware_settings().unwrap();
    /// let io_apic_enabled = firmware_settings.get_io_apic_enabled().unwrap();
    /// ```
    pub fn get_io_apic_enabled(&self) -> Result<bool, VboxError> {
        get_function_result_bool!(self.object, GetIOAPICEnabled)
    }

    /// I/O-APIC support flag.
    ///
    /// If set, VirtualBox will provide an I/O-APIC and support IRQs above 15.
    ///
    /// # Arguments
    ///
    /// * `logo_fade_out` - bool - If set, VirtualBox will provide an I/O-APIC and support IRQs above 15.
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
    /// use virtualbox_rs::enums::{FirmwareType, SessionType};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let mut session = Session::init().unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    /// let machine_mut = session.get_machine().unwrap();
    ///
    /// let firmware_settings = machine_mut.get_firmware_settings().unwrap();
    /// firmware_settings.set_io_apic_enabled(true).unwrap();
    /// ```
    pub fn set_io_apic_enabled(&self, io_apic_enabled: bool) -> Result<(), VboxError> {
        let io_apic_enabled = if io_apic_enabled { 1 } else { 0 };
        get_function_result_unit!(self.object, SetIOAPICEnabled, io_apic_enabled)
    }

    /// Mode of the firmware boot device menu.
    ///
    /// # Returns
    ///
    /// Returns [`APICMode`] on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    /// use virtualbox_rs::VirtualBox;
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    ///
    /// let firmware_settings = machine.get_firmware_settings().unwrap();
    /// let apic_mode = firmware_settings.get_apic_mode().unwrap();
    /// ```
    pub fn get_apic_mode(&self) -> Result<APICMode, VboxError> {
        let apic_mode = get_function_result_number!(self.object, GetAPICMode, u32)?;
        Ok(APICMode::from(apic_mode))
    }

    /// Mode of the firmware boot device menu.
    ///
    /// # Arguments
    ///
    /// * `apic_mode` - [`APICMode`] - Mode of the firmware boot device menu.
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
    /// use virtualbox_rs::enums::{APICMode, FirmwareBootMenuMode, FirmwareType, SessionType};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let mut session = Session::init().unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    /// let machine_mut = session.get_machine().unwrap();
    ///
    /// let firmware_settings = machine_mut.get_firmware_settings().unwrap();
    /// firmware_settings.set_apic_mode(APICMode::APIC).unwrap();
    /// ```
    pub fn set_apic_mode(&self, apic_mode: APICMode) -> Result<(), VboxError> {
        let apic_mode = apic_mode.into();
        get_function_result_unit!(self.object, SetAPICMode, apic_mode)
    }


    /// Offset in milliseconds from the host system time.
    ///
    /// This allows for guests running with a different system date/time than the host. It is equivalent to setting the system date/time in the BIOS except it is not an absolute value but a relative one. Guest Additions time synchronization honors this offset.
    ///
    /// # Returns
    ///
    /// Returns i64 on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    /// use virtualbox_rs::VirtualBox;
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    ///
    /// let firmware_settings = machine.get_firmware_settings().unwrap();
    /// let time_offset = firmware_settings.get_time_offset().unwrap();
    /// ```
    pub fn get_time_offset(&self) -> Result<i64, VboxError> {
        get_function_result_number!(self.object, GetTimeOffset, i64)
    }

    /// Offset in milliseconds from the host system time.
    ///
    /// This allows for guests running with a different system date/time than the host. It is equivalent to setting the system date/time in the BIOS except it is not an absolute value but a relative one. Guest Additions time synchronization honors this offset.
    ///
    /// # Arguments
    ///
    /// * `time_offset` - i64 - Offset in milliseconds from the host system time.
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
    /// use virtualbox_rs::enums::{FirmwareType, SessionType};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let mut session = Session::init().unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    /// let machine_mut = session.get_machine().unwrap();
    ///
    /// let firmware_settings = machine_mut.get_firmware_settings().unwrap();
    /// firmware_settings.set_time_offset(1000).unwrap();
    /// ```
    pub fn set_time_offset(&self, time_offset: i64) -> Result<(), VboxError> {
        get_function_result_unit!(self.object, SetTimeOffset, time_offset)
    }

    /// PXE debug logging flag.
    ///
    /// If set, VirtualBox will write extensive PXE trace information to the release log.
    ///
    /// # Returns
    ///
    /// Returns bool on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    /// use virtualbox_rs::VirtualBox;
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    ///
    /// let firmware_settings = machine.get_firmware_settings().unwrap();
    /// let pxe_debug_enabled = firmware_settings.get_pxe_debug_enabled().unwrap();
    /// ```
    pub fn get_pxe_debug_enabled(&self) -> Result<bool, VboxError> {
        get_function_result_bool!(self.object, GetPXEDebugEnabled)
    }

    /// PXE debug logging flag.
    ///
    /// If set, VirtualBox will write extensive PXE trace information to the release log.
    ///
    /// # Arguments
    ///
    /// * `pxe_debug_enabled` - bool - If set, VirtualBox will provide an I/O-APIC and support IRQs above 15.
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
    /// use virtualbox_rs::enums::{FirmwareType, SessionType};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let mut session = Session::init().unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    /// let machine_mut = session.get_machine().unwrap();
    ///
    /// let firmware_settings = machine_mut.get_firmware_settings().unwrap();
    /// firmware_settings.set_pxe_debug_enabled(true).unwrap();
    /// ```
    pub fn set_pxe_debug_enabled(&self, pxe_debug_enabled: bool) -> Result<(), VboxError> {
        let pxe_debug_enabled = if pxe_debug_enabled { 1 } else { 0 };
        get_function_result_unit!(self.object, SetPXEDebugEnabled, pxe_debug_enabled)
    }

    /// Flag to control whether the SMBIOS system UUID is presented in little endian form to the guest as mandated by the SMBIOS spec chapter 7.2.1.
    ///
    /// Before VirtualBox version 6.1 it was always presented in big endian form and to retain the old behavior this flag was introduced so it can be changed. VMs created with VBox 6.1 will default to true for this flag.
    ///
    /// # Returns
    ///
    /// Returns bool on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    /// use virtualbox_rs::VirtualBox;
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    ///
    /// let firmware_settings = machine.get_firmware_settings().unwrap();
    /// let sm_bios_uuid_little_endian = firmware_settings.get_sm_bios_uuid_little_endian().unwrap();
    /// ```
    pub fn get_sm_bios_uuid_little_endian(&self) -> Result<bool, VboxError> {
        get_function_result_bool!(self.object, GetSMBIOSUuidLittleEndian)
    }

    /// Flag to control whether the SMBIOS system UUID is presented in little endian form to the guest as mandated by the SMBIOS spec chapter 7.2.1.
    ///
    /// Before VirtualBox version 6.1 it was always presented in big endian form and to retain the old behavior this flag was introduced so it can be changed. VMs created with VBox 6.1 will default to true for this flag.
    ///
    /// # Arguments
    ///
    /// * `sm_bios_uuid_little_endian` - bool - Flag to control whether the SMBIOS system UUID is presented in little endian form to the guest as mandated by the SMBIOS spec chapter 7.2.1.
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
    /// use virtualbox_rs::enums::{FirmwareType, SessionType};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let mut session = Session::init().unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    /// let machine_mut = session.get_machine().unwrap();
    ///
    /// let firmware_settings = machine_mut.get_firmware_settings().unwrap();
    /// firmware_settings.set_sm_bios_uuid_little_endian(true).unwrap();
    /// ```
    pub fn set_sm_bios_uuid_little_endian(&self, sm_bios_uuid_little_endian: bool) -> Result<(), VboxError> {
        let sm_bios_uuid_little_endian = if sm_bios_uuid_little_endian { 1 } else { 0 };
        get_function_result_unit!(self.object, SetSMBIOSUuidLittleEndian, sm_bios_uuid_little_endian)
    }

    /// Flag for enabling automatic VM serial number generation.
    ///
    /// # Returns
    ///
    /// Returns bool on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    /// use virtualbox_rs::VirtualBox;
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    ///
    /// let firmware_settings = machine.get_firmware_settings().unwrap();
    /// let auto_serial_num_gen = firmware_settings.get_auto_serial_num_gen().unwrap();
    /// ```
    pub fn get_auto_serial_num_gen(&self) -> Result<bool, VboxError> {
        get_function_result_bool!(self.object, GetAutoSerialNumGen)
    }

    /// Flag for enabling automatic VM serial number generation.
    ///
    /// # Arguments
    ///
    /// * `auto_serial_num_gen` - bool - Flag for enabling automatic VM serial number generation. endian form to the guest as mandated by the SMBIOS spec chapter 7.2.1.
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
    /// use virtualbox_rs::enums::{FirmwareType, SessionType};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let mut session = Session::init().unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    /// let machine_mut = session.get_machine().unwrap();
    ///
    /// let firmware_settings = machine_mut.get_firmware_settings().unwrap();
    /// firmware_settings.set_auto_serial_num_gen(true).unwrap();
    /// ```
    pub fn set_auto_serial_num_gen(&self, auto_serial_num_gen: bool) -> Result<(), VboxError> {
        let auto_serial_num_gen = if auto_serial_num_gen { 1 } else { 0 };
        get_function_result_unit!(self.object, SetAutoSerialNumGen, auto_serial_num_gen)
    }
}