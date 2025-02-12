use crate::enums::{NetworkAdapterPromiscModePolicy, NetworkAdapterType, NetworkAttachmentType};
use crate::utility::macros::macros::{
    get_function_result_bool, get_function_result_number, get_function_result_pointer,
    get_function_result_str, get_function_result_unit,
};
use crate::utility::string_to_c_u64_str;
use crate::{BandwidthGroup, NetworkAdapter, VboxError};
use vbox_raw::sys_lib::IBandwidthGroup;

impl NetworkAdapter {
    /// Type of the virtual network adapter.
    ///
    /// # Returns
    ///
    /// Returns [`NetworkAdapterType`] on success, or a [`VboxError`] on failure.
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
    /// let network_adapter = machine.get_network_adapter(0).unwrap();
    /// let adapter_type = network_adapter.get_adapter_type().unwrap();
    pub fn get_adapter_type(&self) -> Result<NetworkAdapterType, VboxError> {
        let adapter_type = get_function_result_number!(self.object, GetAdapterType, u32)?;
        Ok(NetworkAdapterType::from(adapter_type))
    }

    /// Type of the virtual network adapter.
    ///
    /// # Arguments
    ///
    /// * `adapter_type` - [`NetworkAdapterType`].
    ///
    /// # Returns
    ///
    /// Returns () on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{Session, VirtualBox};
    /// use virtualbox_rs::enums::{NetworkAdapterType, SessionType};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let mut session = Session::init().unwrap();
    /// machine.lock_machine(&mut  session, SessionType::Shared ).unwrap();
    /// let machine_mut = session.get_machine().unwrap();
    /// let network_adapter = machine_mut.get_network_adapter(0).unwrap();
    /// network_adapter.set_adapter_type(NetworkAdapterType::Am79C960).unwrap();
    /// machine_mut.save_settings().unwrap();
    pub fn set_adapter_type(&self, adapter_type: NetworkAdapterType) -> Result<(), VboxError> {
        let adapter_type: u32 = adapter_type.into();
        get_function_result_unit!(self.object, SetAdapterType, adapter_type)
    }

    /// Slot number this adapter is plugged into.
    ///
    /// # Returns
    ///
    /// Returns u32 on success, or a [`VboxError`] on failure.
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
    /// let network_adapter = machine.get_network_adapter(0).unwrap();
    /// let slot = network_adapter.get_slot().unwrap();
    pub fn get_slot(&self) -> Result<u32, VboxError> {
        get_function_result_number!(self.object, GetSlot, u32)
    }

    /// Flag whether the network adapter is present in the guest system.
    ///
    /// # Returns
    ///
    /// Returns bool on success, or a [`VboxError`] on failure.
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
    /// let network_adapter = machine.get_network_adapter(0).unwrap();
    /// let enabled = network_adapter.get_enabled().unwrap();
    pub fn get_enabled(&self) -> Result<bool, VboxError> {
        get_function_result_bool!(self.object, GetEnabled)
    }

    /// Flag whether the network adapter is present in the guest system.
    ///
    /// # Arguments
    ///
    /// * `enabled` - bool.
    ///
    /// # Returns
    ///
    /// Returns () on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{Session, VirtualBox};
    /// use virtualbox_rs::enums::SessionType;
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let mut session = Session::init().unwrap();
    /// machine.lock_machine(&mut  session, SessionType::Shared ).unwrap();
    /// let machine_mut = session.get_machine().unwrap();
    /// let network_adapter = machine_mut.get_network_adapter(0).unwrap();
    /// network_adapter.set_enabled(true).unwrap();
    /// machine_mut.save_settings().unwrap();
    pub fn set_enabled(&self, enabled: bool) -> Result<(), VboxError> {
        let enabled_bool: i32 = if enabled { 1 } else { 0 };
        get_function_result_unit!(self.object, SetEnabled, enabled_bool)
    }

