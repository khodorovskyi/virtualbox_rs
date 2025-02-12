use crate::enums::CPUArchitecture;
use crate::utility::macros::macros::{get_function_result_number, get_function_result_str};
use crate::{CPUProfile, VboxError};

impl CPUProfile {
    /// The name.
    ///
    /// # Returns
    ///
    /// Returns &str on success, or a [`VboxError`] on failure.
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
    /// let cpu_profile = cpu_profiles.get(0).unwrap();
    /// let name = cpu_profile.get_name().unwrap();
    pub fn get_name(&self) -> Result<&'static str, VboxError> {
        get_function_result_str!(self.object, GetName)
    }

    /// The full name.
    ///
    /// # Returns
    ///
    /// Returns &str on success, or a [`VboxError`] on failure.
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
    /// let cpu_profile = cpu_profiles.get(0).unwrap();
    /// let full_name = cpu_profile.get_full_name().unwrap();
    pub fn get_full_name(&self) -> Result<&'static str, VboxError> {
        get_function_result_str!(self.object, GetFullName)
    }

    /// The CPU architecture.
    ///
    /// # Returns
    ///
    /// Returns [`CPUArchitecture`] on success, or a [`VboxError`] on failure.
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
    /// let cpu_profile = cpu_profiles.get(0).unwrap();
    /// let architecture = cpu_profile.get_architecture().unwrap();
    pub fn get_architecture(&self) -> Result<CPUArchitecture, VboxError> {
        let architecture = get_function_result_number!(self.object, GetArchitecture, u32)?;
        Ok(CPUArchitecture::from(architecture))
    }
}
