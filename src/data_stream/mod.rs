mod implementation;

use crate::utility::macros::macros::call_function;
use crate::VboxError;
use log::{debug, error};
use vbox_raw::sys_lib::IDataStream;

/// The IDataStream interface is used to retrieve a data stream.
///
/// **Reference to the official documentation:**
///
/// [https://www.virtualbox.org/sdkref/interface_i_data_stream.html](https://www.virtualbox.org/sdkref/interface_i_data_stream.html)
#[derive(Debug)]
pub struct DataStream {
    pub(crate) object: *mut IDataStream,
}

impl DataStream {
    pub(crate) fn new(object: *mut IDataStream) -> Self {
        Self { object }
    }

    fn release(&self) -> Result<i32, VboxError> {
        call_function!(self.object, Release)
    }
}

impl Drop for DataStream {
    fn drop(&mut self) {
        match self.release() {
            Ok(count) => {
                debug!("DataStream refcount: {}", count)
            }
            Err(err) => {
                error!("Failed drop DataStream. Error: {:?}", err)
            }
        }
    }
}
