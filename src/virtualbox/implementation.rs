use crate::enums::{AccessMode, DeviceType, FirmwareType, MachineState, PlatformArchitecture};
#[cfg(doc)]
use crate::event_detail::MachineRegisteredEvent;
use crate::machine::Machine;
use crate::system_properties::SystemProperties;
use crate::utility::macros::macros::{
    get_function_result_number, get_function_result_pointer, get_function_result_pointer_vec,
    get_function_result_str, get_function_result_str_vec, get_function_result_unit,
};
use crate::utility::{c_u64_str_to_string, str_vec_to_ptr, string_to_c_u64_str};
use crate::virtualbox::VirtualBox;
use crate::{
    Appliance, CloudNetwork, DHCPServer, EventSource, ExtPackManager, GuestOSType, Host,
    HostOnlyNetwork, Medium, NATNetwork, PerformanceCollector, Progress, SharedFolder, Unattended,
    VboxError,
};
use log::debug;
#[cfg(not(is_v_6_1))]
use vbox_raw::sys_lib::IHostOnlyNetwork;
use vbox_raw::sys_lib::{
    IAppliance, ICloudNetwork, IDHCPServer, IEventSource, IExtPackManager, IGuestOSType, IHost,
    IMachine, IMedium, INATNetwork, IPerformanceCollector, IProgress, ISharedFolder,
    ISystemProperties, IUnattended,
};

impl VirtualBox {
    /// A string representing the version number of the product.
    ///
    /// # Returns
    ///
    /// Returns a &str success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::VirtualBox;
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let version = vbox.get_version().unwrap();
    ///
    /// println!("{}", version);

