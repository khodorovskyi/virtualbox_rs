use crate::utility::macros::macros::call_function;
use crate::VboxError;
use log::{debug, error};
use vbox_raw::sys_lib::IPerformanceCollector;

/// The IPerformanceCollector interface represents a service that collects and stores performance metrics data.
///
/// **Reference to the official documentation:**
///
/// [https://www.virtualbox.org/sdkref/interface_i_performance_collector.html](https://www.virtualbox.org/sdkref/interface_i_performance_collector.html)
#[derive(Debug)]
pub struct PerformanceCollector {
    object: *mut IPerformanceCollector,
}

impl PerformanceCollector {
    pub(crate) fn new(object: *mut IPerformanceCollector) -> Self {
        Self { object }
    }

    fn release(&self) -> Result<i32, VboxError> {
        call_function!(self.object, Release)
    }
}

impl Drop for PerformanceCollector {
    fn drop(&mut self) {
        match self.release() {
            Ok(count) => {
                debug!("PerformanceCollector refcount: {}", count)
            }
            Err(err) => {
                error!("Failed drop PerformanceCollector. Error: {:?}", err)
            }
        }
    }
}
