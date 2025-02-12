use crate::enums::GuestSessionStatus;
use crate::utility::macros::macros::call_function;
#[cfg(doc)]
use crate::Guest;
use crate::VboxError;
use log::{debug, error};
use std::collections::BTreeMap;
use std::fmt::{Debug, Display, Formatter};
use vbox_raw::sys_lib::IGuestSession;

mod implementation;

/// A guest session represents one impersonated user account in the guest, so every operation will use the same credentials specified when creating the session object via [`Guest::create_session`]
///
/// **Reference to the official documentation:**
///
/// [https://www.virtualbox.org/sdkref/interface_i_guest_session.html](https://www.virtualbox.org/sdkref/interface_i_guest_session.html)

pub struct GuestSession {
    object: *mut IGuestSession,
}

impl GuestSession {
    pub(crate) fn new(object: *mut IGuestSession) -> Self {
        Self { object }
    }

    fn release(&self) -> Result<i32, VboxError> {
        call_function!(self.object, Release)
    }
}

impl Drop for GuestSession {
    fn drop(&mut self) {
        match self.release() {
            Ok(count) => {
                debug!("GuestSession refcount: {}", count)
            }
            Err(err) => {
                error!("Failed drop GuestSession. Error: {:?}", err)
            }
        }
    }
}

impl Display for GuestSession {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut map = BTreeMap::new();
        map.insert("user", self.get_user().unwrap_or("").to_string());
        map.insert("domain", self.get_domain().unwrap_or("").to_string());
        map.insert("name", self.get_name().unwrap_or("").to_string());
        map.insert("id", self.get_id().unwrap_or(0).to_string());
        map.insert("timeout", self.get_timeout().unwrap_or(0).to_string());
        map.insert(
            "protocol_version",
            self.get_protocol_version().unwrap_or(0).to_string(),
        );
        map.insert(
            "status",
            self.get_status()
                .unwrap_or(GuestSessionStatus::Undefined)
                .to_string(),
        );
        if f.alternate() {
            write!(f, "{}", format!("{:#?}", map))
        } else {
            write!(f, "{}", format!("{:?}", map))
        }
    }
}

impl Debug for GuestSession {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}