    pub fn get_version(&self) -> Result<&'static str, VboxError> {
        get_function_result_str!(self.object, GetVersion)
    }

    /// A string representing the version number of the product, without the publisher information (but still with other tags).
    ///
    /// # Returns
    ///
    /// Returns a &str success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::VirtualBox;
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let version_normalized = vbox.get_version_normalized().unwrap();
    ///
    /// println!("{}", version_normalized);

    pub fn get_version_normalized(&self) -> Result<&'static str, VboxError> {
        get_function_result_str!(self.object, GetVersionNormalized)
    }

    /// The internal build revision number of the product.
    ///
    /// # Returns
    ///
    /// Returns u32 success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::VirtualBox;
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let revision = vbox.get_revision().unwrap();
    ///
    /// println!("{:?}", revision);
    pub fn get_revision(&self) -> Result<u32, VboxError> {
        let revision = get_function_result_number!(self.object, GetRevision, u32)?;
        Ok(revision)
    }

    /// A string representing the package type of this product.
    ///
    /// # Returns
    ///
    /// Returns a &str success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::VirtualBox;
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let package_type = vbox.get_package_type().unwrap();
    ///
    /// println!("{}", package_type);

    pub fn get_package_type(&self) -> Result<&'static str, VboxError> {
        get_function_result_str!(self.object, GetPackageType)
    }

    /// A string representing the VirtualBox API version number.
    ///
    /// # Returns
    ///
    /// Returns a &str success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::VirtualBox;
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let api_version = vbox.get_api_version().unwrap();
    ///
    /// println!("{}", api_version);

    pub fn get_api_version(&self) -> Result<&'static str, VboxError> {
        get_function_result_str!(self.object, GetAPIVersion)
    }

    /// The internal build revision number of the product.
    ///
    /// # Returns
    ///
    /// Returns i64 success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::VirtualBox;
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let api_revision = vbox.get_api_revision().unwrap();
    ///
    /// println!("{:?}", api_revision);
    pub fn get_api_revision(&self) -> Result<i64, VboxError> {
        #[cfg(target_os = "windows")]
        let revision: i64 = get_function_result_number!(self.object, GetAPIRevision, i32)?.into();
        #[cfg(not(target_os = "windows"))]
        let revision = get_function_result_number!(self.object, GetAPIRevision, i64)?;
        Ok(revision)
    }

    /// Full path to the directory where the global settings file, VirtualBox.xml, is stored.
    ///
    /// # Returns
    ///
    /// Returns a &str success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::VirtualBox;
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let home_folder = vbox.get_home_folder().unwrap();
    ///
    /// println!("{}", home_folder);

    pub fn get_home_folder(&self) -> Result<&'static str, VboxError> {
        get_function_result_str!(self.object, GetHomeFolder)
    }
    /// Full name of the global settings file.
    ///
    /// # Returns
    ///
    /// Returns a &str success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::VirtualBox;
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let settings_file_path = vbox.get_settings_file_path().unwrap();
    ///
    /// println!("{}", settings_file_path);

    pub fn get_settings_file_path(&self) -> Result<&'static str, VboxError> {
        get_function_result_str!(self.object, GetSettingsFilePath)
    }
    /// Associated host object.
    ///
    ///
    /// # Returns
    ///
    /// Returns [`Host`] on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{VirtualBox};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    ///
    /// let host = vbox.get_host();
    ///
    pub fn get_host(&self) -> Result<Host, VboxError> {
        let host = get_function_result_pointer!(self.object, GetHost, *mut IHost)?;
        Ok(Host::new(host))
    }

    /// Associated system information object.
    ///
    ///
    /// # Returns
    ///
    /// Returns [`SystemProperties`] on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{SystemProperties, VirtualBox};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    ///
    /// let system_properties = vbox.get_system_properties();
    ///
    pub fn get_system_properties(&self) -> Result<SystemProperties, VboxError> {
        let system_properties =
            get_function_result_pointer!(self.object, GetSystemProperties, *mut ISystemProperties)?;
        Ok(SystemProperties::new(system_properties))
    }
    /// Vector of machine objects registered within this VirtualBox instance.
    ///
    ///
    /// # Returns
    ///
    /// Returns [`Vec<Machine>`] on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{SystemProperties, VirtualBox};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    ///
    /// let machines = vbox.get_machines();
    ///
    pub fn get_machines(&self) -> Result<Vec<Machine>, VboxError> {
        let mut m_count = 0;
        let machines_ptr = get_function_result_pointer!(
            self.object,
            GetMachines,
            *mut *mut IMachine,
            &mut m_count
        )?;

        let raw_vms =
            unsafe { Vec::from_raw_parts(machines_ptr, m_count as usize, m_count as usize) };

        let mut vms = Vec::new();
        for raw_vm in raw_vms {
            let m_tmp = raw_vm.clone();

            vms.push(Machine::new(m_tmp));
        }
        Ok(vms)
    }

    /// Array of all machine group names which are used by the machines which are accessible.
    ///
    ///
    /// # Returns
    ///
    /// Returns [`Vec<&str>`] on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{SystemProperties, VirtualBox};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    ///
    /// let machines = vbox.get_machine_groups().unwrap();
    ///
    pub fn get_machine_groups(&self) -> Result<Vec<&'static str>, VboxError> {
        get_function_result_str_vec!(self.object, GetMachineGroups)
    }

    /// Array of medium objects known to this VirtualBox installation.
    ///
    ///
    /// # Returns
    ///
    /// Returns [`Vec<Medium>`] on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{SystemProperties, VirtualBox};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    ///
    /// let hard_disks = vbox.get_hard_disks().unwrap();
    ///
    pub fn get_hard_disks(&self) -> Result<Vec<Medium>, VboxError> {
        let disks_ptr = get_function_result_pointer_vec!(self.object, GetHardDisks, *mut IMedium)?;
        Ok(disks_ptr
            .iter()
            .map(|disk| Medium::new(disk.clone()))
            .collect())
    }

    /// Array of CD/DVD image objects currently in use by this VirtualBox instance.
    ///
    ///
    /// # Returns
    ///
    /// Returns [`Vec<Medium>`] on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{SystemProperties, VirtualBox};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    ///
    /// let images = vbox.get_dvd_images().unwrap();
    ///
    pub fn get_dvd_images(&self) -> Result<Vec<Medium>, VboxError> {
        let images = get_function_result_pointer_vec!(self.object, GetDVDImages, *mut IMedium)?;
        Ok(images
            .iter()
            .map(|image| Medium::new(image.clone()))
            .collect())
    }

    /// Array of floppy image objects currently in use by this VirtualBox instance.
    ///
    ///
    /// # Returns
    ///
    /// Returns [`Vec<Medium>`] on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{SystemProperties, VirtualBox};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    ///
    /// let images = vbox.get_dvd_images().unwrap();
    ///
    pub fn get_floppy_images(&self) -> Result<Vec<Medium>, VboxError> {
        let images = get_function_result_pointer_vec!(self.object, GetFloppyImages, *mut IMedium)?;
        Ok(images
            .iter()
            .map(|image| Medium::new(image.clone()))
            .collect())
    }

    /// Array of ProgressOperations objects currently in use by this VirtualBox instance.
    ///
    ///
    /// # Returns
    ///
    /// Returns [`Vec<Progress>`] on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{SystemProperties, VirtualBox};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    ///
    /// let progress = vbox.get_progress_operations().unwrap();
    ///
    pub fn get_progress_operations(&self) -> Result<Vec<Progress>, VboxError> {
        let progress =
            get_function_result_pointer_vec!(self.object, GetProgressOperations, *mut IProgress)?;
        Ok(progress.iter().map(|p| Progress::new(p.clone())).collect())
    }

    /// Array of GuestOSTypes objects currently in use by this VirtualBox instance.
    ///
    ///
    /// # Returns
    ///
    /// Returns [`Vec<GuestOSType>`] on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{SystemProperties, VirtualBox};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    ///
    /// let guest_os_types = vbox.get_guest_os_types().unwrap();
    ///
    pub fn get_guest_os_types(&self) -> Result<Vec<GuestOSType>, VboxError> {
        let os_types =
            get_function_result_pointer_vec!(self.object, GetGuestOSTypes, *mut IGuestOSType)?;
        Ok(os_types
            .iter()
            .map(|os_type| GuestOSType::new(os_type.clone()))
            .collect())
    }

    /// Collection of global shared folders.
    ///
    /// Global shared folders are available to all virtual machines.
    ///
    /// New shared folders are added to the collection using createSharedFolder. Existing shared folders can be removed using removeSharedFolder.
    ///
    ///
    /// # Returns
    ///
    /// Returns [`Vec<GuestOSType>`] on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{SystemProperties, VirtualBox};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    ///
    /// let shared_folders = vbox.get_shared_folders().unwrap();
    ///
    pub fn get_shared_folders(&self) -> Result<Vec<SharedFolder>, VboxError> {
        let shared_folders =
            get_function_result_pointer_vec!(self.object, GetSharedFolders, *mut ISharedFolder)?;
        Ok(shared_folders
            .iter()
            .map(|object| SharedFolder::new(object.clone()))
            .collect())
    }

    /// Associated performance collector object.
    ///
    /// # Returns
    ///
    /// Returns [`PerformanceCollector`] on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{SystemProperties, VirtualBox};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    ///
    /// let performance_collector = vbox.get_performance_collector().unwrap();
    ///
    pub fn get_performance_collector(&self) -> Result<PerformanceCollector, VboxError> {
        let performance_collector = get_function_result_pointer!(
            self.object,
            GetPerformanceCollector,
            *mut IPerformanceCollector
        )?;
        Ok(PerformanceCollector::new(performance_collector))
    }

    /// DHCP servers.
    ///
    ///
    /// # Returns
    ///
    /// Returns [`Vec<DHCPServer>`] on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{SystemProperties, VirtualBox};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    ///
    /// let dhcp_servers = vbox.get_dhcp_servers().unwrap();
    ///
    pub fn get_dhcp_servers(&self) -> Result<Vec<DHCPServer>, VboxError> {
        let dhcp_servers =
            get_function_result_pointer_vec!(self.object, GetDHCPServers, *mut IDHCPServer)?;
        Ok(dhcp_servers
            .iter()
            .map(|object| DHCPServer::new(object.clone()))
            .collect())
    }

    /// NAT Network.
    ///
    ///
    /// # Returns
    ///
    /// Returns [`Vec<NATNetwork>`] on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{SystemProperties, VirtualBox};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    ///
    /// let nat_networks = vbox.get_nat_networks().unwrap();
    ///
    pub fn get_nat_networks(&self) -> Result<Vec<NATNetwork>, VboxError> {
        let nat_networks =
            get_function_result_pointer_vec!(self.object, GetNATNetworks, *mut INATNetwork)?;
        Ok(nat_networks
            .iter()
            .map(|object| NATNetwork::new(object.clone()))
            .collect())
    }

    /// Event source for VirtualBox events.
    ///
    /// # Returns
    ///
    /// Returns [`EventSource`] on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{SystemProperties, VirtualBox};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    ///
    /// let event_source = vbox.get_event_source();
    ///
    pub fn get_event_source(&self) -> Result<EventSource, VboxError> {
        let event_source =
            get_function_result_pointer!(self.object, GetEventSource, *mut IEventSource)?;
        Ok(EventSource::new(event_source))
    }

    /// The extension pack manager.
    ///
    /// # Returns
    ///
    /// Returns [`ExtPackManager`] on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{SystemProperties, VirtualBox};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    ///
    /// let extension_pack_manager = vbox.get_extension_pack_manager().unwrap();
    ///
    pub fn get_extension_pack_manager(&self) -> Result<ExtPackManager, VboxError> {
        let extension_pack_manager = get_function_result_pointer!(
            self.object,
            GetExtensionPackManager,
            *mut IExtPackManager
        )?;
        Ok(ExtPackManager::new(extension_pack_manager))
    }

    /// Names of all internal networks.
    ///
    /// # Returns
    ///
    /// Returns [`Vec<&str>`] on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{SystemProperties, VirtualBox};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    ///
    /// let internal_networks = vbox.get_internal_networks().unwrap();
    ///
    pub fn get_internal_networks(&self) -> Result<Vec<&'static str>, VboxError> {
        get_function_result_str_vec!(self.object, GetInternalNetworks)
    }

    /// Names of all generic network drivers.
    ///
    /// # Returns
    ///
    /// Returns [`Vec<&str>`] on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{SystemProperties, VirtualBox};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    ///
    /// let generic_network_drivers = vbox.get_generic_network_drivers().unwrap();
    ///
    pub fn get_generic_network_drivers(&self) -> Result<Vec<&'static str>, VboxError> {
        get_function_result_str_vec!(self.object, GetGenericNetworkDrivers)
    }

    // TODO GetCloudNetworks
    // TODO GetCloudProviderManager

    /// Returns a recommended full path of the settings file name for a new virtual machine.
    ///
    /// This API serves two purposes:
    ///
    /// It gets called by createMachine if empty string (which is recommended) is specified for the settingsFile argument there, which means that API should use a recommended default file name.
    ///
    /// It can be called manually by a client software before creating a machine, e.g. if that client wants to pre-create the machine directory to create virtual hard disks in that directory together with the new machine settings file. In that case, the file name should be stripped from the full settings file path returned by this function to obtain the machine directory.
    ///
    ///
    /// See [`Machine::get_name`] and [`VirtualBox::create_machine`] for more details about the machine name.
    ///
    /// groupName defines which additional subdirectory levels should be included. It must be either a valid group name or null or empty string which designates that the machine will not be related to a machine group.
    ///
    /// If baseFolder is a null or empty string (which is recommended), the default machine settings folder (see ISystemProperties::defaultMachineFolder) will be used as a base folder for the created machine, resulting in a file name like "/home/user/VirtualBox VMs/name/name.vbox". Otherwise the given base folder will be used.
    ///
    /// This method does not access the host disks. In particular, it does not check for whether a machine with this name already exists.
    ///
    ///
    /// # Arguments
    ///
    /// * `name` - &str. Suggested machine name.
    /// * `group` - &str. Machine group name for the new machine or machine group. It is used to determine the right subdirectory.
    /// * `create_flags` - &str.  Machine creation flags, see [`VirtualBox::create_machine`] (optional).
    /// * `base_folder` - &str. Base machine folder (optional).
    ///
    ///
    /// # Returns
    ///
    /// Returns fully qualified path where the machine would be created (&str) on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{SystemProperties, VirtualBox};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    ///
    /// let machine_filename = vbox.compose_machine_filename(
    ///     "machine1",
    ///     "",
    ///     "",
    ///     ""
    /// ).unwrap();
    ///
    pub fn compose_machine_filename(
        &self,
        name: &str,
        group: &str,
        create_flags: &str,
        base_folder: &str,
    ) -> Result<&'static str, VboxError> {
        let name = string_to_c_u64_str(name)?;
        let group = string_to_c_u64_str(group)?;
        let create_flags = string_to_c_u64_str(create_flags)?;
        let base_folder = string_to_c_u64_str(base_folder)?;

        get_function_result_str!(
            self.object,
            ComposeMachineFilename,
            name,
            group,
            create_flags,
            base_folder
        )
    }

    #[cfg(is_v_7_1)]
    /// Creates a new virtual machine by creating a machine settings file at the given location.
    ///
    /// VirtualBox machine settings files use a custom XML dialect. Starting with VirtualBox 4.0, a ".vbox" extension is recommended, but not enforced, and machine files can be created at arbitrary locations.
    ///
    /// However, it is recommended that machines are created in the default machine folder (e.g. "/home/user/VirtualBox VMs/name/name.vbox"; see [`SystemProperties::get_default_machine_folder`]). If you specify null or empty string (which is recommended) for the settingsFile argument, [`VirtualBox::compose_machine_filename`] is called automatically to have such a recommended name composed based on the machine name given in the name argument and the primary group.
    ///
    /// If the resulting settings file already exists, this method will fail, unless the forceOverwrite flag is set.
    ///
    /// The new machine is created unregistered, with the initial configuration set according to the specified guest OS type. A typical sequence of actions to create a new virtual machine is as follows:
    ///
    /// Call this method to have a new machine created. The returned machine object will be "mutable" allowing to change any machine property.
    ///
    /// Configure the machine using the appropriate attributes and methods.
    ///
    /// Call [`Machine::save_settings`] to write the settings to the machine's XML settings file. The configuration of the newly created machine will not be saved to disk until this method is called.
    ///
    /// Call [`VirtualBox::register_machine`] to add the machine to the list of machines known to VirtualBox.
    ///
    ///
    /// The specified guest OS type identifier must match an ID of one of known guest OS types listed in the IVirtualBox::guestOSTypes array.
    ///
    /// # Arguments
    ///
    /// * `settings_file` - &str. Fully qualified path where the settings file should be created, empty string for a default folder and file based on the name argument and the primary group. (see [`VirtualBox::compose_machine_filename`]).
    /// * `name` - &str. Machine name.
    /// * `groups` - Vec<&str>. Array of group names. null or an empty array have the same meaning as an array with just the empty string or "/", i.e. create a machine without group association.
    /// * `os_type_id` - &str. Guest OS Type ID.
    /// * `flags` - &str. Additional property parameters, passed as a comma-separated list of "name=value" type entries. The following ones are recognized: forceOverwrite=1 to overwrite an existing machine settings file, UUID=uuid to specify a machine UUID and directoryIncludesUUID=1 to switch to a special VM directory naming scheme which should not be used unless necessary.
    /// * `cipher` - &str. The cipher. It should be empty if encryption is not required.
    /// * `password_id` - &str. The password id. It should be empty if encryption is not required.
    /// * `password` - &str. The password. It should be empty if encryption is not required.
    ///
    ///
    /// # Returns
    ///
    /// Returns [`Machine`] on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{SystemProperties, VirtualBox};use virtualbox_rs::enums::PlatformArchitecture;
    ///
    /// let vbox = VirtualBox::init().unwrap();
    ///
    /// let machine = vbox.create_machine(
    ///     "",
    ///     "machine1",
    ///     PlatformArchitecture::X86,
    ///     vec![],
    ///     "Ubuntu (64-bit)",
    ///     "",
    ///     "",
    ///     "",
    ///     ""
    /// ).unwrap();
    /// machine.save_settings().unwrap();
    ///```
    pub fn create_machine(
        &self,
        settings_file: &str,
        name: &str,
        platform: PlatformArchitecture,
        groups: Vec<&str>,
        os_type_id: &str,
        flags: &str,
        cipher: &str,
        password_id: &str,
        password: &str,
    ) -> Result<Machine, VboxError> {
        let settings_file = string_to_c_u64_str(settings_file)?;
        let name = string_to_c_u64_str(name)?;
        let platform = platform.into();
        let (groups_size, groups_ptr) = str_vec_to_ptr(groups)?;

        let os_type_id = string_to_c_u64_str(os_type_id)?;
        let flags = string_to_c_u64_str(flags)?;
        let cipher = string_to_c_u64_str(cipher)?;
        let password_id = string_to_c_u64_str(password_id)?;
        let password = string_to_c_u64_str(password)?;

        let machine = get_function_result_pointer!(
            self.object,
            CreateMachine,
            *mut IMachine,
            settings_file,
            name,
            platform,
            groups_size,
            groups_ptr,
            os_type_id,
            flags,
            cipher,
            password_id,
            password
        )?;
        Ok(Machine::new(machine))
    }

    /// Registers the machine previously created using createMachine or opened using openMachine within this VirtualBox installation.
    ///
    /// After successful method invocation, the [`MachineRegisteredEvent`] event is fired.
    ///
    /// This method implicitly calls [`Machine::save_settings`] to save all current machine settings before registering it.
    ///
    /// # Arguments
    ///
    /// * `machine` - [`Machine`].
    ///
    /// # Returns
    ///
    /// Returns () on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{SystemProperties, VirtualBox};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    ///
    /// let machine = vbox.open_machine(
    ///     "/home/host_user/VirtualBox VMs/machine1/machine1.vbox",
    ///     ""
    /// ).unwrap();
    /// vbox.register_machine(&machine).unwrap()
    ///
    pub fn register_machine(&self, machine: &Machine) -> Result<(), VboxError> {
        let machine_ptr = machine.object;
        get_function_result_unit!(self.object, RegisterMachine, machine_ptr)
    }

    /// Attempts to find a virtual machine given its name or UUID.
    ///
    /// # Arguments
    ///
    ///  * `name_or_id` - What to search for. This can either be the UUID or the name of a virtual machine.
    ///
    /// # Returns
    ///
    /// Returns [`Machine`] on success, or a [`VboxError`] on failure.
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
    pub fn find_machines(&self, name_or_id: &str) -> Result<Machine, VboxError> {
        debug!("VirtualBox::find_machines::name_or_id: {}", name_or_id);
        let name_or_id = string_to_c_u64_str(name_or_id)?;
        let machine_ptr =
            get_function_result_pointer!(self.object, FindMachine, *mut IMachine, name_or_id)?;
        Ok(Machine::new(machine_ptr))
    }

    /// Gets all machine references which are in one of the specified groups.
    ///
    /// # Arguments
    ///
    ///  * `groups` - Vec<&str>. What groups to match. The usual group list rules apply, i.e. passing an empty list will match VMs in the toplevel group, likewise the empty string.
    /// # Returns
    ///
    /// Returns [`Vec<Machine>`] on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::VirtualBox;
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let machines = vbox.
    ///         get_machines_by_groups(vec!["/"]).unwrap();
    pub fn get_machines_by_groups(&self, groups: Vec<&str>) -> Result<Vec<Machine>, VboxError> {
        let (groups_size, groups_ptr) = str_vec_to_ptr(groups)?;

        let machines = get_function_result_pointer_vec!(
            self.object,
            GetMachinesByGroups,
            *mut IMachine,
            groups_size,
            groups_ptr
        )?;
        Ok(machines
            .iter()
            .map(|object| Machine::new(object.clone()))
            .collect())
    }

    /// Gets the state of several machines in a single operation.
    ///
    /// # Arguments
    ///
    ///  * `machines` - [`Vec<Machine>`]. Vector with the machine references.
    /// # Returns
    ///
    /// Returns [`Vec<Machine>`] on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::VirtualBox;
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let machines = vbox.
    ///         get_machines_by_groups(vec!["/"]).unwrap();
    /// let states = vbox.get_machine_states(machines.iter().collect()).unwrap();
    pub fn get_machine_states(
        &self,
        machines: Vec<&Machine>,
    ) -> Result<Vec<MachineState>, VboxError> {
        let machines_size = machines.len() as u32;
        let mut machines: Vec<*mut IMachine> = machines.iter().map(|m| m.object.clone()).collect();
        let machines = machines.as_mut_ptr();

        let machines = get_function_result_pointer_vec!(
            self.object,
            GetMachineStates,
            u32,
            machines_size,
            machines
        )?;
        Ok(machines
            .iter()
            .map(|object| MachineState::from(object.clone()))
            .collect())
    }

    /// Creates a new appliance object, which represents an appliance in the Open Virtual Machine Format (OVF).
    ///
    /// This can then be used to import an OVF appliance into VirtualBox or to export machines as an OVF appliance; see the documentation for [`Appliance`] for details.
    ///
    /// # Returns
    ///
    /// Returns [`Appliance`] on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::VirtualBox;
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let appliance = vbox.create_appliance().unwrap();
    pub fn create_appliance(&self) -> Result<Appliance, VboxError> {
        let appliance =
            get_function_result_pointer!(self.object, CreateAppliance, *mut IAppliance)?;
        Ok(Appliance::new(appliance))
    }

    /// Creates a new [`Unattended`] guest installation object.
    ///
    /// This can be used to analyze an installation ISO to create and configure a new machine for it to be installed on. It can also be used to (re)install an existing machine.
    ///
    /// # Returns
    ///
    /// Returns [`Unattended`] on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::VirtualBox;
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let unattended_installer = vbox.create_unattended_installer().unwrap();
    pub fn create_unattended_installer(&self) -> Result<Unattended, VboxError> {
        let unattended_installer =
            get_function_result_pointer!(self.object, CreateUnattendedInstaller, *mut IUnattended)?;
        Ok(Unattended::new(unattended_installer))
    }

    /// Creates a new base medium object that will use the given storage format and location for medium data.
    ///
    /// # Arguments
    ///
    /// * `format` - &str. Identifier of the storage format to use for the new medium.
    /// * `location` - &str. Location of the storage unit for the new medium.
    /// * `access_mode` - [`AccessMode`]. Whether to open the image in read/write or read-only mode. For a "DVD" device type, this is ignored and read-only mode is always assumed.
    /// * `a_device_type_type` - [`DeviceType`]. Must be one of "HardDisk", "DVD" or "Floppy".
    ///
    /// # Returns
    ///
    /// Returns [`Medium`] on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{VirtualBox};
    /// use virtualbox_rs::enums::{AccessMode, DeviceType};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let medium = vbox.create_medium(
    ///     "",
    ///     "/",
    ///     AccessMode::ReadWrite,
    ///     DeviceType::HardDisk
    /// ).unwrap();
    pub fn create_medium(
        &self,
        format: &str,
        location: &str,
        access_mode: AccessMode,
        a_device_type_type: DeviceType,
    ) -> Result<Medium, VboxError> {
        let format = string_to_c_u64_str(format)?;
        let location = string_to_c_u64_str(location)?;
        let access_mode = access_mode.into();
        let a_device_type_type = a_device_type_type.into();
        let medium = get_function_result_pointer!(
            self.object,
            CreateMedium,
            *mut IMedium,
            format,
            location,
            access_mode,
            a_device_type_type
        )?;
        Ok(Medium::new(medium))
    }

    /// Finds existing media or opens a medium from an existing storage location.
    ///
    /// # Arguments
    ///
    /// * `location` - &str. Location of the storage unit for the new medium.
    /// * `a_device_type_type` - [`DeviceType`]. Must be one of "HardDisk", "DVD" or "Floppy".
    /// * `access_mode` - [`AccessMode`]. Whether to open the image in read/write or read-only mode. For a "DVD" device type, this is ignored and read-only mode is always assumed.
    /// * `force_new_uuid` - bool. Allows the caller to request a completely new medium UUID for the image which is to be opened. Useful if one intends to open an exact copy of a previously opened image, as this would normally fail due to the duplicate UUID.
    ///
    /// # Returns
    ///
    /// Returns [`Medium`] on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{ VirtualBox};
    /// use virtualbox_rs::enums::{AccessMode, DeviceType};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let medium = vbox.open_medium(
    ///     "",
    ///     DeviceType::HardDisk,
    ///     AccessMode::ReadWrite,
    ///     false
    /// ).unwrap();
    pub fn open_medium(
        &self,
        location: &str,
        a_device_type_type: DeviceType,
        access_mode: AccessMode,
        force_new_uuid: bool,
    ) -> Result<Medium, VboxError> {
        let location = string_to_c_u64_str(location)?;
        let a_device_type_type = a_device_type_type.into();
        let access_mode = access_mode.into();
        let force_new_uuid = if force_new_uuid { 1 } else { 0 };
        let medium = get_function_result_pointer!(
            self.object,
            OpenMedium,
            *mut IMedium,
            location,
            a_device_type_type,
            access_mode,
            force_new_uuid
        )?;
        Ok(Medium::new(medium))
    }
    /// Returns an object describing the specified guest OS type.
    ///
    /// The requested guest OS type is specified using a string which is a mnemonic identifier of the guest operating system, such as "win31" or "ubuntu". The guest OS type ID of a particular virtual machine can be read or set using the [`Machine::get_os_type_id`] attribute.
    ///
    /// The [`VirtualBox::get_guest_os_types`] collection contains all available guest OS type objects. Each object has an IGuestOSType::id attribute which contains an identifier of the guest OS this object describes.
    ///
    /// While this function returns an error for unknown guest OS types, they can be still used without serious problems (if one accepts the fact that there is no default VM config information).
    ///
    /// # Arguments
    ///
    /// * `id` - &str. Guest OS type ID string.
    ///
    /// # Returns
    ///
    /// Returns [`GuestOSType`] on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{VirtualBox};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let medium = vbox.get_guest_os_type(
    ///     "FreeBSD"
    /// ).unwrap();
    pub fn get_guest_os_type(&self, id: &str) -> Result<GuestOSType, VboxError> {
        let id = string_to_c_u64_str(id)?;
        let os_type =
            get_function_result_pointer!(self.object, GetGuestOSType, *mut IGuestOSType, id)?;
        Ok(GuestOSType::new(os_type))
    }

    /// Creates a new global shared folder by associating the given logical name with the given host path, adds it to the collection of shared folders and starts sharing it.
    ///
    /// Refer to the description of [`SharedFolder`] to read more about logical names.
    /// # Arguments
    ///
    /// * `name` - &str. Unique logical name of the shared folder.
    /// * `host_path` - &str. Full path to the shared folder in the host file system.
    /// * `writable` - bool. Whether the share is writable or readonly
    /// * `automount` - bool. Whether the share gets automatically mounted by the guest or not.
    /// * `auto_mount_point` - &str. Where the guest should automatically mount the folder, if possible. For Windows and OS/2 guests this should be a drive letter, while other guests it should be bsolute directory.
    ///
    /// # Returns
    ///
    /// Returns () on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{VirtualBox};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// vbox.create_shared_folder(
    ///     "shared1",
    ///     "/mnt",
    ///     false,
    ///     false,
    ///     ""
    /// ).unwrap();
    pub fn create_shared_folder(
        &self,
        name: &str,
        host_path: &str,
        writable: bool,
        automount: bool,
        auto_mount_point: &str,
    ) -> Result<(), VboxError> {
        let name = string_to_c_u64_str(name)?;
        let host_path = string_to_c_u64_str(host_path)?;
        let writable = if writable { 1 } else { 0 };
        let automount = if automount { 1 } else { 0 };
        let auto_mount_point = string_to_c_u64_str(auto_mount_point)?;
        get_function_result_unit!(
            self.object,
            CreateSharedFolder,
            name,
            host_path,
            writable,
            automount,
            auto_mount_point
        )
    }
    /// Removes the global shared folder with the given name previously created by [`VirtualBox::create_shared_folder`] from the collection of shared folders and stops sharing it.
    ///
    /// # Arguments
    ///
    /// * `name` - &str. Logical name of the shared folder to remove.
    ///
    /// # Returns
    ///
    /// Returns () on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{VirtualBox};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// vbox.remove_shared_folder(
    ///     "shared1"
    /// ).unwrap();
    pub fn remove_shared_folder(&self, name: &str) -> Result<(), VboxError> {
        let name = string_to_c_u64_str(name)?;
        get_function_result_unit!(self.object, RemoveSharedFolder, name)
    }

    /// Returns an array representing the global extra data keys which currently have values defined.
    ///
    ///
    /// # Returns
    ///
    /// Returns [`Vec<&str>`] on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{VirtualBox};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let extra_data_keys = vbox.get_extra_data_keys().unwrap();
    pub fn get_extra_data_keys(&self) -> Result<Vec<&'static str>, VboxError> {
        get_function_result_str_vec!(self.object, GetExtraDataKeys)
    }
    /// Returns associated global extra data.
    ///
    /// If the requested data key does not exist, this function will succeed and return an empty string in the value argument.
    ///
    /// # Arguments
    ///
    /// * `key` - &str. Name of the data key to get.
    ///
    /// # Returns
    ///
    /// Returns [`&str`] on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{VirtualBox};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let extra_data = vbox.get_extra_data("GUI/LanguageID").unwrap();
    pub fn get_extra_data(&self, key: &str) -> Result<&'static str, VboxError> {
        let key = string_to_c_u64_str(key)?;
        get_function_result_str!(self.object, GetExtraData, key)
    }

    /// Sets associated global extra data.
    ///
    /// If you pass  an empty string as a key value, the given key will be deleted.
    /// # Arguments
    ///
    /// * `key` - &str. Name of the data key to set.
    /// * `value` - &str. Value to assign to the key.
    ///
    /// # Returns
    ///
    /// Returns () on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{VirtualBox};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// vbox.set_extra_data("GUI/LanguageID", "en").unwrap();
    pub fn set_extra_data(&self, key: &str, value: &str) -> Result<(), VboxError> {
        let key = string_to_c_u64_str(key)?;
        let value = string_to_c_u64_str(value)?;
        get_function_result_unit!(self.object, SetExtraData, key, value)
    }

    /// Unlocks the secret data by passing the unlock password to the server.
    ///
    /// The server will cache the password for that machine.
    ///
    /// # Arguments
    ///
    /// * `password` - &str. The cipher key.
    ///
    /// # Returns
    ///
    /// Returns () on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{VirtualBox};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// vbox.set_settings_secret("123").unwrap();
    pub fn set_settings_secret(&self, password: &str) -> Result<(), VboxError> {
        let password = string_to_c_u64_str(password)?;
        get_function_result_unit!(self.object, SetSettingsSecret, password)
    }

    /// Creates a DHCP server settings to be used for the given internal network name.
    ///
    /// # Arguments
    ///
    /// * `name` - &str. server name
    ///
    /// # Returns
    ///
    /// Returns [`DHCPServer`] on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{VirtualBox};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let dhcp_server = vbox.create_dhcp_server("DHCP1").unwrap();
    pub fn create_dhcp_server(&self, name: &str) -> Result<DHCPServer, VboxError> {
        let name = string_to_c_u64_str(name)?;
        let dhcp_server =
            get_function_result_pointer!(self.object, CreateDHCPServer, *mut IDHCPServer, name)?;
        Ok(DHCPServer::new(dhcp_server))
    }

    /// Searches a DHCP server settings to be used for the given internal network name.
    ///
    /// # Arguments
    ///
    /// * `name` - &str. server name
    ///
    /// # Returns
    ///
    /// Returns [`DHCPServer`] on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{VirtualBox};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let dhcp_server = vbox.find_dhcp_server_by_network_name("DHCP1").unwrap();
    pub fn find_dhcp_server_by_network_name(&self, name: &str) -> Result<DHCPServer, VboxError> {
        let name = string_to_c_u64_str(name)?;
        let dhcp_server = get_function_result_pointer!(
            self.object,
            FindDHCPServerByNetworkName,
            *mut IDHCPServer,
            name
        )?;
        Ok(DHCPServer::new(dhcp_server))
    }

    /// Removes the DHCP server settings.
    ///
    /// # Arguments
    ///
    /// * `server` - &str. DHCP server settings to be removed
    ///
    /// # Returns
    ///
    /// Returns () on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{VirtualBox};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let dhcp_server = vbox.find_dhcp_server_by_network_name("DHCP1").unwrap();
    /// vbox.remove_dhcp_server(&dhcp_server).unwrap()
    pub fn remove_dhcp_server(&self, server: &DHCPServer) -> Result<(), VboxError> {
        let server = server.object;
        get_function_result_unit!(self.object, RemoveDHCPServer, server)
    }

    /// Creates a NATNetwork
    ///
    /// # Arguments
    ///
    /// * `network_name` - &str. Network name
    ///
    /// # Returns
    ///
    /// Returns NATNetwork on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{VirtualBox};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let dhcp_server = vbox.find_dhcp_server_by_network_name("DHCP1").unwrap();
    pub fn create_nat_network(&self, network_name: &str) -> Result<NATNetwork, VboxError> {
        let network_name = string_to_c_u64_str(network_name)?;

        let nat_network = get_function_result_pointer!(
            self.object,
            CreateNATNetwork,
            *mut INATNetwork,
            network_name
        )?;
        Ok(NATNetwork::new(nat_network))
    }

    /// Searches a NATNetwork server settings to be used for the given internal network name.
    ///
    /// # Arguments
    ///
    /// * `name` - &str. network name
    ///
    /// # Returns
    ///
    /// Returns [`DHCPServer`] on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{VirtualBox};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let nat_network = vbox.find_nat_network_by_name("DHCP1").unwrap();
    pub fn find_nat_network_by_name(&self, name: &str) -> Result<NATNetwork, VboxError> {
        let name = string_to_c_u64_str(name)?;
        let nat_network = get_function_result_pointer!(
            self.object,
            FindNATNetworkByName,
            *mut INATNetwork,
            name
        )?;
        Ok(NATNetwork::new(nat_network))
    }

    /// Removes the NATNetwork settings.
    ///
    /// # Arguments
    ///
    /// * `network` - &str. NATNetwork settings to be removed
    ///
    /// # Returns
    ///
    /// Returns () on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{VirtualBox};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let network = vbox.find_nat_network_by_name("DHCP1").unwrap();
    /// vbox.remove_nat_network(&network).unwrap()
    pub fn remove_nat_network(&self, network: &NATNetwork) -> Result<(), VboxError> {
        let network = network.object;
        get_function_result_unit!(self.object, RemoveNATNetwork, network)
    }

    /// Creates a CloudNetwork
    ///
    /// # Arguments
    ///
    /// * `network_name` - &str. Network name
    ///
    /// # Returns
    ///
    /// Returns [`CloudNetwork`] on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{VirtualBox};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let cloud_network = vbox.creat_cloud_network("CN1").unwrap();
    pub fn creat_cloud_network(&self, network_name: &str) -> Result<CloudNetwork, VboxError> {
        let network_name = string_to_c_u64_str(network_name)?;

        let cloud_network = get_function_result_pointer!(
            self.object,
            CreateCloudNetwork,
            *mut ICloudNetwork,
            network_name
        )?;
        Ok(CloudNetwork::new(cloud_network))
    }

    /// Searches a CloudNetwork server settings to be used for the given internal network name.
    ///
    /// # Arguments
    ///
    /// * `name` - &str. network name
    ///
    /// # Returns
    ///
    /// Returns [`CloudNetwork`] on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{VirtualBox};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let nat_network = vbox.find_nat_network_by_name("CN1").unwrap();
    pub fn find_cloud_network_by_name(&self, name: &str) -> Result<CloudNetwork, VboxError> {
        let name = string_to_c_u64_str(name)?;
        let cloud_network = get_function_result_pointer!(
            self.object,
            FindCloudNetworkByName,
            *mut ICloudNetwork,
            name
        )?;
        Ok(CloudNetwork::new(cloud_network))
    }

    /// Removes the CloudNetwork settings.
    ///
    /// # Arguments
    ///
    /// * `network` - &str. CloudNetwork settings to be removed
    ///
    /// # Returns
    ///
    /// Returns () on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{VirtualBox};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let network = vbox.find_cloud_network_by_name("CN1").unwrap();
    /// vbox.remove_cloud_network(&network).unwrap()
    pub fn remove_cloud_network(&self, network: &CloudNetwork) -> Result<(), VboxError> {
        let network = network.object;
        get_function_result_unit!(self.object, RemoveCloudNetwork, network)
    }
    #[cfg(is_v_7_1)]
    /// Check if this VirtualBox installation has a firmware of the given type available, either system-wide or per-user.
    ///
    /// Optionally, this may return a hint where this firmware can be downloaded from.
    ///
    /// # Arguments
    ///
    /// * `firmware_type` - [`FirmwareType`]. Type of firmware to check.
    /// * `version` - [`&str`]. Expected version number, usually empty string (presently ignored).
    ///
    /// # Returns
    ///
    /// The method returns a tuple with the following values:
    ///
    /// * `&str` - Suggested URL to download this firmware from.
    ///
    /// * `&str` - Filename of firmware, only valid if `result == true`.
    ///
    /// * `bool` - Indicates if firmware of this type and version is available.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{VirtualBox};
    /// use virtualbox_rs::enums::{FirmwareType, PlatformArchitecture};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let firmware_present = vbox.check_firmware_present(PlatformArchitecture::X86, FirmwareType::BIOS, "1.0.0").unwrap();
    pub fn check_firmware_present(
        &self,
        platform_architecture: PlatformArchitecture,
        firmware_type: FirmwareType,
        version: &str,
    ) -> Result<(&str, &str, bool), VboxError> {
        let platform_architecture = platform_architecture.into();
        let firmware_type = firmware_type.into();
        let version = string_to_c_u64_str(version)?;
        let mut url: *mut u16 = std::ptr::null_mut();
        let mut file: *mut u16 = std::ptr::null_mut();
        let mut result = 0;

        get_function_result_unit!(
            self.object,
            CheckFirmwarePresent,
            platform_architecture,
            firmware_type,
            version,
            &mut url,
            &mut file,
            &mut result
        )?;
        let url = c_u64_str_to_string(url)?;
        let file = c_u64_str_to_string(file)?;
        let result = result == 1;
        Ok((url, file, result))
    }

    #[cfg(not(is_v_6_1))]
    /// Names of all host-only networks.
    ///
    ///
    /// # Returns
    ///
    /// Returns [`Vec<HostOnlyNetwork>`] on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{SystemProperties, VirtualBox};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    ///
    /// let networks = vbox.get_host_only_networks().unwrap();
    ///
    pub fn get_host_only_networks(&self) -> Result<Vec<HostOnlyNetwork>, VboxError> {
        let networks = get_function_result_pointer_vec!(
            self.object,
            GetHostOnlyNetworks,
            *mut IHostOnlyNetwork
        )?;
        Ok(networks
            .iter()
            .map(|object| HostOnlyNetwork::new(object.clone()))
            .collect())
    }

    #[cfg(not(is_v_6_1))]
    /// Opens a virtual machine from the existing settings file.
    ///
    /// The opened machine remains unregistered until you call [`VirtualBox::register_machine`].
    ///
    /// The specified settings file name must be fully qualified. The file must exist and be a valid machine XML settings file whose contents will be used to construct the machine object.
    ///
    /// # Arguments
    ///
    /// * `settings_file` - &str. Name of the machine settings file.
    /// * `password` - &str. The password. If the machine is not encrypted the parameter is ignored.
    ///
    ///
    /// # Returns
    ///
    /// Returns [`Machine`] on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{SystemProperties, VirtualBox};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    ///
    /// let machine = vbox.open_machine(
    ///     "/home/host_user/VirtualBox VMs/machine1/machine1.vbox",
    ///     ""
    /// ).unwrap();
    ///
    pub fn open_machine(&self, settings_file: &str, password: &str) -> Result<Machine, VboxError> {
        let settings_file = string_to_c_u64_str(settings_file)?;
        let password = string_to_c_u64_str(password)?;

        let machine = get_function_result_pointer!(
            self.object,
            OpenMachine,
            *mut IMachine,
            settings_file,
            password
        )?;
        Ok(Machine::new(machine))
    }

    #[cfg(not(is_v_6_1))]
    /// Creates a HostOnlyNetwork
    ///
    /// # Arguments
    ///
    /// * `network_name` - &str. Network name
    ///
    /// # Returns
    ///
    /// Returns [`HostOnlyNetwork`] on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{VirtualBox};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let host_only_network = vbox.create_host_only_network("HON").unwrap();

    pub fn create_host_only_network(
        &self,
        network_name: &str,
    ) -> Result<HostOnlyNetwork, VboxError> {
        let network_name = string_to_c_u64_str(network_name)?;

        let host_only_network = get_function_result_pointer!(
            self.object,
            CreateHostOnlyNetwork,
            *mut IHostOnlyNetwork,
            network_name
        )?;
        Ok(HostOnlyNetwork::new(host_only_network))
    }

    #[cfg(not(is_v_6_1))]
    /// Searches a HostOnlyNetwork server settings to be used for the given internal network name.
    ///
    /// # Arguments
    ///
    /// * `name` - &str. network name
    ///
    /// # Returns
    ///
    /// Returns [`HostOnlyNetwork`] on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{VirtualBox};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let nat_network = vbox.find_nat_network_by_name("HON").unwrap();
    pub fn find_host_only_network_by_name(&self, name: &str) -> Result<HostOnlyNetwork, VboxError> {
        let name = string_to_c_u64_str(name)?;
        let host_only_network = get_function_result_pointer!(
            self.object,
            FindHostOnlyNetworkByName,
            *mut IHostOnlyNetwork,
            name
        )?;
        Ok(HostOnlyNetwork::new(host_only_network))
    }
    #[cfg(not(is_v_6_1))]
    /// Searches a HostOnlyNetwork server settings to be used for the id.
    ///
    /// # Arguments
    ///
    /// * `id` - &str. network id
    ///
    /// # Returns
    ///
    /// Returns [`HostOnlyNetwork`] on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{VirtualBox};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let nat_network = vbox.find_host_only_network_by_id("HON").unwrap();
    pub fn find_host_only_network_by_id(&self, id: &str) -> Result<HostOnlyNetwork, VboxError> {
        let id = string_to_c_u64_str(id)?;
        let host_only_network = get_function_result_pointer!(
            self.object,
            FindHostOnlyNetworkById,
            *mut IHostOnlyNetwork,
            id
        )?;
        Ok(HostOnlyNetwork::new(host_only_network))
    }

    #[cfg(not(is_v_6_1))]

    /// Removes the NATNetwork settings.
    ///
    /// # Arguments
    ///
    /// * `network` - &str. NATNetwork settings to be removed
    ///
    /// # Returns
    ///
    /// Returns () on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{VirtualBox};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let network = vbox.find_host_only_network_by_name("HON").unwrap();
    /// vbox.remove_host_only_network(&network).unwrap()
    pub fn remove_host_only_network(&self, network: &HostOnlyNetwork) -> Result<(), VboxError> {
        let network = network.object;
        get_function_result_unit!(self.object, RemoveHostOnlyNetwork, network)
    }
    #[cfg(not(is_v_6_1))]
    /// Searches through all progress objects known to VBoxSVC for an instance with the given GUID.
    ///
    /// # Arguments
    ///
    /// * `id` - &str. GUID of the progress object to search for.
    ///
    /// # Returns
    ///
    /// Returns [`Progress`] on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{VirtualBox};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let progress = vbox.find_progress_by_id("ID").unwrap();
    pub fn find_progress_by_id(&self, id: &str) -> Result<Progress, VboxError> {
        let id = string_to_c_u64_str(id)?;

        let progress =
            get_function_result_pointer!(self.object, FindProgressById, *mut IProgress, id)?;
        Ok(Progress::new(progress))
    }
}
#[cfg(not(is_v_7_1))]
impl VirtualBox {
    /// Check if this VirtualBox installation has a firmware of the given type available, either system-wide or per-user.
    ///
    /// Optionally, this may return a hint where this firmware can be downloaded from.
    ///
    /// # Arguments
    ///
    /// * `firmware_type` - [`FirmwareType`]. Type of firmware to check.
    /// * `version` - [`&str`]. Expected version number, usually empty string (presently ignored).
    ///
    /// # Returns
    ///
    /// The method returns a tuple with the following values:
    ///
    /// * `&str` - Suggested URL to download this firmware from.
    ///
    /// * `&str` - Filename of firmware, only valid if `result == true`.
    ///
    /// * `bool` - Indicates if firmware of this type and version is available.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{VirtualBox};
    /// use virtualbox_rs::enums::{FirmwareType, PlatformArchitecture};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let firmware_present = vbox.check_firmware_present(PlatformArchitecture::X86, FirmwareType::BIOS, "1.0.0").unwrap();
    pub fn check_firmware_present(
        &self,
        _platform_architecture: PlatformArchitecture,
        firmware_type: FirmwareType,
        version: &str,
    ) -> Result<(&str, &str, bool), VboxError> {
        let firmware_type = firmware_type.into();
        let version = string_to_c_u64_str(version)?;
        let mut url: *mut u16 = std::ptr::null_mut();
        let mut file: *mut u16 = std::ptr::null_mut();
        let mut result = 0;

        get_function_result_unit!(
            self.object,
            CheckFirmwarePresent,
            firmware_type,
            version,
            &mut url,
            &mut file,
            &mut result
        )?;
        let url = c_u64_str_to_string(url)?;
        let file = c_u64_str_to_string(file)?;
        let result = result == 1;
        Ok((url, file, result))
    }
}

