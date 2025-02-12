use crate::enums::CPUArchitecture;
use crate::VboxError;

/// Placeholder Struct
///
/// This struct serves as a placeholder for versions of the API where the actual struct is not available.
/// This struct is intended to ensure that the codebase can be compiled against multiple API versions
/// without modification.
///
/// Supported from API version: v7_0
#[derive(Debug)]
pub struct CPUProfile {}

impl CPUProfile {
    /// Placeholder Method
    ///
    /// This method serves as a placeholder for versions of the API where the actual method is not available.
    /// Attempting to call this method will result in an `UnsupportedInCurrentApiVersion` error.
    /// This method is intended to ensure that the codebase can be compiled against multiple API versions
    /// without modification.
    ///
    /// Supported from API version: v7_0
    pub fn get_name(&self) -> Result<&'static str, VboxError> {
        Err(VboxError::unsupported_in_current_api_version(
            "AudioSettings::get_adapter",
            "v7_0",
        ))
    }

    /// Placeholder Method
    ///
    /// This method serves as a placeholder for versions of the API where the actual method is not available.
    /// Attempting to call this method will result in an `UnsupportedInCurrentApiVersion` error.
    /// This method is intended to ensure that the codebase can be compiled against multiple API versions
    /// without modification.
    ///
    /// Supported from API version: v7_0
    pub fn get_full_name(&self) -> Result<&'static str, VboxError> {
        Err(VboxError::unsupported_in_current_api_version(
            "AudioSettings::get_adapter",
            "v7_0",
        ))
    }

    /// Placeholder Method
    ///
    /// This method serves as a placeholder for versions of the API where the actual method is not available.
    /// Attempting to call this method will result in an `UnsupportedInCurrentApiVersion` error.
    /// This method is intended to ensure that the codebase can be compiled against multiple API versions
    /// without modification.
    ///
    /// Supported from API version: v7_0
    pub fn get_architecture(&self) -> Result<CPUArchitecture, VboxError> {
        Err(VboxError::unsupported_in_current_api_version(
            "AudioSettings::get_adapter",
            "v7_0",
        ))
    }
}
