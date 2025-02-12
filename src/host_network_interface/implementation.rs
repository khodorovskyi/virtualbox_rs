use crate::enums::{
    HostNetworkInterfaceMediumType, HostNetworkInterfaceStatus, HostNetworkInterfaceType,
};
use crate::utility::macros::macros::{
    get_function_result_bool, get_function_result_number, get_function_result_str,
};
use crate::{HostNetworkInterface, VboxError};

impl HostNetworkInterface {
    /// Returns the host network interface name.
    ///
    /// # Returns
    ///
    /// Returns a &str success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::Host;
    ///
    /// let host = Host::init().unwrap();
    /// let network_interfaces = host.get_network_interfaces().unwrap();
    /// let network_interface = network_interfaces.get(0).unwrap();
    /// let name = network_interface.get_name().unwrap();
    pub fn get_name(&self) -> Result<&'static str, VboxError> {
        get_function_result_str!(self.object, GetName)
    }
    /// Returns the host network interface short name.
    ///
    /// # Returns
    ///
    /// Returns a &str success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::Host;
    ///
    /// let host = Host::init().unwrap();
    /// let network_interfaces = host.get_network_interfaces().unwrap();
    /// let network_interface = network_interfaces.get(0).unwrap();
    /// let name = network_interface.get_short_name().unwrap();
    pub fn get_short_name(&self) -> Result<&'static str, VboxError> {
        get_function_result_str!(self.object, GetShortName)
    }

    /// Returns the interface UUID.
    ///
    /// # Returns
    ///
    /// Returns a &str success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::Host;
    ///
    /// let host = Host::init().unwrap();
    /// let network_interfaces = host.get_network_interfaces().unwrap();
    /// let network_interface = network_interfaces.get(0).unwrap();
    /// let id = network_interface.get_short_name().unwrap();
    pub fn get_id(&self) -> Result<&'static str, VboxError> {
        get_function_result_str!(self.object, GetId)
    }

    /// Returns the name of a virtual network the interface gets attached to.
    ///
    /// # Returns
    ///
    /// Returns a &str success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::Host;
    ///
    /// let host = Host::init().unwrap();
    /// let network_interfaces = host.get_network_interfaces().unwrap();
    /// let network_interface = network_interfaces.get(0).unwrap();
    /// let name = network_interface.get_network_name().unwrap();
    pub fn get_network_name(&self) -> Result<&'static str, VboxError> {
        get_function_result_str!(self.object, GetNetworkName)
    }
    /// Specifies whether the DHCP is enabled for the interface.
    ///
    /// # Returns
    ///
    /// Returns a bool success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::Host;
    ///
    /// let host = Host::init().unwrap();
    /// let network_interfaces = host.get_network_interfaces().unwrap();
    /// let network_interface = network_interfaces.get(0).unwrap();
    /// let dhcp_enabled = network_interface.get_network_name().unwrap();
    pub fn get_dhcp_enabled(&self) -> Result<bool, VboxError> {
        get_function_result_bool!(self.object, GetDHCPEnabled)
    }
    /// Returns the IP V4 address of the interface.
    ///
    /// # Returns
    ///
    /// Returns a &str success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::Host;
    ///
    /// let host = Host::init().unwrap();
    /// let network_interfaces = host.get_network_interfaces().unwrap();
    /// let network_interface = network_interfaces.get(0).unwrap();
    /// let ip_address = network_interface.get_ip_address().unwrap();
    pub fn get_ip_address(&self) -> Result<&'static str, VboxError> {
        get_function_result_str!(self.object, GetIPAddress)
    }

    /// Returns the network mask of the interface.
    ///
    /// # Returns
    ///
    /// Returns a &str success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::Host;
    ///
    /// let host = Host::init().unwrap();
    /// let network_interfaces = host.get_network_interfaces().unwrap();
    /// let network_interface = network_interfaces.get(0).unwrap();
    /// let network_mask = network_interface.get_ip_address().unwrap();
    pub fn get_network_mask(&self) -> Result<&'static str, VboxError> {
        get_function_result_str!(self.object, GetNetworkMask)
    }

    /// Specifies whether the IP V6 is supported/enabled for the interface.
    ///
    /// # Returns
    ///
    /// Returns a bool success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::Host;
    ///
    /// let host = Host::init().unwrap();
    /// let network_interfaces = host.get_network_interfaces().unwrap();
    /// let network_interface = network_interfaces.get(0).unwrap();
    /// let ipv6_supported = network_interface.get_ipv6_supported().unwrap();
    pub fn get_ipv6_supported(&self) -> Result<bool, VboxError> {
        get_function_result_bool!(self.object, GetIPV6Supported)
    }

    /// Returns the IP V6 address of the interface.
    ///
    /// # Returns
    ///
    /// Returns a &str success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::Host;
    ///
    /// let host = Host::init().unwrap();
    /// let network_interfaces = host.get_network_interfaces().unwrap();
    /// let network_interface = network_interfaces.get(0).unwrap();
    /// let ipv6_address = network_interface.get_ipv6_address().unwrap();
    pub fn get_ipv6_address(&self) -> Result<&'static str, VboxError> {
        get_function_result_str!(self.object, GetIPV6Address)
    }

    /// Returns the IP V6 address of the interface.
    ///
    /// # Returns
    ///
    /// Returns u32 success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::Host;
    ///
    /// let host = Host::init().unwrap();
    /// let network_interfaces = host.get_network_interfaces().unwrap();
    /// let network_interface = network_interfaces.get(0).unwrap();
    /// let ipv6_network_mask_prefix_length = network_interface.get_ipv6_network_mask_prefix_length().unwrap();
    pub fn get_ipv6_network_mask_prefix_length(&self) -> Result<u32, VboxError> {
        let ipv6_network_mask_prefix_length =
            get_function_result_number!(self.object, GetIPV6NetworkMaskPrefixLength, u32)?;
        Ok(ipv6_network_mask_prefix_length)
    }

    /// Returns the IP V6 address of the interface.
    ///
    /// # Returns
    ///
    /// Returns a &str success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::Host;
    ///
    /// let host = Host::init().unwrap();
    /// let network_interfaces = host.get_network_interfaces().unwrap();
    /// let network_interface = network_interfaces.get(0).unwrap();
    /// let hardware_address = network_interface.get_hardware_address().unwrap();
    pub fn get_hardware_address(&self) -> Result<&'static str, VboxError> {
        get_function_result_str!(self.object, GetHardwareAddress)
    }
    /// Type of protocol encapsulation used.
    ///
    /// # Returns
    ///
    /// Returns [`HostNetworkInterfaceMediumType`] success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::Host;
    ///
    /// let host = Host::init().unwrap();
    /// let network_interfaces = host.get_network_interfaces().unwrap();
    /// let network_interface = network_interfaces.get(0).unwrap();
    /// let ipv6_network_mask_prefix_length = network_interface.get_medium_type().unwrap();
    pub fn get_medium_type(&self) -> Result<HostNetworkInterfaceMediumType, VboxError> {
        let medium_type = get_function_result_number!(self.object, GetMediumType, u32)?;
        Ok(HostNetworkInterfaceMediumType::from(medium_type))
    }

    /// Status of the interface.
    ///
    /// # Returns
    ///
    /// Returns [`HostNetworkInterfaceStatus`] success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::Host;
    ///
    /// let host = Host::init().unwrap();
    /// let network_interfaces = host.get_network_interfaces().unwrap();
    /// let network_interface = network_interfaces.get(0).unwrap();
    /// let status = network_interface.get_status().unwrap();
    pub fn get_status(&self) -> Result<HostNetworkInterfaceStatus, VboxError> {
        let status = get_function_result_number!(self.object, GetStatus, u32)?;
        Ok(HostNetworkInterfaceStatus::from(status))
    }

    /// specifies the host interface type.
    ///
    /// # Returns
    ///
    /// Returns [`HostNetworkInterfaceStatus`] success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::Host;
    ///
    /// let host = Host::init().unwrap();
    /// let network_interfaces = host.get_network_interfaces().unwrap();
    /// let network_interface = network_interfaces.get(0).unwrap();
    /// let status = network_interface.get_interface_type().unwrap();
    pub fn get_interface_type(&self) -> Result<HostNetworkInterfaceType, VboxError> {
        let interface_type = get_function_result_number!(self.object, GetInterfaceType, u32)?;
        Ok(HostNetworkInterfaceType::from(interface_type))
    }

    /// Specifies whether the interface is wireless.
    ///
    /// # Returns
    ///
    /// Returns a bool success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::Host;
    ///
    /// let host = Host::init().unwrap();
    /// let network_interfaces = host.get_network_interfaces().unwrap();
    /// let network_interface = network_interfaces.get(0).unwrap();
    /// let is_wireless = network_interface.get_wireless().unwrap();
    pub fn get_wireless(&self) -> Result<bool, VboxError> {
        get_function_result_bool!(self.object, GetWireless)
    }
}