#[cfg(is_v_7_0)]
impl VirtualBox {
    /// Creates a new virtual machine by creating a machine settings file at the given location.
    ///
    /// VirtualBox machine settings files use a custom XML dialect. Starting with VirtualBox 4.0, a ".vbox" extension is recommended, but not enforced, and machine files can be created at arbitrary locations.
    ///
    /// However, it is recommended that machines are created in the default machine folder (e.g. "/home/user/VirtualBox VMs/name/name.vbox"; see [`SystemProperties::get_default_machine_folder`]). If you specify null or empty string (which is recommended) for the settingsFile argument, [`VirtualBox::compose_machine_filename`] is called automatically to have such a recommended name composed based on the machine name given in the name argument and the primary group.
    ///
    /// If the resulting settings file already exists, this method will fail, unless the forceOverwrite flag is set.
    ///
    /// The new machine is created unregistered, with the initial configuration set according to the specified guest OS type. A typical sequence of actions to create a new virtual machine is as follows:
    ///
    /// Call this method to have a new machine created. The returned machine object will be "mutable" allowing to change any machine property.
    ///
    /// Configure the machine using the appropriate attributes and methods.
    ///
    /// Call [`Machine::save_settings`] to write the settings to the machine's XML settings file. The configuration of the newly created machine will not be saved to disk until this method is called.
    ///
    /// Call [`VirtualBox::register_machine`] to add the machine to the list of machines known to VirtualBox.
    ///
    ///
    /// The specified guest OS type identifier must match an ID of one of known guest OS types listed in the IVirtualBox::guestOSTypes array.
    ///
    /// # Arguments
    ///
    /// * `settings_file` - &str. Fully qualified path where the settings file should be created, empty string for a default folder and file based on the name argument and the primary group. (see [`VirtualBox::compose_machine_filename`]).
    /// * `name` - &str. Machine name.
    /// * `groups` - Vec<&str>. Array of group names. null or an empty array have the same meaning as an array with just the empty string or "/", i.e. create a machine without group association.
    /// * `os_type_id` - &str. Guest OS Type ID.
    /// * `flags` - &str. Additional property parameters, passed as a comma-separated list of "name=value" type entries. The following ones are recognized: forceOverwrite=1 to overwrite an existing machine settings file, UUID=uuid to specify a machine UUID and directoryIncludesUUID=1 to switch to a special VM directory naming scheme which should not be used unless necessary.
    /// * `cipher` - &str. The cipher. It should be empty if encryption is not required.
    /// * `password_id` - &str. The password id. It should be empty if encryption is not required.
    /// * `password` - &str. The password. It should be empty if encryption is not required.
    ///
    ///
    /// # Returns
    ///
    /// Returns [`Machine`] on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{SystemProperties, VirtualBox};use virtualbox_rs::enums::PlatformArchitecture;
    ///
    /// let vbox = VirtualBox::init().unwrap();
    ///
    /// let machine = vbox.create_machine(
    ///     "",
    ///     "machine1",
    ///     PlatformArchitecture::None,
    ///     vec![],
    ///     "Ubuntu (64-bit)",
    ///     "",
    ///     "",
    ///     "",
    ///     ""
    /// ).unwrap();
    /// machine.save_settings().unwrap();
    ///```
    pub fn create_machine(
        &self,
        settings_file: &str,
        name: &str,
        _platform: PlatformArchitecture,
        groups: Vec<&str>,
        os_type_id: &str,
        flags: &str,
        cipher: &str,
        password_id: &str,
        password: &str,
    ) -> Result<Machine, VboxError> {
        let settings_file = string_to_c_u64_str(settings_file)?;
        let name = string_to_c_u64_str(name)?;
        let (groups_size, groups_ptr) = str_vec_to_ptr(groups)?;

        let os_type_id = string_to_c_u64_str(os_type_id)?;
        let flags = string_to_c_u64_str(flags)?;
        let cipher = string_to_c_u64_str(cipher)?;
        let password_id = string_to_c_u64_str(password_id)?;
        let password = string_to_c_u64_str(password)?;

        let machine = get_function_result_pointer!(
            self.object,
            CreateMachine,
            *mut IMachine,
            settings_file,
            name,
            groups_size,
            groups_ptr,
            os_type_id,
            flags,
            cipher,
            password_id,
            password
        )?;
        Ok(Machine::new(machine))
    }
}
#[cfg(is_v_6_1)]
impl VirtualBox {
    /// Placeholder Method
    ///
    /// This method serves as a placeholder for versions of the API where the actual method is not available.
    /// Attempting to call this method will result in an `UnsupportedInCurrentApiVersion` error.
    /// This method is intended to ensure that the codebase can be compiled against multiple API versions
    /// without modification.
    ///
    /// Supported from API version: v7_0
    pub fn get_host_only_networks(&self) -> Result<Vec<HostOnlyNetwork>, VboxError> {
        Err(VboxError::unsupported_in_current_api_version(
            "AudioSettings::get_adapter",
            "v7_0",
        ))
    }

