use crate::{Token, VboxError};
use crate::utility::macros::macros::get_function_result_unit;

impl Token {
    /// Releases this token.
    ///
    /// Cannot be undone in any way, and makes the token object unusable (even the dummy method will return an error), ready for releasing. It is a more defined way than just letting the reference count drop to 0, because the latter (depending on the platform) can trigger asynchronous cleanup activity.
    ///
    /// # Returns
    ///
    /// Returns () on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::VirtualBox;
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let mediums = vbox.get_hard_disks().unwrap();
    /// let medium = mediums.get(0).unwrap();
    /// let token = medium.lock_write().unwrap();
    /// token.abandon().unwrap();
    ///```
    pub fn abandon(&self) -> Result<(), VboxError> {
        get_function_result_unit!(self.object, Abandon)
    }

    /// Purely a NOOP.
    ///
    /// Useful when using proxy type API bindings (e.g. the webservice) which manage objects on behalf of the actual client, using an object reference expiration time based garbage collector.
    ///
    /// # Returns
    ///
    /// Returns () on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::VirtualBox;
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let mediums = vbox.get_hard_disks().unwrap();
    /// let medium = mediums.get(0).unwrap();
    /// let token = medium.lock_write().unwrap();
    /// token.dummy().unwrap();
    ///```
    pub fn dummy(&self) -> Result<(), VboxError> {
        get_function_result_unit!(self.object, Dummy)
    }
}