use crate::utility::macros::macros::{
    get_function_result_number, get_function_result_pointer, get_function_result_str,
};
use crate::{Host, HostNetworkInterface, VboxError};
use vbox_raw::sys_lib::IHostNetworkInterface;

impl Host {
    /// Number of (logical) CPUs installed in the host system.
    ///
    /// # Arguments
    ///
    /// * `cpu_id` - Identifier of the CPU.
    ///
    /// # Returns
    ///
    /// Returns u32 success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{Host, VirtualBox};
    ///
    /// let host = Host::init().unwrap();
    /// let speed = host.get_processor_speed(0).unwrap();
    ///
    /// println!("{:?}", speed);
    pub fn get_processor_speed(&self, cpu_id: u32) -> Result<u32, VboxError> {
        let count = get_function_result_number!(self.object, GetProcessorSpeed, u32, cpu_id)?;
        Ok(count)
    }

    /// Query the model string of a specified host CPU.
    ///
    /// # Arguments
    ///
    /// * `cpu_id` - Identifier of the CPU.
    ///
    /// # Returns
    ///
    /// Returns 7str success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{Host, VirtualBox};
    ///
    /// let host = Host::init().unwrap();
    /// let processor_description = host.get_processor_description(0).unwrap();
    ///
    /// println!("{:?}", processor_description);
    pub fn get_processor_description(&self, cpu_id: u32) -> Result<&'static str, VboxError> {
        get_function_result_str!(self.object, GetProcessorDescription, cpu_id)
    }
    /// Vector of host network interfaces currently defined on the host.
    ///
    ///
    /// # Returns
    ///
    /// Returns [`Vec<HostNetworkInterface>`] on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{Host, SystemProperties, VirtualBox};
    ///
    /// let host = Host::init().unwrap();
    ///
    /// let network_interfaces = host.get_network_interfaces().unwrap();
    pub fn get_network_interfaces(&self) -> Result<Vec<HostNetworkInterface>, VboxError> {
        let mut m_count = 0;
        let network_interfaces_ptr = get_function_result_pointer!(
            self.object,
            GetNetworkInterfaces,
            *mut *mut IHostNetworkInterface,
            &mut m_count
        )?;

        let raw_nets = unsafe {
            Vec::from_raw_parts(network_interfaces_ptr, m_count as usize, m_count as usize)
        };

        let mut nets = Vec::new();
        for raw_net in raw_nets {
            let net_tmp = raw_net.clone();

            nets.push(HostNetworkInterface::new(net_tmp));
        }
        Ok(nets)
    }
    /// Number of (logical) CPUs installed in the host system.
    ///
    /// # Returns
    ///
    /// Returns u32 success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{Host, VirtualBox};
    ///
    /// let host = Host::init().unwrap();
    /// let count = host.get_processor_count().unwrap();
    ///
    /// println!("{:?}", count);
    pub fn get_processor_count(&self) -> Result<u32, VboxError> {
        let count = get_function_result_number!(self.object, GetProcessorCount, u32)?;
        Ok(count)
    }
    /// Number of (logical) CPUs online in the host system.
    ///
    /// # Returns
    ///
    /// Returns u32 success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{Host, VirtualBox};
    ///
    /// let host = Host::init().unwrap();
    /// let count = host.get_processor_online_count().unwrap();
    ///
    /// println!("{:?}", count);
    pub fn get_processor_online_count(&self) -> Result<u32, VboxError> {
        let count = get_function_result_number!(self.object, GetProcessorOnlineCount, u32)?;
        Ok(count)
    }

    /// Number of physical processor cores installed in the host system.
    ///
    /// # Returns
    ///
    /// Returns u32 success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{Host, VirtualBox};
    ///
    /// let host = Host::init().unwrap();
    /// let count = host.get_processor_core_count().unwrap();
    ///
    /// println!("{:?}", count);
    pub fn get_processor_core_count(&self) -> Result<u32, VboxError> {
        let count = get_function_result_number!(self.object, GetProcessorCoreCount, u32)?;
        Ok(count)
    }
    /// Number of physical processor cores online in the host system.
    ///
    /// # Returns
    ///
    /// Returns u32 success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{Host, VirtualBox};
    ///
    /// let host = Host::init().unwrap();
    /// let count = host.get_processor_online_core_count().unwrap();
    ///
    /// println!("{:?}", count);
    pub fn get_processor_online_core_count(&self) -> Result<u32, VboxError> {
        let count = get_function_result_number!(self.object, GetProcessorOnlineCoreCount, u32)?;
        Ok(count)
    }

    /// Amount of system memory in megabytes installed in the host system.
    ///
    /// # Returns
    ///
    /// Returns u32 success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{Host, VirtualBox};
    ///
    /// let host = Host::init().unwrap();
    /// let memory_size = host.get_memory_size().unwrap();
    ///
    /// println!("{:?}", memory_size);
    pub fn get_memory_size(&self) -> Result<u32, VboxError> {
        let count = get_function_result_number!(self.object, GetMemorySize, u32)?;
        Ok(count)
    }

    /// Available system memory in the host system.
    ///
    /// # Returns
    ///
    /// Returns u32 success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{Host, VirtualBox};
    ///
    /// let host = Host::init().unwrap();
    /// let memory_size = host.get_memory_available().unwrap();
    ///
    /// println!("{:?}", memory_size);
    pub fn get_memory_available(&self) -> Result<u32, VboxError> {
        let count = get_function_result_number!(self.object, GetMemoryAvailable, u32)?;
        Ok(count)
    }

    /// Name of the host system's operating system.
    ///
    /// # Returns
    ///
    /// Returns &srt success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{Host, VirtualBox};
    ///
    /// let host = Host::init().unwrap();
    /// let operating_system = host.get_operating_system().unwrap();
    ///
    /// println!("{:?}", operating_system);
    pub fn get_operating_system(&self) -> Result<&'static str, VboxError> {
        get_function_result_str!(self.object, GetOperatingSystem)
    }

    /// Host operating system's version string.
    ///
    /// # Returns
    ///
    /// Returns &srt success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{Host, VirtualBox};
    ///
    /// let host = Host::init().unwrap();
    /// let os_version = host.get_os_version().unwrap();
    ///
    /// println!("{:?}", os_version);
    pub fn get_os_version(&self) -> Result<&'static str, VboxError> {
        get_function_result_str!(self.object, GetOSVersion)
    }
}
