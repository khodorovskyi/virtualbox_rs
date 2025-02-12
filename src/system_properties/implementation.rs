use crate::enums::{CPUArchitecture, FrontEndName};
use crate::system_properties::SystemProperties;
#[cfg(not(is_v_7_1))]
use crate::utility::macros::macros::get_function_result_number;
use crate::utility::macros::macros::{get_function_result_pointer_vec};
use crate::utility::macros::macros::{get_function_result_str, get_function_result_unit};
use crate::utility::string_to_c_u64_str;
use crate::{CPUProfile, MediumFormat, VboxError};
#[cfg(doc)]
use crate::{Machine, VirtualBox};
#[cfg(not(is_v_6_1))]
use vbox_raw::sys_lib::ICPUProfile;
use vbox_raw::sys_lib::{IMediumFormat};
#[cfg(is_v_7_1)]
use vbox_raw::sys_lib::{ IPlatformProperties};
#[cfg(is_v_7_1)]
use crate::{PlatformProperties};
#[cfg(is_v_7_1)]
use crate::utility::macros::macros::{get_function_result_pointer};

impl SystemProperties {
    #[cfg(is_v_7_1)]
    /// Platform properties of the VirtualBox installation.
    ///
    /// # Returns
    ///
    /// Returns [`PlatformProperties`] on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::SystemProperties;
    ///
    /// let system_properties = SystemProperties::init().unwrap();
    ///
    /// let platform_properties = system_properties.get_platform_properties().unwrap();
    ///
    pub fn get_platform_properties(&self) -> Result<PlatformProperties, VboxError> {
        let platform_properties = get_function_result_pointer!(self.object, GetPlatform, *mut IPlatformProperties)?;
        Ok(PlatformProperties::new(platform_properties))
    }

    #[cfg(is_v_7_1)]
    /// Maximum device position in the boot order.
    ///
    /// # Returns
    ///
    /// Returns [u32] on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::SystemProperties;
    ///
    /// let system_properties = SystemProperties::init().unwrap();
    ///
    /// let count = system_properties.get_max_boot_position().unwrap();
    ///
    pub fn get_max_boot_position(&self) -> Result<u32, VboxError> {
        PlatformProperties::init()?.get_max_boot_position()
    }

    #[cfg(is_v_7_1)]
    /// Maximum number of serial ports associated with every [`Machine`] instance.
    ///
    /// # Returns
    ///
    /// Returns [u32] on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::SystemProperties;
    ///
    /// let system_properties = SystemProperties::init().unwrap();
    ///
    /// let count = system_properties.get_serial_port_count().unwrap();
    ///
    pub fn get_serial_port_count(&self) -> Result<u32, VboxError> {
        PlatformProperties::init()?.get_serial_port_count()
    }

    /// List of all medium storage formats supported by this VirtualBox installation.
    ///
    /// Keep in mind that the medium format identifier ([`MediumFormat::get_id`]) used in other API calls like [`VirtualBox::create_medium`] to refer to a particular medium format is a case-insensitive string. This means that, for example, all of the following strings:
    ///
    /// * VDI
    /// * vdi
    /// * VdI
    ///
    /// refer to the same medium format.
    ///
    /// Note that the virtual medium framework is backend-based, therefore the list of supported formats depends on what backends are currently installed.
    ///
    /// # Returns
    ///
    /// Returns [`Vec<MediumFormat>`] on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::SystemProperties;
    ///
    /// let system_properties = SystemProperties::init().unwrap();
    ///
    /// let medium_formats = system_properties.get_medium_formats().unwrap();
    ///
    pub fn get_medium_formats(&self) -> Result<Vec<MediumFormat>, VboxError> {
        let medium_formats =
            get_function_result_pointer_vec!(self.object, GetMediumFormats, *mut IMediumFormat)?;
        Ok(medium_formats
            .iter()
            .map(|object| MediumFormat::new(object.clone()))
            .collect())
    }

    #[cfg(is_v_7_1)]
    /// Maximum number of parallel ports associated with every [`Machine`] instance.
    ///
    /// # Returns
    ///
    /// Returns [u32] on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::SystemProperties;
    ///
    /// let system_properties = SystemProperties::init().unwrap();
    ///
    /// let count = system_properties.get_parallel_port_count().unwrap();
    ///
    pub fn get_parallel_port_count(&self) -> Result<u32, VboxError> {
        PlatformProperties::init()?.get_parallel_port_count()
    }

