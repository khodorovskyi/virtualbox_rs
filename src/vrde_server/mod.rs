mod implementation;

use std::collections::BTreeMap;
use std::fmt::{Debug, Formatter};
use crate::utility::macros::macros::call_function;
use crate::{VboxError};
use log::{debug, error};
use vbox_raw::sys_lib::IVRDEServer;
use crate::enums::AuthType;

///
///
/// **Reference to the official documentation:**
///
/// [https://www.virtualbox.org/sdkref/interface_i_v_r_d_e_server.html](https://www.virtualbox.org/sdkref/interface_i_v_r_d_e_server.html)
#[derive(Clone)]
pub struct VRDEServer {
    object: *mut IVRDEServer,
}

impl VRDEServer {
    pub(crate) fn new(object: *mut IVRDEServer) -> Self {
        Self { object }
    }

    fn release(&self) -> Result<i32, VboxError> {
        call_function!(self.object, Release)
    }
}

impl Drop for VRDEServer {
    fn drop(&mut self) {
        match self.release() {
            Ok(count) => {
                debug!("VRDEServer refcount: {}", count)
            }
            Err(err) => {
                error!("Failed drop VRDEServer. Error: {:?}", err)
            }
        }
    }
}


impl std::fmt::Display for VRDEServer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut map = BTreeMap::new();
        map.insert("enabled", self.get_enabled().unwrap_or(false).to_string());
        map.insert("auth_type",  self.get_auth_type().unwrap_or(AuthType::Null).to_string());
        map.insert("auth_timeout",  self.get_auth_timeout().unwrap_or(0).to_string());
        map.insert("allow_multi_connection",  self.get_allow_multi_connection().unwrap_or(false).to_string());
        map.insert("reuse_single_connection",  self.get_reuse_single_connection().unwrap_or(false).to_string());
        map.insert("get_vrde_ext_pack", self.get_vrde_ext_pack().unwrap_or("").to_string());
        map.insert("auth_library",  self.get_auth_library().unwrap_or("").to_string());
        if f.alternate() {
            write!(f, "{}", format!("{:#?}", map))
        } else {
            write!(f, "{}", format!("{:?}", map))
        }
    }
}
impl Debug for VRDEServer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}