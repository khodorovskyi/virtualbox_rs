use crate::{VRDEServer, VboxError};
#[cfg(doc)]
use crate::SystemProperties;
use crate::enums::AuthType;
use crate::utility::macros::macros::{get_function_result_bool, get_function_result_number, get_function_result_str, get_function_result_str_vec, get_function_result_unit};
use crate::utility::string_to_c_u64_str;

impl VRDEServer {
    /// Get the VRDE server for this virtual machine.
    ///
    /// # Returns
    ///
    /// Returns `bool` on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    /// use virtualbox_rs::VirtualBox;
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let vm = vbox.find_machines("Freebsd_15").unwrap();
    /// let vrde = vm.get_vrde_server().unwrap();
    /// let enabled = vrde.get_enabled().unwrap();
    /// ```
    pub fn get_enabled(&self) -> Result<bool, VboxError> {
        get_function_result_bool!(self.object, GetEnabled)
    }

    /// Set the VRDE server for this virtual machine.
    ///
    ///
    /// # Arguments
    ///
    /// * `enabled` - bool. Flag indicating whether the VRDE server is enabled or not.
    ///
    /// # Returns
    ///
    /// Returns () on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    /// use virtualbox_rs::{Session, VirtualBox};
    /// use virtualbox_rs::enums::SessionType;
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let vm = vbox.find_machines("Freebsd_15").unwrap();
    /// let mut session = Session::init().unwrap();
    /// vm.lock_machine(&mut  session, SessionType::Shared ).unwrap();
    /// let vm_mut = session.get_machine().unwrap();
    /// let vrde = vm_mut.get_vrde_server().unwrap();
    /// vrde.set_enabled(true).unwrap();
    /// ```
    pub fn set_enabled(&self, enabled: bool) -> Result<(), VboxError> {
        let enabled_bool: i32 = if enabled { 1 } else { 0 };
        get_function_result_unit!(self.object, SetEnabled, enabled_bool)
    }

    /// Get the authentication type for the VRDE server.
    ///
    /// # Returns
    ///
    /// Returns [`AuthType`] on success, or a [`VboxError`] on failure.
    ///
    /// # Example
    /// ```no_run
    ///
    /// use virtualbox_rs::VirtualBox;
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let vm = vbox.find_machines("Freebsd_15").unwrap();
    /// let vrde = vm.get_vrde_server().unwrap();
    /// let auth_type = vrde.get_auth_type().unwrap();
    /// ```
    pub fn get_auth_type(&self) -> Result<AuthType, VboxError> {
        let auth_type = get_function_result_number!(self.object, GetAuthType, u32)?;
        Ok(AuthType::from(auth_type))
    }

    /// Set the authentication type for the VRDE server.
    ///
    ///
    /// # Arguments
    ///
    /// * `auth_type` - [`AuthType`]. The authentication type for the VRDE server.
    ///
    /// # Returns
    ///
    /// Returns () on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    /// use virtualbox_rs::{Session, VirtualBox};
    /// use virtualbox_rs::enums::{AuthType, SessionType};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let vm = vbox.find_machines("Freebsd_15").unwrap();
    /// let mut session = Session::init().unwrap();
    /// vm.lock_machine(&mut  session, SessionType::Shared ).unwrap();
    /// let vm_mut = session.get_machine().unwrap();
    /// let vrde = vm_mut.get_vrde_server().unwrap();
    /// vrde.set_auth_type(AuthType::External).unwrap()
    /// ```
    pub fn set_auth_type(&self, auth_type: AuthType) -> Result<(), VboxError> {
        let auth_type: u32 = auth_type.into();
        get_function_result_unit!(self.object, SetAuthType, auth_type)
    }

    /// Get the authentication timeout for the VRDE server.
    ///
    /// # Returns
    ///
    /// Returns `u32` on success, or a [`VboxError`] on failure.
    ///
    /// # Example
    /// ```no_run
    ///
    /// use virtualbox_rs::VirtualBox;
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let vm = vbox.find_machines("Freebsd_15").unwrap();
    /// let vrde = vm.get_vrde_server().unwrap();
    /// let auth_type = vrde.get_auth_timeout();
    /// ```
    pub fn get_auth_timeout(&self) -> Result<u32, VboxError> {
        get_function_result_number!(self.object, GetAuthTimeout, u32)
    }

    /// Set the authentication timeout for the VRDE server.
    ///
    ///
    /// # Arguments
    ///
    /// * `auth_timeout` - u32. Timeout for guest authentication in milliseconds.
    ///
    /// # Returns
    ///
    /// Returns () on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    /// use virtualbox_rs::{Session, VirtualBox};
    /// use virtualbox_rs::enums::{AuthType, SessionType};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let vm = vbox.find_machines("Freebsd_15").unwrap();
    /// let mut session = Session::init().unwrap();
    /// vm.lock_machine(&mut  session, SessionType::Shared ).unwrap();
    /// let vm_mut = session.get_machine().unwrap();
    /// let vrde = vm_mut.get_vrde_server().unwrap();
    /// vrde.set_auth_timeout(500).unwrap()
    /// ```
    pub fn set_auth_timeout(&self, auth_timeout: u32) -> Result<(), VboxError> {
        get_function_result_unit!(self.object, SetAuthTimeout, auth_timeout)
    }

