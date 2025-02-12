use crate::{VboxError};
use crate::enums::{GuestDebugIoProvider, GuestDebugProvider};

/// Placeholder Struct
///
/// This struct serves as a placeholder for versions of the API where the actual struct is not available.
/// This struct is intended to ensure that the codebase can be compiled against multiple API versions
/// without modification.
///
/// Supported from API version: v7_0
#[derive(Debug)]
pub struct GuestDebugControl {}

impl GuestDebugControl {
    /// Placeholder Method
    ///
    /// This method serves as a placeholder for versions of the API where the actual method is not available.
    /// Attempting to call this method will result in an `UnsupportedInCurrentApiVersion` error.
    /// This method is intended to ensure that the codebase can be compiled against multiple API versions
    /// without modification.
    ///
    /// Supported from API version: v7_0
    pub fn get_debug_provider(&self) -> Result<GuestDebugProvider, VboxError> {
        Err(VboxError::unsupported_in_current_api_version(
            "GuestDebugControl::get_debug_provider",
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
    pub fn set_debug_provider(&self, _debug_provider: GuestDebugProvider) -> Result<(), VboxError> {
        Err(VboxError::unsupported_in_current_api_version(
            "GuestDebugControl::set_debug_provider",
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
    pub fn get_debug_io_provider(&self) -> Result<GuestDebugIoProvider, VboxError> {
        Err(VboxError::unsupported_in_current_api_version(
            "GuestDebugControl::get_debug_io_provider",
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
    pub fn set_debug_io_provider(
        &self,
        _debug_io_provider: GuestDebugIoProvider,
    ) -> Result<(), VboxError> {
        Err(VboxError::unsupported_in_current_api_version(
            "GuestDebugControl::set_debug_io_provider",
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
    pub fn get_debug_address(&self) -> Result<&'static str, VboxError> {
        Err(VboxError::unsupported_in_current_api_version(
            "GuestDebugControl::get_debug_address",
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
    pub fn set_debug_address(&self, _debug_address: &str) -> Result<(), VboxError> {
        Err(VboxError::unsupported_in_current_api_version(
            "GuestDebugControl::set_debug_address",
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
    pub fn get_debug_port(&self) -> Result<u32, VboxError> {
        Err(VboxError::unsupported_in_current_api_version(
            "GuestDebugControl::get_debug_port",
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
    pub fn set_debug_port(&self, _debug_port: u32) -> Result<(), VboxError> {
        Err(VboxError::unsupported_in_current_api_version(
            "GuestDebugControl::set_debug_port",
            "v7_0",
        ))
    }
}