    /// Creates a new virtual machine by creating a machine settings file at the given location.
    ///
    /// VirtualBox machine settings files use a custom XML dialect. Starting with VirtualBox 4.0, a ".vbox" extension is recommended, but not enforced, and machine files can be created at arbitrary locations.
    ///
    /// However, it is recommended that machines are created in the default machine folder (e.g. "/home/user/VirtualBox VMs/name/name.vbox"; see [`SystemProperties::get_default_machine_folder`]). If you specify null or empty string (which is recommended) for the settingsFile argument, [`VirtualBox::compose_machine_filename`] is called automatically to have such a recommended name composed based on the machine name given in the name argument and the primary group.
    ///
    /// If the resulting settings file already exists, this method will fail, unless the forceOverwrite flag is set.
    ///
    /// The new machine is created unregistered, with the initial configuration set according to the specified guest OS type. A typical sequence of actions to create a new virtual machine is as follows:
    ///
    /// Call this method to have a new machine created. The returned machine object will be "mutable" allowing to change any machine property.
    ///
    /// Configure the machine using the appropriate attributes and methods.
    ///
    /// Call [`Machine::save_settings`] to write the settings to the machine's XML settings file. The configuration of the newly created machine will not be saved to disk until this method is called.
    ///
    /// Call [`VirtualBox::register_machine`] to add the machine to the list of machines known to VirtualBox.
    ///
    ///
    /// The specified guest OS type identifier must match an ID of one of known guest OS types listed in the IVirtualBox::guestOSTypes array.
    ///
    /// # Arguments
    ///
    /// * `settings_file` - &str. Fully qualified path where the settings file should be created, empty string for a default folder and file based on the name argument and the primary group. (see [`VirtualBox::compose_machine_filename`]).
    /// * `name` - &str. Machine name.
    /// * `groups` - Vec<&str>. Array of group names. null or an empty array have the same meaning as an array with just the empty string or "/", i.e. create a machine without group association.
    /// * `os_type_id` - &str. Guest OS Type ID.
    /// * `flags` - &str. Additional property parameters, passed as a comma-separated list of "name=value" type entries. The following ones are recognized: forceOverwrite=1 to overwrite an existing machine settings file, UUID=uuid to specify a machine UUID and directoryIncludesUUID=1 to switch to a special VM directory naming scheme which should not be used unless necessary.
    /// * `cipher` - &str. This parameter is retained for compatibility but is unused in version 6_1.
    /// * `password_id` - &str. This parameter is retained for compatibility but is unused in version 6_1.
    /// * `password` - &str. This parameter is retained for compatibility but is unused in version 6_1.
    ///
    ///
    /// # Returns
    ///
    /// Returns [`Machine`] on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{SystemProperties, VirtualBox};use virtualbox_rs::enums::PlatformArchitecture;
    ///
    /// let vbox = VirtualBox::init().unwrap();
    ///
    /// let machine = vbox.create_machine(
    ///     "",
    ///     "machine1",
    ///     PlatformArchitecture::None,
    ///     vec![],
    ///     "Ubuntu (64-bit)",
    ///     "",
    ///     "",
    ///     "",
    ///     ""
    /// ).unwrap();
    /// machine.save_settings().unwrap();
    ///```
    pub fn create_machine(
        &self,
        settings_file: &str,
        name: &str,
        _platform: PlatformArchitecture,
        groups: Vec<&str>,
        os_type_id: &str,
        flags: &str,
        _cipher: &str,
        _password_id: &str,
        _password: &str,
    ) -> Result<Machine, VboxError> {
        let settings_file = string_to_c_u64_str(settings_file)?;
        let name = string_to_c_u64_str(name)?;
        let (groups_size, groups_ptr) = str_vec_to_ptr(groups)?;

        let os_type_id = string_to_c_u64_str(os_type_id)?;
        let flags = string_to_c_u64_str(flags)?;
        let machine = get_function_result_pointer!(
            self.object,
            CreateMachine,
            *mut IMachine,
            settings_file,
            name,
            groups_size,
            groups_ptr,
            os_type_id,
            flags
        )?;
        Ok(Machine::new(machine))
    }