    /// Get whether multiple simultaneous connections to the VM are permitted.
    ///
    /// # Note
    /// Note that this will be replaced by a more powerful mechanism in the future.
    ///
    /// # Returns
    ///
    /// Returns `bool` on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    /// use virtualbox_rs::VirtualBox;
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let vm = vbox.find_machines("Freebsd_15").unwrap();
    /// let vrde = vm.get_vrde_server().unwrap();
    /// let allow_multi_connection = vrde.get_allow_multi_connection().unwrap();
    /// ```
    pub fn get_allow_multi_connection(&self) -> Result<bool, VboxError> {
        get_function_result_bool!(self.object, GetAllowMultiConnection)
    }

    /// Set whether multiple simultaneous connections to the VM are permitted.
    ///
    /// # Arguments
    ///
    /// * `enabled` - bool. Flag indicating whether multiple simultaneous connections to the VM are permitted.
    ///
    /// # Returns
    ///
    /// Returns () on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    /// use virtualbox_rs::{Session, VirtualBox};
    /// use virtualbox_rs::enums::SessionType;
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let vm = vbox.find_machines("Freebsd_15").unwrap();
    /// let mut session = Session::init().unwrap();
    /// vm.lock_machine(&mut  session, SessionType::Shared ).unwrap();
    /// let vm_mut = session.get_machine().unwrap();
    /// let vrde = vm_mut.get_vrde_server().unwrap();
    /// vrde.set_allow_multi_connection(true).unwrap();
    /// ```
    pub fn set_allow_multi_connection(&self, enabled: bool) -> Result<(), VboxError> {
        let enabled_bool: i32 = if enabled { 1 } else { 0 };
        get_function_result_unit!(self.object, SetAllowMultiConnection, enabled_bool)
    }

    /// Flag whether the existing connection must be dropped and a new connection must be established by the VRDE server,
    /// when a new client connects in single connection mode.
    ///
    /// # Returns
    ///
    /// Returns `bool` on success, or a [`VboxError`] on failure.
    ///
    /// # Example
    /// ```no_run
    /// use virtualbox_rs::VirtualBox;
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let vm = vbox.find_machines("Freebsd_15").unwrap();
    /// let vrde = vm.get_vrde_server().unwrap();
    /// let reuse = vrde.get_reuse_single_connection().unwrap();
    /// ```
    pub fn get_reuse_single_connection(&self) -> Result<bool, VboxError> {
        get_function_result_bool!(self.object, GetReuseSingleConnection)
    }

    /// Set whether the existing connection must be dropped and a new connection must be established by the VRDE server,
    /// when a new client connects in single connection mode.
    ///
    /// # Arguments
    ///
    /// * `reuse` - bool. If true, the existing connection will be dropped and the new client will be connected.
    ///
    /// # Returns
    ///
    /// Returns () on success, or a [`VboxError`] on failure.
    ///
    /// # Example
    /// ```no_run
    /// use virtualbox_rs::{Session, VirtualBox};
    /// use virtualbox_rs::enums::SessionType;
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let vm = vbox.find_machines("Freebsd_15").unwrap();
    /// let mut session = Session::init().unwrap();
    /// vm.lock_machine(&mut  session, SessionType::Shared ).unwrap();
    /// let vm_mut = session.get_machine().unwrap();
    /// let vrde = vm_mut.get_vrde_server().unwrap();
    /// vrde.set_reuse_single_connection(true).unwrap();
    /// ```
    pub fn set_reuse_single_connection(&self, reuse: bool) -> Result<(), VboxError> {
        let reuse_bool: i32 = if reuse { 1 } else { 0 };
        get_function_result_unit!(self.object, SetReuseSingleConnection, reuse_bool)
    }

