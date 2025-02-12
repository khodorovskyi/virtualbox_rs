use vbox_raw::sys_lib::{IPlatformARM, IPlatformProperties, IPlatformX86};
use crate::{Platform, PlatformARM, PlatformProperties, PlatformX86, VboxError};
use crate::enums::{ChipsetType, IommuType, PlatformArchitecture};
use crate::utility::macros::macros::{get_function_result_bool, get_function_result_number, get_function_result_pointer, get_function_result_unit};

impl Platform {
    /// Returns the platform architecture.
    ///
    /// # Returns
    ///
    /// Returns [`PlatformArchitecture`] on success, or a [`VboxError`] on failure.
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
    /// let architecture = platform.get_architecture().unwrap();
    pub fn get_architecture(&self) -> Result<PlatformArchitecture, VboxError> {
        let architecture = get_function_result_number!(self.object, GetArchitecture, u32)?;
        Ok(PlatformArchitecture::from(architecture))
    }
    /// The platform architecture.
    ///
    /// # Arguments
    ///
    /// * `architecture` - [`PlatformArchitecture`] The new platform architecture.
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
    /// use virtualbox_rs::enums::{PlatformArchitecture, SessionType};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let mut session = Session::init().unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    /// let machine_mut = session.get_machine().unwrap();
    /// let platform = machine_mut.get_platform().unwrap();
    /// platform.set_architecture(PlatformArchitecture::X86).unwrap();
    pub fn set_architecture(&self, architecture: PlatformArchitecture) -> Result<(), VboxError> {
        let architecture = architecture.into();
        get_function_result_unit!(self.object, SetArchitecture, architecture)
    }

    /// Returns the platform architecture.
    ///
    /// # Returns
    ///
    /// Returns [`PlatformProperties`] on success, or a [`VboxError`] on failure.
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
    /// let properties = platform.get_properties().unwrap();
    pub fn get_properties(&self) -> Result<PlatformProperties, VboxError> {
        let properties = get_function_result_pointer!(self.object, GetProperties, *mut IPlatformProperties)?;
        Ok(PlatformProperties::new(properties))
    }

    /// Platform object for x86-specific platform properties.
    ///
    /// # Returns
    ///
    /// Returns [`PlatformX86`] on success, or a [`VboxError`] on failure.
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
    pub fn get_x86(&self) -> Result<PlatformX86, VboxError> {
        let platform_x86 = get_function_result_pointer!(self.object, GetX86, *mut IPlatformX86)?;
        Ok(PlatformX86::new(platform_x86))
    }

    /// Platform object for ARM-specific platform properties.
    ///
    /// # Returns
    ///
    /// Returns [`PlatformARM`] on success, or a [`VboxError`] on failure.
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
    /// let platform_arm = platform.get_arm().unwrap();
    pub fn get_arm(&self) -> Result<PlatformARM, VboxError> {
        let platform_arm = get_function_result_pointer!(self.object, GetARM, *mut IPlatformARM)?;
        Ok(PlatformARM::new(platform_arm))
    }

    /// Chipset type used in this VM.
    ///
    /// # Returns
    ///
    /// Returns [`ChipsetType`] on success, or a [`VboxError`] on failure.
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
    /// let chipset_type = platform.get_chipset_type().unwrap();
    pub fn get_chipset_type(&self) -> Result<ChipsetType, VboxError> {
        let chipset_type = get_function_result_number!(self.object, GetChipsetType, u32)?;
        Ok(ChipsetType::from(chipset_type))
    }
    /// Chipset type used in this VM.
    ///
    /// # Arguments
    ///
    /// * `architecture` - [`ChipsetType`] Chipset type used in this VM.
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
    /// use virtualbox_rs::enums::{ChipsetType, SessionType};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let mut session = Session::init().unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    /// let machine_mut = session.get_machine().unwrap();
    /// let platform = machine_mut.get_platform().unwrap();
    /// platform.set_chipset_type(ChipsetType::ICH9).unwrap();
    pub fn set_chipset_type(&self, chipset_type: ChipsetType) -> Result<(), VboxError> {
        let chipset_type = chipset_type.into();
        get_function_result_unit!(self.object, SetChipsetType, chipset_type)
    }
    /// IOMMU type used in this VM.
    ///
    /// # Returns
    ///
    /// Returns [`IommuType`] on success, or a [`VboxError`] on failure.
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
    /// let iommu_type = platform.get_iommu_type().unwrap();
    pub fn get_iommu_type(&self) -> Result<IommuType, VboxError> {
        let iommu_type = get_function_result_number!(self.object, GetIommuType, u32)?;
        Ok(IommuType::from(iommu_type))
    }

    /// IOMMU type used in this VM.
    ///
    /// # Arguments
    ///
    /// * `architecture` - [`IommuType`] IOMMU type used in this VM.
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
    /// use virtualbox_rs::enums::{IommuType, SessionType};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let mut session = Session::init().unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    /// let machine_mut = session.get_machine().unwrap();
    /// let platform = machine_mut.get_platform().unwrap();
    /// platform.set_iommu_type(IommuType::Intel).unwrap();
    pub fn set_iommu_type(&self, iommu_type: IommuType) -> Result<(), VboxError> {
        let iommu_type = iommu_type.into();
        get_function_result_unit!(self.object, SetIommuType, iommu_type)
    }

    /// When set to true, the RTC device of the virtual machine will run in UTC time, otherwise in local time.
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
    /// let rtc_use_utc = platform.get_rtc_use_utc().unwrap();
    pub fn get_rtc_use_utc(&self) -> Result<bool, VboxError> {
        get_function_result_bool!(self.object, GetRTCUseUTC)
    }

    /// When set to true, the RTC device of the virtual machine will run in UTC time, otherwise in local time.
    ///
    /// # Arguments
    ///
    /// * `architecture` -  When set to true, the RTC device of the virtual machine will run in UTC time, otherwise in local time.
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
    /// platform.set_rtc_use_utc(true).unwrap();
    pub fn set_rtc_use_utc(&self, rtc_use_utc: bool) -> Result<(), VboxError> {
        let rtc_use_utc = if rtc_use_utc { 1 } else { 0 };
        get_function_result_unit!(self.object, SetRTCUseUTC, rtc_use_utc)
    }
}