    /// Full path to the default directory used to create new or open existing machines when a machine settings file name contains no path.
    ///
    /// Starting with VirtualBox 4.0, by default, this attribute contains the full path of folder named "VirtualBox VMs" in the user's home directory, which depends on the host platform.
    ///
    /// When setting this attribute, a full path must be specified. Setting this property to null or an empty string or the special value "Machines" (for compatibility reasons) will restore that default value.
    ///
    /// If the folder specified herein does not exist, it will be created automatically as needed.
    ///
    /// # Returns
    ///
    /// Returns &str on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::SystemProperties;
    ///
    /// let system_properties = SystemProperties::init().unwrap();
    ///
    /// let folder = system_properties.get_default_machine_folder().unwrap();
    ///
    pub fn get_default_machine_folder(&self) -> Result<&'static str, VboxError> {
        get_function_result_str!(self.object, GetDefaultMachineFolder)
    }

    /// Selects which VM frontend should be used by default when launching a VM through the [`Machine::launch_vm_process`] method.
    ///
    /// # Returns
    ///
    /// Returns [`FrontEndName`] success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::SystemProperties;
    ///
    /// let system_properties = SystemProperties::init().unwrap();
    ///
    /// let folder = system_properties.get_default_machine_folder().unwrap();
    pub fn get_default_frontend(&self) -> Result<FrontEndName, VboxError> {
        let default_frontend = get_function_result_str!(self.object, GetDefaultFrontend)?;
        Ok(FrontEndName::from(default_frontend))
    }

    /// Selects which VM frontend should be used by default when launching a VM through the [`Machine::launch_vm_process`] method.
    ///
    /// Empty or null strings do not define a particular default, it is up to [`Machine::launch_vm_process`] to select one. See the description of [`Machine::launch_vm_process`] for the valid frontend types.
    ///
    /// This global setting is overridden by the per-VM attribute [`Machine::get_default_frontend`] or a frontend type passed to [`Machine::launch_vm_process`].
    /// # Arguments
    ///
    /// * `default_frontend` - [`FrontEndName`].
    ///
    /// # Returns
    ///
    /// Returns () success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::enums::FrontEndName;
    /// use virtualbox_rs::SystemProperties;
    ///
    /// let system_properties = SystemProperties::init().unwrap();
    ///
    /// let folder = system_properties.set_default_frontend(FrontEndName::Headless).unwrap();
    pub fn set_default_frontend(&self, default_frontend: FrontEndName) -> Result<(), VboxError> {
        let default_frontend = string_to_c_u64_str(default_frontend.into())?;
        get_function_result_unit!(self.object, SetDefaultFrontend, default_frontend)
    }

    #[cfg(not(is_v_6_1))]
    /// Returns CPU profiles matching the given criteria.
    ///
    /// # Arguments
    ///
    /// * `architecture` - [`CPUArchitecture`]. The architecture to get profiles for. Required.
    /// * `name_pattern` - &str. Name pattern. Simple wildcard matching using asterisk (*) and question mark (?).
    ///
    /// # Returns
    ///
    /// Returns [`Vec<CPUProfile>`] on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::enums::CPUArchitecture;
    /// use virtualbox_rs::SystemProperties;
    ///
    /// let system_properties = SystemProperties::init().unwrap();
    ///
    /// let cpu_profiles = system_properties.get_cpu_profiles(CPUArchitecture::Any, "*").unwrap();
    pub fn get_cpu_profiles(
        &self,
        architecture: CPUArchitecture,
        name_pattern: &str,
    ) -> Result<Vec<CPUProfile>, VboxError> {
        let architecture = architecture.into();
        let name_pattern = string_to_c_u64_str(name_pattern)?;

        let cpu_profiles = get_function_result_pointer_vec!(
            self.object,
            GetCPUProfiles,
            *mut ICPUProfile,
            architecture,
            name_pattern
        )?;
        Ok(cpu_profiles
            .iter()
            .map(|object| CPUProfile::new(object.clone()))
            .collect())
    }
}
#[cfg(not(is_v_7_1))]
impl SystemProperties {
    /// Maximum device position in the boot order.
    ///
    /// # Returns
    ///
    /// Returns [u32] on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::SystemProperties;
    ///
    /// let system_properties = SystemProperties::init().unwrap();
    ///
    /// let count = system_properties.get_max_boot_position().unwrap();
    ///
    pub fn get_max_boot_position(&self) -> Result<u32, VboxError> {
        get_function_result_number!(self.object, GetMaxBootPosition, u32)
    }

    /// Maximum number of serial ports associated with every [`Machine`] instance.
    ///
    /// # Returns
    ///
    /// Returns [u32] on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::SystemProperties;
    ///
    /// let system_properties = SystemProperties::init().unwrap();
    ///
    /// let count = system_properties.get_serial_port_count().unwrap();
    ///
    pub fn get_serial_port_count(&self) -> Result<u32, VboxError> {
        get_function_result_number!(self.object, GetSerialPortCount, u32)
    }

    /// Maximum number of parallel ports associated with every [`Machine`] instance.
    ///
    /// # Returns
    ///
    /// Returns [u32] on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::SystemProperties;
    ///
    /// let system_properties = SystemProperties::init().unwrap();
    ///
    /// let count = system_properties.get_parallel_port_count().unwrap();
    ///
    pub fn get_parallel_port_count(&self) -> Result<u32, VboxError> {
        get_function_result_number!(self.object, GetParallelPortCount, u32)
    }
}
#[cfg(is_v_6_1)]
impl SystemProperties {
    /// Placeholder Method
    ///
    /// This method serves as a placeholder for versions of the API where the actual method is not available.
    /// Attempting to call this method will result in an `UnsupportedInCurrentApiVersion` error.
    /// This method is intended to ensure that the codebase can be compiled against multiple API versions
    /// without modification.
    ///
    /// Supported from API version: v7_0
    pub fn get_cpu_profiles(
        &self,
        _architecture: CPUArchitecture,
        _name_pattern: &str,
    ) -> Result<Vec<CPUProfile>, VboxError> {
        Err(VboxError::unsupported_in_current_api_version(
            "SystemProperties::get_cpu_profiles",
            "v7_0",
        ))
    }
}
