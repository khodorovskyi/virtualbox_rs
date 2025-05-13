#![allow(non_camel_case_types)]

use std::fmt::Display;

#[derive(Debug, Eq, PartialEq)]
pub enum VboxErrorType {
    ErrorInit,
    GetFnError,
    NullPointerError,
    ParseError,
    ReleaseError,
    ConvertStringError,
    /// This error occurs when the requested method is not supported by the current API version.
    UnsupportedInCurrentApiVersion,
    IncorrectVersion {
        raw_ver: String,
        vbox_sys_ver: String,
        vbox_ver: u32,
        vbox_api_ver: u32,
        build_ver: u32,
    },
    /// An internal error occurs when vectors have different lengths, which is required for consistent processing.
    VectorsLengthMismatch,
    /// A drag and drop error has occurred.
    VBOX_E_DND_ERROR,
    /// Returned when an unexpected error occurs
    NS_ERROR_UNEXPECTED,
    /// Returned when a class cannot be registered, but may be tried again later
    NS_ERROR_FACTORY_REGISTER_AGAIN,
    /// File not accessible or erroneous file contents.
    VBOX_E_FILE_ERROR,
    /// Guest Control reported an error from the guest side.
    VBOX_E_GSTCTL_GUEST_ERROR,
    /// Returned when an operation can't complete due to an unavailable resource
    NS_ERROR_NOT_AVAILABLE,
    /// A maximum has been reached.
    VBOX_E_MAXIMUM_REACHED,
    /// Modification request refused.
    E_ACCESSDENIED,
    /// Pluggable Device Manager error.
    VBOX_E_PDM_ERROR,
    /// Returned by a not implemented function
    NS_ERROR_NOT_IMPLEMENTED,
    /// Returned when a given interface is not supported.
    NS_NOINTERFACE,
    /// Returned when a class doesn't allow aggregation
    NS_ERROR_NO_AGGREGATION,
    /// A provided password was incorrect.
    VBOX_E_PASSWORD_INCORRECT,
    /// Returned when an unexpected error occurs
    NS_ERROR_BASE,
    /// Returned when a function fails
    NS_ERROR_FAILURE,
    /// Returned when a pointer is invalid
    NS_ERROR_INVALID_POINTER,
    /// Returned when a dynamically loaded factory couldn't be found
    NS_ERROR_FACTORY_NOT_LOADED,
    /// Could not parse the settings file.
    VBOX_E_XML_ERROR,
    /// Current session state prohibits operation.
    VBOX_E_INVALID_SESSION_STATE,
    /// Object being in use prohibits operation.
    VBOX_E_OBJECT_IN_USE,
    /// Runtime subsystem error.
    VBOX_E_IPRT_ERROR,
    /// Requested operation is not supported.
    VBOX_E_NOT_SUPPORTED,
    /// Returned when a memory allocation fails
    NS_ERROR_OUT_OF_MEMORY,
    /// Returned when a function aborts
    NS_ERROR_ABORT,
    /// Returned when an illegal value is passed
    NS_ERROR_ILLEGAL_VALUE,
    /// Host operating system related error
    VBOX_E_HOST_ERROR,
    /// Current object state prohibits operation.
    VBOX_E_INVALID_OBJECT_STATE,
    /// Returned when a class is not registered
    NS_ERROR_FACTORY_NOT_REGISTERED,
    /// The operation ran into an explicitly requested timeout.
    VBOX_E_TIMEOUT,
    /// Virtual machine error occurred attempting the operation.
    VBOX_E_VM_ERROR,
    /// Current virtual machine state prevents the operation.
    VBOX_E_INVALID_VM_STATE,
    /// Object corresponding to the supplied arguments does not exist.
    VBOX_E_OBJECT_NOT_FOUND,
    /// Platform architecture is not supported.
    VBOX_E_PLATFORM_ARCH_NOT_SUPPORTED,
    /// A recording error has occurred.
    VBOX_E_RECORDING_ERROR,
    /// Unknown error
    UNKNOWN(u32),
}

