use crate::session::Session;
use crate::utility::macros::macros::get_function_result_pointer;
use crate::virtualbox::VirtualBox;
use crate::virtualbox_client::VirtualBoxClient;
use crate::VboxError;
use vbox_raw::sys_lib::{ISession, IVirtualBox};

impl VirtualBoxClient {
    /// Reference to the server-side API root object.
    ///
    ///
    /// # Returns
    ///
    /// Returns [`VirtualBox`] on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{VirtualBox, VirtualBoxClient};
    ///
    /// let vbox_client = VirtualBoxClient::init().unwrap();
    ///
    /// let vbox = vbox_client.get_virtualbox().unwrap();
    ///
    pub fn get_virtualbox(&self) -> Result<VirtualBox, VboxError> {
        let vbox = get_function_result_pointer!(self.object, GetVirtualBox, *mut IVirtualBox)?;
        Ok(VirtualBox::new(vbox))
    }

    /// Create a new session object and return it.
    ///
    ///
    /// # Returns
    ///
    /// Returns [`Session`] on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{VirtualBox, VirtualBoxClient};
    ///
    /// let vbox_client = VirtualBoxClient::init().unwrap();
    ///
    /// let session = vbox_client.get_session().unwrap();
    ///
    pub fn get_session(&self) -> Result<Session, VboxError> {
        let session = get_function_result_pointer!(self.object, GetSession, *mut ISession)?;
        Ok(Session::new(session))
    }
}