    /// Opens a virtual machine from the existing settings file.
    ///
    /// The opened machine remains unregistered until you call [`VirtualBox::register_machine`].
    ///
    /// The specified settings file name must be fully qualified. The file must exist and be a valid machine XML settings file whose contents will be used to construct the machine object.
    ///
    /// # Arguments
    ///
    /// * `settings_file` - &str. Name of the machine settings file.
    /// * `password` - &str. This parameter is retained for compatibility but is unused in version 6_1
    ///
    ///
    /// # Returns
    ///
    /// Returns [`Machine`] on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{SystemProperties, VirtualBox};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    ///
    /// let machine = vbox.open_machine(
    ///     "/home/host_user/VirtualBox VMs/machine1/machine1.vbox",
    ///     ""
    /// ).unwrap();
    ///
    pub fn open_machine(&self, settings_file: &str, _password: &str) -> Result<Machine, VboxError> {
        let settings_file = string_to_c_u64_str(settings_file)?;

        let machine =
            get_function_result_pointer!(self.object, OpenMachine, *mut IMachine, settings_file)?;
        Ok(Machine::new(machine))
    }

    /// Placeholder Method
    ///
    /// This method serves as a placeholder for versions of the API where the actual method is not available.
    /// Attempting to call this method will result in an `UnsupportedInCurrentApiVersion` error.
    /// This method is intended to ensure that the codebase can be compiled against multiple API versions
    /// without modification.
    ///
    /// Supported from API version: v7_0
    pub fn create_host_only_network(
        &self,
        _network_name: &str,
    ) -> Result<HostOnlyNetwork, VboxError> {
        Err(VboxError::unsupported_in_current_api_version(
            "AudioSettings::get_adapter",
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
    pub fn find_host_only_network_by_name(
        &self,
        _name: &str,
    ) -> Result<HostOnlyNetwork, VboxError> {
        Err(VboxError::unsupported_in_current_api_version(
            "AudioSettings::get_adapter",
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
    pub fn find_host_only_network_by_id(&self, _id: &str) -> Result<HostOnlyNetwork, VboxError> {
        Err(VboxError::unsupported_in_current_api_version(
            "AudioSettings::get_adapter",
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
    pub fn remove_host_only_network(&self, _network: HostOnlyNetwork) -> Result<(), VboxError> {
        Err(VboxError::unsupported_in_current_api_version(
            "AudioSettings::get_adapter",
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
    pub fn find_progress_by_id(&self, _id: &str) -> Result<Progress, VboxError> {
        Err(VboxError::unsupported_in_current_api_version(
            "AudioSettings::get_adapter",
            "v7_0",
        ))
    }
}