impl From<u32> for VboxErrorType {
    fn from(value: u32) -> Self {
        let value_str = vbox_raw::get_error_msg(value);
        match value_str.as_str() {
            "VBOX_E_DND_ERROR" => Self::VBOX_E_DND_ERROR,
            "NS_ERROR_UNEXPECTED" => Self::NS_ERROR_UNEXPECTED,
            "NS_ERROR_FACTORY_REGISTER_AGAIN" => Self::NS_ERROR_FACTORY_REGISTER_AGAIN,
            "VBOX_E_FILE_ERROR" => Self::VBOX_E_FILE_ERROR,
            "VBOX_E_GSTCTL_GUEST_ERROR" => Self::VBOX_E_GSTCTL_GUEST_ERROR,
            "NS_ERROR_NOT_AVAILABLE" => Self::NS_ERROR_NOT_AVAILABLE,
            "VBOX_E_MAXIMUM_REACHED" => Self::VBOX_E_MAXIMUM_REACHED,
            "E_ACCESSDENIED" => Self::E_ACCESSDENIED,
            "VBOX_E_PDM_ERROR" => Self::VBOX_E_PDM_ERROR,
            "NS_ERROR_NOT_IMPLEMENTED" => Self::NS_ERROR_NOT_IMPLEMENTED,
            "NS_NOINTERFACE" => Self::NS_NOINTERFACE,
            "NS_ERROR_NO_AGGREGATION" => Self::NS_ERROR_NO_AGGREGATION,
            "VBOX_E_PASSWORD_INCORRECT" => Self::VBOX_E_PASSWORD_INCORRECT,
            "NS_ERROR_BASE" => Self::NS_ERROR_BASE,
            "NS_ERROR_FAILURE" => Self::NS_ERROR_FAILURE,
            "NS_ERROR_INVALID_POINTER" => Self::NS_ERROR_INVALID_POINTER,
            "NS_ERROR_FACTORY_NOT_LOADED" => Self::NS_ERROR_FACTORY_NOT_LOADED,
            "VBOX_E_XML_ERROR" => Self::VBOX_E_XML_ERROR,
            "VBOX_E_INVALID_SESSION_STATE" => Self::VBOX_E_INVALID_SESSION_STATE,
            "VBOX_E_OBJECT_IN_USE" => Self::VBOX_E_OBJECT_IN_USE,
            "VBOX_E_IPRT_ERROR" => Self::VBOX_E_IPRT_ERROR,
            "VBOX_E_NOT_SUPPORTED" => Self::VBOX_E_NOT_SUPPORTED,
            "NS_ERROR_OUT_OF_MEMORY" => Self::NS_ERROR_OUT_OF_MEMORY,
            "NS_ERROR_ABORT" => Self::NS_ERROR_ABORT,
            "NS_ERROR_ILLEGAL_VALUE" => Self::NS_ERROR_ILLEGAL_VALUE,
            "VBOX_E_HOST_ERROR" => Self::VBOX_E_HOST_ERROR,
            "VBOX_E_INVALID_OBJECT_STATE" => Self::VBOX_E_INVALID_OBJECT_STATE,
            "NS_ERROR_FACTORY_NOT_REGISTERED" => Self::NS_ERROR_FACTORY_NOT_REGISTERED,
            "VBOX_E_TIMEOUT" => Self::VBOX_E_TIMEOUT,
            "VBOX_E_VM_ERROR" => Self::VBOX_E_VM_ERROR,
            "VBOX_E_INVALID_VM_STATE" => Self::VBOX_E_INVALID_VM_STATE,
            "VBOX_E_OBJECT_NOT_FOUND" => Self::VBOX_E_OBJECT_NOT_FOUND,
            "VBOX_E_PLATFORM_ARCH_NOT_SUPPORTED" => Self::VBOX_E_PLATFORM_ARCH_NOT_SUPPORTED,
            "VBOX_E_RECORDING_ERROR" => Self::VBOX_E_RECORDING_ERROR,
            _ => Self::UNKNOWN(value),
        }
    }
}

impl Display for VboxErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self))
    }
}
