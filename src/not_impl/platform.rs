use crate::{PlatformARM, PlatformX86, VboxError};

/// Placeholder Struct
///
/// This struct serves as a placeholder for versions of the API where the actual struct is not available.
/// This struct is intended to ensure that the codebase can be compiled against multiple API versions
/// without modification.
///
/// Supported from API version: v7_1
#[derive(Debug)]
pub struct Platform {}

impl Platform {
    /// Placeholder Method
    ///
    /// This method serves as a placeholder for versions of the API where the actual method is not available.
    /// Attempting to call this method will result in an `UnsupportedInCurrentApiVersion` error.
    /// This method is intended to ensure that the codebase can be compiled against multiple API versions
    /// without modification.
    ///
    /// Supported from API version: v7_1
    pub fn get_arm(&self) -> Result<PlatformARM, VboxError> {
        Err(VboxError::unsupported_in_current_api_version(
            "AudioSettings::get_arm",
            "v7_1",
        ))
    }
    /// Placeholder Method
    ///
    /// This method serves as a placeholder for versions of the API where the actual method is not available.
    /// Attempting to call this method will result in an `UnsupportedInCurrentApiVersion` error.
    /// This method is intended to ensure that the codebase can be compiled against multiple API versions
    /// without modification.
    ///
    /// Supported from API version: v7_1
    pub fn get_x86(&self) -> Result<PlatformX86, VboxError> {
        Err(VboxError::unsupported_in_current_api_version(
            "AudioSettings::get_x86",
            "v7_1",
        ))
    }
}