    /// Ethernet MAC address of the adapter, 12 hexadecimal characters.
    ///
    /// # Returns
    ///
    /// Returns &str on success, or a [`VboxError`] on failure.
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
    /// let network_adapter = machine.get_network_adapter(0).unwrap();
    /// let mac_address = network_adapter.get_mac_address().unwrap();
    pub fn get_mac_address(&self) -> Result<&'static str, VboxError> {
        get_function_result_str!(self.object, GetMACAddress)
    }

    /// Ethernet MAC address of the adapter, 12 hexadecimal characters.
    ///
    /// # Arguments
    ///
    /// * `enabled` - &str.
    ///
    /// # Returns
    ///
    /// Returns () on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{Session, VirtualBox};
    /// use virtualbox_rs::enums::SessionType;
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let mut session = Session::init().unwrap();
    /// machine.lock_machine(&mut  session, SessionType::Shared ).unwrap();
    /// let machine_mut = session.get_machine().unwrap();
    /// let network_adapter = machine_mut.get_network_adapter(0).unwrap();
    /// network_adapter.set_mac_address("00FFFFFFFFFF").unwrap();
    /// machine_mut.save_settings().unwrap();

    pub fn set_mac_address(&self, mac_address: &str) -> Result<(), VboxError> {
        let mac_address_ptr = string_to_c_u64_str(mac_address)?;
        get_function_result_unit!(self.object, SetMACAddress, mac_address_ptr)
    }

    /// Gets network attachment type of this network adapter.
    ///
    /// # Returns
    ///
    /// Returns [`NetworkAttachmentType`] on success, or a [`VboxError`] on failure.
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
    /// let network_adapter = machine.get_network_adapter(0).unwrap();
    /// let attachment_type = network_adapter.get_attachment_type().unwrap();

    pub fn get_attachment_type(&self) -> Result<NetworkAttachmentType, VboxError> {
        let attachment_type = get_function_result_number!(self.object, GetAttachmentType, u32)?;
        Ok(NetworkAttachmentType::from(attachment_type))
    }

    /// Sets network attachment type of this network adapter.
    ///
    /// # Arguments
    ///
    /// * `attachment_type` - [`NetworkAttachmentType`].
    ///
    /// # Returns
    ///
    /// Returns () on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{Session, VirtualBox};
    /// use virtualbox_rs::enums::{NetworkAttachmentType, SessionType};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let mut session = Session::init().unwrap();
    /// machine.lock_machine(&mut  session, SessionType::Shared ).unwrap();
    /// let machine_mut = session.get_machine().unwrap();
    /// let network_adapter = machine_mut.get_network_adapter(0).unwrap();
    /// network_adapter.set_attachment_type(NetworkAttachmentType::Bridged).unwrap();
    /// machine_mut.save_settings().unwrap();

    pub fn set_attachment_type(
        &self,
        attachment_type: NetworkAttachmentType,
    ) -> Result<(), VboxError> {
        let attachment_type: u32 = attachment_type.into();
        get_function_result_unit!(self.object, SetAttachmentType, attachment_type)
    }

    /// Name of the network interface the VM should be bridged to.
    ///
    /// # Returns
    ///
    /// Returns &str on success, or a [`VboxError`] on failure.
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
    /// let network_adapter = machine.get_network_adapter(0).unwrap();
    /// let bridged_interface = network_adapter.get_bridged_interface().unwrap();
    pub fn get_bridged_interface(&self) -> Result<&'static str, VboxError> {
        get_function_result_str!(self.object, GetBridgedInterface)
    }

    /// Name of the network interface the VM should be bridged to.
    ///
    /// # Arguments
    ///
    /// * `bridged_interface` - &str.
    ///
    /// # Returns
    ///
    /// Returns () on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{Session, VirtualBox};
    /// use virtualbox_rs::enums::SessionType;
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let mut session = Session::init().unwrap();
    /// machine.lock_machine(&mut  session, SessionType::Shared ).unwrap();
    /// let machine_mut = session.get_machine().unwrap();
    /// let network_adapter = machine_mut.get_network_adapter(0).unwrap();
    /// network_adapter.set_bridged_interface("em0").unwrap();
    /// machine_mut.save_settings().unwrap();
    pub fn set_bridged_interface(&self, bridged_interface: &str) -> Result<(), VboxError> {
        let bridged_interface_ptr = string_to_c_u64_str(bridged_interface)?;
        get_function_result_unit!(self.object, SetBridgedInterface, bridged_interface_ptr)
    }

    /// Name of the host only network interface the VM is attached to.
    ///
    /// # Returns
    ///
    /// Returns &str on success, or a [`VboxError`] on failure.
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
    /// let network_adapter = machine.get_network_adapter(0).unwrap();
    /// let host_only_interface = network_adapter.get_host_only_interface().unwrap();
    pub fn get_host_only_interface(&self) -> Result<&'static str, VboxError> {
        get_function_result_str!(self.object, GetHostOnlyInterface)
    }

    /// Name of the host only network interface the VM is attached to.
    ///
    /// # Arguments
    ///
    /// * `host_only_interface` - &str.
    ///
    /// # Returns
    ///
    /// Returns () on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{Session, VirtualBox};
    /// use virtualbox_rs::enums::SessionType;
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let mut session = Session::init().unwrap();
    /// machine.lock_machine(&mut  session, SessionType::Shared ).unwrap();
    /// let machine_mut = session.get_machine().unwrap();
    /// let network_adapter = machine_mut.get_network_adapter(0).unwrap();
    /// network_adapter.set_host_only_interface("he0").unwrap();
    /// machine_mut.save_settings().unwrap();

    pub fn set_host_only_interface(&self, host_only_interface: &str) -> Result<(), VboxError> {
        let host_only_interface_ptr = string_to_c_u64_str(host_only_interface)?;
        get_function_result_unit!(self.object, SetHostOnlyInterface, host_only_interface_ptr)
    }

    /// Name of the internal network the VM is attached to.
    ///
    /// # Returns
    ///
    /// Returns &str on success, or a [`VboxError`] on failure.
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
    /// let network_adapter = machine.get_network_adapter(0).unwrap();
    /// let internal_network = network_adapter.get_internal_network().unwrap();

    pub fn get_internal_network(&self) -> Result<&'static str, VboxError> {
        get_function_result_str!(self.object, GetInternalNetwork)
    }

    /// Name of the internal network the VM is attached to.
    ///
    /// # Arguments
    ///
    /// * `internal_network` - &str.
    ///
    /// # Returns
    ///
    /// Returns () on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{Session, VirtualBox};
    /// use virtualbox_rs::enums::SessionType;
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let mut session = Session::init().unwrap();
    /// machine.lock_machine(&mut  session, SessionType::Shared ).unwrap();
    /// let machine_mut = session.get_machine().unwrap();
    /// let network_adapter = machine_mut.get_network_adapter(0).unwrap();
    /// network_adapter.set_internal_network("in0").unwrap();
    /// machine_mut.save_settings().unwrap();
    pub fn set_internal_network(&self, internal_network: &str) -> Result<(), VboxError> {
        let internal_network_ptr = string_to_c_u64_str(internal_network)?;
        get_function_result_unit!(self.object, SetInternalNetwork, internal_network_ptr)
    }

    /// Name of the NAT network the VM is attached to.
    ///
    /// # Returns
    ///
    /// Returns &str on success, or a [`VboxError`] on failure.
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
    /// let network_adapter = machine.get_network_adapter(0).unwrap();
    /// let nat_network = network_adapter.get_nat_network().unwrap();

    pub fn get_nat_network(&self) -> Result<&'static str, VboxError> {
        get_function_result_str!(self.object, GetNATNetwork)
    }

    /// Name of the NAT network the VM is attached to.
    ///
    /// # Arguments
    ///
    /// * `nat_network` - &str.
    ///
    /// # Returns
    ///
    /// Returns () on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{Session, VirtualBox};
    /// use virtualbox_rs::enums::SessionType;
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let mut session = Session::init().unwrap();
    /// machine.lock_machine(&mut  session, SessionType::Shared ).unwrap();
    /// let machine_mut = session.get_machine().unwrap();
    /// let network_adapter = machine_mut.get_network_adapter(0).unwrap();
    /// network_adapter.set_internal_network("in0").unwrap();
    /// machine_mut.save_settings().unwrap();

    pub fn set_nat_network(&self, nat_network: &str) -> Result<(), VboxError> {
        let nat_network_ptr = string_to_c_u64_str(nat_network)?;
        get_function_result_unit!(self.object, SetNATNetwork, nat_network_ptr)
    }

    /// Name of the driver to use for the "Generic" network attachment type.
    ///
    /// # Returns
    ///
    /// Returns &str on success, or a [`VboxError`] on failure.
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
    /// let network_adapter = machine.get_network_adapter(0).unwrap();
    /// let generic_driver = network_adapter.get_generic_driver().unwrap();
    pub fn get_generic_driver(&self) -> Result<&'static str, VboxError> {
        get_function_result_str!(self.object, GetGenericDriver)
    }

    /// Name of the driver to use for the "Generic" network attachment type.
    ///
    /// # Arguments
    ///
    /// * `generic_driver` - &str.
    ///
    /// # Returns
    ///
    /// Returns () on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{Session, VirtualBox};
    /// use virtualbox_rs::enums::SessionType;
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let mut session = Session::init().unwrap();
    /// machine.lock_machine(&mut  session, SessionType::Shared ).unwrap();
    /// let machine_mut = session.get_machine().unwrap();
    /// let network_adapter = machine_mut.get_network_adapter(0).unwrap();
    /// network_adapter.set_generic_driver("in0").unwrap();
    /// machine_mut.save_settings().unwrap();
    pub fn set_generic_driver(&self, generic_driver: &str) -> Result<(), VboxError> {
        let generic_driver_ptr = string_to_c_u64_str(generic_driver)?;
        get_function_result_unit!(self.object, SetGenericDriver, generic_driver_ptr)
    }

    /// Name of the cloud network the VM is attached to.
    ///
    /// # Returns
    ///
    /// Returns &str on success, or a [`VboxError`] on failure.
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
    /// let network_adapter = machine.get_network_adapter(0).unwrap();
    /// let cloud_network = network_adapter.get_cloud_network().unwrap();
    pub fn get_cloud_network(&self) -> Result<&'static str, VboxError> {
        get_function_result_str!(self.object, GetCloudNetwork)
    }

    /// Name of the cloud network the VM is attached to.
    ///
    /// # Arguments
    ///
    /// * `cloud_network` - &str.
    ///
    /// # Returns
    ///
    /// Returns () on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{Session, VirtualBox};
    /// use virtualbox_rs::enums::SessionType;
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let mut session = Session::init().unwrap();
    /// machine.lock_machine(&mut  session, SessionType::Shared ).unwrap();
    /// let machine_mut = session.get_machine().unwrap();
    /// let network_adapter = machine_mut.get_network_adapter(0).unwrap();
    /// network_adapter.set_cloud_network("cn0").unwrap();
    /// machine_mut.save_settings().unwrap();
    pub fn set_cloud_network(&self, cloud_network: &str) -> Result<(), VboxError> {
        let cloud_network_ptr = string_to_c_u64_str(cloud_network)?;
        get_function_result_unit!(self.object, SetCloudNetwork, cloud_network_ptr)
    }

    /// Flag whether the adapter reports the cable as connected or not.
    ///
    /// It can be used to report offline situations to a VM.
    ///
    /// # Returns
    ///
    /// Returns bool on success, or a [`VboxError`] on failure.
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
    /// let network_adapter = machine.get_network_adapter(0).unwrap();
    /// let cable_connected = network_adapter.get_cable_connected().unwrap();

    pub fn get_cable_connected(&self) -> Result<bool, VboxError> {
        get_function_result_bool!(self.object, GetCableConnected)
    }

    /// Flag whether the adapter reports the cable as connected or not.
    ///
    /// It can be used to report offline situations to a VM.
    ///
    /// # Arguments
    ///
    /// * `cable_connected` - bool.
    ///
    /// # Returns
    ///
    /// Returns () on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{Session, VirtualBox};
    /// use virtualbox_rs::enums::SessionType;
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let mut session = Session::init().unwrap();
    /// machine.lock_machine(&mut  session, SessionType::Shared ).unwrap();
    /// let machine_mut = session.get_machine().unwrap();
    /// let network_adapter = machine_mut.get_network_adapter(0).unwrap();
    /// network_adapter.set_cable_connected(true).unwrap();
    /// machine_mut.save_settings().unwrap();
    pub fn set_cable_connected(&self, cable_connected: bool) -> Result<(), VboxError> {
        let cable_connected_bool: i32 = if cable_connected { 1 } else { 0 };
        get_function_result_unit!(self.object, SetCableConnected, cable_connected_bool)
    }

    /// Line speed reported by custom drivers, in units of 1 kbps.
    ///
    /// It can be used to report offline situations to a VM.
    ///
    /// # Returns
    ///
    /// Returns bool on success, or a [`VboxError`] on failure.
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
    /// let network_adapter = machine.get_network_adapter(0).unwrap();
    /// let line_speed = network_adapter.get_line_speed().unwrap();

    pub fn get_line_speed(&self) -> Result<u32, VboxError> {
        get_function_result_number!(self.object, GetLineSpeed, u32)
    }

    /// Line speed reported by custom drivers, in units of 1 kbps.
    ///
    /// It can be used to report offline situations to a VM.
    ///
    /// # Arguments
    ///
    /// * `line_speed` - u32.
    ///
    /// # Returns
    ///
    /// Returns () on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{Session, VirtualBox};
    /// use virtualbox_rs::enums::SessionType;
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let mut session = Session::init().unwrap();
    /// machine.lock_machine(&mut  session, SessionType::Shared ).unwrap();
    /// let machine_mut = session.get_machine().unwrap();
    /// let network_adapter = machine_mut.get_network_adapter(0).unwrap();
    /// network_adapter.set_line_speed(10000).unwrap();
    /// machine_mut.save_settings().unwrap();
    pub fn set_line_speed(&self, line_speed: u32) -> Result<(), VboxError> {
        get_function_result_unit!(self.object, SetLineSpeed, line_speed)
    }

    /// The promiscuous mode policy of the network adapter when attached to an internal network, host only network or a bridge.
    ///
    /// It can be used to report offline situations to a VM.
    ///
    /// # Returns
    ///
    /// Returns [`NetworkAdapterPromiscModePolicy`] on success, or a [`VboxError`] on failure.
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
    /// let network_adapter = machine.get_network_adapter(0).unwrap();
    /// let promisc_mode_policy = network_adapter.get_promisc_mode_policy().unwrap();
    pub fn get_promisc_mode_policy(&self) -> Result<NetworkAdapterPromiscModePolicy, VboxError> {
        let mode_policy = get_function_result_number!(self.object, GetPromiscModePolicy, u32)?;
        Ok(NetworkAdapterPromiscModePolicy::from(mode_policy))
    }

    /// The promiscuous mode policy of the network adapter when attached to an internal network, host only network or a bridge.
    ///
    /// It can be used to report offline situations to a VM.
    ///
    /// # Arguments
    ///
    /// * `promisc_mode_policy` - [`NetworkAdapterPromiscModePolicy`].
    ///
    /// # Returns
    ///
    /// Returns () on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{Session, VirtualBox};
    /// use virtualbox_rs::enums::{NetworkAdapterPromiscModePolicy, SessionType};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let mut session = Session::init().unwrap();
    /// machine.lock_machine(&mut  session, SessionType::Shared ).unwrap();
    /// let machine_mut = session.get_machine().unwrap();
    /// let network_adapter = machine_mut.get_network_adapter(0).unwrap();
    /// network_adapter.set_promisc_mode_policy(NetworkAdapterPromiscModePolicy::AllowAll).unwrap();
    /// machine_mut.save_settings().unwrap();

    pub fn set_promisc_mode_policy(
        &self,
        promisc_mode_policy: NetworkAdapterPromiscModePolicy,
    ) -> Result<(), VboxError> {
        let promisc_mode_policy: u32 = promisc_mode_policy.into();
        get_function_result_unit!(self.object, SetPromiscModePolicy, promisc_mode_policy)
    }

    /// Flag whether network traffic from/to the network card should be traced.
    ///
    /// # Returns
    ///
    /// Returns bool on success, or a [`VboxError`] on failure.
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
    /// let network_adapter = machine.get_network_adapter(0).unwrap();
    /// let trace_enabled = network_adapter.get_trace_enabled().unwrap();
    pub fn get_trace_enabled(&self) -> Result<bool, VboxError> {
        get_function_result_bool!(self.object, GetTraceEnabled)
    }

    /// Flag whether network traffic from/to the network card should be traced.
    ///
    /// Can only be toggled when the VM is turned off.
    ///
    /// # Arguments
    ///
    /// * `trace_enabled` - bool.
    ///
    /// # Returns
    ///
    /// Returns () on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{Session, VirtualBox};
    /// use virtualbox_rs::enums::SessionType;
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let mut session = Session::init().unwrap();
    /// machine.lock_machine(&mut  session, SessionType::Shared ).unwrap();
    /// let machine_mut = session.get_machine().unwrap();
    /// let network_adapter = machine_mut.get_network_adapter(0).unwrap();
    /// network_adapter.set_trace_enabled(true).unwrap();
    /// machine_mut.save_settings().unwrap();
    pub fn set_trace_enabled(&self, trace_enabled: bool) -> Result<(), VboxError> {
        let trace_enabled_bool: i32 = if trace_enabled { 1 } else { 0 };
        get_function_result_unit!(self.object, SetTraceEnabled, trace_enabled_bool)
    }

    /// Filename where a network trace will be stored.
    ///
    /// If not set, VBox-pid.pcap will be used.
    ///
    /// # Returns
    ///
    /// Returns &str on success, or a [`VboxError`] on failure.
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
    /// let network_adapter = machine.get_network_adapter(0).unwrap();
    /// let trace_file = network_adapter.get_trace_file().unwrap();
    pub fn get_trace_file(&self) -> Result<&'static str, VboxError> {
        get_function_result_str!(self.object, GetTraceFile)
    }

    /// Filename where a network trace will be stored.
    ///
    /// If not set, VBox-pid.pcap will be used.
    ///
    /// # Arguments
    ///
    /// * `trace_file` - &str.
    ///
    /// # Returns
    ///
    /// Returns () on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{Session, VirtualBox};
    /// use virtualbox_rs::enums::SessionType;
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let mut session = Session::init().unwrap();
    /// machine.lock_machine(&mut  session, SessionType::Shared ).unwrap();
    /// let machine_mut = session.get_machine().unwrap();
    /// let network_adapter = machine_mut.get_network_adapter(0).unwrap();
    /// network_adapter.set_trace_file("/tmp/vm_trace.pcap").unwrap();
    /// machine_mut.save_settings().unwrap();
    pub fn set_trace_file(&self, trace_file: &str) -> Result<(), VboxError> {
        let trace_file_ptr = string_to_c_u64_str(trace_file)?;
        get_function_result_unit!(self.object, SetTraceFile, trace_file_ptr)
    }

    // TODO GetNATEngine

    /// Network boot priority of the adapter.
    ///
    /// Priority 1 is highest. If not set, the priority is considered to be at the lowest possible setting.
    ///
    /// # Returns
    ///
    /// Returns u32 on success, or a [`VboxError`] on failure.
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
    /// let network_adapter = machine.get_network_adapter(0).unwrap();
    /// let boot_priority = network_adapter.get_boot_priority().unwrap();

    pub fn get_boot_priority(&self) -> Result<u32, VboxError> {
        get_function_result_number!(self.object, GetBootPriority, u32)
    }
    /// Network boot priority of the adapter.
    ///
    /// Priority 1 is highest. If not set, the priority is considered to be at the lowest possible setting.
    ///
    /// # Arguments
    ///
    /// * `trace_file` - u32.
    ///
    /// # Returns
    ///
    /// Returns () on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{Session, VirtualBox};
    /// use virtualbox_rs::enums::SessionType;
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let mut session = Session::init().unwrap();
    /// machine.lock_machine(&mut  session, SessionType::Shared ).unwrap();
    /// let machine_mut = session.get_machine().unwrap();
    /// let network_adapter = machine_mut.get_network_adapter(0).unwrap();
    /// network_adapter.set_boot_priority(1).unwrap();
    /// machine_mut.save_settings().unwrap();
    pub fn set_boot_priority(&self, boot_priority: u32) -> Result<(), VboxError> {
        get_function_result_unit!(self.object, SetBootPriority, boot_priority)
    }

    /// The bandwidth group this network adapter is assigned to.
    ///
    /// # Returns
    ///
    /// Returns [`BandwidthGroup`] on success, or a [`VboxError`] on failure.
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
    /// let network_adapter = machine.get_network_adapter(0).unwrap();
    /// let bandwidth_group = network_adapter.get_bandwidth_group().unwrap();

    pub fn get_bandwidth_group(&self) -> Result<BandwidthGroup, VboxError> {
        let bandwidth_group =
            get_function_result_pointer!(self.object, GetBandwidthGroup, *mut IBandwidthGroup)?;
        Ok(BandwidthGroup::new(bandwidth_group))
    }
    // TODO SetBandwidthGroup

    #[cfg(not(is_v_6_1))]
    /// Name of the host only network the VM is attached to.
    ///
    /// # Returns
    ///
    /// Returns &str on success, or a [`VboxError`] on failure.
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
    /// let network_adapter = machine.get_network_adapter(0).unwrap();
    /// let host_only_network = network_adapter.get_host_only_network().unwrap();
    pub fn get_host_only_network(&self) -> Result<&'static str, VboxError> {
        get_function_result_str!(self.object, GetHostOnlyNetwork)
    }

    #[cfg(not(is_v_6_1))]
    /// Name of the host only network interface the VM is attached to.
    ///
    /// # Arguments
    ///
    /// * `bridged_interface` - &str.
    ///
    /// # Returns
    ///
    /// Returns () on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{Session, VirtualBox};
    /// use virtualbox_rs::enums::SessionType;
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let mut session = Session::init().unwrap();
    /// machine.lock_machine(&mut  session, SessionType::Shared ).unwrap();
    /// let machine_mut = session.get_machine().unwrap();
    /// let network_adapter = machine_mut.get_network_adapter(0).unwrap();
    /// network_adapter.set_host_only_network("hn0").unwrap();
    /// machine_mut.save_settings().unwrap();

    pub fn set_host_only_network(&self, host_only_network: &str) -> Result<(), VboxError> {
        let host_only_network_ptr = string_to_c_u64_str(host_only_network)?;
        get_function_result_unit!(self.object, SetHostOnlyNetwork, host_only_network_ptr)
    }
}

#[cfg(is_v_6_1)]
impl NetworkAdapter {
    /// Placeholder Method
    ///
    /// This method serves as a placeholder for versions of the API where the actual method is not available.
    /// Attempting to call this method will result in an `UnsupportedInCurrentApiVersion` error.
    /// This method is intended to ensure that the codebase can be compiled against multiple API versions
    /// without modification.
    ///
    /// Supported from API version: v7_0
    pub fn get_host_only_network(&self) -> Result<&'static str, VboxError> {
        Err(VboxError::unsupported_in_current_api_version(
            "NetworkAdapter::get_host_only_network",
            "v7_0",
        ))
    }

    /// Placeholder Method
    ///
    /// This method serves as a placeholder for versions of the API where the actual method is not available.
    /// Attempting to call this method will result in an `UnsupportedInCurrentApiVersion` error.
    /// This method is intended to ensure that the codebase can be compiled against multiple API versions
    /// without modification.
    ///
    /// Supported from API version: v7_0
    pub fn set_host_only_network(&self, _host_only_network: &str) -> Result<(), VboxError> {
        Err(VboxError::unsupported_in_current_api_version(
            "NetworkAdapter::set_host_only_network",
            "v7_0",
        ))
    }
}
