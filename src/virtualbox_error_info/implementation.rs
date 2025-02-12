use crate::utility::macros::macros::get_function_result_str;
use crate::virtualbox_error_info::VirtualBoxErrorInfo;
use crate::VboxError;
use log::debug;

impl VirtualBoxErrorInfo {
    /// Text description of the error.
    ///
    ///
    /// # Returns
    ///
    /// Returns [&str] on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::VirtualBox;
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let access_error = machine.get_access_error().unwrap();
    ///
    /// let text = access_error.get_text().unwrap();
    ///
    pub fn get_text(&self) -> Result<&'static str, VboxError> {
        get_function_result_str!(self.object, GetText)
    }
}
