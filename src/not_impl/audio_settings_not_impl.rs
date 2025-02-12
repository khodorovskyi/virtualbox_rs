use crate::{AudioAdapter, VboxError};

/// Placeholder Struct
///
/// This struct serves as a placeholder for versions of the API where the actual struct is not available.
/// This struct is intended to ensure that the codebase can be compiled against multiple API versions
/// without modification.
///
/// Supported from API version: v7_0
pub struct AudioSettings {}

impl AudioSettings {
    /// Placeholder Method
    ///
    /// This method serves as a placeholder for versions of the API where the actual method is not available.
    /// Attempting to call this method will result in an `UnsupportedInCurrentApiVersion` error.
    /// This method is intended to ensure that the codebase can be compiled against multiple API versions
    /// without modification.
    ///
    /// Supported from API version: v7_0
    pub fn get_adapter(&self) -> Result<AudioAdapter, VboxError> {
        Err(VboxError::unsupported_in_current_api_version(
            "AudioSettings::get_adapter",
            "v7_0",
        ))
    }
}
