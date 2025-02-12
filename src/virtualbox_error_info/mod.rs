mod implementation;

use crate::utility::macros::macros::{call_function, get_function_result_str};
use crate::VboxError;
use log::{debug, error};
use std::fmt::{Debug, Display, Formatter};
use vbox_raw::sys_lib::IVirtualBoxErrorInfo;
/// The IVirtualBoxErrorInfo interface represents extended error information.
///
/// **Reference to the official documentation:**
///
/// [https://www.virtualbox.org/sdkref/interface_i_virtual_box_error_info.html](https://www.virtualbox.org/sdkref/interface_i_virtual_box_error_info.html)

pub struct VirtualBoxErrorInfo {
    object: *mut IVirtualBoxErrorInfo,
}

impl VirtualBoxErrorInfo {
    pub(crate) fn new(object: *mut IVirtualBoxErrorInfo) -> Self {
        Self { object }
    }

    fn err_to_string(&self) -> Result<&'static str, VboxError> {
        get_function_result_str!(self.object, ToString)
    }

    fn release(&self) -> Result<i32, VboxError> {
        call_function!(self.object, Release)
    }
}

impl Drop for VirtualBoxErrorInfo {
    fn drop(&mut self) {
        match self.release() {
            Ok(count) => {
                debug!("VirtualBoxErrorInfo refcount: {}", count)
            }
            Err(err) => {
                error!("Failed drop VirtualBoxErrorInfo Error: {:?}", err)
            }
        }
    }
}

impl Display for VirtualBoxErrorInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let err_str = match self.err_to_string() {
            Ok(err_str) => err_str.to_string(),
            Err(err) => {
                format!("{}", err)
            }
        };
        write!(f, "{}", format!("{}", err_str))
    }
}
impl Debug for VirtualBoxErrorInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}
