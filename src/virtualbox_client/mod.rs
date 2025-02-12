mod implementation;

use crate::core::{g_pVBoxFuncs, get_version, pfnGetAPIVersion, pfnGetVersion};
use crate::utility::macros::macros::call_function;
use crate::VboxError;
use log::{debug, error};
use vbox_raw::sys_lib::IVirtualBoxClient;
use vbox_raw::BUILD_VER;

/// Convenience interface for client applications.
///
/// **Reference to the official documentation:**
///
/// [https://www.virtualbox.org/sdkref/interface_i_virtual_box_client.html](https://www.virtualbox.org/sdkref/interface_i_virtual_box_client.html)

pub struct VirtualBoxClient {
    object: *mut IVirtualBoxClient,
}

impl VirtualBoxClient {
    /// Initializes the VirtualBox client.
    ///
    /// # Returns
    ///
    /// Returns [`VirtualBoxClient`] on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    /// use virtualbox_rs::VirtualBoxClient;
    ///
    /// let client = VirtualBoxClient::init().unwrap();
    /// ```
    ///
    /// # Details
    ///
    /// This method checks the VirtualBox version before initializing the client.
    /// It ensures compatibility and prevents potential issues due to version mismatches.
    /// Use this method if you have not checked the version beforehand.

    pub fn init() -> Result<Self, VboxError> {
        Self::check_version()?;
        Self::init_unchecked()
    }

    /// Initializes the VirtualBox client without checking the version.
    ///
    /// # Returns
    ///
    /// Returns [`VirtualBoxClient`] on success, or a [`VboxError`] on failure.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use virtualbox_rs::VirtualBoxClient;
    ///
    /// let client = VirtualBoxClient::init_unchecked().unwrap();
    /// ```
    ///
    /// # Details
    ///
    /// This method skips the version check and directly initializes the client.
    /// Use this method only if you have already checked the version and are confident it is correct.
    /// If the version is not checked and does not match, the application may crash with a core dump on random method calls.
    /// The speed of calling `init_unchecked` is minimally different from the regular `init`.
    pub fn init_unchecked() -> Result<Self, VboxError> {
        debug!("get_vboxclient");
        Self::check_version()?;
        let api = g_pVBoxFuncs()?;
        let mut virtualbox_client_ptr: *mut IVirtualBoxClient = std::ptr::null_mut();

        let fn_ptr = unsafe { (*api).pfnClientInitialize }
            .ok_or(VboxError::get_fn_error("GetVirtualBox"))?;
        let result = unsafe { fn_ptr(std::ptr::null(), &mut virtualbox_client_ptr) };
        debug!("{}", result);
        if result != 0 || virtualbox_client_ptr.is_null() {
            // return Err(VboxError::from(result))
            return Err(VboxError::new(
                result,
                "VirtualBoxClient::init",
                "".to_string(),
                None,
            ));
        }
        Ok(Self {
            object: virtualbox_client_ptr,
        })
    }

    /// Validates the compatibility of the VirtualBox version with the library.
    ///
    /// # Returns
    ///
    /// - `Ok(())` if the detected VirtualBox version matches the required `major.minor` version.
    /// - `Err(VboxError)` if the VirtualBox version is incompatible or cannot be determined.
    ///
    /// # Details
    ///
    /// This function checks the version of the installed VirtualBox by querying its API.
    /// The comparison is performed against the `major.minor` version required by the library.
    ///
    /// If the version is incompatible (e.g., major or minor version mismatch), this function
    /// returns an error. Using the library with an incompatible VirtualBox version can lead
    /// to undefined behavior, including crashes (e.g., `Segmentation fault`).
    ///
    /// # Example
    ///
    /// ```no_run
    ///
    ///     use virtualbox_rs::VirtualBoxClient;
    ///
    ///     VirtualBoxClient::check_version().unwrap();
    ///     println!("VirtualBox version is compatible!");
    /// ```
    ///
    /// # Notes
    ///
    /// - It is strongly recommended to call this method at the start of your application.
    /// - If the check fails, the program should terminate immediately to avoid unexpected behavior.

    pub fn check_version() -> Result<(), VboxError> {
        let raw_ver = vbox_raw::get_version();
        let vbox_sys_ver = get_version();
        let vbox_ver = pfnGetVersion()?;
        let current_major = vbox_ver / 1_000_000;
        let current_minor = (vbox_ver / 1_000) % 1_000;
        let vbox_api_ver = pfnGetAPIVersion()?;
        let error = Err(VboxError::incorrect_version(
            raw_ver.clone(),
            vbox_sys_ver.to_string(),
            vbox_ver,
            vbox_api_ver,
            BUILD_VER,
        ));

        if raw_ver != vbox_sys_ver {
            return error;
        }

        if raw_ver == "v6_1".to_string()
            && (current_major != 6 || current_minor != 1 || BUILD_VER != 61)
        {
            return error;
        }

        if raw_ver == "v7_0".to_string()
            && (current_major != 7 || current_minor != 1 || BUILD_VER != 70)
        {
            return error;
        }

        if raw_ver == "v7_1".to_string()
            && (current_major != 7 || current_minor != 1 || BUILD_VER != 71)
        {
            return error;
        }

        Ok(())
    }

    fn release(&self) -> Result<i32, VboxError> {
        call_function!(self.object, Release)
    }
}

impl Drop for VirtualBoxClient {
    fn drop(&mut self) {
        match self.release() {
            Ok(count) => {
                debug!("VirtualBoxClient refcount: {}", count)
            }
            Err(err) => {
                error!("Failed drop VirtualBoxClient Error: {:?}", err)
            }
        }
    }
}
