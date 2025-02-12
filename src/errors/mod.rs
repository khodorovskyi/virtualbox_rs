use crate::VboxErrorType;
use std::fmt::Display;

pub(crate) mod vbox_error_type;
#[derive(Debug, Eq, PartialEq)]
pub struct VboxError {
    pub error_type: VboxErrorType,
    pub code: u32,
    pub fn_name: String,
    pub msg: String,
}

impl VboxError {
    pub fn new(code: u32, fn_name: &str, msg: String, error_type: Option<VboxErrorType>) -> Self {
        let error_type = match error_type {
            Some(error_type) => error_type,
            None => VboxErrorType::from(code),
        };

        Self {
            error_type,
            code,
            fn_name: fn_name.to_string(),
            msg,
        }
    }
    pub fn incorrect_version(
        raw_ver: String,
        vbox_sys_ver: String,
        vbox_ver: u32,
        vbox_api_ver: u32,
        build_ver: u32,
    ) -> Self {
        Self {
            error_type: VboxErrorType::IncorrectVersion {
                raw_ver,
                vbox_sys_ver,
                vbox_ver,
                vbox_api_ver,
                build_ver,
            },
            code: 0,
            fn_name: "".to_string(),
            msg: "".to_string(),
        }
    }

    pub fn get_fn_error(fn_name: &str) -> Self {
        Self {
            error_type: VboxErrorType::GetFnError,
            code: 0,
            fn_name: fn_name.to_string(),
            msg: "".to_string(),
        }
    }

    pub fn null_pointer_error(fn_name: &str) -> Self {
        Self {
            error_type: VboxErrorType::NullPointerError,
            code: 0,
            fn_name: fn_name.to_string(),
            msg: "".to_string(),
        }
    }
    pub fn release_error(fn_name: &str) -> Self {
        Self {
            error_type: VboxErrorType::ReleaseError,
            code: 0,
            fn_name: fn_name.to_string(),
            msg: "".to_string(),
        }
    }
    pub fn error_init() -> Self {
        Self {
            error_type: VboxErrorType::ErrorInit,
            code: 0,
            fn_name: "".to_string(),
            msg: "".to_string(),
        }
    }

    pub fn unsupported_in_current_api_version(fn_name: &str, supported_version: &str) -> Self {
        Self {
            error_type: VboxErrorType::UnsupportedInCurrentApiVersion,
            code: 0,
            fn_name: fn_name.to_string(),
            msg: format!("This error occurs when the requested method is not supported by the current API version. Supported from version: {}", supported_version),
        }
    }
    pub fn vectors_length_mismatch(fn_name: &str) -> Self {
        Self {
            error_type: VboxErrorType::VectorsLengthMismatch,
            code: 0,
            fn_name: fn_name.to_string(),
            msg: "".to_string(),
        }
    }
    pub fn is_null(&self) -> bool {
        self.error_type == VboxErrorType::NullPointerError
    }
}

impl Display for VboxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self))
    }
}