    ///
    /// Get the name of the Extension Pack providing VRDE for this VM.
    ///
    /// # Note
    /// This overrides ISystemProperties::defaultVRDEExtPack [`SystemProperties::get_default_vrde_ext_pack`].
    ///
    /// # Returns
    ///
    /// Returns `String` on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::VirtualBox;
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let vm = vbox.find_machines("Freebsd_15").unwrap();
    /// let vrde = vm.get_vrde_server().unwrap();
    /// let vrde_ext_pack = vrde.get_vrde_ext_pack().unwrap();
    /// ```
    pub fn get_vrde_ext_pack(&self) -> Result<&'static str, VboxError> {
        get_function_result_str!(self.object, GetVRDEExtPack)
    }

    /// Set the name of the Extension Pack providing VRDE for this VM.
    ///
    /// # Arguments
    ///
    /// * `vrde_ext_pack` - &str. The name of the Extension Pack providing VRDE for this VM.
    ///
    /// # Returns
    ///
    /// Returns a () success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{Session, VirtualBox};
    /// use virtualbox_rs::enums::SessionType;
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let vm = vbox.find_machines("Freebsd_15").unwrap();
    /// let mut session = Session::init().unwrap();
    /// vm.lock_machine(&mut  session, SessionType::Shared ).unwrap();
    /// let vm_mut = session.get_machine().unwrap();
    /// let vrde = vm_mut.get_vrde_server().unwrap();
    /// vrde.set_vrde_ext_pack("VNC").unwrap();
    /// ```
    //TODO: need to check
    pub fn set_vrde_ext_pack(&self, vrde_ext_pack: &str) -> Result<(), VboxError> {
        let vrde_ext_pack = string_to_c_u64_str(vrde_ext_pack)?;
        get_function_result_unit!(self.object, SetVRDEExtPack, vrde_ext_pack)
    }

    /// Get the name of the authentication library used for authentication of RDP clients by this VM.
    ///
    /// # Note
    /// This overrides ISystemProperties::VRDEAuthLibrary [`SystemProperties::get_vrde_auth_library`].
    ///
    /// # Returns
    ///
    /// Returns `String` on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::VirtualBox;
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let vm = vbox.find_machines("Freebsd_15").unwrap();
    /// let vrde = vm.get_vrde_server().unwrap();
    /// let auth_library = vrde.get_auth_library().unwrap();
    /// ```
    pub fn get_auth_library(&self) -> Result<&'static str, VboxError> {
        get_function_result_str!(self.object, GetAuthLibrary)
    }

    /// Set the name of the authentication library used for authentication of RDP clients by this VM.
    ///
    /// # Note
    /// This overrides ISystemProperties::VRDEAuthLibrary [`SystemProperties::get_vrde_auth_library`].
    ///
    /// # Arguments
    ///
    /// * `auth_library` - &str. The name of the authentication library used for authentication of RDP clients by this VM.
    ///
    /// # Returns
    ///
    /// Returns a () success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{Session, VirtualBox};
    /// use virtualbox_rs::enums::SessionType;
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let vm = vbox.find_machines("Freebsd_15").unwrap();
    /// let mut session = Session::init().unwrap();
    /// vm.lock_machine(&mut  session, SessionType::Shared ).unwrap();
    /// let vm_mut = session.get_machine().unwrap();
    /// let vrde = vm_mut.get_vrde_server().unwrap();
    /// vrde.set_auth_library("VBoxAuth").unwrap();
    /// ```
    //TODO: need to check
    pub fn set_auth_library(&self, auth_library: &str) -> Result<(), VboxError> {
        let auth_library = string_to_c_u64_str(auth_library)?;
        get_function_result_unit!(self.object, SetAuthLibrary, auth_library)
    }

    /// Get the names of properties, which are supported by this VRDE server.
    ///
    /// # Returns
    ///
    /// Returns `Vec<&str>` on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::VirtualBox;
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let vm = vbox.find_machines("Freebsd_15").unwrap();
    /// let vrde = vm.get_vrde_server().unwrap();
    /// let vrde_properties = vrde.get_vrde_properties().unwrap();
    /// ```
    pub fn get_vrde_properties(&self) -> Result<Vec<&'static str>, VboxError> {
        get_function_result_str_vec!(self.object, GetVRDEProperties)
    }

    /// Get the value of a VRDE specific property.
    ///
    /// Returns `&str` on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::VirtualBox;
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let vm = vbox.find_machines("Freebsd_15").unwrap();
    /// let vrde = vm.get_vrde_server().unwrap();
    /// let vrde_property = vrde.get_vrde_property("TCP/Ports").unwrap();
    /// ```
    pub fn get_vrde_property(&self, key: &str) -> Result<&'static str, VboxError> {
        let key = string_to_c_u64_str(key)?;
        get_function_result_str!(self.object, GetVRDEProperty, key)
    }

    /// Set a VRDE specific property string.
    ///
    /// If you pass null or empty string as a key value, the given key will be deleted.
    ///
    /// # Arguments
    /// * `key` - &str. Name of the key to set.
    /// * `value` - &str. Value to assign to the key.
    ///
    /// # Returns
    ///
    /// Returns () success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{Session, VirtualBox};
    /// use virtualbox_rs::enums::SessionType;
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let vm = vbox.find_machines("Freebsd_15").unwrap();
    /// let mut session = Session::init().unwrap();
    /// vm.lock_machine(&mut  session, SessionType::Shared ).unwrap();
    /// let vm_mut = session.get_machine().unwrap();
    /// let vrde = vm_mut.get_vrde_server().unwrap();
    /// vrde.set_vrde_property("TCP/Ports", "9000").unwrap();
    /// ```
    pub fn set_vrde_property(&self, key: &str, value: &str) -> Result<(), VboxError> {
        let key = string_to_c_u64_str(key)?;
        let value = string_to_c_u64_str(value)?;
        get_function_result_unit!(self.object, SetVRDEProperty, key, value)
    }
}