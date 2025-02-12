use crate::utility::macros::macros::call_function;
use crate::VboxError;
use log::{debug, error};
use vbox_raw::sys_lib::IToken;

pub mod implementation;
/// The Token interface represents a token passed to an API client, which triggers cleanup actions when it is explicitly released by calling the abandon method (preferred, as it is accurately defined when the release happens), or when the object reference count drops to 0.
///
/// **Reference to the official documentation:**
///
/// [https://www.virtualbox.org/sdkref/interface_i_token.html](https://www.virtualbox.org/sdkref/interface_i_token.html)
#[derive(Debug)]
pub struct Token {
    object: *mut IToken,
}

impl Token {
    pub(crate) fn new(object: *mut IToken) -> Self {
        Self { object }
    }

    fn release(&self) -> Result<i32, VboxError> {
        call_function!(self.object, Release)
    }
}

impl Drop for Token {
    fn drop(&mut self) {
        match self.release() {
            Ok(count) => {
                debug!("Token refcount: {}", count)
            }
            Err(err) => {
                error!("Failed drop Token. Error: {:?}", err)
            }
        }
    }
}
