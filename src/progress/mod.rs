pub mod implementation;

use crate::utility::macros::macros::call_function;
use crate::VboxError;
use log::{debug, error};
use std::collections::BTreeMap;
use std::fmt::{Debug, Display, Formatter};
use vbox_raw::sys_lib::IProgress;

/// The Progress interface is used to track and control asynchronous tasks within VirtualBox.
///
/// **Reference to the official documentation:**
///
/// [https://www.virtualbox.org/sdkref/interface_i_progress.html](https://www.virtualbox.org/sdkref/interface_i_progress.html)
///
///
/// **Note:**
///
/// The [`Progress`] fields are typically filled during the [`Progress::wait_for_completion`] or
/// [`Progress::wait_for_operation_completion`] calls. If you check [`Progress::get_completed`] without prior
/// waiting via these methods, it will always return false, even after the operation
/// has completed.
///
pub struct Progress {
    object: *mut IProgress,
}

impl Progress {
    pub(crate) fn new(object: *mut IProgress) -> Self {
        Self { object }
    }

    fn release(&self) -> Result<i32, VboxError> {
        call_function!(self.object, Release)
    }
}

impl Drop for Progress {
    fn drop(&mut self) {
        match self.release() {
            Ok(count) => {
                debug!("Progress refcount: {}", count)
            }
            Err(err) => {
                error!("Failed drop Progress Error: {:?}", err)
            }
        }
    }
}

unsafe impl Send for Progress {}

impl Display for Progress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut map = BTreeMap::new();
        map.insert("completed", self.get_completed().unwrap_or(false).to_string());
        map.insert("id", self.get_id().unwrap_or("").to_string());
        map.insert(
            "description",
            self.get_description().unwrap_or("").to_string(),
        );
        map.insert(
            "cancelable",
            self.get_cancelable().unwrap_or(false).to_string(),
        );
        map.insert("percent", self.get_percent().unwrap_or(0).to_string());
        map.insert(
            "result_code",
            self.get_result_code().unwrap_or(0).to_string(),
        );
        map.insert(
            "operation_count",
            self.get_operation_count().unwrap_or(0).to_string(),
        );
        map.insert("operation", self.get_operation().unwrap_or(0).to_string());
        map.insert(
            "operation_description",
            self.get_operation_description().unwrap_or("").to_string(),
        );
        map.insert(
            "operation_percent",
            self.get_operation_percent().unwrap_or(0).to_string(),
        );
        map.insert(
            "operation_weight",
            self.get_operation_weight().unwrap_or(0).to_string(),
        );
        map.insert("timeout", self.get_timeout().unwrap_or(0).to_string());
        match self.get_error_info() {
            Ok(error_info) => {
                if f.alternate() {
                    map.insert("error_info", format!("{:#?}", error_info));
                }else {
                    map.insert("error_info", format!("{:?}", error_info));
                }
            }
            Err(_) => {}
        }

        if f.alternate() {
            write!(f, "{}", format!("{:#?}", map))
        } else {
            write!(f, "{}", format!("{:?}", map))
        }
    }
}

impl Debug for Progress {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}
