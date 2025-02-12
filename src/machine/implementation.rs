use crate::enums::{
    AutostopType, BitmapFormat, CPUPropertyType, ChipsetType, CleanupMode, ClipboardMode,
    CloneMode, CloneOptions, DeviceType, DnDMode, FirmwareType, FrontEndName, HWVirtExPropertyType,
    IommuType, KeyboardHIDType, MachineState, ParavirtProvider, PointingHIDType, SessionState,
    SessionType, StorageBus, USBControllerType, VMProcPriority,
};

use crate::machine::Machine;
use crate::progress::Progress;
use crate::session::Session;
use crate::snapshot::Snapshot;
use crate::utility::macros::macros::{
    get_function_result_bool, get_function_result_number, get_function_result_pointer,
    get_function_result_pointer_vec, get_function_result_str, get_function_result_str_vec,
    get_function_result_unit,
};
use crate::utility::{c_u64_str_to_string, str_vec_to_ptr, string_to_c_u64_str};
use crate::virtualbox_error_info::VirtualBoxErrorInfo;
#[cfg(doc)]
use crate::Console;

use crate::{
    Appliance, AudioAdapter, AudioSettings, BIOSSettings, BandwidthControl, BandwidthGroup,
    FirmwareSettings, GraphicsAdapter, GuestDebugControl, Medium, MediumAttachment, NetworkAdapter,
    NvramStore, PCIDeviceAttachment, ParallelPort, Platform, RecordingSettings, SerialPort,
    SharedFolder, StorageController, SystemProperties, TrustedPlatformModule, USBController,
    USBDeviceFilters, VRDEServer, VirtualSystemDescription,
};
use crate::{VboxError, VboxErrorType, VirtualBox};
use log::debug;
use std::slice;
#[cfg(is_v_6_1)]
use vbox_raw::sys_lib::IAudioAdapter;
#[cfg(not(is_v_7_1))]
use vbox_raw::sys_lib::IBIOSSettings;
#[cfg(not(is_v_6_1))]
use vbox_raw::sys_lib::{IAudioSettings, IGuestDebugControl, INvramStore, ITrustedPlatformModule};
#[cfg(is_v_7_1)]
use vbox_raw::sys_lib::{IFirmwareSettings, IPlatform};

use vbox_raw::sys_lib::{
    IBandwidthControl, IGraphicsAdapter, IMedium, IMediumAttachment, INetworkAdapter,
    IPCIDeviceAttachment, IParallelPort, IProgress, IRecordingSettings, ISerialPort, ISharedFolder,
    ISnapshot, IStorageController, IUSBController, IUSBDeviceFilters, IVRDEServer, IVirtualBox,
    IVirtualBoxErrorInfo, IVirtualSystemDescription, PRUint8, PRUnichar,
};
impl Machine {
    /// Associated parent object.
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
    /// use virtualbox_rs::VirtualBox;
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let parent = machine.get_parent().unwrap();
    pub fn get_parent(&self) -> Result<VirtualBox, VboxError> {
        let parent = get_function_result_pointer!(self.object, GetParent, *mut IVirtualBox)?;
        Ok(VirtualBox::new(parent))
    }

    /// Overridden VM Icon details.
    ///
    ///
    /// # Returns
    ///
    /// Returns &[u8] on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use std::fs::File;
    /// use std::io::Write;
    /// use virtualbox_rs::VirtualBox;
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let icon = machine.get_icon().unwrap();
    /// let mut file1 = File::create("output.png").unwrap();
    ///  file1.write_all(icon).unwrap();
    pub fn get_icon(&self) -> Result<&[u8], VboxError> {
        let mut icon_data_size: u32 = 0;

        let icon_ptr =
            get_function_result_pointer!(self.object, GetIcon, *mut PRUint8, &mut icon_data_size)?;
        let icon_data_slice = unsafe { slice::from_raw_parts(icon_ptr, icon_data_size as usize) };
        Ok(icon_data_slice)
    }

    /// Overridden VM Icon details.
    ///
    /// # Arguments
    ///
    /// * `icon` - [`Vec<u8>`]. VM Icon details
    ///
    /// # Returns
    ///
    /// Returns () on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use std::fs::File;
    /// use std::io::Read;
    /// use virtualbox_rs::{Session, VirtualBox};
    /// use virtualbox_rs::enums::SessionType;
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let mut session = Session::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    /// let machine_mut = session.get_machine().unwrap();
    /// let mut file = File::open("icon.png").unwrap();
    /// let mut icon = Vec::new();
    /// file.read_to_end(&mut icon).unwrap();
    /// machine_mut.set_icon(icon).unwrap();
    /// machine_mut.save_settings().unwrap();
    pub fn set_icon(&self, mut icon: Vec<u8>) -> Result<(), VboxError> {
        let icon_data_size: u32 = icon.len() as u32;
        let icon = icon.as_mut_ptr();

        get_function_result_unit!(self.object, SetIcon, icon_data_size, icon)
    }

    /// Whether this virtual machine is currently accessible or not.
    ///
    /// # Returns
    ///
    /// Returns a [bool] success, or a [`VboxError`] on failure.
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
    /// let accessible = machine.get_accessible().unwrap();
    ///
    pub fn get_accessible(&self) -> Result<bool, VboxError> {
        get_function_result_bool!(self.object, GetAccessible)
    }

    /// Error information describing the reason of machine inaccessibility.
    /// # Returns
    ///
    /// Returns a [`VirtualBoxErrorInfo`] success, or a [`VboxError`] on failure.
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
    pub fn get_access_error(&self) -> Result<VirtualBoxErrorInfo, VboxError> {
        let error =
            get_function_result_pointer!(self.object, GetAccessError, *mut IVirtualBoxErrorInfo)?;
        Ok(VirtualBoxErrorInfo::new(error))
    }

    /// Name of the virtual machine.
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
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let name = machine.get_name().unwrap();
    ///
    /// println!("{}", name);

    pub fn get_name(&self) -> Result<&'static str, VboxError> {
        get_function_result_str!(self.object, GetName)
    }

    /// Name of the virtual machine.
    ///
    /// # Arguments
    ///
    /// * `name` - name of the virtual machine.
    ///

    /// # Returns
    ///
    /// Returns oK(()) success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{Session, VirtualBox};
    /// use virtualbox_rs::enums::SessionType;
    ///
    /// let vbox = VirtualBox::init().unwrap();
    ///     let mut session = Session::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    ///  machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    /// let machine_mut = session.get_machine().unwrap();
    /// machine_mut.set_name("Freebsd_14_1").unwrap();

    pub fn set_name(&self, name: &str) -> Result<(), VboxError> {
        let name_ptr = string_to_c_u64_str(name)?;

        get_function_result_unit!(self.object, SetName, name_ptr)
    }

    /// Description of the virtual machine.
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
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let description = machine.get_description().unwrap();
    ///
    /// println!("{}", description);

    pub fn get_description(&self) -> Result<&'static str, VboxError> {
        get_function_result_str!(self.object, GetDescription)
    }

    /// Description of the virtual machine.
    ///
    /// # Arguments
    ///
    /// * `description` - Description of the virtual machine.
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
    /// let machine = vbox.
    ///     find_machines("Freebsd_14").unwrap();
    /// let mut session = Session::init().unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    /// let machine_mut = session.get_machine().unwrap();
    /// machine_mut.set_description("Freebsd_14_1").unwrap();
    /// machine_mut.save_settings().unwrap();
    pub fn set_description(&self, description: &str) -> Result<(), VboxError> {
        let description_ptr = string_to_c_u64_str(description)?;

        get_function_result_unit!(self.object, SetDescription, description_ptr)
    }

    /// UUID of the virtual machine.
    ///
    /// # Returns
    ///
    /// Returns a [&str] success, or a [`VboxError`] on failure.
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
    /// let id = machine.get_id().unwrap();
    pub fn get_id(&self) -> Result<&'static str, VboxError> {
        get_function_result_str!(self.object, GetId)
    }

    /// Array of machine group names of which this machine is a member.
    ///
    /// # Returns
    ///
    /// Returns a Vec<[&str]> success, or a [`VboxError`] on failure.
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
    /// let groups = machine.get_groups().unwrap();

    pub fn get_groups(&self) -> Result<Vec<&'static str>, VboxError> {
        get_function_result_str_vec!(self.object, GetGroups)
    }

    /// Array of machine group names of which this machine is a member.
    ///
    /// # Arguments
    ///
    /// * `groups` - Vec<&str>. Array of machine group names.
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
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let mut session = Session::init().unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    /// let machine_mut = session.get_machine().unwrap();
    /// machine_mut.set_groups(vec!["/"]).unwrap();

    pub fn set_groups(&self, groups: Vec<&str>) -> Result<(), VboxError> {
        let (groups_size, groups_ptr) = str_vec_to_ptr(groups)?;

        get_function_result_unit!(self.object, SetGroups, groups_size, groups_ptr)
    }

    /// User-defined identifier of the Guest OS type.
    ///
    ///
    /// # Returns
    ///
    /// Returns a [&str] success, or a [`VboxError`] on failure.
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
    /// let os_type = machine.get_os_type_id().unwrap();
    pub fn get_os_type_id(&self) -> Result<&'static str, VboxError> {
        get_function_result_str!(self.object, GetOSTypeId)
    }

    /// User-defined identifier of the Guest OS type.
    ///
    /// # Arguments
    ///
    /// * `os_type_id` - &str.User-defined identifier of the Guest OS type.
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
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let mut session = Session::init().unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    /// let machine_mut = session.get_machine().unwrap();
    /// machine_mut.set_os_type_id("FreeBSD_64").unwrap();
    /// machine_mut.save_settings().unwrap();
    pub fn set_os_type_id(&self, os_type_id: &str) -> Result<(), VboxError> {
        let os_type_id = string_to_c_u64_str(os_type_id)?;

        get_function_result_unit!(self.object, SetOSTypeId, os_type_id)
    }

    /// Hardware version identifier.
    ///
    /// Internal use only for now.
    ///
    /// # Returns
    ///
    /// Returns a [&str] success, or a [`VboxError`] on failure.
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
    /// let hardware_version = machine.get_hardware_version().unwrap();
    pub fn get_hardware_version(&self) -> Result<&'static str, VboxError> {
        get_function_result_str!(self.object, GetHardwareVersion)
    }

    /// Hardware version identifier.
    ///
    /// Internal use only for now.
    ///
    /// # Arguments
    ///
    /// * `hardware_version` - &str. Hardware version identifier.
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
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let mut session = Session::init().unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    /// let machine_mut = session.get_machine().unwrap();
    /// machine_mut.set_hardware_version("2").unwrap();
    /// machine_mut.save_settings().unwrap();
    pub fn set_hardware_version(&self, hardware_version: &str) -> Result<(), VboxError> {
        let hardware_version = string_to_c_u64_str(hardware_version)?;

        get_function_result_unit!(self.object, SetHardwareVersion, hardware_version)
    }

    /// The UUID presented to the guest via memory tables, hardware and guest properties.
    ///
    /// For most VMs this is the same as the id, but for VMs which have been cloned or teleported it may be the same as the source VM. The latter is because the guest shouldn't notice that it was cloned or teleported.
    ///
    /// # Returns
    ///
    /// Returns a [&str] success, or a [`VboxError`] on failure.
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
    /// let hardware_version = machine.get_hardware_uuid().unwrap();
    pub fn get_hardware_uuid(&self) -> Result<&'static str, VboxError> {
        get_function_result_str!(self.object, GetHardwareUUID)
    }

    /// The UUID presented to the guest via memory tables, hardware and guest properties.
    ///
    /// # Arguments
    ///
    /// * `hardware_uuid` - &str. The UUID presented to the guest via memory tables, hardware and guest properties.
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
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let mut session = Session::init().unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    /// let machine_mut = session.get_machine().unwrap();
    /// machine_mut.set_hardware_uuid("b9b13564-b9c4-4b8b-be65-ccd849e3c161").unwrap();
    /// machine_mut.save_settings().unwrap();
    pub fn set_hardware_uuid(&self, hardware_uuid: &str) -> Result<(), VboxError> {
        let hardware_uuid = string_to_c_u64_str(hardware_uuid)?;

        get_function_result_unit!(self.object, SetHardwareUUID, hardware_uuid)
    }
    /// Number of virtual CPUs in the VM.
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
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let count = machine.get_cpu_count().unwrap();
    pub fn get_cpu_count(&self) -> Result<u32, VboxError> {
        get_function_result_number!(self.object, GetCPUCount, u32)
    }

    /// Number of virtual CPUs in the VM.
    ///
    /// # Arguments
    ///
    /// * `count` - u32. Number of virtual CPUs in the VM.
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
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let mut session = Session::init().unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    /// let machine_mut = session.get_machine().unwrap();
    /// machine_mut.set_cpu_count(2).unwrap();
    /// machine_mut.save_settings().unwrap();
    pub fn set_cpu_count(&self, count: u32) -> Result<(), VboxError> {
        get_function_result_unit!(self.object, SetCPUCount, count)
    }

    /// This setting determines whether VirtualBox allows CPU hotplugging for this machine.
    ///
    /// # Returns
    ///
    /// Returns bool success, or a [`VboxError`] on failure.
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
    /// let count = machine.get_cpu_hot_plug_enabled().unwrap();
    pub fn get_cpu_hot_plug_enabled(&self) -> Result<bool, VboxError> {
        get_function_result_bool!(self.object, GetCPUHotPlugEnabled)
    }

    /// This setting determines whether VirtualBox allows CPU hotplugging for this machine.
    ///
    /// # Arguments
    ///
    /// * `hot_plug_enabled` - bool.
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
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let mut session = Session::init().unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    /// let machine_mut = session.get_machine().unwrap();
    /// machine_mut.set_cpu_hot_plug_enabled(true).unwrap();
    /// machine_mut.save_settings().unwrap();
    pub fn set_cpu_hot_plug_enabled(&self, hot_plug_enabled: bool) -> Result<(), VboxError> {
        let hot_plug_enabled = if hot_plug_enabled { 1 } else { 0 };
        get_function_result_unit!(self.object, SetCPUHotPlugEnabled, hot_plug_enabled)
    }

    /// Means to limit the number of CPU cycles a guest can use.
    ///
    /// The unit is percentage of host CPU cycles per second. The valid range is 1 - 100. 100 (the default) implies no limit.
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
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let cpu_execution_cap = machine.get_cpu_execution_cap().unwrap();
    pub fn get_cpu_execution_cap(&self) -> Result<u32, VboxError> {
        get_function_result_number!(self.object, GetCPUExecutionCap, u32)
    }

    /// Means to limit the number of CPU cycles a guest can use.
    ///
    /// The unit is percentage of host CPU cycles per second. The valid range is 1 - 100. 100 (the default) implies no limit..
    ///
    /// # Arguments
    ///
    /// * `cpu_execution_cap` - u32.
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
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let mut session = Session::init().unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    /// let machine_mut = session.get_machine().unwrap();
    /// machine_mut.set_cpu_execution_cap(50).unwrap();
    /// machine_mut.save_settings().unwrap();
    pub fn set_cpu_execution_cap(&self, cpu_execution_cap: u32) -> Result<(), VboxError> {
        get_function_result_unit!(self.object, SetCPUExecutionCap, cpu_execution_cap)
    }

    /// Virtual CPUID portability level, the higher number the fewer newer or vendor specific CPU feature is reported to the guest (via the CPUID instruction).
    ///
    /// The default level of zero (0) means that all virtualized features supported by the host is pass thru to the guest. While the three (3) is currently the level supressing the most features.
    ///
    /// Exactly which of the CPUID features are left out by the VMM at which level is subject to change with each major version.
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
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let cpu_id_portability_level = machine.get_cpu_id_portability_level().unwrap();
    pub fn get_cpu_id_portability_level(&self) -> Result<u32, VboxError> {
        get_function_result_number!(self.object, GetCPUIDPortabilityLevel, u32)
    }

    /// Virtual CPUID portability level, the higher number the fewer newer or vendor specific CPU feature is reported to the guest (via the CPUID instruction).
    ///
    /// The default level of zero (0) means that all virtualized features supported by the host is pass thru to the guest. While the three (3) is currently the level supressing the most features.
    ///
    /// Exactly which of the CPUID features are left out by the VMM at which level is subject to change with each major version.
    ///
    /// # Arguments
    ///
    /// * `cpu_id_portability_level` - u32.
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
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let mut session = Session::init().unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    /// let machine_mut = session.get_machine().unwrap();
    /// machine_mut.set_cpu_id_portability_level(3).unwrap();
    /// machine_mut.save_settings().unwrap();
    pub fn set_cpu_id_portability_level(
        &self,
        cpu_id_portability_level: u32,
    ) -> Result<(), VboxError> {
        get_function_result_unit!(
            self.object,
            SetCPUIDPortabilityLevel,
            cpu_id_portability_level
        )
    }

    /// System memory size in megabytes.
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
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let memory_size = machine.get_memory_size().unwrap();
    pub fn get_memory_size(&self) -> Result<u32, VboxError> {
        get_function_result_number!(self.object, GetMemorySize, u32)
    }

    ///System memory size in megabytes.
    ///
    /// # Arguments
    ///
    /// * `memory_size` - u32. System memory size in megabytes.
    ///
    /// # Returns
    ///
    /// Returns () success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    ///```no_run
    ///
    /// use virtualbox_rs::{Session, VirtualBox};
    /// use virtualbox_rs::enums::SessionType;
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let mut session = Session::init().unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    /// let machine_mut = session.get_machine().unwrap();
    /// machine_mut.set_memory_size(2048).unwrap();
    /// machine_mut.save_settings().unwrap();
    pub fn set_memory_size(&self, memory_size: u32) -> Result<(), VboxError> {
        get_function_result_unit!(self.object, SetMemorySize, memory_size)
    }

    /// Memory balloon size in megabytes.
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
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let memory_balloon_size = machine.get_memory_balloon_size().unwrap();
    pub fn get_memory_balloon_size(&self) -> Result<u32, VboxError> {
        get_function_result_number!(self.object, GetMemoryBalloonSize, u32)
    }

    /// Memory balloon size in megabytes.
    ///
    /// # Arguments
    ///
    /// * `memory_balloon_size` - u32. Memory balloon size in megabytes.
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
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let mut session = Session::init().unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    /// let machine_mut = session.get_machine().unwrap();
    /// machine_mut.set_memory_balloon_size(16).unwrap();
    /// machine_mut.save_settings().unwrap();
    pub fn set_memory_balloon_size(&self, memory_balloon_size: u32) -> Result<(), VboxError> {
        get_function_result_unit!(self.object, SetMemoryBalloonSize, memory_balloon_size)
    }

    /// This setting determines whether VirtualBox allows page fusion for this machine (64-bit hosts only).
    ///
    /// # Returns
    ///
    /// Returns bool success, or a [`VboxError`] on failure.
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
    /// let page_fusion_enabled = machine.get_page_fusion_enabled().unwrap();
    pub fn get_page_fusion_enabled(&self) -> Result<bool, VboxError> {
        get_function_result_bool!(self.object, GetPageFusionEnabled)
    }

    /// This setting determines whether VirtualBox allows page fusion for this machine (64-bit hosts only).
    ///
    /// # Arguments
    ///
    /// * `page_fusion_enabled` - bool.
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
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let mut session = Session::init().unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    /// let machine_mut = session.get_machine().unwrap();
    /// machine_mut.set_page_fusion_enabled(true).unwrap();
    /// machine_mut.save_settings().unwrap();
    pub fn set_page_fusion_enabled(&self, page_fusion_enabled: bool) -> Result<(), VboxError> {
        let page_fusion_enabled = if page_fusion_enabled { 1 } else { 0 };
        get_function_result_unit!(self.object, SetPageFusionEnabled, page_fusion_enabled)
    }

    /// Graphics adapter object.
    ///
    /// # Returns
    ///
    /// Returns [`GraphicsAdapter`] success, or a [`VboxError`] on failure.
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
    /// let graphics_adapter = machine.get_graphics_adapter().unwrap();

    pub fn get_graphics_adapter(&self) -> Result<GraphicsAdapter, VboxError> {
        let graphics_adapter =
            get_function_result_pointer!(self.object, GetGraphicsAdapter, *mut IGraphicsAdapter)?;
        Ok(GraphicsAdapter::new(graphics_adapter))
    }

    #[cfg(is_v_7_1)]
    /// Platform object for a virtual machine.
    ///
    /// # Returns
    ///
    /// Returns [`Platform`] success, or a [`VboxError`] on failure.
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
    /// let platform = machine.get_platform().unwrap();

    pub fn get_platform(&self) -> Result<Platform, VboxError> {
        let platform = get_function_result_pointer!(self.object, GetPlatform, *mut IPlatform)?;
        Ok(Platform::new(platform))
    }

    #[cfg(is_v_7_1)]
    /// Object containing all firmware settings.
    ///
    /// # Returns
    ///
    /// Returns [`FirmwareSettings`] success, or a [`VboxError`] on failure.
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
    /// let firmware_settings = machine.get_firmware_settings().unwrap();

    pub fn get_firmware_settings(&self) -> Result<FirmwareSettings, VboxError> {
        let firmware_settings =
            get_function_result_pointer!(self.object, GetFirmwareSettings, *mut IFirmwareSettings)?;
        Ok(FirmwareSettings::new(firmware_settings))
    }

    #[cfg(is_v_7_1)]
    /// Placeholder Method
    ///
    /// This method serves as a placeholder for versions of the API where the actual method is not available.
    /// Attempting to call this method will result in an `UnsupportedInCurrentApiVersion` error.
    /// This method is intended to ensure that the codebase can be compiled against multiple API versions
    /// without modification.
    ///
    /// Supported from API version: v7_0
    pub fn get_bios_settings(&self) -> Result<BIOSSettings, VboxError> {
        Err(VboxError::unsupported_in_current_api_version(
            "Machine::get_bios_settings",
            "v7_0",
        ))
    }

    /// Object containing all recording settings.
    ///
    /// # Returns
    ///
    /// Returns [`RecordingSettings`] success, or a [`VboxError`] on failure.
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
    /// let recording_settings = machine.get_recording_settings().unwrap();

    pub fn get_recording_settings(&self) -> Result<RecordingSettings, VboxError> {
        let recording_settings = get_function_result_pointer!(
            self.object,
            GetRecordingSettings,
            *mut IRecordingSettings
        )?;
        Ok(RecordingSettings::new(recording_settings))
    }
    #[cfg(is_v_7_1)]
    /// Type of firmware (such as legacy BIOS or EFI), used for initial bootstrap in this VM.
    ///
    /// # Returns
    ///
    /// Returns [`FirmwareType`] success, or a [`VboxError`] on failure.
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
    /// let firmware_type = machine.get_firmware_type().unwrap();

    pub fn get_firmware_type(&self) -> Result<FirmwareType, VboxError> {
        self.get_firmware_settings()?.get_firmware_type()
    }

    #[cfg(is_v_7_1)]
    /// Type of firmware (such as legacy BIOS or EFI), used for initial bootstrap in this VM.
    ///
    /// # Arguments
    ///
    /// * `firmware_type` - [`FirmwareType`].
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
    /// use virtualbox_rs::enums::{FirmwareType, SessionType};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let mut session = Session::init().unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    /// let machine_mut = session.get_machine().unwrap();
    /// machine_mut.set_firmware_type(FirmwareType::BIOS).unwrap();
    /// machine_mut.save_settings().unwrap();

    pub fn set_firmware_type(&self, firmware_type: FirmwareType) -> Result<(), VboxError> {
        self.get_firmware_settings()?
            .set_firmware_type(firmware_type)
    }

    /// Type of pointing HID (such as mouse or tablet) used in this VM.
    ///
    /// The default is typically "PS2Mouse" but can vary depending on the requirements of the guest operating system.
    ///
    /// # Returns
    ///
    /// Returns [`PointingHIDType`] success, or a [`VboxError`] on failure.
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
    /// let pointing_hid_type = machine.get_pointing_hid_type().unwrap();

    pub fn get_pointing_hid_type(&self) -> Result<PointingHIDType, VboxError> {
        let pointing_hid_type = get_function_result_number!(self.object, GetPointingHIDType, u32)?;
        Ok(PointingHIDType::from(pointing_hid_type))
    }

    /// Type of pointing HID (such as mouse or tablet) used in this VM.
    ///
    /// The default is typically "PS2Mouse" but can vary depending on the requirements of the guest operating system.
    ///
    /// # Arguments
    ///
    /// * `pointing_hid_type` - [`PointingHIDType`].
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
    /// use virtualbox_rs::enums::{PointingHIDType, SessionType};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let mut session = Session::init().unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    /// let machine_mut = session.get_machine().unwrap();
    /// machine_mut.set_pointing_hid_type(PointingHIDType::USBMouse).unwrap();
    /// machine_mut.save_settings().unwrap();

    pub fn set_pointing_hid_type(
        &self,
        pointing_hid_type: PointingHIDType,
    ) -> Result<(), VboxError> {
        let pointing_hid_type = pointing_hid_type.into();
        get_function_result_unit!(self.object, SetPointingHIDType, pointing_hid_type)
    }

    /// Type of keyboard HID used in this VM.
    ///
    /// The default is typically "PS2Keyboard" but can vary depending on the requirements of the guest operating system.
    ///
    /// # Returns
    ///
    /// Returns [`PointingHIDType`] success, or a [`VboxError`] on failure.
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
    /// let pointing_hid_type = machine.get_keyboard_hid_type().unwrap();

    pub fn get_keyboard_hid_type(&self) -> Result<KeyboardHIDType, VboxError> {
        let keyboard_hid_type = get_function_result_number!(self.object, GetKeyboardHIDType, u32)?;
        Ok(KeyboardHIDType::from(keyboard_hid_type))
    }

    /// Type of keyboard HID used in this VM.
    ///
    /// The default is typically "PS2Keyboard" but can vary depending on the requirements of the guest operating system.
    ///
    /// # Arguments
    ///
    /// * `keyboard_hid_type` - [`KeyboardHIDType`].
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
    /// use virtualbox_rs::enums::{KeyboardHIDType, SessionType};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let mut session = Session::init().unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    /// let machine_mut = session.get_machine().unwrap();
    /// machine_mut.set_keyboard_hid_type(KeyboardHIDType::USBKeyboard).unwrap();
    /// machine_mut.save_settings().unwrap();

    pub fn set_keyboard_hid_type(
        &self,
        keyboard_hid_type: KeyboardHIDType,
    ) -> Result<(), VboxError> {
        let keyboard_hid_type = keyboard_hid_type.into();
        get_function_result_unit!(self.object, SetKeyboardHIDType, keyboard_hid_type)
    }

    #[cfg(is_v_7_1)]
    /// This attribute controls if High Precision Event Timer (HPET) is enabled in this VM.
    ///
    /// Use this property if you want to provide guests with additional time source, or if guest requires HPET to function correctly. Default is false.
    ///
    /// # Returns
    ///
    /// Returns bool success, or a [`VboxError`] on failure.
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
    /// let enabled = machine.get_hpet_enabled().unwrap();
    pub fn get_hpet_enabled(&self) -> Result<bool, VboxError> {
        self.get_platform()?.get_x86()?.get_hpet_enabled()
    }

    #[cfg(is_v_7_1)]
    /// This attribute controls if High Precision Event Timer (HPET) is enabled in this VM.
    ///
    /// Use this property if you want to provide guests with additional time source, or if guest requires HPET to function correctly. Default is false.
    ///
    /// # Arguments
    ///
    /// * `enabled` - bool.
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
    /// use virtualbox_rs::enums::{SessionType};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let mut session = Session::init().unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    /// let machine_mut = session.get_machine().unwrap();
    /// machine_mut.set_hpet_enabled(true).unwrap();
    /// machine_mut.save_settings().unwrap();
    pub fn set_hpet_enabled(&self, enabled: bool) -> Result<(), VboxError> {
        self.get_platform()?.get_x86()?.set_hpet_enabled(enabled)
    }

    #[cfg(is_v_7_1)]
    /// Chipset type used in this VM.
    ///
    /// # Returns
    ///
    /// Returns [`ChipsetType`] success, or a [`VboxError`] on failure.
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
    /// let chipset_type = machine.get_chipset_type().unwrap();

    pub fn get_chipset_type(&self) -> Result<ChipsetType, VboxError> {
        self.get_platform()?.get_chipset_type()
    }

    #[cfg(is_v_7_1)]
    /// Chipset type used in this VM.
    ///
    /// # Arguments
    ///
    /// * `chipset_type` - [`ChipsetType`].
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
    /// use virtualbox_rs::enums::{ChipsetType, SessionType};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let mut session = Session::init().unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    /// let machine_mut = session.get_machine().unwrap();
    /// machine_mut.set_chipset_type(ChipsetType::PIIX3).unwrap();
    /// machine_mut.save_settings().unwrap();

    pub fn set_chipset_type(&self, chipset_type: ChipsetType) -> Result<(), VboxError> {
        let chipset_type = chipset_type.into();
        self.get_platform()?.set_chipset_type(chipset_type)
    }

    /// Full path to the directory used to store snapshot data (differencing media and saved state files) of this machine.
    ///
    /// # Returns
    ///
    /// Returns &str success, or a [`VboxError`] on failure.
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
    /// let snapshot_folder = machine.get_snapshot_folder().unwrap();

    pub fn get_snapshot_folder(&self) -> Result<&'static str, VboxError> {
        get_function_result_str!(self.object, GetSnapshotFolder)
    }

    /// Full path to the directory used to store snapshot data (differencing media and saved state files) of this machine.
    ///
    /// Currently, it is an error to try to change this property on a machine that has snapshots (because this would require to move possibly large files to a different location). A separate method will be available for this purpose later.
    ///
    /// # Arguments
    ///
    /// * `snapshot_folder` - &str.
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
    /// use virtualbox_rs::enums::{SessionType};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let mut session = Session::init().unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    /// let machine_mut = session.get_machine().unwrap();
    /// machine_mut.set_snapshot_folder("/home/host_user/VirtualBox VMs/Freebsd_14/Snapshots").unwrap();
    /// machine_mut.save_settings().unwrap();

    pub fn set_snapshot_folder(&self, snapshot_folder: &str) -> Result<(), VboxError> {
        let snapshot_folder = string_to_c_u64_str(snapshot_folder)?;
        get_function_result_unit!(self.object, SetSnapshotFolder, snapshot_folder)
    }

    /// VirtualBox Remote Desktop Extension (VRDE) server object.
    ///
    /// # Returns
    ///
    /// Returns [`VRDEServer`] success, or a [`VboxError`] on failure.
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
    /// let vrde_server = machine.get_vrde_server().unwrap();

    pub fn get_vrde_server(&self) -> Result<VRDEServer, VboxError> {
        let vrde_server =
            get_function_result_pointer!(self.object, GetVRDEServer, *mut IVRDEServer)?;
        Ok(VRDEServer::new(vrde_server))
    }

    /// GetEmulatedUSBCardReaderEnabled
    ///
    /// # Returns
    ///
    /// Returns bool success, or a [`VboxError`] on failure.
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
    /// let enabled = machine.get_emulated_usb_card_reader_enabled().unwrap();
    pub fn get_emulated_usb_card_reader_enabled(&self) -> Result<bool, VboxError> {
        get_function_result_bool!(self.object, GetEmulatedUSBCardReaderEnabled)
    }

    /// SetEmulatedUSBCardReaderEnabled
    ///
    /// # Arguments
    ///
    /// * `enabled` - bool.
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
    /// use virtualbox_rs::enums::{SessionType};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let mut session = Session::init().unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    /// let machine_mut = session.get_machine().unwrap();
    /// machine_mut.set_emulated_usb_card_reader_enabled(true).unwrap();
    /// machine_mut.save_settings().unwrap();
    pub fn set_emulated_usb_card_reader_enabled(&self, enabled: bool) -> Result<(), VboxError> {
        let enabled = if enabled { 1 } else { 0 };
        get_function_result_unit!(self.object, SetEmulatedUSBCardReaderEnabled, enabled)
    }

    /// Array of media attached to this machine.
    ///
    /// # Returns
    ///
    /// Returns [`Vec<MediumAttachment>`] success, or a [`VboxError`] on failure.
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
    /// let medium_attachments = machine.get_medium_attachments().unwrap();

    pub fn get_medium_attachments(&self) -> Result<Vec<MediumAttachment>, VboxError> {
        let medium_attachments = get_function_result_pointer_vec!(
            self.object,
            GetMediumAttachments,
            *mut IMediumAttachment
        )?;
        Ok(medium_attachments
            .iter()
            .map(|object| MediumAttachment::new(object.clone()))
            .collect())
    }

    /// Array of USB controllers attached to this machine.
    ///
    /// # Returns
    ///
    /// Returns [`Vec<USBController>`] success, or a [`VboxError`] on failure.
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
    /// let usb_controllers = machine.get_usb_controllers().unwrap();

    pub fn get_usb_controllers(&self) -> Result<Vec<USBController>, VboxError> {
        let usb_controllers =
            get_function_result_pointer_vec!(self.object, GetUSBControllers, *mut IUSBController)?;
        Ok(usb_controllers
            .iter()
            .map(|object| USBController::new(object.clone()))
            .collect())
    }

    /// Associated USB device filters object.
    ///
    /// # Returns
    ///
    /// Returns [`USBDeviceFilters`] success, or a [`VboxError`] on failure.
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
    /// let usb_device_filters = machine.get_usb_device_filters().unwrap();

    pub fn get_usb_device_filters(&self) -> Result<USBDeviceFilters, VboxError> {
        let usb_device_filters =
            get_function_result_pointer!(self.object, GetUSBDeviceFilters, *mut IUSBDeviceFilters)?;
        Ok(USBDeviceFilters::new(usb_device_filters))
    }

    /// Array of storage controllers attached to this machine.
    ///
    /// # Returns
    ///
    /// Returns [`Vec<StorageController>`] success, or a [`VboxError`] on failure.
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
    /// let storage_controllers = machine.get_storage_controllers().unwrap();

    pub fn get_storage_controllers(&self) -> Result<Vec<StorageController>, VboxError> {
        let storage_controllers = get_function_result_pointer_vec!(
            self.object,
            GetStorageControllers,
            *mut IStorageController
        )?;
        Ok(storage_controllers
            .iter()
            .map(|object| StorageController::new(object.clone()))
            .collect())
    }

    /// Full name of the file containing machine settings data.
    ///
    /// # Returns
    ///
    /// Returns &str success, or a [`VboxError`] on failure.
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
    /// let settings_file_path = machine.get_settings_file_path().unwrap();

    pub fn get_settings_file_path(&self) -> Result<&'static str, VboxError> {
        get_function_result_str!(self.object, GetSettingsFilePath)
    }

    /// Full name of the file containing auxiliary machine settings data.
    ///
    /// # Returns
    ///
    /// Returns &str success, or a [`VboxError`] on failure.
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
    /// let settings_aux_file_path = machine.get_settings_aux_file_path().unwrap();

    pub fn get_settings_aux_file_path(&self) -> Result<&'static str, VboxError> {
        get_function_result_str!(self.object, GetSettingsAuxFilePath)
    }

    /// Whether the settings of this machine have been modified (but neither yet saved nor discarded).
    ///
    /// Reading this property is only valid on instances returned by [`Session::get_machine`] and on new machines created by [`VirtualBox::create_machine`] or opened by [`VirtualBox::open_machine`] but not yet registered, or on unregistered machines after calling [`Machine::unregister`]. For all other cases, the settings can never be modified.
    ///
    /// For newly created unregistered machines, the value of this property is always true until [`Machine::save_settings`] is called (no matter if any machine settings have been changed after the creation or not). For opened machines the value is set to false (and then follows to normal rules).
    ///
    /// # Returns
    ///
    /// Returns bool success, or a [`VboxError`] on failure.
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
    /// let settings_modified = machine.get_settings_modified().unwrap();

    pub fn get_settings_modified(&self) -> Result<bool, VboxError> {
        get_function_result_bool!(self.object, GetSettingsModified)
    }

    /// Current session state for this machine.
    ///
    /// # Returns
    ///
    /// Returns a [`SessionState`] success, or a [`VboxError`] on failure.
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
    /// let state = machine.get_session_state().unwrap();
    ///
    /// println!("{:?}", state);
    ///
    pub fn get_session_state(&self) -> Result<SessionState, VboxError> {
        let state = get_function_result_number!(self.object, GetSessionState, u32)?;
        Ok(SessionState::from(state))
    }

    /// Name of the session.
    ///
    /// # Returns
    ///
    /// Returns &str success, or a [`VboxError`] on failure.
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
    /// let session_name = machine.get_session_name().unwrap();

    pub fn get_session_name(&self) -> Result<&'static str, VboxError> {
        get_function_result_str!(self.object, GetSessionName)
    }

    /// Identifier of the session process.
    ///
    /// This attribute contains the platform-dependent identifier of the process whose session was used with [`Machine::lock_machine`] call. The returned value is only valid if [`Machine::get_session_state`] is Locked or Unlocking by the time this property is read.
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
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let session_name = machine.get_session_name().unwrap();
    pub fn get_session_pid(&self) -> Result<u32, VboxError> {
        get_function_result_number!(self.object, GetSessionPID, u32)
    }

    /// Current execution state of this machine.
    ///
    /// # Returns
    ///
    /// Returns a [`MachineState`] success, or a [`VboxError`] on failure.
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
    /// let state = machine.get_state().unwrap();
    pub fn get_state(&self) -> Result<MachineState, VboxError> {
        let state = get_function_result_number!(self.object, GetState, u32)?;
        Ok(MachineState::from(state))
    }

    /// Timestamp of the last execution state change, in milliseconds since 1970-01-01 UTC.
    ///
    /// # Returns
    ///
    /// Returns a i64 success, or a [`VboxError`] on failure.
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
    /// let last_state_change = machine.get_last_state_change().unwrap();
    pub fn get_last_state_change(&self) -> Result<i64, VboxError> {
        get_function_result_number!(self.object, GetLastStateChange, i64)
    }

    /// Full path to the file that stores the execution state of the machine when it is in either the [`MachineState::Saved`] or [`MachineState::AbortedSaved`] state.
    ///
    /// # Returns
    ///
    /// Returns &str success, or a [`VboxError`] on failure.
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
    /// let state_file_path = machine.get_state_file_path().unwrap();
    pub fn get_state_file_path(&self) -> Result<&'static str, VboxError> {
        get_function_result_str!(self.object, GetStateFilePath)
    }

    /// Full path to the folder that stores a set of rotated log files recorded during machine execution.
    ///
    /// The most recent log file is named VBox.log, the previous log file is named VBox.log.1 and so on (up to VBox.log.3 in the current version).
    ///
    /// # Returns
    ///
    /// Returns &str success, or a [`VboxError`] on failure.
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
    /// let log_folder = machine.get_log_folder().unwrap();
    pub fn get_log_folder(&self) -> Result<&'static str, VboxError> {
        get_function_result_str!(self.object, GetLogFolder)
    }

    /// Current snapshot of this machine.
    ///
    /// # Returns
    ///
    /// Returns a new instance of the [`Option<Snapshot>`] on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// let vbox = virtualbox_rs::VirtualBox::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let snapshot = machine.get_current_snapshot().unwrap();
    ///
    pub fn get_current_snapshot(&self) -> Result<Option<Snapshot>, VboxError> {
        let snapshot: Result<*mut ISnapshot, VboxError> =
            get_function_result_pointer!(self.object, GetCurrentSnapshot, *mut ISnapshot);
        match snapshot {
            Ok(snapshot) => Ok(Some(Snapshot::new(snapshot))),
            Err(err) => match err.error_type {
                VboxErrorType::NullPointerError => Ok(None),
                _ => Err(err),
            },
        }
    }

    /// Number of snapshots taken on this machine.
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
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();

    pub fn get_snapshot_count(&self) -> Result<u32, VboxError> {
        get_function_result_number!(self.object, GetSnapshotCount, u32)
    }
    /// Number of snapshots taken on this machine.
    ///
    /// # Returns
    ///
    /// Returns bool success, or a [`VboxError`] on failure.
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
    /// let current_state_modified = machine.get_current_state_modified().unwrap();
    pub fn get_current_state_modified(&self) -> Result<bool, VboxError> {
        get_function_result_bool!(self.object, GetCurrentStateModified)
    }

    /// Collection of shared folders for this machine (permanent shared folders).
    ///
    /// These folders are shared automatically at machine startup and available only to the guest OS installed within this machine.
    ///
    /// New shared folders are added to the collection using [`Machine::create_shared_folder`]. Existing shared folders can be removed using [`Machine::remove_shared_folder`].
    ///
    /// # Returns
    ///
    /// Returns a [`Vec<SharedFolder>`] success, or a [`VboxError`] on failure.
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
    /// let shared_folders = machine.get_shared_folders().unwrap();
    pub fn get_shared_folders(&self) -> Result<Vec<SharedFolder>, VboxError> {
        let shared_folders =
            get_function_result_pointer_vec!(self.object, GetSharedFolders, *mut ISharedFolder)?;
        Ok(shared_folders
            .iter()
            .map(|disk| SharedFolder::new(disk.clone()))
            .collect())
    }

    /// Synchronization mode between the host OS clipboard and the guest OS clipboard.
    ///
    /// # Returns
    ///
    /// Returns [`ClipboardMode`] success, or a [`VboxError`] on failure.
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
    /// let clipboard_mode = machine.get_clipboard_mode().unwrap();
    pub fn get_clipboard_mode(&self) -> Result<ClipboardMode, VboxError> {
        let clipboard_mode = get_function_result_number!(self.object, GetClipboardMode, u32)?;
        Ok(ClipboardMode::from(clipboard_mode))
    }

    /// Synchronization mode between the host OS clipboard and the guest OS clipboard.
    ///
    /// # Arguments
    ///
    /// * `clipboard_mode` - [`ClipboardMode`]. The guest monitor to take screenshot from.
    ///
    /// # Returns
    ///
    /// Returns () success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::enums::{ClipboardMode, SessionType};
    /// use virtualbox_rs::{Session, VirtualBox};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let mut session = Session::init().unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    /// let machine_mut = session.get_machine().unwrap();
    /// machine_mut.set_clipboard_mode(ClipboardMode::GuestToHost).unwrap();
    pub fn set_clipboard_mode(&self, clipboard_mode: ClipboardMode) -> Result<(), VboxError> {
        let clipboard_mode = clipboard_mode.into();
        get_function_result_unit!(self.object, SetClipboardMode, clipboard_mode)
    }

    ///Rretrieves whether clipboard file transfers are allowed or not.
    ///
    /// When set to true, clipboard file transfers between supported host and guest OSes are allowed.
    ///
    /// # Returns
    ///
    /// Returns bool success, or a [`VboxError`] on failure.
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
    /// let enabled = machine.get_clipboard_file_transfers_enabled().unwrap();
    pub fn get_clipboard_file_transfers_enabled(&self) -> Result<bool, VboxError> {
        get_function_result_bool!(self.object, GetClipboardFileTransfersEnabled)
    }

    /// Sets whether clipboard file transfers are allowed or not.
    ///
    /// When set to true, clipboard file transfers between supported host and guest OSes are allowed.
    ///
    /// # Arguments
    ///
    /// * `enabled` - bool. When set to true, clipboard file transfers between supported host and guest OSes are allowed.
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
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let mut session = Session::init().unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    /// let machine_mut = session.get_machine().unwrap();
    /// machine_mut.set_clipboard_file_transfers_enabled(true).unwrap();
    pub fn set_clipboard_file_transfers_enabled(&self, enabled: bool) -> Result<(), VboxError> {
        let enabled = if enabled { 1 } else { 0 };
        get_function_result_unit!(self.object, SetClipboardFileTransfersEnabled, enabled)
    }

    /// Retrieves the current drag'n drop mode.
    ///
    /// # Returns
    ///
    /// Returns [`DnDMode`] success, or a [`VboxError`] on failure.
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
    /// let dnd_mode = machine.get_dnd_mode().unwrap();
    pub fn get_dnd_mode(&self) -> Result<DnDMode, VboxError> {
        let dnd_mode = get_function_result_number!(self.object, GetDnDMode, u32)?;
        Ok(DnDMode::from(dnd_mode))
    }

    /// Sets the current drag'n drop mode.
    ///
    /// # Arguments
    ///
    /// * `dnd_mode` - [`DnDMode`]. Drag and drop interchange mode.
    ///
    /// # Returns
    ///
    /// Returns () success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::enums::{DnDMode, SessionType};
    /// use virtualbox_rs::{Session, VirtualBox};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let mut session = Session::init().unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    /// let machine_mut = session.get_machine().unwrap();
    /// machine_mut.set_dnd_mode(DnDMode::GuestToHost).unwrap();
    pub fn set_dnd_mode(&self, clipboard_mode: DnDMode) -> Result<(), VboxError> {
        let dnd_mode = clipboard_mode.into();
        get_function_result_unit!(self.object, SetDnDMode, dnd_mode)
    }

    /// When set to true, the virtual machine becomes a target teleporter the next time it is powered on.
    ///
    /// This can only set to true when the VM is in the PoweredOff or Aborted state.
    ///
    /// # Returns
    ///
    /// Returns bool success, or a [`VboxError`] on failure.
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
    /// let enabled = machine.get_teleporter_enabled().unwrap();
    pub fn get_teleporter_enabled(&self) -> Result<bool, VboxError> {
        get_function_result_bool!(self.object, GetTeleporterEnabled)
    }

    /// When set to true, the virtual machine becomes a target teleporter the next time it is powered on.
    ///
    /// This can only set to true when the VM is in the PoweredOff or Aborted state.
    ///
    /// # Arguments
    ///
    /// * `enabled` - bool. When set to true, the virtual machine becomes a target teleporter the next time it is powered on.
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
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let mut session = Session::init().unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    /// let machine_mut = session.get_machine().unwrap();
    /// machine_mut.set_teleporter_enabled(true).unwrap();
    pub fn set_teleporter_enabled(&self, enabled: bool) -> Result<(), VboxError> {
        let enabled = if enabled { 1 } else { 0 };
        get_function_result_unit!(self.object, SetTeleporterEnabled, enabled)
    }

    /// The TCP port the target teleporter will listen for incoming teleportations on.
    ///
    /// 0 means the port is automatically selected upon power on. The actual value can be read from this property wh
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
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let teleporter_port = machine.get_teleporter_port().unwrap();
    pub fn get_teleporter_port(&self) -> Result<u32, VboxError> {
        get_function_result_number!(self.object, GetTeleporterPort, u32)
    }

    /// The TCP port the target teleporter will listen for incoming teleportations on.
    ///
    /// 0 means the port is automatically selected upon power on. The actual value can be read from this property wh
    ///
    /// # Arguments
    ///
    /// * `teleporter_port` - u32. The TCP port the target teleporter will listen for incoming teleportations on.
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
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let mut session = Session::init().unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    /// let machine_mut = session.get_machine().unwrap();
    /// machine_mut.set_teleporter_port(10001).unwrap();
    /// machine_mut.save_settings().unwrap();
    pub fn set_teleporter_port(&self, teleporter_port: u32) -> Result<(), VboxError> {
        get_function_result_unit!(self.object, SetTeleporterPort, teleporter_port)
    }

    /// The address the target teleporter will listen on.
    ///
    /// If set to an empty string, it will listen on all addresses.
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
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let teleporter_address = machine.get_teleporter_address().unwrap();

    pub fn get_teleporter_address(&self) -> Result<&'static str, VboxError> {
        get_function_result_str!(self.object, GetTeleporterAddress)
    }

    /// The address the target teleporter will listen on.
    ///
    /// If set to an empty string, it will listen on all addresses.
    ///
    /// # Arguments
    ///
    /// * `teleporter_address` - The address the target teleporter will listen on.
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
    /// let machine = vbox.
    ///     find_machines("Freebsd_14").unwrap();
    /// let mut session = Session::init().unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    /// let machine_mut = session.get_machine().unwrap();
    /// machine_mut.set_teleporter_address("192.168.1.42").unwrap();
    /// machine_mut.save_settings().unwrap();
    pub fn set_teleporter_address(&self, description: &str) -> Result<(), VboxError> {
        let teleporter_address = string_to_c_u64_str(description)?;
        get_function_result_unit!(self.object, SetTeleporterAddress, teleporter_address)
    }

    /// The password to check for on the target teleporter.
    ///
    /// This is just a very basic measure to prevent simple hacks and operators accidentally beaming a virtual machine to the wrong place.
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
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let teleporter_password = machine.get_teleporter_password().unwrap();

    pub fn get_teleporter_password(&self) -> Result<&'static str, VboxError> {
        get_function_result_str!(self.object, GetTeleporterPassword)
    }

    /// The password to check for on the target teleporter.
    ///
    /// This is just a very basic measure to prevent simple hacks and operators accidentally beaming a virtual machine to the wrong place.
    ///
    /// Note that you SET a plain text password while reading back a HASHED password. Setting a hashed password is currently not supported.
    ///
    /// # Arguments
    ///
    /// * `teleporter_password` - The password to check for on the target teleporter.
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
    /// let machine = vbox.
    ///     find_machines("Freebsd_14").unwrap();
    /// let mut session = Session::init().unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    /// let machine_mut = session.get_machine().unwrap();
    /// machine_mut.set_teleporter_password("pass").unwrap();
    /// machine_mut.save_settings().unwrap();
    pub fn set_teleporter_password(&self, description: &str) -> Result<(), VboxError> {
        let teleporter_password = string_to_c_u64_str(description)?;
        get_function_result_unit!(self.object, SetTeleporterPassword, teleporter_password)
    }

    /// The paravirtualized guest interface provider.
    ///
    /// # Returns
    ///
    /// Returns [`ParavirtProvider`] success, or a [`VboxError`] on failure.
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
    /// let paravirt_provider = machine.get_paravirt_provider().unwrap();
    pub fn get_paravirt_provider(&self) -> Result<ParavirtProvider, VboxError> {
        let paravirt_provider = get_function_result_number!(self.object, GetParavirtProvider, u32)?;
        Ok(ParavirtProvider::from(paravirt_provider))
    }

    /// The paravirtualized guest interface provider.
    ///
    /// # Arguments
    ///
    /// * `paravirt_provider` - [`ParavirtProvider`]. The paravirtualized guest interface provider.
    ///
    /// # Returns
    ///
    /// Returns () success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::enums::{ParavirtProvider, SessionType};
    /// use virtualbox_rs::{Session, VirtualBox};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let mut session = Session::init().unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    /// let machine_mut = session.get_machine().unwrap();
    /// machine_mut.set_paravirt_provider(ParavirtProvider::Legacy).unwrap();
    pub fn set_paravirt_provider(&self, clipboard_mode: ParavirtProvider) -> Result<(), VboxError> {
        let paravirt_provider = clipboard_mode.into();
        get_function_result_unit!(self.object, SetParavirtProvider, paravirt_provider)
    }

    #[cfg(is_v_7_1)]
    /// When set to true, the RTC device of the virtual machine will run in UTC time, otherwise in local time.
    ///
    /// Especially Unix guests prefer the time in UTC.
    ///
    /// # Returns
    ///
    /// Returns bool success, or a [`VboxError`] on failure.
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
    /// let rtc_use_utc = machine.get_rtc_use_utc().unwrap();
    pub fn get_rtc_use_utc(&self) -> Result<bool, VboxError> {
        self.get_platform()?.get_rtc_use_utc()
    }

    #[cfg(is_v_7_1)]
    /// When set to true, the RTC device of the virtual machine will run in UTC time, otherwise in local time.
    ///
    /// Especially Unix guests prefer the time in UTC.
    ///
    /// # Arguments
    ///
    /// * `rtc_use_utc` - bool. When set to true, the RTC device of the virtual machine will run in UTC time, otherwise in local time.
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
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let mut session = Session::init().unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    /// let machine_mut = session.get_machine().unwrap();
    /// machine_mut.set_rtc_use_utc(true).unwrap();
    pub fn set_rtc_use_utc(&self, rtc_use_utc: bool) -> Result<(), VboxError> {
        self.get_platform()?.set_rtc_use_utc(rtc_use_utc)
    }

    /// When set to true, the builtin I/O cache of the virtual machine will be enabled.
    ///
    /// # Returns
    ///
    /// Returns bool success, or a [`VboxError`] on failure.
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
    /// let enabled = machine.get_io_cache_enabled().unwrap();
    pub fn get_io_cache_enabled(&self) -> Result<bool, VboxError> {
        get_function_result_bool!(self.object, GetIOCacheEnabled)
    }

    /// When set to true, the builtin I/O cache of the virtual machine will be enabled.
    ///
    /// # Arguments
    ///
    /// * `enabled` - bool. When set to true, the builtin I/O cache of the virtual machine will be enabled.
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
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let mut session = Session::init().unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    /// let machine_mut = session.get_machine().unwrap();
    /// machine_mut.set_io_cache_enabled(true).unwrap();
    pub fn set_io_cache_enabled(&self, enabled: bool) -> Result<(), VboxError> {
        let enabled = if enabled { 1 } else { 0 };
        get_function_result_unit!(self.object, SetIOCacheEnabled, enabled)
    }

    /// Maximum size of the I/O cache in MB.
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
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let io_cache_size = machine.get_io_cache_size().unwrap();
    pub fn get_io_cache_size(&self) -> Result<u32, VboxError> {
        get_function_result_number!(self.object, GetIOCacheSize, u32)
    }

    /// Maximum size of the I/O cache in MB.
    ///
    /// # Arguments
    ///
    /// * `io_cache_size` - u32. Maximum size of the I/O cache in MB.
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
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let mut session = Session::init().unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    /// let machine_mut = session.get_machine().unwrap();
    /// machine_mut.set_io_cache_size(16).unwrap();
    /// machine_mut.save_settings().unwrap();
    pub fn set_io_cache_size(&self, io_cache_size: u32) -> Result<(), VboxError> {
        get_function_result_unit!(self.object, SetIOCacheSize, io_cache_size)
    }

    /// Array of PCI devices assigned to this machine, to get list of all PCI devices attached to the machine use [`Console::get_attached_pci_devices`] attribute, as this attribute is intended to list only devices additional to what described in virtual hardware config.
    ///
    /// Usually, this list keeps host's physical devices assigned to the particular machine.
    ///
    /// # Returns
    ///
    /// Returns [`Vec<PCIDeviceAttachment>`] success, or a [`VboxError`] on failure.
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
    /// let pci_device_assignments = machine.get_pci_device_assignments().unwrap();
    pub fn get_pci_device_assignments(&self) -> Result<Vec<PCIDeviceAttachment>, VboxError> {
        let pci_device_assignments = get_function_result_pointer_vec!(
            self.object,
            GetPCIDeviceAssignments,
            *mut IPCIDeviceAttachment
        )?;
        Ok(pci_device_assignments
            .iter()
            .map(|disk| PCIDeviceAttachment::new(disk.clone()))
            .collect())
    }

    /// Bandwidth control manager.
    ///
    /// # Returns
    ///
    /// Returns [`BandwidthControl`] on success, or a [`VboxError`] on failure.
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
    /// let bandwidth_control = machine.get_bandwidth_control().unwrap();
    ///
    pub fn get_bandwidth_control(&self) -> Result<BandwidthControl, VboxError> {
        let bandwidth_control_ptr =
            get_function_result_pointer!(self.object, GetBandwidthControl, *mut IBandwidthControl)?;
        Ok(BandwidthControl::new(bandwidth_control_ptr))
    }

    /// Enables the tracing facility in the VMM (including PDM devices + drivers).
    ///
    /// # Returns
    ///
    /// Returns bool success, or a [`VboxError`] on failure.
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
    /// let tracing_enabled = machine.get_tracing_enabled().unwrap();
    pub fn get_tracing_enabled(&self) -> Result<bool, VboxError> {
        get_function_result_bool!(self.object, GetTracingEnabled)
    }

    /// Enables the tracing facility in the VMM (including PDM devices + drivers).
    ///
    /// The VMM will consume about 0.5MB of more memory when enabled and there may be some extra overhead from tracepoints that are always enabled.
    ///
    /// # Arguments
    ///
    /// * `tracing_enabled` - bool.
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
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let mut session = Session::init().unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    /// let machine_mut = session.get_machine().unwrap();
    /// machine_mut.set_tracing_enabled(true).unwrap();
    /// machine_mut.save_settings().unwrap();
    pub fn set_tracing_enabled(&self, tracing_enabled: bool) -> Result<(), VboxError> {
        let tracing_enabled = if tracing_enabled { 1 } else { 0 };
        get_function_result_unit!(self.object, SetTracingEnabled, tracing_enabled)
    }

    /// Tracepoint configuration to apply at startup when [`Machine::get_tracing_enabled`] is true.
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
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let tracing_config = machine.get_tracing_config().unwrap();

    pub fn get_tracing_config(&self) -> Result<&'static str, VboxError> {
        get_function_result_str!(self.object, GetTracingConfig)
    }

    /// Tracepoint configuration to apply at startup when [`Machine::get_tracing_enabled`] is true.
    ///
    /// The string specifies a space separated of tracepoint group names to enable. The special group 'all' enables all tracepoints. Check DBGFR3TracingConfig for more details on available tracepoint groups and such.
    ///
    /// Note that on hosts supporting DTrace (or similar), a lot of the tracepoints may be implemented exclusively as DTrace probes. So, the effect of the same config may differ between Solaris and Windows for example.
    ///
    /// # Arguments
    ///
    /// * `set_tracing_config` - Tracepoint configuration to apply at startup
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
    /// let machine = vbox.
    ///     find_machines("Freebsd_14").unwrap();
    /// let mut session = Session::init().unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    /// let machine_mut = session.get_machine().unwrap();
    /// machine_mut.set_tracing_config("all").unwrap();
    /// machine_mut.save_settings().unwrap();
    pub fn set_tracing_config(&self, tracing_config: &str) -> Result<(), VboxError> {
        let tracing_config = string_to_c_u64_str(tracing_config)?;
        get_function_result_unit!(self.object, SetTracingConfig, tracing_config)
    }

    /// Enables tracepoints in PDM devices and drivers to use the VMCPU or VM structures when firing off trace points.
    ///
    /// # Returns
    ///
    /// Returns bool success, or a [`VboxError`] on failure.
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
    /// let allow_tracing_to_access_vm = machine.get_allow_tracing_to_access_vm().unwrap();
    pub fn get_allow_tracing_to_access_vm(&self) -> Result<bool, VboxError> {
        get_function_result_bool!(self.object, GetAllowTracingToAccessVM)
    }

    /// Enables tracepoints in PDM devices and drivers to use the VMCPU or VM structures when firing off trace points.
    ///
    /// This is especially useful with DTrace tracepoints, as it allows you to use the VMCPU or VM pointer to obtain useful information such as guest register state.
    ///
    /// This is disabled by default because devices and drivers normally has no business accessing the VMCPU or VM structures, and are therefore unable to get any pointers to these.
    ///
    /// # Arguments
    ///
    /// * `allow_tracing_to_access_vm` - bool.
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
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let mut session = Session::init().unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    /// let machine_mut = session.get_machine().unwrap();
    /// machine_mut.set_allow_tracing_to_access_vm(true).unwrap();
    /// machine_mut.save_settings().unwrap();
    pub fn set_allow_tracing_to_access_vm(
        &self,
        allow_tracing_to_access_vm: bool,
    ) -> Result<(), VboxError> {
        let allow_tracing_to_access_vm = if allow_tracing_to_access_vm { 1 } else { 0 };
        get_function_result_unit!(
            self.object,
            SetAllowTracingToAccessVM,
            allow_tracing_to_access_vm
        )
    }

    /// Enables autostart of the VM during system boot.
    ///
    /// # Returns
    ///
    /// Returns bool success, or a [`VboxError`] on failure.
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
    /// let autostart_enabled = machine.get_autostart_enabled().unwrap();
    pub fn get_autostart_enabled(&self) -> Result<bool, VboxError> {
        get_function_result_bool!(self.object, GetAutostartEnabled)
    }

    /// Enables autostart of the VM during system boot.
    ///
    /// # Arguments
    ///
    /// * `autostart_enabled` - bool.
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
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let mut session = Session::init().unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    /// let machine_mut = session.get_machine().unwrap();
    /// machine_mut.set_autostart_enabled(true).unwrap();
    /// machine_mut.save_settings().unwrap();
    pub fn set_autostart_enabled(&self, autostart_enabled: bool) -> Result<(), VboxError> {
        let autostart_enabled = if autostart_enabled { 1 } else { 0 };
        get_function_result_unit!(self.object, SetAutostartEnabled, autostart_enabled)
    }

    /// Number of seconds to wait until the VM should be started during system boot.
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
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let autostart_delay = machine.get_autostart_delay().unwrap();
    pub fn get_autostart_delay(&self) -> Result<u32, VboxError> {
        get_function_result_number!(self.object, GetAutostartDelay, u32)
    }

    /// Number of seconds to wait until the VM should be started during system boot.
    ///
    /// # Arguments
    ///
    /// * `autostart_delay` - u32. Number of seconds to wait until the VM should be started during system boot.
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
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let mut session = Session::init().unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    /// let machine_mut = session.get_machine().unwrap();
    /// machine_mut.set_autostart_delay(16).unwrap();
    /// machine_mut.save_settings().unwrap();
    pub fn set_autostart_delay(&self, autostart_delay: u32) -> Result<(), VboxError> {
        get_function_result_unit!(self.object, SetAutostartDelay, autostart_delay)
    }

    /// Action type to do when the system is shutting down.
    ///
    /// # Returns
    ///
    /// Returns [`AutostopType`] success, or a [`VboxError`] on failure.
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
    /// let autostop_type = machine.get_autostop_type().unwrap();
    pub fn get_autostop_type(&self) -> Result<AutostopType, VboxError> {
        let autostop_type = get_function_result_number!(self.object, GetAutostopType, u32)?;
        Ok(AutostopType::from(autostop_type))
    }

    /// Action type to do when the system is shutting down.
    ///
    /// # Arguments
    ///
    /// * `autostop_type` - [`AutostopType`]. Action type to do when the system is shutting down.
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
    /// use virtualbox_rs::enums::{AutostopType, SessionType};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let mut session = Session::init().unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    /// let machine_mut = session.get_machine().unwrap();
    /// machine_mut.set_autostop_type(AutostopType::PowerOff).unwrap();
    /// machine_mut.save_settings().unwrap();
    pub fn set_autostop_type(&self, autostop_type: AutostopType) -> Result<(), VboxError> {
        let autostop_type = autostop_type.into();
        get_function_result_unit!(self.object, SetAutostopType, autostop_type)
    }

    /// Selects which VM frontend should be used by default when launching this VM through the [`Machine::launch_vm_process`] method.
    ///
    /// # Returns
    ///
    /// Returns [`FrontEndName`] success, or a [`VboxError`] on failure.
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
    /// let default_frontend = machine.get_default_frontend().unwrap();
    pub fn get_default_frontend(&self) -> Result<FrontEndName, VboxError> {
        let default_frontend = get_function_result_str!(self.object, GetDefaultFrontend)?;
        Ok(FrontEndName::from(default_frontend))
    }

    /// Selects which VM frontend should be used by default when launching this VM through the [`Machine::launch_vm_process`] method.
    ///
    /// Empty or null strings do not define a particular default, it is up to [`Machine::launch_vm_process`] to select one. See the description of [`Machine::launch_vm_process`] for the valid frontend types.
    ///
    /// This per-VM setting overrides the default defined by [`SystemProperties::get_default_frontend`] attribute, and is overridden by a frontend type passed to [`Machine::launch_vm_process`].
    ///
    /// # Arguments
    ///
    /// * `default_frontend` - [`FrontEndName`].
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
    /// use virtualbox_rs::enums::{FrontEndName, SessionType};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let mut session = Session::init().unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    /// let machine_mut = session.get_machine().unwrap();
    /// machine_mut.set_default_frontend(FrontEndName::Headless).unwrap();
    /// machine_mut.save_settings().unwrap();
    pub fn set_default_frontend(&self, default_frontend: FrontEndName) -> Result<(), VboxError> {
        let default_frontend = string_to_c_u64_str(default_frontend.into())?;
        get_function_result_unit!(self.object, SetDefaultFrontend, default_frontend)
    }

    /// Returns whether there is a USB proxy available.
    ///
    /// # Returns
    ///
    /// Returns bool success, or a [`VboxError`] on failure.
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
    /// let usb_proxy_available = machine.get_usb_proxy_available().unwrap();
    pub fn get_usb_proxy_available(&self) -> Result<bool, VboxError> {
        get_function_result_bool!(self.object, GetUSBProxyAvailable)
    }

    /// Priority of the VM process.
    ///
    /// # Returns
    ///
    /// Returns [`VMProcPriority`] success, or a [`VboxError`] on failure.
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
    /// let vm_process_priority = machine.get_vm_process_priority().unwrap();
    pub fn get_vm_process_priority(&self) -> Result<VMProcPriority, VboxError> {
        let vm_process_priority =
            get_function_result_number!(self.object, GetVMProcessPriority, u32)?;
        Ok(VMProcPriority::from(vm_process_priority))
    }

    /// Sets the priority of the VM process.
    ///
    /// It is a VM setting which can be changed both before starting the VM and at runtime.
    ///
    /// The default value is 'Default', which selects the default process priority.
    ///
    /// # Arguments
    ///
    /// * `vm_process_priority` - [`VMProcPriority`]. Priority of the VM process.
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
    /// use virtualbox_rs::enums::{SessionType, VMProcPriority};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let mut session = Session::init().unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    /// let machine_mut = session.get_machine().unwrap();
    /// machine_mut.set_vm_process_priority(VMProcPriority::Normal).unwrap();
    /// machine_mut.save_settings().unwrap();
    pub fn set_vm_process_priority(
        &self,
        vm_process_priority: VMProcPriority,
    ) -> Result<(), VboxError> {
        let vm_process_priority = vm_process_priority.into();
        get_function_result_unit!(self.object, SetVMProcessPriority, vm_process_priority)
    }

    /// Experimental feature to select the guest CPU profile.
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
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let cpu_profile = machine.get_cpu_profile().unwrap();

    pub fn get_cpu_profile(&self) -> Result<&'static str, VboxError> {
        get_function_result_str!(self.object, GetCPUProfile)
    }

    /// Experimental feature to select the guest CPU profile.
    ///
    /// The default is "host", which indicates the host CPU. All other names are subject to change.
    ///
    /// Use the [`SystemProperties::get_cpu_profiles`] method to get currently available CPU profiles.
    ///
    /// # Arguments
    ///
    /// * `cpu_profile` - Guest CPU profile.
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
    /// let machine = vbox.
    ///     find_machines("Freebsd_14").unwrap();
    /// let mut session = Session::init().unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    /// let machine_mut = session.get_machine().unwrap();
    /// machine_mut.set_cpu_profile("Intel Core i7-6700K").unwrap();
    /// machine_mut.save_settings().unwrap();
    pub fn set_cpu_profile(&self, cpu_profile: &str) -> Result<(), VboxError> {
        let cpu_profile = string_to_c_u64_str(cpu_profile)?;
        get_function_result_unit!(self.object, SetCPUProfile, cpu_profile)
    }

    /// Locks the machine for the given session to enable
    /// the caller to make changes to the machine
    /// or start the VM or control VM execution.
    ///
    /// **Reference to the official documentation:**
    ///
    /// [https://www.virtualbox.org/sdkref/interface_i_machine.html](https://www.virtualbox.org/sdkref/interface_i_machine.html#af28da645b00a821547d9cb8e92f8b7b0)
    ///
    /// # Arguments
    ///
    /// * `session` - Session object for which the machine will be locked.
    ///
    /// * `lockType` - If set to Write, then attempt to acquire an exclusive write lock or fail.
    /// If set to Shared, then either acquire an exclusive write lock or establish a link to an existing session.
    ///
    /// # Returns
    ///
    /// Returns Ok(()) success, or a [`VboxError`] on failure.
    ///
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    ///
    /// use virtualbox_rs::{Session, VirtualBox};
    /// use virtualbox_rs::enums::SessionType;
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let mut session = Session::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    ///
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    pub fn lock_machine(
        &self,
        session: &mut Session,
        session_type: SessionType,
    ) -> Result<(), VboxError> {
        let session_type: u32 = session_type.into();
        get_function_result_unit!(self.object, LockMachine, session.object, session_type)
    }

    /// Spawns a new process that will execute the virtual machine
    /// and obtains a shared lock on the machine for the calling session.
    ///
    /// **Reference to the official documentation:**
    ///
    /// [https://www.virtualbox.org/sdkref/interface_i_machine.html](https://www.virtualbox.org/sdkref/interface_i_machine.html#aa75e76da42ef908795d235d995262c6f)
    ///
    ///
    /// # Arguments
    ///
    /// * `session` - Client session object to which the VM process will be connected (this must be in "Unlocked" state).
    /// * `front_end_name` - [`FrontEndName`] to use for the new VM processIf set to Shared, then either acquire an exclusive write lock or establish a link to an existing session.
    ///
    /// # Returns
    ///
    /// Returns [`Progress`] success, or a [`VboxError`] on failure.
    ///
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    ///
    /// use virtualbox_rs::{Session, VirtualBox};
    /// use virtualbox_rs::enums::FrontEndName;
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let mut session = Session::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    ///
    /// let progress = machine
    ///     .launch_vm_process(&mut session, FrontEndName::Headless)
    ///     .unwrap();
    pub fn launch_vm_process(
        &self,
        session: &mut Session,
        front_end_name: FrontEndName,
    ) -> Result<Progress, VboxError> {
        let front_end_name: &str = front_end_name.into();
        let name = string_to_c_u64_str(front_end_name)?;
        let mut env: *mut PRUnichar = std::ptr::null_mut();
        let count = 0;
        debug!("front_end_name: {front_end_name}");
        let progress = get_function_result_pointer!(
            self.object,
            LaunchVMProcess,
            *mut IProgress,
            session.object,
            name,
            count,
            &mut env
        )?;
        Ok(Progress::new(progress))
    }

    /// Puts the given device to the specified position in the boot order.
    ///
    /// # Arguments
    ///
    /// * `position` - Position in the boot order
    /// (1 to the total number of devices the machine can boot from,
    /// as returned by SystemProperties::get_max_boot_position).
    ///
    /// * `boot_device_type` - The [`DeviceType`] of the device used to boot at the given position.
    ///
    /// # Returns
    ///
    /// Returns Ok(()) success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{VirtualBox};
    /// use virtualbox_rs::enums::DeviceType;
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// machine.set_boot_order(1, DeviceType::DVD).unwrap();
    pub fn set_boot_order(
        &self,
        position: u32,
        boot_device_type: DeviceType,
    ) -> Result<(), VboxError> {
        let boot_device_type: u32 = boot_device_type.into();
        get_function_result_unit!(self.object, SetBootOrder, position, boot_device_type)
    }

    /// Returns the device type that occupies the specified position in the boot order.
    ///
    /// # Arguments
    ///
    /// * `position` - Position in the boot order
    /// (1 to the total number of devices the machine can boot from,
    /// as returned by SystemProperties::get_max_boot_position).
    ///
    /// # Returns
    ///
    /// Returns [`DeviceType`] success, or a [`VboxError`] on failure.
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
    /// let boot_type = machine.get_boot_order(1).unwrap();
    ///
    /// println!("{:?}", boot_type);
    ///
    pub fn get_boot_order(&self, position: u32) -> Result<DeviceType, VboxError> {
        let count = get_function_result_number!(self.object, GetBootOrder, u32, position)?;
        Ok(DeviceType::from(count))
    }

    /// Attaches a device and optionally mounts a medium to the given storage controller ([`StorageController`], identified by name), at the indicated port and device.
    ///
    /// **Reference to the official documentation:**
    ///
    /// [https://www.virtualbox.org/sdkref/interface_i_machine.html](https://www.virtualbox.org/sdkref/interface_i_machine.html#a8e51fafec7442a48a86a59edec5ec217)
    ///
    /// # Arguments
    ///
    /// * `name` - &str. Name of the storage controller to attach the device to.
    /// * `controller_port` - i32. Port to attach the device to. For an IDE controller, 0 specifies the primary controller and 1 specifies the secondary controller. For a SCSI controller, this must range from 0 to 15; for a SATA controller, from 0 to 29; for an SAS controller, from 0 to 7
    /// * `device` - i32. Device slot in the given port to attach the device to. This is only relevant for IDE controllers, for which 0 specifies the master device and 1 specifies the slave device. For all other controller types, this must be 0.
    /// * `type_` - [`DeviceType`]. Device type of the attached device. For media opened by [`VirtualBox::open_medium`], this must match the device type specified there.
    /// * `medium` - [`Option<Medium>`]. Medium to mount or [`None`] for an empty drive.
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
    /// use virtualbox_rs::enums::{DeviceType, SessionType};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let mut session = Session::init().unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    /// let machine_mut = session.get_machine().unwrap();
    ///
    ///     machine_mut.attach_device(
    ///         "Floppy",
    ///         0,
    ///         0,
    ///         DeviceType::Floppy,
    ///         None
    ///     ).unwrap();

    pub fn attach_device(
        &self,
        name: &str,
        controller_port: i32,
        device: i32,
        type_: DeviceType,
        medium: Option<&Medium>,
    ) -> Result<(), VboxError> {
        let name = string_to_c_u64_str(name)?;
        let type_ = type_.into();
        let medium = match medium {
            None => {
                let m: *mut IMedium = std::ptr::null_mut();
                m
            }
            Some(m) => m.object,
        };
        get_function_result_unit!(
            self.object,
            AttachDevice,
            name,
            controller_port,
            device,
            type_,
            medium
        )
    }

    /// Attaches a device and optionally mounts a medium to the given storage controller ([`StorageController`], identified by name), at the indicated port and device.
    ///
    /// **Reference to the official documentation:**
    ///
    /// [https://www.virtualbox.org/sdkref/interface_i_machine.html](https://www.virtualbox.org/sdkref/interface_i_machine.html#ad4f8c3614866c1fe78c30357d6e70146)
    ///
    /// # Arguments
    ///
    /// * `name` - &str. Name of the storage controller to attach the device to.
    /// * `controller_port` - i32. Port to attach the device to. For an IDE controller, 0 specifies the primary controller and 1 specifies the secondary controller. For a SCSI controller, this must range from 0 to 15; for a SATA controller, from 0 to 29; for an SAS controller, from 0 to 7
    /// * `device` - i32. Device slot in the given port to attach the device to. This is only relevant for IDE controllers, for which 0 specifies the master device and 1 specifies the slave device. For all other controller types, this must be 0.
    /// * `type_` - [`DeviceType`]. Device type of the attached device. For media opened by [`VirtualBox::open_medium`], this must match the device type specified there.
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
    /// use virtualbox_rs::enums::{DeviceType, SessionType};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let mut session = Session::init().unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    /// let machine_mut = session.get_machine().unwrap();
    ///
    ///     machine_mut.attach_device_without_medium(
    ///         "Floppy",
    ///         0,
    ///         0,
    ///         DeviceType::Floppy
    ///     ).unwrap();

    pub fn attach_device_without_medium(
        &self,
        name: &str,
        controller_port: i32,
        device: i32,
        type_: DeviceType,
    ) -> Result<(), VboxError> {
        let name = string_to_c_u64_str(name)?;
        let type_ = type_.into();
        get_function_result_unit!(
            self.object,
            AttachDeviceWithoutMedium,
            name,
            controller_port,
            device,
            type_
        )
    }

    /// Detaches the device attached to a device slot of the specified bus.
    ///
    /// **Reference to the official documentation:**
    ///
    /// [https://www.virtualbox.org/sdkref/interface_i_machine.html](https://www.virtualbox.org/sdkref/interface_i_machine.html#a59a8e406027e901cc260488f8617fb22)
    ///
    /// # Arguments
    ///
    /// * `name` - &str. Name of the storage controller to detach the medium from.
    /// * `controller_port` - i32. Port number to detach the medium from.
    /// * `device` - i32. Device slot number to detach the medium from.
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
    /// use virtualbox_rs::enums::{SessionType};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let mut session = Session::init().unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    /// let machine_mut = session.get_machine().unwrap();
    ///
    ///     machine_mut.detach_device(
    ///         "Floppy",
    ///         0,
    ///         0
    ///     ).unwrap();
    /// machine_mut.save_settings().unwrap();

    pub fn detach_device(
        &self,
        name: &str,
        controller_port: i32,
        device: i32,
    ) -> Result<(), VboxError> {
        let name = string_to_c_u64_str(name)?;
        get_function_result_unit!(self.object, DetachDevice, name, controller_port, device)
    }

    /// Sets the passthrough mode of an existing DVD device.
    ///
    /// Changing the setting while the VM is running is forbidden. The setting is only used if at VM start the device is configured as a host DVD drive, in all other cases it is ignored. The device must already exist; see [`Machine::attach_device`] for how to attach a new device.
    ///
    /// The controllerPort and device parameters specify the device slot and have the same meaning as with [`Machine::attach_device`].
    ///
    /// # Arguments
    ///
    /// * `name` - &str. Name of the storage controller.
    /// * `controller_port` - i32. Storage controller port.
    /// * `device` - i32. Device slot in the given port.
    /// * `passthrough` - bool. New value for the passthrough setting.
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
    /// use virtualbox_rs::enums::{SessionType};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let mut session = Session::init().unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    /// let machine_mut = session.get_machine().unwrap();
    ///
    ///     machine_mut.passthrough_device(
    ///         "AHCI",
    ///         1,
    ///         0,
    ///         true
    ///     ).unwrap();
    /// machine_mut.save_settings().unwrap();

    pub fn passthrough_device(
        &self,
        name: &str,
        controller_port: i32,
        device: i32,
        passthrough: bool,
    ) -> Result<(), VboxError> {
        let name = string_to_c_u64_str(name)?;
        let passthrough = if passthrough { 1 } else { 0 };
        get_function_result_unit!(
            self.object,
            PassthroughDevice,
            name,
            controller_port,
            device,
            passthrough
        )
    }

    /// Sets the behavior for guest-triggered medium eject.
    ///
    /// In some situations it is desirable that such ejects update the VM configuration, and in others the eject should keep the VM configuration. The device must already exist; see [`Machine::attach_device`] for how to attach a new device.
    ///
    /// The controllerPort and device parameters specify the device slot and have the same meaning as with [`Machine::attach_device`].
    ///
    /// # Arguments
    ///
    /// * `name` - &str. Name of the storage controller.
    /// * `controller_port` - i32. Storage controller port.
    /// * `device` - i32. Device slot in the given port.
    /// * `temporary_eject` - bool. New value for the eject behavior.
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
    /// use virtualbox_rs::enums::{SessionType};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let mut session = Session::init().unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    /// let machine_mut = session.get_machine().unwrap();
    ///
    ///     machine_mut.temporary_eject_device(
    ///         "AHCI",
    ///         1,
    ///         0,
    ///         true
    ///     ).unwrap();
    /// machine_mut.save_settings().unwrap();

    pub fn temporary_eject_device(
        &self,
        name: &str,
        controller_port: i32,
        device: i32,
        temporary_eject: bool,
    ) -> Result<(), VboxError> {
        let name = string_to_c_u64_str(name)?;
        let temporary_eject = if temporary_eject { 1 } else { 0 };
        get_function_result_unit!(
            self.object,
            TemporaryEjectDevice,
            name,
            controller_port,
            device,
            temporary_eject
        )
    }

    /// Sets a flag in the device information which indicates that the medium is not based on rotational technology, i.e.
    ///
    /// that the access times are more or less independent of the position on the medium. This may or may not be supported by a particular drive, and is silently ignored in the latter case. At the moment only hard disks (which is a misnomer in this context) accept this setting. Changing the setting while the VM is running is forbidden. The device must already exist; see [`Machine::attach_device`] for how to attach a new device.
    ///
    /// The controllerPort and device parameters specify the device slot and have the same meaning as with [`Machine::attach_device`].
    ///
    /// # Arguments
    ///
    /// * `name` - &str. Name of the storage controller.
    /// * `controller_port` - i32. Storage controller port.
    /// * `device` - i32. Device slot in the given port.
    /// * `non_rotational` - bool. New value for the non-rotational device flag.
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
    /// use virtualbox_rs::enums::{SessionType};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let mut session = Session::init().unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    /// let machine_mut = session.get_machine().unwrap();
    ///
    ///     machine_mut.non_rotational_device(
    ///         "AHCI",
    ///         0,
    ///         0,
    ///         true
    ///     ).unwrap();
    /// machine_mut.save_settings().unwrap();

    pub fn non_rotational_device(
        &self,
        name: &str,
        controller_port: i32,
        device: i32,
        non_rotational: bool,
    ) -> Result<(), VboxError> {
        let name = string_to_c_u64_str(name)?;
        let non_rotational = if non_rotational { 1 } else { 0 };
        get_function_result_unit!(
            self.object,
            NonRotationalDevice,
            name,
            controller_port,
            device,
            non_rotational
        )
    }

    /// Sets a flag in the device information which indicates that the medium supports discarding unused blocks (called trimming for SATA or unmap for SCSI devices) .This may or may not be supported by a particular drive, and is silently ignored in the latter case.
    ///
    /// At the moment only hard disks (which is a misnomer in this context) accept this setting. Changing the setting while the VM is running is forbidden. The device must already exist; see [`Machine::attach_device`] for how to attach a new device.
    ///
    /// The controllerPort and device parameters specify the device slot and have the same meaning as with [`Machine::attach_device`].
    ///
    /// # Arguments
    ///
    /// * `name` - &str. Name of the storage controller.
    /// * `controller_port` - i32. Storage controller port.
    /// * `device` - i32. Device slot in the given port.
    /// * `discard` - bool. New value for the discard device flag.
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
    /// use virtualbox_rs::enums::{SessionType};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let mut session = Session::init().unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    /// let machine_mut = session.get_machine().unwrap();
    ///
    ///     machine_mut.set_auto_discard_for_device(
    ///         "AHCI",
    ///         0,
    ///         0,
    ///         true
    ///     ).unwrap();
    /// machine_mut.save_settings().unwrap();

    pub fn set_auto_discard_for_device(
        &self,
        name: &str,
        controller_port: i32,
        device: i32,
        discard: bool,
    ) -> Result<(), VboxError> {
        let name = string_to_c_u64_str(name)?;
        let discard = if discard { 1 } else { 0 };
        get_function_result_unit!(
            self.object,
            SetAutoDiscardForDevice,
            name,
            controller_port,
            device,
            discard
        )
    }

    /// Sets a flag in the device information which indicates that the attached device is hot pluggable or not.
    ///
    /// This may or may not be supported by a particular controller and/or drive, and is silently ignored in the latter case. Changing the setting while the VM is running is forbidden. The device must already exist; see [`Machine::attach_device`] for how to attach a new device.
    ///
    /// The controllerPort and device parameters specify the device slot and have the same meaning as with [`Machine::attach_device`].
    ///
    /// # Arguments
    ///
    /// * `name` - &str. Name of the storage controller.
    /// * `controller_port` - i32. Storage controller port.
    /// * `device` - i32. Device slot in the given port.
    /// * `hot_pluggable` - bool. New value for the hot-pluggable device flag.
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
    /// use virtualbox_rs::enums::{SessionType};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let mut session = Session::init().unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    /// let machine_mut = session.get_machine().unwrap();
    ///
    ///     machine_mut.set_hot_pluggable_for_device(
    ///         "AHCI",
    ///         0,
    ///         0,
    ///         true
    ///     ).unwrap();
    /// machine_mut.save_settings().unwrap();

    pub fn set_hot_pluggable_for_device(
        &self,
        name: &str,
        controller_port: i32,
        device: i32,
        hot_pluggable: bool,
    ) -> Result<(), VboxError> {
        let name = string_to_c_u64_str(name)?;
        let hot_pluggable = if hot_pluggable { 1 } else { 0 };
        get_function_result_unit!(
            self.object,
            SetHotPluggableForDevice,
            name,
            controller_port,
            device,
            hot_pluggable
        )
    }

    /// Sets the bandwidth group of an existing storage device.
    ///
    /// The device must already exist; see [`Machine::attach_device`] for how to attach a new device.
    ///
    /// The controllerPort and device parameters specify the device slot and have the same meaning as with [`Machine::attach_device`].
    /// # Arguments
    ///
    /// * `name` - &str. Name of the storage controller.
    /// * `controller_port` - i32. Storage controller port.
    /// * `device` - i32. Device slot in the given port.
    /// * `bandwidth_group` - [`BandwidthGroup`]. New value for the bandwidth group or null for no group.
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
    /// use virtualbox_rs::enums::{BandwidthGroupType, SessionType};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let mut session = Session::init().unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    /// let machine_mut = session.get_machine().unwrap();
    /// let bandwidth_control = machine_mut.get_bandwidth_control().unwrap();
    /// bandwidth_control.create_bandwidth_group(
    ///         "g1",
    ///         BandwidthGroupType::Disk,
    ///         1000000
    ///     ).unwrap();
    /// let bandwidth_group = bandwidth_control.get_bandwidth_group("g1").unwrap();
    /// machine_mut.set_bandwidth_group_for_device(
    ///     "SATA",
    ///     0,
    ///     0,
    ///     &bandwidth_group
    /// ).unwrap();
    /// machine_mut.save_settings().unwrap();
    pub fn set_bandwidth_group_for_device(
        &self,
        name: &str,
        controller_port: i32,
        device: i32,
        bandwidth_group: &BandwidthGroup,
    ) -> Result<(), VboxError> {
        let name = string_to_c_u64_str(name)?;
        let bandwidth_group = bandwidth_group.object;
        get_function_result_unit!(
            self.object,
            SetBandwidthGroupForDevice,
            name,
            controller_port,
            device,
            bandwidth_group
        )
    }

    ///  Sets no bandwidth group for an existing storage device.
    ///
    /// The device must already exist; see [`Machine::attach_device`] for how to attach a new device. The controllerPort and device parameters specify the device slot and have have the same meaning as with [`Machine::attach_device`].
    ///
    /// # Arguments
    ///
    /// * `name` - &str. Name of the storage controller.
    /// * `controller_port` - i32. Storage controller port.
    /// * `device` - i32. Device slot in the given port.
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
    /// use virtualbox_rs::enums::{SessionType};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let mut session = Session::init().unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    /// let machine_mut = session.get_machine().unwrap();
    ///
    /// machine_mut.set_no_bandwidth_group_for_device(
    ///     "SATA",
    ///     0,
    ///     0
    /// ).unwrap();
    /// machine_mut.save_settings().unwrap();
    pub fn set_no_bandwidth_group_for_device(
        &self,
        name: &str,
        controller_port: i32,
        device: i32,
    ) -> Result<(), VboxError> {
        let name = string_to_c_u64_str(name)?;
        get_function_result_unit!(
            self.object,
            SetNoBandwidthGroupForDevice,
            name,
            controller_port,
            device
        )
    }

    /// Unmounts any currently mounted medium (IMedium, identified by the given UUID id) to the given storage controller ([`StorageController`], identified by name), at the indicated port and device.
    ///
    /// The device must already exist;
    ///
    /// This method is intended only for managing removable media, where the device is fixed but media is changeable at runtime (such as DVDs and floppies). It cannot be used for fixed media such as hard disks.
    ///
    /// The controllerPort and device parameters specify the device slot and have the same meaning as with [`Machine::attach_device`].
    ///
    /// The specified device slot must have a medium mounted, which will be unmounted. If there is no mounted medium it will do nothing. See [`Medium`] for more detailed information about attaching/unmounting media.
    ///
    /// # Arguments
    ///
    /// * `name` - &str. Name of the storage controller to unmount the medium from.
    /// * `controller_port` - i32. Port to unmount the medium from.
    /// * `device` - i32. Device slot in the given port to unmount the medium from.
    /// * `force` - bool. Allows to force unmount of a medium which is locked by the device slot in the given port medium is attached to.
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
    /// use virtualbox_rs::enums::{SessionType};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let mut session = Session::init().unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    /// let machine_mut = session.get_machine().unwrap();
    /// machine_mut.unmount_medium(
    ///     "SATA",
    ///     0,
    ///     0,
    ///     true
    /// ).unwrap()
    pub fn unmount_medium(
        &self,
        name: &str,
        controller_port: i32,
        device: i32,
        force: bool,
    ) -> Result<(), VboxError> {
        let name = string_to_c_u64_str(name)?;
        let force = if force { 1 } else { 0 };
        get_function_result_unit!(
            self.object,
            UnmountMedium,
            name,
            controller_port,
            device,
            force
        )
    }

    ///  Mounts a medium ([`Medium`], identified by the given UUID id) to the given storage controller ([`StorageController`], identified by name), at the indicated port and device.
    ///
    /// The device must already exist; see [`Machine::attach_device`] for how to attach a new device.
    ///
    /// This method is intended only for managing removable media, where the device is fixed but media is changeable at runtime (such as DVDs and floppies). It cannot be used for fixed media such as hard disks.
    ///
    /// The controllerPort and device parameters specify the device slot and have have the same meaning as with [`Machine::attach_device`].
    ///
    /// The specified device slot can have a medium mounted, which will be unmounted first. Specifying a zero UUID (or an empty string) for medium does just an unmount.
    ///
    /// See [`Medium`] for more detailed information about attaching media.
    ///
    /// # Arguments
    ///
    /// * `name` - &str. Name of the storage controller to attach the medium to.
    /// * `controller_port` - i32. Port to attach the medium to.
    /// * `device` - i32. Device slot in the given port to attach the medium to.
    /// * `medium` - i32. Medium to mount or None for an empty drive.
    /// * `force` - bool. Allows to force unmount of a medium which is locked by the device slot in the given port medium is attached to.
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
    /// use virtualbox_rs::enums::{SessionType};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let mut session = Session::init().unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    /// let machine_mut = session.get_machine().unwrap();
    /// let mediums = vbox.get_dvd_images().unwrap();
    /// let medium = mediums.get(0);
    /// machine_mut.mount_medium(
    ///     "SATA",
    ///     1,
    ///     0,
    ///     medium,
    ///     true
    /// ).unwrap();
    /// machine_mut.save_settings().unwrap();
    pub fn mount_medium(
        &self,
        name: &str,
        controller_port: i32,
        device: i32,
        medium: Option<&Medium>,
        force: bool,
    ) -> Result<(), VboxError> {
        let name = string_to_c_u64_str(name)?;
        let medium = match medium {
            None => {
                let m: *mut IMedium = std::ptr::null_mut();
                m
            }
            Some(m) => m.object,
        };
        let force = if force { 1 } else { 0 };

        get_function_result_unit!(
            self.object,
            MountMedium,
            name,
            controller_port,
            device,
            medium,
            force
        )
    }

    ///  Returns the virtual medium attached to a device slot of the specified bus.
    ///
    /// Note that if the medium was indirectly attached by [`Machine::mount_medium`] to the given device slot then this method will return not the same object as passed to the mountMedium call. See IMedium for more detailed information about mounting a medium.
    ///
    /// # Arguments
    ///
    /// * `name` - &str. Name of the storage controller the medium is attached to.
    /// * `controller_port` - i32. Port to query.
    /// * `device` - i32. Device slot in the given port to query.
    ///
    /// # Returns
    ///
    /// Returns [`Medium`] success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{VirtualBox};
    /// use virtualbox_rs::enums::{SessionType};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let medium = machine.get_medium(
    ///     "SATA",
    ///     1,
    ///     0
    /// ).unwrap();
    pub fn get_medium(
        &self,
        name: &str,
        controller_port: i32,
        device: i32,
    ) -> Result<Medium, VboxError> {
        let name = string_to_c_u64_str(name)?;
        let medium = get_function_result_pointer!(
            self.object,
            GetMedium,
            *mut IMedium,
            name,
            controller_port,
            device
        )?;
        Ok(Medium::new(medium))
    }

    ///  Returns an array of medium attachments which are attached to the the controller with the given name.
    ///
    /// # Arguments
    ///
    /// * `name` - &str. Name of the storage controller.
    ///
    /// # Returns
    ///
    /// Returns [`Vec<MediumAttachment>`] success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{VirtualBox};
    /// use virtualbox_rs::enums::{SessionType};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let medium = machine.get_medium_attachments_of_controller(
    ///     "SATA"
    /// ).unwrap();
    pub fn get_medium_attachments_of_controller(
        &self,
        name: &str,
    ) -> Result<Vec<MediumAttachment>, VboxError> {
        let name = string_to_c_u64_str(name)?;
        let medium_attachments = get_function_result_pointer_vec!(
            self.object,
            GetMediumAttachmentsOfController,
            *mut IMediumAttachment,
            name
        )?;
        Ok(medium_attachments
            .iter()
            .map(|object| MediumAttachment::new(object.clone()))
            .collect())
    }

    ///  Returns a medium attachment which corresponds to the controller with the given name, on the given port and device slot.
    ///
    /// # Arguments
    ///
    /// * `name` - &str. Name of the storage controller.
    /// * `controller_port` - i32. Port to query.
    /// * `device` - i32. Device slot in the given port to query.
    ///
    /// # Returns
    ///
    /// Returns [`MediumAttachment`] success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{VirtualBox};
    /// use virtualbox_rs::enums::{SessionType};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let medium_attachment = machine.get_medium_attachment(
    ///     "SATA",
    ///     1,
    ///     0
    /// ).unwrap();
    pub fn get_medium_attachment(
        &self,
        name: &str,
        controller_port: i32,
        device: i32,
    ) -> Result<MediumAttachment, VboxError> {
        let name = string_to_c_u64_str(name)?;
        let medium_attachment = get_function_result_pointer!(
            self.object,
            GetMediumAttachment,
            *mut IMediumAttachment,
            name,
            controller_port,
            device
        )?;
        Ok(MediumAttachment::new(medium_attachment))
    }

    // TODO AttachHostPCIDevice

    // TODO DetachHostPCIDevice

    ///  Returns a medium attachment which corresponds to the controller with the given name, on the given port and device slot.
    ///
    /// # Arguments
    ///
    /// * `name` - &str. Name of the storage controller.
    /// * `controller_port` - i32. Port to query.
    ///
    /// # Returns
    ///
    /// Returns [`StorageController`] success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{Session, VirtualBox};
    /// use virtualbox_rs::enums::{SessionType, StorageBus};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let mut session = Session::init().unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    /// let machine_mut = session.get_machine().unwrap();
    /// let storage_controller = machine_mut.add_storage_controller(
    ///     "IDE1",
    ///     StorageBus::IDE
    /// ).unwrap();
    pub fn add_storage_controller(
        &self,
        name: &str,
        connection_type: StorageBus,
    ) -> Result<StorageController, VboxError> {
        let name = string_to_c_u64_str(name)?;
        let connection_type = connection_type.into();
        let medium_attachment = get_function_result_pointer!(
            self.object,
            AddStorageController,
            *mut IStorageController,
            name,
            connection_type
        )?;
        Ok(StorageController::new(medium_attachment))
    }
    ///  Returns a storage controller with the given name.
    ///
    /// # Arguments
    ///
    /// * `name` - &str. Name of the storage controller.
    ///
    /// # Returns
    ///
    /// Returns [`StorageController`] success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{VirtualBox};
    /// use virtualbox_rs::enums::{SessionType};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let storage_controller = machine.get_storage_controller_by_name(
    ///     "IDE1"
    /// ).unwrap();
    pub fn get_storage_controller_by_name(
        &self,
        name: &str,
    ) -> Result<StorageController, VboxError> {
        let name = string_to_c_u64_str(name)?;
        let medium_attachment = get_function_result_pointer!(
            self.object,
            GetStorageControllerByName,
            *mut IStorageController,
            name
        )?;
        Ok(StorageController::new(medium_attachment))
    }

    ///  Returns a storage controller of a specific storage bus with the given instance number.
    ///
    /// # Arguments
    ///
    /// * `connection_type` - [`StorageBus`].
    /// * `instance` - u32.
    ///
    /// # Returns
    ///
    /// Returns [`StorageController`] success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{VirtualBox};
    /// use virtualbox_rs::enums::{SessionType, StorageBus};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let storage_controller = machine.get_storage_controller_by_instance(
    ///     StorageBus::SATA,
    ///     0
    /// ).unwrap();
    pub fn get_storage_controller_by_instance(
        &self,
        connection_type: StorageBus,
        instance: u32,
    ) -> Result<StorageController, VboxError> {
        let connection_type = connection_type.into();
        let medium_attachment = get_function_result_pointer!(
            self.object,
            GetStorageControllerByInstance,
            *mut IStorageController,
            connection_type,
            instance
        )?;
        Ok(StorageController::new(medium_attachment))
    }
    ///  Removes a storage controller from the machine with all devices attached to it.
    ///
    /// # Arguments
    ///
    /// * `name` - &str. Name of the storage controller.
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
    /// use virtualbox_rs::enums::{SessionType};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let mut session = Session::init().unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    /// let machine_mut = session.get_machine().unwrap();
    /// let storage_controller = machine_mut.remove_storage_controller(
    ///     "IDE1"
    /// ).unwrap();
    pub fn remove_storage_controller(&self, name: &str) -> Result<(), VboxError> {
        let name = string_to_c_u64_str(name)?;
        get_function_result_unit!(self.object, RemoveStorageController, name)
    }

    ///  Sets the bootable flag of the storage controller with the given name.
    ///
    /// # Arguments
    ///
    /// * `name` - &str. Name of the storage controller.
    /// * `bootable` - bool.
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
    /// use virtualbox_rs::enums::{SessionType};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let mut session = Session::init().unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    /// let machine_mut = session.get_machine().unwrap();
    /// machine_mut.set_storage_controller_bootable(
    ///     "IDE1",
    ///     true
    /// ).unwrap();
    pub fn set_storage_controller_bootable(
        &self,
        name: &str,
        bootable: bool,
    ) -> Result<(), VboxError> {
        let name = string_to_c_u64_str(name)?;
        let bootable = if bootable { 1 } else { 0 };
        get_function_result_unit!(self.object, SetStorageControllerBootable, name, bootable)
    }

    /// Adds a new USB controller to the machine and returns it as an instance of [`USBController`].
    ///
    /// # Arguments
    ///
    /// * `name` - &str. Name of the USB controller.
    /// * `type_` - [`USBControllerType`]. Port to query.
    ///
    /// # Returns
    ///
    /// Returns [`USBController`] success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{Session, VirtualBox};
    /// use virtualbox_rs::enums::{SessionType, StorageBus, USBControllerType};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let mut session = Session::init().unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    /// let machine_mut = session.get_machine().unwrap();
    /// let usb_controller = machine_mut.add_usb_controller(
    ///     "USB",
    ///     USBControllerType::EHCI
    /// ).unwrap();
    pub fn add_usb_controller(
        &self,
        name: &str,
        type_: USBControllerType,
    ) -> Result<USBController, VboxError> {
        let name = string_to_c_u64_str(name)?;
        let type_ = type_.into();
        let usb_controller = get_function_result_pointer!(
            self.object,
            AddUSBController,
            *mut IUSBController,
            name,
            type_
        )?;
        Ok(USBController::new(usb_controller))
    }

    /// Removes a USB controller from the machine.
    ///
    /// # Arguments
    ///
    /// * `name` - &str. Name of the USB controller.
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
    /// use virtualbox_rs::enums::{SessionType, USBControllerType};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let mut session = Session::init().unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    /// let machine_mut = session.get_machine().unwrap();
    /// machine_mut.remove_usb_controller(
    ///     "USB"
    /// ).unwrap();
    pub fn remove_usb_controller(&self, name: &str) -> Result<(), VboxError> {
        let name = string_to_c_u64_str(name)?;
        get_function_result_unit!(self.object, RemoveUSBController, name)
    }
    /// Returns a USB controller with the given name.
    ///
    /// # Arguments
    ///
    /// * `name` - &str. Name of the USB controller.
    ///
    /// # Returns
    ///
    /// Returns [`USBController`] success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{Session, VirtualBox};
    /// use virtualbox_rs::enums::{SessionType, StorageBus, USBControllerType};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let mut session = Session::init().unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    /// let machine_mut = session.get_machine().unwrap();
    /// let usb_controller = machine_mut.get_usb_controller_by_name(
    ///     "USB"
    /// ).unwrap();
    pub fn get_usb_controller_by_name(&self, name: &str) -> Result<USBController, VboxError> {
        let name = string_to_c_u64_str(name)?;
        let usb_controller = get_function_result_pointer!(
            self.object,
            GetUSBControllerByName,
            *mut IUSBController,
            name
        )?;
        Ok(USBController::new(usb_controller))
    }

    /// Returns the number of USB controllers of the given type attached to the VM.
    ///
    /// # Arguments
    ///
    /// * `type_` - [`USBControllerType`]. Port to query.
    ///
    /// # Returns
    ///
    /// Returns u32 success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{Session, VirtualBox};
    /// use virtualbox_rs::enums::{SessionType, StorageBus, USBControllerType};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let mut session = Session::init().unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    /// let machine_mut = session.get_machine().unwrap();
    /// let usb_controller_count = machine_mut.get_usb_controller_count_by_type(
    ///     USBControllerType::EHCI
    /// ).unwrap();
    pub fn get_usb_controller_count_by_type(
        &self,
        type_: USBControllerType,
    ) -> Result<u32, VboxError> {
        let type_ = type_.into();
        get_function_result_number!(self.object, GetUSBControllerCountByType, u32, type_)
    }

    /// Returns the serial port associated with the given slot.
    ///
    /// Slots are numbered sequentially, starting with zero. The total number of serial ports per machine is defined by the [`SystemProperties::get_serial_port_count`] property, so the maximum slot number is one less than that property's value.
    ///
    /// # Arguments
    ///
    /// * `slot` - u32. Name of the USB controller.
    ///
    /// # Returns
    ///
    /// Returns [`SerialPort`] success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{VirtualBox};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let serial_port = machine.get_serial_port(
    ///     0
    /// ).unwrap();
    pub fn get_serial_port(&self, slot: u32) -> Result<SerialPort, VboxError> {
        let serial_port =
            get_function_result_pointer!(self.object, GetSerialPort, *mut ISerialPort, slot)?;
        Ok(SerialPort::new(serial_port))
    }

    /// Returns the parallel port associated with the given slot.
    ///
    /// Slots are numbered sequentially, starting with zero. The total number of parallel ports per machine is defined by the [`SystemProperties::get_parallel_port_count`] property, so the maximum slot number is one less than that property's value.
    ///
    /// # Arguments
    ///
    /// * `slot` - u32. Name of the USB controller.
    ///
    /// # Returns
    ///
    /// Returns [`ParallelPort`] success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{VirtualBox};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let parallel_port = machine.get_parallel_port(
    ///     0
    /// ).unwrap();
    pub fn get_parallel_port(&self, slot: u32) -> Result<ParallelPort, VboxError> {
        let parallel_port =
            get_function_result_pointer!(self.object, GetParallelPort, *mut IParallelPort, slot)?;
        Ok(ParallelPort::new(parallel_port))
    }

    /// Returns an array representing the machine-specific extra data keys which currently have values defined.
    ///
    /// # Returns
    ///
    /// Returns [`Vec<&str>`] success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{VirtualBox};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let extra_data_keys = machine.get_extra_data_keys().unwrap();
    pub fn get_extra_data_keys(&self) -> Result<Vec<&'static str>, VboxError> {
        get_function_result_str_vec!(self.object, GetExtraDataKeys)
    }

    /// Returns associated machine-specific extra data.
    ///
    /// If the requested data key does not exist, this function will succeed and return an empty string in the value argument.
    ///
    /// # Arguments
    ///
    /// * `key` - &str. Name of the data key to get.
    ///
    /// # Returns
    ///
    /// Returns [`ParallelPort`] success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{VirtualBox};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let extra_data = machine.get_extra_data("GUI/LastCloseAction").unwrap();
    pub fn get_extra_data(&self, key: &str) -> Result<&'static str, VboxError> {
        let key = string_to_c_u64_str(key).unwrap();
        get_function_result_str!(self.object, GetExtraData, key)
    }

    /// Sets associated machine-specific extra data.
    ///
    /// If you pass an empty string as a key value, the given key will be deleted.
    ///
    /// # Arguments
    ///
    /// * `key` - &str. Name of the data key to set.
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
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let mut session = Session::init().unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    /// let machine_mut = session.get_machine().unwrap();
    /// machine_mut.set_extra_data("key", "value").unwrap()
    pub fn set_extra_data(&self, key: &str, value: &str) -> Result<(), VboxError> {
        let key = string_to_c_u64_str(key).unwrap();
        let value = string_to_c_u64_str(value).unwrap();
        get_function_result_unit!(self.object, SetExtraData, key, value)
    }

    #[cfg(is_v_7_1)]
    /// Returns the virtual CPU boolean value of the specified property.
    ///
    /// # Arguments
    ///
    /// * `property` - [`CPUPropertyType`]. Property type to query.
    ///
    /// # Returns
    ///
    /// Returns bool success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{VirtualBox};
    /// use virtualbox_rs::enums::CPUPropertyType;
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let cpu_property = machine.get_cpu_property(CPUPropertyType::APIC).unwrap();
    pub fn get_cpu_property(&self, property: CPUPropertyType) -> Result<bool, VboxError> {
        self.get_platform()?.get_x86()?.get_cpu_property(property)
    }

    #[cfg(is_v_7_1)]
    ///  Sets the virtual CPU boolean value of the specified property.
    ///
    /// # Arguments
    ///
    /// * `property` - [`CPUPropertyType`]. Property type to query.
    /// * `value` - bool. Property value.
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
    /// use virtualbox_rs::enums::{CPUPropertyType, SessionType};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let mut session = Session::init().unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    /// let machine_mut = session.get_machine().unwrap();
    /// machine_mut.set_cpu_property(CPUPropertyType::APIC, true).unwrap();
    pub fn set_cpu_property(
        &self,
        property: CPUPropertyType,
        value: bool,
    ) -> Result<(), VboxError> {
        self.get_platform()?
            .get_x86()?
            .set_cpu_property(property, value)
    }

    // TODO GetCPUIDLeafByOrdinal
    // TODO GetCPUIDLeaf
    // TODO SetCPUIDLeaf
    // TODO RemoveCPUIDLeaf
    // TODO RemoveAllCPUIDLeaves

    #[cfg(is_v_7_1)]
    /// Returns the value of the specified hardware virtualization boolean property.
    ///
    /// # Arguments
    ///
    /// * `property` - [`HWVirtExPropertyType`]. Property type to query.
    ///
    /// # Returns
    ///
    /// Returns bool success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{VirtualBox};
    /// use virtualbox_rs::enums::{HWVirtExPropertyType};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let property = machine.get_hw_virt_ex_property(HWVirtExPropertyType::NestedPaging).unwrap();
    pub fn get_hw_virt_ex_property(
        &self,
        property: HWVirtExPropertyType,
    ) -> Result<bool, VboxError> {
        self.get_platform()?
            .get_x86()?
            .get_hw_virt_ex_property(property)
    }

    #[cfg(is_v_7_1)]
    ///  Sets a new value for the specified hardware virtualization boolean property.
    ///
    /// # Arguments
    ///
    /// * `property` - [`HWVirtExPropertyType`]. Property type to set.
    /// * `value` - bool. New property value.
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
    /// use virtualbox_rs::enums::{HWVirtExPropertyType, SessionType};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let mut session = Session::init().unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    /// let machine_mut = session.get_machine().unwrap();
    /// machine_mut.set_hw_virt_ex_property(HWVirtExPropertyType::NestedPaging, true).unwrap();
    pub fn set_hw_virt_ex_property(
        &self,
        property: HWVirtExPropertyType,
        value: bool,
    ) -> Result<(), VboxError> {
        self.get_platform()?
            .get_x86()?
            .set_hw_virt_ex_property(property, value)
    }

    ///  Currently, it is an error to change this property on any machine.
    ///
    /// # Arguments
    ///
    /// * `settings_file_path` - &str. New settings file path, will be used to determine the new location for the attached media if it is in the same directory or below as the original settings file.
    ///
    /// # Returns
    ///
    /// Returns [`Progress`] success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{Session, VirtualBox};
    /// use virtualbox_rs::enums::{SessionType};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let mut session = Session::init().unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    /// let machine_mut = session.get_machine().unwrap();
    /// let progress = machine_mut.set_settings_file_path("/home/host_user/VM/Freebsd_14/").unwrap();
    pub fn set_settings_file_path(&self, settings_file_path: &str) -> Result<Progress, VboxError> {
        let settings_file_path = string_to_c_u64_str(settings_file_path)?;
        let progress = get_function_result_pointer!(
            self.object,
            SetSettingsFilePath,
            *mut IProgress,
            settings_file_path
        )?;
        Ok(Progress::new(progress))
    }

    /// Saves any changes to machine settings made since the session has been opened or a new machine has been created, or since the last call to [`Machine::save_settings`] or [`Machine::discard_settings`].
    ///
    /// # Returns
    ///
    /// Returns () class on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{Session, VirtualBox};
    /// use virtualbox_rs::enums::SessionType;
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let mut session = Session::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    ///
    /// let machine_mut = session.get_machine().unwrap();
    ///
    /// machine_mut.save_settings().unwrap();
    pub fn save_settings(&self) -> Result<(), VboxError> {
        get_function_result_unit!(self.object, SaveSettings)
    }

    /// Discards any changes to the machine settings made since the session has been opened or since the last call to  [`Machine::save_settings`] or [`Machine::discard_settings`].
    ///
    /// # Returns
    ///
    /// Returns () class on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{Session, VirtualBox};
    /// use virtualbox_rs::enums::SessionType;
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let mut session = Session::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    ///
    /// let machine_mut = session.get_machine().unwrap();
    ///
    /// machine_mut.discard_settings().unwrap();
    pub fn discard_settings(&self) -> Result<(), VboxError> {
        get_function_result_unit!(self.object, DiscardSettings)
    }

    /// Unregisters a machine previously registered with [`VirtualBox::register_machine`] and optionally do additional cleanup before the machine is unregistered.
    ///
    /// **Reference to the official documentation:**
    ///
    /// [https://www.virtualbox.org/sdkref/interface_i_machine.html](https://www.virtualbox.org/sdkref/interface_i_machine.html#aa75e76da42ef908795d235d995262c6f)
    ///
    /// # Arguments
    ///
    /// * `cleanup_mode` - [`CleanupMode`]	List of media detached from the machine, depending on the cleanupMode parameter.
    ///
    /// # Returns
    ///
    /// Returns [`Vec<Medium>`] on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{VirtualBox};
    /// use virtualbox_rs::enums::{CleanupMode, SessionType};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let medias = machine.unregister(CleanupMode::Full).unwrap();
    pub fn unregister(&self, cleanup_mode: CleanupMode) -> Result<Vec<Medium>, VboxError> {
        let cleanup_mode = cleanup_mode.into();
        let medias =
            get_function_result_pointer_vec!(self.object, Unregister, *mut IMedium, cleanup_mode)?;
        Ok(medias
            .iter()
            .map(|object| Medium::new(object.clone()))
            .collect())
    }

    /// Deletes the files associated with this machine from disk.
    ///
    /// If medium objects are passed in with the aMedia argument, they are closed and, if closing was successful, their storage files are deleted as well. For convenience, this array of media files can be the same as the one returned from a previous [`Machine::unregister`] call.
    ///
    /// **Reference to the official documentation:**
    ///
    /// [https://www.virtualbox.org/sdkref/interface_i_machine.html](https://www.virtualbox.org/sdkref/interface_i_machine.html#a8645f1405cd6d834a3be005c8d533d68)
    ///
    /// # Arguments
    ///
    /// * `media` - [`Vec<Medium>`]. List of media to be closed and whose storage files will be deleted.
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
    /// use virtualbox_rs::enums::{CleanupMode};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let medias = machine.unregister(CleanupMode::Full).unwrap();
    /// let progress = machine.delete_config(medias).unwrap();
    pub fn delete_config(&self, media: Vec<Medium>) -> Result<Progress, VboxError> {
        let mut media: Vec<*mut IMedium> = media.iter().map(|m| m.object).collect();
        let media_ptr = media.as_mut_ptr();
        let media_count = media.len() as u32;
        let progress = get_function_result_pointer!(
            self.object,
            DeleteConfig,
            *mut IProgress,
            media_count,
            media_ptr
        )?;
        Ok(Progress::new(progress))
    }

    /// Exports the machine to an OVF appliance.
    ///
    /// See [`Appliance`] for the steps required to export VirtualBox machines to OVF.
    ///
    /// # Arguments
    ///
    /// * `appliance` - [`Appliance`]. Appliance to export this machine to.
    /// * `location` - &str. The target location.
    ///
    /// # Returns
    ///
    /// Returns [`VirtualSystemDescription`] on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{Appliance, VirtualBox};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let appliance = Appliance::init().unwrap();
    /// let description = machine.export_to(&appliance, "").unwrap();
    pub fn export_to(
        &self,
        appliance: &Appliance,
        location: &str,
    ) -> Result<VirtualSystemDescription, VboxError> {
        let appliance = appliance.object;
        let location = string_to_c_u64_str(location)?;
        let description = get_function_result_pointer!(
            self.object,
            ExportTo,
            *mut IVirtualSystemDescription,
            appliance,
            location
        )?;
        Ok(VirtualSystemDescription::new(description))
    }

    /// Returns a snapshot of this machine with the given name or UUID.
    ///
    /// Returns a snapshot of this machine with the given UUID. A null argument can be used to obtain the first snapshot taken on this machine. To traverse the whole tree of snapshots starting from the root, inspect the root snapshot's [`Snapshot::get_children`] attribute and recurse over those children.
    ///
    /// # Arguments
    ///
    /// * `name_or_id` - &str. What to search for. Name or UUID of the snapshot to find
    ///
    /// # Returns
    ///
    /// Returns [`VirtualSystemDescription`] on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{VirtualBox};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let snapshot = machine.find_snapshot("Snapshot_1").unwrap();
    pub fn find_snapshot(&self, name_or_id: &str) -> Result<Snapshot, VboxError> {
        let name_or_id = string_to_c_u64_str(name_or_id)?;
        let snapshot =
            get_function_result_pointer!(self.object, FindSnapshot, *mut ISnapshot, name_or_id)?;
        Ok(Snapshot::new(snapshot))
    }

    /// Creates a new permanent shared folder by associating the given logical name with the given host path, adds it to the collection of shared folders and starts sharing it.
    ///
    /// Refer to the description of [`SharedFolder`] to read more about logical names.
    ///
    /// # Arguments
    ///
    /// * `name` - &str. Unique logical name of the shared folder.
    /// * `host_path` - &str. Full path to the shared folder in the host file system.
    /// * `writable` - bool. Whether the share is writable or read-only.
    /// * `automount` - bool. Whether the share gets automatically mounted by the guest or not.
    /// * `auto_mount_point` - &str. Where the guest should automatically mount the folder, if possible. For Windows and OS/2 guests this should be a drive letter, while other guests it should be a absolute directory.
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
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let mut session = Session::init().unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    /// let machine_mut = session.get_machine().unwrap();
    /// machine_mut.create_shared_folder(
    ///     "sf1",
    ///     "/home/host_user/sf1",
    ///     true,
    ///     true,
    ///     "/mnt/sf1"
    /// ).unwrap();
    /// machine_mut.save_settings().unwrap();
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

    /// Removes the permanent shared folder with the given name previously created by [`Machine::create_shared_folder`] from the collection of shared folders and stops sharing it.
    ///
    /// # Arguments
    ///
    /// * `name` - Logical name of the shared folder to remove.
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
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let mut session = Session::init().unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    /// let machine_mut = session.get_machine().unwrap();
    /// machine_mut.remove_shared_folder("sf1").unwrap();
    pub fn remove_shared_folder(&self, name: &str) -> Result<(), VboxError> {
        let name = string_to_c_u64_str(name)?;
        get_function_result_unit!(self.object, RemoveSharedFolder, name)
    }

    /// Returns true if the VM console process can activate the console window and bring it to foreground on the desktop of the host PC.
    ///
    /// # Returns
    ///
    /// Returns bool success, or a [`VboxError`] on failure.
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
    /// let can_show_console_window = machine.can_show_console_window().unwrap();
    pub fn can_show_console_window(&self) -> Result<bool, VboxError> {
        get_function_result_bool!(self.object, CanShowConsoleWindow)
    }

    /// Activates the console window and brings it to foreground on the desktop of the host PC.
    ///
    /// Many modern window managers on many platforms implement some sort of focus stealing prevention logic, so that it may be impossible to activate a window without the help of the currently active application. In this case, this method will return a non-zero identifier that represents the top-level window of the VM console process. The caller, if it represents a currently active process, is responsible to use this identifier (in a platform-dependent manner) to perform actual window activation.
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
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let win_id = machine.show_console_window().unwrap();
    pub fn show_console_window(&self) -> Result<i64, VboxError> {
        get_function_result_number!(self.object, ShowConsoleWindow, i64)
    }

    /// Reads an entry from the machine's guest property store.
    ///
    /// # Arguments
    ///
    /// * `name` -	&str. Reads an entry from the machine's guest property store.
    ///
    /// # Returns
    ///
    /// Returns (value, timestamp, flags) success, or a [`VboxError`] on failure.
    ///
    /// * `value` -	&str. The value of the property. If the property does not exist then this will be empty.
    /// * `timestamp` -	i64. The time at which the property was last modified, as seen by the server process.
    /// * `flags` -	&str. Additional property parameters, passed as a comma-separated list of "name=value" type entries.
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
    /// let guest_property = machine.get_guest_property("property1").unwrap();
    pub fn get_guest_property(
        &self,
        name: &str,
    ) -> Result<(&'static str, i64, &'static str), VboxError> {
        let name = string_to_c_u64_str(name)?;
        let mut value: *mut u16 = std::ptr::null_mut();
        let mut timestamp: i64 = 0;
        let mut flags: *mut u16 = std::ptr::null_mut();

        get_function_result_unit!(
            self.object,
            GetGuestProperty,
            name,
            &mut value,
            &mut timestamp,
            &mut flags
        )?;

        if value.is_null() || flags.is_null() {
            return Err(VboxError::null_pointer_error("Mashine::get_guest_property"));
        }
        let value_str = c_u64_str_to_string(value)?;
        let flags_str = c_u64_str_to_string(flags)?;
        Ok((value_str, timestamp, flags_str))
    }

    /// Reads a value from the machine's guest property store.
    ///
    /// # Arguments
    ///
    /// * `property` -	&str. The name of the property to read.
    ///
    /// # Returns
    ///
    /// Returns &str success, or a [`VboxError`] on failure.
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
    /// let guest_property_value = machine.get_guest_property_value("property1").unwrap();
    pub fn get_guest_property_value(&self, property: &str) -> Result<&'static str, VboxError> {
        let property = string_to_c_u64_str(property)?;
        get_function_result_str!(self.object, GetGuestPropertyValue, property)
    }

    /// Reads a property timestamp from the machine's guest property store.
    ///
    /// # Arguments
    ///
    /// * `property` -	&str. The name of the property to read.
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
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let guest_property_timestamp = machine.get_guest_property_timestamp("property1").unwrap();
    pub fn get_guest_property_timestamp(&self, property: &str) -> Result<i64, VboxError> {
        let property = string_to_c_u64_str(property)?;
        get_function_result_number!(self.object, GetGuestPropertyTimestamp, i64, property)
    }

    /// Sets, changes or deletes an entry in the machine's guest property store.
    ///
    /// # Arguments
    ///
    /// * `property` -	&str. The name of the property to set, change or delete.
    /// * `value` -	&str. The new value of the property to set, change or delete. If the property does not yet exist and value is non-empty, it will be created. If the value is empty, the property will be deleted if it exists.
    /// * `flags` -	&str. Additional property parameters, passed as a comma-separated list of "name=value" type entries.
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
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let mut session = Session::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    /// let machine_mut = session.get_machine().unwrap();
    ///
    /// machine_mut.set_guest_property("property", "value", "").unwrap();
    /// machine_mut.save_settings().unwrap();
    pub fn set_guest_property(
        &self,
        property: &str,
        value: &str,
        flags: &str,
    ) -> Result<(), VboxError> {
        let property = string_to_c_u64_str(property)?;
        let value = string_to_c_u64_str(value)?;
        let flags = string_to_c_u64_str(flags)?;
        get_function_result_unit!(self.object, SetGuestProperty, property, value, flags)
    }

    /// Sets or changes a value in the machine's guest property store.
    ///
    /// The flags field will be left unchanged or created empty for a new property
    ///
    /// # Arguments
    ///
    /// * `property` -	&str. The name of the property to set or change.
    /// * `value` -	&str. The new value of the property to set or change. If the property does not yet exist and value is non-empty, it will be created.
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
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let mut session = Session::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    /// let machine_mut = session.get_machine().unwrap();
    ///
    /// machine_mut.set_guest_property_value("property", "value1").unwrap();
    /// machine_mut.save_settings().unwrap();
    pub fn set_guest_property_value(&self, property: &str, value: &str) -> Result<(), VboxError> {
        let property = string_to_c_u64_str(property)?;
        let value = string_to_c_u64_str(value)?;
        get_function_result_unit!(self.object, SetGuestPropertyValue, property, value)
    }

    /// Deletes an entry from the machine's guest property store.
    ///
    /// # Arguments
    ///
    /// * `name` -	&str. The name of the property to set or change.
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
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let mut session = Session::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    /// let machine_mut = session.get_machine().unwrap();
    ///
    /// machine_mut.delete_guest_property("property").unwrap();
    /// machine_mut.save_settings().unwrap();
    pub fn delete_guest_property(&self, name: &str) -> Result<(), VboxError> {
        let name = string_to_c_u64_str(name)?;
        get_function_result_unit!(self.object, DeleteGuestProperty, name)
    }

    /// Return a list of the guest properties matching a set of patterns along with their values, timestamps and flags.
    ///
    /// # Arguments
    ///
    /// * `patterns` -	&str. The patterns to match the properties against, separated by '|' characters. If this is empty, all properties will match.
    ///
    /// # Returns
    ///
    /// Returns Vec<(name, value, timestamp, flags)> success, or a [`VboxError`] on failure.
    ///
    /// * `names` -	&str. The names of the properties returned.
    /// * `value` -	&str. The values of the properties returned.
    /// * `timestamp` -	i64. The timestamps of the properties returned.
    /// * `flags` -	&str. The flags of the properties returned.
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
    /// let guest_property = machine.enumerate_guest_properties("").unwrap();
    pub fn enumerate_guest_properties(
        &self,
        patterns: &str,
    ) -> Result<Vec<(&'static str, &'static str, i64, &'static str)>, VboxError> {
        let patterns = string_to_c_u64_str(patterns)?;
        let mut name_size = 0;
        let mut name: *mut *mut u16 = std::ptr::null_mut();
        let mut value_size = 0;
        let mut value: *mut *mut u16 = std::ptr::null_mut();
        let mut timestamp_size = 0;
        let mut timestamp: *mut i64 = std::ptr::null_mut();
        let mut flags_size = 0;
        let mut flags: *mut *mut u16 = std::ptr::null_mut();

        get_function_result_unit!(
            self.object,
            EnumerateGuestProperties,
            patterns,
            &mut name_size,
            &mut name,
            &mut value_size,
            &mut value,
            &mut timestamp_size,
            &mut timestamp,
            &mut flags_size,
            &mut flags
        )?;

        let mut result = Vec::new();
        if name.is_null() || value.is_null() || timestamp.is_null() || flags.is_null() {
            return Err(VboxError::null_pointer_error("Mashine::get_guest_property"));
        }
        let name_vec = unsafe { Vec::from_raw_parts(name, name_size as usize, name_size as usize) };
        let value_vec =
            unsafe { Vec::from_raw_parts(value, value_size as usize, value_size as usize) };
        let timestamp_vec = unsafe {
            Vec::from_raw_parts(timestamp, timestamp_size as usize, timestamp_size as usize)
        };
        let flags_vec =
            unsafe { Vec::from_raw_parts(flags, flags_size as usize, flags_size as usize) };

        let name_vec: Vec<&str> = name_vec
            .iter()
            .map(|s| c_u64_str_to_string(s.clone()).unwrap_or(""))
            .collect();

        let value_vec: Vec<&str> = value_vec
            .iter()
            .map(|s| c_u64_str_to_string(s.clone()).unwrap_or(""))
            .collect();

        let flags_vec: Vec<&str> = flags_vec
            .iter()
            .map(|s| c_u64_str_to_string(s.clone()).unwrap_or(""))
            .collect();

        for i in 0..name_size {
            result.push((
                name_vec.get(i as usize).unwrap_or(&"").to_owned(),
                value_vec.get(i as usize).unwrap_or(&"").to_owned(),
                timestamp_vec.get(i as usize).unwrap_or(&0).clone(),
                flags_vec.get(i as usize).unwrap_or(&"").to_owned(),
            ));
        }
        Ok(result)
    }

    /// Returns the guest dimensions from the saved state.
    ///
    /// # Arguments
    ///
    /// * `screen_id` -	&str. Saved guest screen to query info from.
    ///
    /// # Returns
    ///
    /// Returns (origin_x, origin_y, width, height, enabled) success, or a [`VboxError`] on failure.
    ///
    /// * `origin_x` -	u32. The X position of the guest monitor top left corner.
    /// * `origin_y` -	u32. The Y position of the guest monitor top left corner.
    /// * `width` -	u32. Guest width at the time of the saved state was taken.
    /// * `height` - u32. Guest height at the time of the saved state was taken.
    /// * `enabled` - bool. Guest height at the time of the saved state was taken.
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
    /// let saved_guest_screen_info = machine.query_saved_guest_screen_info(0).unwrap();
    pub fn query_saved_guest_screen_info(
        &self,
        screen_id: u32,
    ) -> Result<(u32, u32, u32, u32, bool), VboxError> {
        let mut origin_x: u32 = 0;
        let mut origin_y: u32 = 0;
        let mut width: u32 = 0;
        let mut height: u32 = 0;
        let mut enabled: i32 = 0;
        get_function_result_unit!(
            self.object,
            QuerySavedGuestScreenInfo,
            screen_id,
            &mut origin_x,
            &mut origin_y,
            &mut width,
            &mut height,
            &mut enabled
        )?;
        let enabled = enabled == 1;
        Ok((origin_x, origin_y, width, height, enabled))
    }

    /// Thumbnail is retrieved to an array of bytes in the requested format.
    ///
    /// # Arguments
    ///
    /// * `screen_id` -	&str. Saved guest screen to read from.
    /// * `bitmap_format` -	[`BitmapFormat`]. The requested format.
    ///
    /// # Returns
    ///
    /// Returns (width, height, data) success, or a [`VboxError`] on failure.
    ///
    /// * `width` -	u32. Bitmap width.
    /// * `height` - u32. Bitmap height.
    /// * `data` -	`&[u8]`. Array with resulting bitmap data.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use std::fs::File;
    /// use std::io::Write;
    /// use virtualbox_rs::enums::BitmapFormat;
    /// use virtualbox_rs::VirtualBox;
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let saved_thumbnail  = machine.read_saved_thumbnail_to_array(0, BitmapFormat::PNG).unwrap();
    /// let mut file = File::create("output.png").unwrap();
    /// file.write_all(saved_thumbnail.2).unwrap();
    pub fn read_saved_thumbnail_to_array(
        &self,
        screen_id: u32,
        bitmap_format: BitmapFormat,
    ) -> Result<(u32, u32, &[u8]), VboxError> {
        let bitmap_format = bitmap_format.into();
        let mut width: u32 = 0;
        let mut height: u32 = 0;
        let mut data_size: u32 = 0;
        let mut data: *mut u8 = std::ptr::null_mut();

        get_function_result_unit!(
            self.object,
            ReadSavedThumbnailToArray,
            screen_id,
            bitmap_format,
            &mut width,
            &mut height,
            &mut data_size,
            &mut data
        )?;
        let data_slice = unsafe { slice::from_raw_parts(data, data_size as usize) };

        Ok((width, height, data_slice))
    }

    /// Returns available formats and size of the screenshot from saved state.
    ///
    /// # Arguments
    ///
    /// * `screen_id` -	&str. Saved guest screen to query info from.
    ///
    /// # Returns
    ///
    /// Returns (width, height, data) success, or a [`VboxError`] on failure.
    ///
    /// * `width` -	u32. Bitmap width.
    /// * `height` - u32. Bitmap height.
    /// * `bitmap_formats` - `Vec<BitmapFormat>`. Formats supported by readSavedScreenshotToArray.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use std::fs::File;
    /// use std::io::Write;
    /// use virtualbox_rs::enums::BitmapFormat;
    /// use virtualbox_rs::VirtualBox;
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let saved_screenshot_info = machine.query_saved_screenshot_info(0).unwrap();
    pub fn query_saved_screenshot_info(
        &self,
        screen_id: u32,
    ) -> Result<(u32, u32, Vec<BitmapFormat>), VboxError> {
        let mut width: u32 = 0;
        let mut height: u32 = 0;
        let mut bitmap_formats_size: u32 = 0;
        let mut bitmap_formats: *mut u32 = std::ptr::null_mut();

        get_function_result_unit!(
            self.object,
            QuerySavedScreenshotInfo,
            screen_id,
            &mut width,
            &mut height,
            &mut bitmap_formats_size,
            &mut bitmap_formats
        )?;
        let bitmap_formats =
            unsafe { slice::from_raw_parts(bitmap_formats, bitmap_formats_size as usize) };
        let bitmap_formats = bitmap_formats
            .iter()
            .map(|f| BitmapFormat::from(f.clone()))
            .collect();
        Ok((width, height, bitmap_formats))
    }

    /// Screenshot in requested format is retrieved to an array of bytes.
    ///
    /// # Arguments
    ///
    /// * `screen_id` -	&str. Saved guest screen to read from.
    /// * `bitmap_format` -	[`BitmapFormat`]. The requested format.
    ///
    /// # Returns
    ///
    /// Returns (width, height, data) success, or a [`VboxError`] on failure.
    ///
    /// * `width` -	u32. Bitmap width.
    /// * `height` - u32. Bitmap height.
    /// * `data` -	`&[u8]`. Array with resulting bitmap data.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use std::fs::File;
    /// use std::io::Write;
    /// use virtualbox_rs::enums::BitmapFormat;
    /// use virtualbox_rs::VirtualBox;
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let saved_screenshot  = machine.read_saved_screenshot_to_array(0, BitmapFormat::PNG).unwrap();
    /// let mut file = File::create("output.png").unwrap();
    /// file.write_all(saved_screenshot.2).unwrap();
    pub fn read_saved_screenshot_to_array(
        &self,
        screen_id: u32,
        bitmap_format: BitmapFormat,
    ) -> Result<(u32, u32, &[u8]), VboxError> {
        let bitmap_format = bitmap_format.into();
        let mut width: u32 = 0;
        let mut height: u32 = 0;
        let mut data_size: u32 = 0;
        let mut data: *mut u8 = std::ptr::null_mut();

        get_function_result_unit!(
            self.object,
            ReadSavedScreenshotToArray,
            screen_id,
            bitmap_format,
            &mut width,
            &mut height,
            &mut data_size,
            &mut data
        )?;
        let data_slice = unsafe { slice::from_raw_parts(data, data_size as usize) };

        Ok((width, height, data_slice))
    }

    /// Plugs a CPU into the machine.
    ///
    /// # Arguments
    ///
    /// * `cpu` - u32. The CPU id to insert.
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
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let mut session = Session::init().unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    ///
    /// let machine_mut = session.get_machine().unwrap();
    /// machine_mut.hot_plug_cpu(1).unwrap();
    pub fn hot_plug_cpu(&self, cpu: u32) -> Result<(), VboxError> {
        get_function_result_unit!(self.object, HotPlugCPU, cpu)
    }

    /// Removes a CPU from the machine.
    ///
    /// # Arguments
    ///
    /// * `cpu` - u32. The CPU id to remove.
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
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let mut session = Session::init().unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    ///
    /// let machine_mut = session.get_machine().unwrap();
    /// machine_mut.hot_unplug_cpu(1).unwrap();
    pub fn hot_unplug_cpu(&self, cpu: u32) -> Result<(), VboxError> {
        get_function_result_unit!(self.object, HotUnplugCPU, cpu)
    }

    /// Returns the current status of the given CPU.
    ///
    /// # Arguments
    ///
    /// * `cpu` - u32. The CPU id to check for.
    ///
    /// # Returns
    ///
    /// Returns bool success, or a [`VboxError`] on failure.
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
    /// let cpu_status = machine.get_cpu_status(1).unwrap();
    pub fn get_cpu_status(&self, cpu: u32) -> Result<bool, VboxError> {
        get_function_result_bool!(self.object, GetCPUStatus, cpu)
    }

    /// Returns the effective paravirtualization provider for this VM.
    ///
    /// # Returns
    ///
    /// Returns u32 success, or a [`VboxError`] on failure.
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
    /// let paravirt_provider = machine.get_effective_paravirt_provider().unwrap();
    pub fn get_effective_paravirt_provider(&self) -> Result<ParavirtProvider, VboxError> {
        let paravirt_provider =
            get_function_result_number!(self.object, GetEffectiveParavirtProvider, u32)?;
        Ok(ParavirtProvider::from(paravirt_provider))
    }

    /// Queries for the VM log file name of an given index.
    ///
    /// Returns an empty string if a log file with that index doesn't exists.
    ///
    /// # Arguments
    ///
    /// * `idx` - u32. Which log file name to query. 0=current log file.
    ///
    /// # Returns
    ///
    /// Returns u32 success, or a [`VboxError`] on failure.
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
    /// let log_filename = machine.query_log_filename(0).unwrap();
    pub fn query_log_filename(&self, idx: u32) -> Result<&'static str, VboxError> {
        get_function_result_str!(self.object, QueryLogFilename, idx)
    }

    /// Reads the VM log file.
    ///
    /// The chunk size is limited, so even if you ask for a big piece there might be less data returned.
    ///
    /// # Arguments
    ///
    /// * `idx` -	u32. Which log file to read. 0=current log file.
    /// * `offset` -	i64. Offset in the log file.
    /// * `size` -	i64. Chunk size to read in the log file.
    ///
    /// # Returns
    ///
    /// Returns &[u8] success, or a [`VboxError`] on failure.
    ///
    /// Data read from the log file. A data size of 0 means end of file if the requested chunk size was not 0. This is the unprocessed file data, i.e. the line ending style depends on the platform of the system the server is running on.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use std::io::read_to_string;
    /// use virtualbox_rs::VirtualBox;
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let log  = machine.read_log(0, 0, 1024).unwrap();
    /// let log_str = read_to_string(log).unwrap();
    pub fn read_log(&self, idx: u32, offset: i64, size: i64) -> Result<&[u8], VboxError> {
        let mut data_size: u32 = 0;
        let mut data: *mut u8 = std::ptr::null_mut();

        get_function_result_unit!(
            self.object,
            ReadLog,
            idx,
            offset,
            size,
            &mut data_size,
            &mut data
        )?;
        let data_slice = unsafe { slice::from_raw_parts(data, data_size as usize) };

        Ok(data_slice)
    }

    /// Creates a clone of this machine, either as a full clone (which means creating independent copies of the hard disk media, save states and so on), or as a linked clone (which uses its own differencing media, sharing the parent media with the source machine).
    ///
    /// The target machine object must have been created previously with [`VirtualBox::create_machine`], and all the settings will be transferred except the VM name and the hardware UUID. You can set the VM name and the new hardware UUID when creating the target machine. The network MAC addresses are newly created for all enabled network adapters. You can change that behaviour with the options parameter. The operation is performed asynchronously, so the machine object will be not be usable until the progress object signals completion.
    /// # Arguments
    ///
    /// * `target` - [`Machine`]. Target machine object.
    /// * `mode` -	[`CloneMode`]. Which states should be cloned.
    /// * `options` - [`Vec<CloneOptions>`]. Options for the cloning operation.
    ///
    /// # Returns
    ///
    /// Returns &[u8] success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::enums::{CloneMode, PlatformArchitecture};
    /// use virtualbox_rs::VirtualBox;
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let name = format!("{}_clone", machine.get_name().unwrap());
    /// let os_type = machine.get_os_type_id().unwrap();
    /// let machine_clone = vbox
    ///         .create_machine("", name.as_str(), PlatformArchitecture::X86, vec![], os_type, "", "", "", "")
    ///         .unwrap();
    /// machine_clone.save_settings().unwrap();
    /// let progress = machine
    ///         .clone_to(&machine_clone, CloneMode::AllStates, vec![])
    ///         .unwrap();
    /// progress.wait_for_completion(-1).unwrap();
    /// vbox.register_machine(&machine_clone).unwrap();
    pub fn clone_to(
        &self,
        target: &Machine,
        mode: CloneMode,
        options: Vec<CloneOptions>,
    ) -> Result<Progress, VboxError> {
        let target = target.object;
        let mode = mode.into();
        let mut options: Vec<u32> = options.iter().map(|o| (*o).into()).collect();
        let options_ptr = options.as_mut_ptr();
        let options_size = options.len() as u32;
        let progress = get_function_result_pointer!(
            self.object,
            CloneTo,
            *mut IProgress,
            target,
            mode,
            options_size,
            options_ptr
        )?;
        Ok(Progress::new(progress))
    }

    /// Creates a clone of this machine, either as a full clone (which means creating independent copies of the hard disk media, save states and so on), or as a linked clone (which uses its own differencing media, sharing the parent media with the source machine).
    ///
    /// The target machine object must have been created previously with [`VirtualBox::create_machine`], and all the settings will be transferred except the VM name and the hardware UUID. You can set the VM name and the new hardware UUID when creating the target machine. The network MAC addresses are newly created for all enabled network adapters. You can change that behaviour with the options parameter. The operation is performed asynchronously, so the machine object will be not be usable until the progress object signals completion.
    ///
    /// # Arguments
    ///
    /// * `target` - [`Machine`]. Target machine object.
    /// * `mode` -	[`CloneMode`]. Which states should be cloned.
    /// * `options` - [`Vec<CloneOptions>`]. Options for the cloning operation.
    ///
    /// # Returns
    ///
    /// Returns &[u8] success, or a [`VboxError`] on failure.
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
    /// machine
    ///     .lock_machine(&mut session, SessionType::Shared)
    ///     .unwrap();
    /// let machine_mut = session.get_machine().unwrap();
    /// let progress = machine_mut.move_to("", "").unwrap();
    /// progress.wait_for_completion(-1).unwrap();

    pub fn move_to(&self, folder: &str, type_: &str) -> Result<Progress, VboxError> {
        let folder = string_to_c_u64_str(folder)?;
        let type_ = string_to_c_u64_str(type_)?;
        let progress =
            get_function_result_pointer!(self.object, MoveTo, *mut IProgress, folder, type_)?;
        Ok(Progress::new(progress))
    }

    /// Saves the current execution state of a running virtual machine and stops its execution.
    ///
    /// # Returns
    ///
    /// Returns a new instance of the [`Progress`] class on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{Session, VirtualBox};
    /// use virtualbox_rs::enums::SessionType;
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let mut session = Session::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    ///
    /// let machine_mut = session.get_machine().unwrap();
    ///
    /// let progress = machine_mut.save_state().unwrap();

    pub fn save_state(&self) -> Result<Progress, VboxError> {
        let progress = get_function_result_pointer!(self.object, SaveState, *mut IProgress)?;
        Ok(Progress::new(progress))
    }

    /// Associates the given saved state file to the virtual machine.
    ///
    /// On success, the machine will go to the Saved state. The next time it is powered up it will be restored from the adopted saved state and continue execution from the place where the saved state file was created.
    ///
    /// The specified saved state file path may be absolute or relative to the folder the VM normally saves the state to (usually, [`Machine::get_snapshot_folder`]).
    ///
    /// # Arguments
    ///
    /// * `saved_state_file` - &str. Path to the saved state file to adopt.
    ///
    /// # Returns
    ///
    /// Returns a &str class on success, or a [`VboxError`] on failure.
    ///
    /// <div class="warning">
    ///  It's a caller's responsibility to make sure the given saved state file is compatible with the settings of this virtual machine that represent its virtual hardware (memory size, storage disk configuration etc.). If there is a mismatch, the behavior of the virtual machine is undefined.
    /// </div>
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{Session, VirtualBox};
    /// use virtualbox_rs::enums::SessionType;
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let mut session = Session::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    ///
    /// let machine_mut = session.get_machine().unwrap();
    ///
    /// machine_mut.adopt_saved_state("123/").unwrap();

    pub fn adopt_saved_state(&self, saved_state_file: &str) -> Result<(), VboxError> {
        let saved_state_file = string_to_c_u64_str(saved_state_file)?;
        get_function_result_unit!(self.object, AdoptSavedState, saved_state_file)
    }

    /// Forcibly resets the machine to "Powered Off" state if it is currently in the [`MachineState::Saved`] state previously created by [`Machine::save_state`]) or in the "AbortedSaved" state.
    ///
    /// The next time the machine is powered up a clean boot will occur.
    ///
    /// If f_remove_file is true, the file in the machine directory into which the machine state was saved is also deleted. If this is false, then the state can be recovered and later re-inserted into a machine using [`Machine::adopt_saved_state`]. The location of the file can be found in the [`Machine::get_state_file_path`] attribute.
    ///
    /// # Arguments
    ///
    /// * `saved_state_file` - &str. Whether to also remove the saved state file.
    ///
    /// # Returns
    ///
    /// Returns a &str class on success, or a [`VboxError`] on failure.
    ///
    /// <div class="warning">
    ///  This operation is equivalent to resetting or powering off the machine without doing a proper shutdown of the guest operating system; as with resetting a running phyiscal computer, it can can lead to data loss.
    /// </div>
    ///
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{Session, VirtualBox};
    /// use virtualbox_rs::enums::SessionType;
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let mut session = Session::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    ///
    /// let machine_mut = session.get_machine().unwrap();
    ///
    /// machine_mut.discard_saved_state(true).unwrap();
    pub fn discard_saved_state(&self, f_remove_file: bool) -> Result<(), VboxError> {
        let f_remove_file = if f_remove_file { 1 } else { 0 };
        get_function_result_unit!(self.object, DiscardSavedState, f_remove_file)
    }

    /// Saves the current execution state and all settings of the machine and creates differencing images for all normal (non-independent) media.
    ///
    /// See [`Snapshot`] for an introduction to snapshots.
    ///
    /// This method can be called for a PoweredOff, Saved (see [`Machine::save_state`]), AbortedSaved, Running or Paused virtual machine. When the machine is PoweredOff, an offline snapshot is created. When the machine is Running a live snapshot is created, and an online snapshot is created when Paused.
    ///
    /// The taken snapshot is always based on the current snapshot of the associated virtual machine and becomes a new current snapshot.
    ///
    /// # Arguments
    ///
    /// * `name` - &str. Short name for the snapshot.
    /// * `description` - &str. Optional description of the snapshot.
    /// * `pause` - bool. Whether the VM should be paused while taking the snapshot. Only relevant when the VM is running, and distinguishes between online (true) and live (false) snapshots. When the VM is not running the result is always an offline snapshot.
    ///
    /// # Returns
    ///
    /// Returns a (id, progress) class on success, or a [`VboxError`] on failure.
    ///
    /// * `id` - &str. UUID of the snapshot which will be created. Useful for follow-up operations after the snapshot has been created.
    /// * `progress` - &str. Progress object to track the operation completion.
    ///
    /// <div class="warning">
    /// If this method is called on an immutable machine instance, it will result in an NS_ERROR_NOT_IMPLEMENTED error.
    /// This seems to be an issue in the SDK.
    /// </div>
    ///
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{Session, VirtualBox};
    /// use virtualbox_rs::enums::SessionType;
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let mut session = Session::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    ///
    /// let machine_mut = session.get_machine().unwrap();
    ///
    /// let (id, progress) = machine_mut.take_snapshot("snapshot1", "", true).unwrap();
    /// progress.wait_for_completion(-1).unwrap()
    pub fn take_snapshot(
        &self,
        name: &str,
        description: &str,
        pause: bool,
    ) -> Result<(&'static str, Progress), VboxError> {
        let name = string_to_c_u64_str(name)?;
        let description = string_to_c_u64_str(description)?;
        let pause = if pause { 1 } else { 0 };
        let mut id: *mut PRUnichar = std::ptr::null_mut();

        let progress = get_function_result_pointer!(
            self.object,
            TakeSnapshot,
            *mut IProgress,
            name,
            description,
            pause,
            &mut id
        )?;
        let progress = Progress::new(progress);
        let id = c_u64_str_to_string(id)?;
        Ok((id, progress))
    }

    /// Starts deleting the specified snapshot asynchronously.
    ///
    /// See [`Snapshot`] for an introduction to snapshots.
    ///
    /// **Reference to the official documentation:**
    ///
    /// [https://www.virtualbox.org/sdkref/interface_i_machine.html](https://www.virtualbox.org/sdkref/interface_i_machine.html#a4d541f845c2744f78315d146cb86084c)
    ///
    /// # Arguments
    ///
    /// * `id` - &str. UUID of the snapshot to delete.
    ///
    /// # Returns
    ///
    /// Returns [`Progress`] class on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{Session, VirtualBox};
    /// use virtualbox_rs::enums::SessionType;
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let mut session = Session::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    ///
    /// let machine_mut = session.get_machine().unwrap();
    ///
    /// let progress = machine_mut.delete_snapshot("278ef54a-2e75-4aba-b212-551af4c69725").unwrap();
    /// progress.wait_for_completion(-1).unwrap()
    pub fn delete_snapshot(&self, id: &str) -> Result<Progress, VboxError> {
        let id = string_to_c_u64_str(id)?;
        let progress =
            get_function_result_pointer!(self.object, DeleteSnapshot, *mut IProgress, id)?;
        Ok(Progress::new(progress))
    }

    /// Starts deleting the specified snapshot and all its children asynchronously.
    ///
    /// See [`Snapshot`] for an introduction to snapshots. The conditions and many details are the same as with [`Machine::delete_snapshot`].
    ///
    /// This operation is very fast if the snapshot subtree does not include the current state. It is still significantly faster than deleting the snapshots one by one if the current state is in the subtree and there are more than one snapshots from current state to the snapshot which marks the subtree, since it eliminates the incremental image merging.
    ///
    /// # Arguments
    ///
    /// * `id` - &str. UUID of the snapshot to delete, including all its children.
    ///
    /// # Returns
    ///
    /// Returns [`Progress`] class on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{Session, VirtualBox};
    /// use virtualbox_rs::enums::SessionType;
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let mut session = Session::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    ///
    /// let machine_mut = session.get_machine().unwrap();
    ///
    /// let progress = machine_mut.delete_snapshot_and_all_children("278ef54a-2e75-4aba-b212-551af4c69725").unwrap();
    /// progress.wait_for_completion(-1).unwrap()
    pub fn delete_snapshot_and_all_children(&self, id: &str) -> Result<Progress, VboxError> {
        let id = string_to_c_u64_str(id)?;
        let progress = get_function_result_pointer!(
            self.object,
            DeleteSnapshotAndAllChildren,
            *mut IProgress,
            id
        )?;
        Ok(Progress::new(progress))
    }

    /// Starts deleting the specified snapshot range.
    ///
    /// This is limited to linear snapshot lists, which means there may not be any other child snapshots other than the direct sequence between the start and end snapshot. If the start and end snapshot point to the same snapshot this method is completely equivalent to [`Machine::delete_snapshot`]. See [`Snapshot`] for an introduction to snapshots. The conditions and many details are the same as with [`Machine::delete_snapshot`].
    ///
    /// This operation is generally faster than deleting snapshots one by one and often also needs less extra disk space before freeing up disk space by deleting the removed disk images corresponding to the snapshot.
    ///
    /// # Arguments
    ///
    /// * `start_id` - &str. UUID of the first snapshot to delete.
    /// * `end_id` - &str. UUID of the last snapshot to delete.
    ///
    /// # Returns
    ///
    /// Returns [`Progress`] class on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{Session, VirtualBox};
    /// use virtualbox_rs::enums::SessionType;
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let mut session = Session::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    ///
    /// let machine_mut = session.get_machine().unwrap();
    ///
    /// let progress = machine_mut.delete_snapshot_range(
    ///     "278ef54a-2e75-4aba-b212-551af4c69725",
    ///     "c9f5d71f-1a8b-42b4-bb4d-abc9d71b9dcd").unwrap();
    /// progress.wait_for_completion(-1).unwrap()
    pub fn delete_snapshot_range(
        &self,
        start_id: &str,
        end_id: &str,
    ) -> Result<Progress, VboxError> {
        let start_id = string_to_c_u64_str(start_id)?;
        let end_id = string_to_c_u64_str(end_id)?;
        let progress = get_function_result_pointer!(
            self.object,
            DeleteSnapshotRange,
            *mut IProgress,
            start_id,
            end_id
        )?;
        Ok(Progress::new(progress))
    }

    /// Starts resetting the machine's current state to the state contained in the given snapshot, asynchronously.
    ///
    /// All current settings of the machine will be reset and changes stored in differencing media will be lost. See [`Snapshot`] for an introduction to snapshots.
    ///
    /// After this operation is successfully completed, new empty differencing media are created for all normal media of the machine.
    ///
    /// If the given snapshot is an online snapshot, the machine will go to the [`MachineState::Saved`] state, so that the next time it is powered on, the execution state will be restored from the state of the snapshot.
    ///
    /// # Arguments
    ///
    /// * `snapshot` - [`Snapshot`]. The snapshot to restore the VM state from.
    ///
    /// # Returns
    ///
    /// Returns [`Progress`] class on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{Session, VirtualBox};
    /// use virtualbox_rs::enums::SessionType;
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let mut session = Session::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    ///
    /// let machine_mut = session.get_machine().unwrap();
    /// let snapshot = machine_mut.get_current_snapshot().unwrap().unwrap();
    /// let progress = machine_mut.restore_snapshot(&snapshot).unwrap();
    /// progress.wait_for_completion(-1).unwrap()
    pub fn restore_snapshot(&self, snapshot: &Snapshot) -> Result<Progress, VboxError> {
        let snapshot = snapshot.object;
        let progress =
            get_function_result_pointer!(self.object, RestoreSnapshot, *mut IProgress, snapshot)?;
        Ok(Progress::new(progress))
    }

    /// Starts resetting the machine's current state to the state contained in the given snapshot, asynchronously.
    ///
    /// All current settings of the machine will be reset and changes stored in differencing media will be lost. See [`Snapshot`] for an introduction to snapshots.
    ///
    /// After this operation is successfully completed, new empty differencing media are created for all normal media of the machine.
    ///
    /// If the given snapshot is an online snapshot, the machine will go to the [`MachineState::Saved`] state, so that the next time it is powered on, the execution state will be restored from the state of the snapshot.
    ///
    /// # Arguments
    ///
    /// * `snapshot` - [`Snapshot`]. The snapshot to restore the VM state from.
    ///
    /// # Returns
    ///
    /// Returns [`Progress`] class on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{VirtualBox};
    /// use virtualbox_rs::enums::PlatformArchitecture;
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let machine = vbox
    ///         .create_machine("", "Ubuntu", PlatformArchitecture::X86, vec![], "Ubuntu (64-bit)", "", "", "", "")
    ///         .unwrap();
    ///  machine.apply_defaults("").unwrap();
    pub fn apply_defaults(&self, flags: &str) -> Result<(), VboxError> {
        let flags = string_to_c_u64_str(flags)?;
        get_function_result_unit!(self.object, ApplyDefaults, flags)
    }

    /// Returns the array of device type boot order.
    ///
    ///
    /// # Returns
    ///
    /// Returns [`Vec<DeviceType>`] success, or a [`VboxError`] on failure.
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
    /// let boot_type = machine.get_boot_order_array().unwrap();
    ///
    /// println!("{:?}", boot_type);
    ///
    pub fn get_boot_order_array(&self) -> Result<Vec<DeviceType>, VboxError> {
        let system_properties = SystemProperties::init()?;
        let boot_count = system_properties.get_max_boot_position()?;
        let mut boot_vec = Vec::new();
        for i in 1..boot_count {
            let boot = self.get_boot_order(i)?;
            boot_vec.push(boot);
        }
        Ok(boot_vec)
    }

    /// Returns the network adapter associated with the given slot.
    ///
    //TODO
    /// Slots are numbered sequentially, starting with zero. The total number of adapters per machine is defined by the ISystemProperties::getMaxNetworkAdapters property, so the maximum slot number is one less than that property's value.
    ///
    /// # Returns
    ///
    /// Returns [`NetworkAdapter`] on success, or a [`VboxError`] on failure.
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
    ///
    pub fn get_network_adapter(&self, slot: u32) -> Result<NetworkAdapter, VboxError> {
        let network_adapter_ptr = get_function_result_pointer!(
            self.object,
            GetNetworkAdapter,
            *mut INetworkAdapter,
            slot
        )?;
        Ok(NetworkAdapter::new(network_adapter_ptr))
    }

    #[cfg(not(is_v_6_1))]
    /// Object containing all TPM settings.
    ///
    /// # Returns
    ///
    /// Returns [`TrustedPlatformModule`] success, or a [`VboxError`] on failure.
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
    /// let trusted_platform_module = machine.get_trusted_platform_module().unwrap();

    pub fn get_trusted_platform_module(&self) -> Result<TrustedPlatformModule, VboxError> {
        let trusted_platform_module = get_function_result_pointer!(
            self.object,
            GetTrustedPlatformModule,
            *mut ITrustedPlatformModule
        )?;
        Ok(TrustedPlatformModule::new(trusted_platform_module))
    }

    #[cfg(not(is_v_6_1))]
    /// Object to manipulate data in the non volatile storage file.
    ///
    /// # Returns
    ///
    /// Returns [`NvramStore`] success, or a [`VboxError`] on failure.
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
    /// let store = machine.get_non_volatile_store().unwrap();

    pub fn get_non_volatile_store(&self) -> Result<NvramStore, VboxError> {
        let store =
            get_function_result_pointer!(self.object, GetNonVolatileStore, *mut INvramStore)?;
        Ok(NvramStore::new(store))
    }

    #[cfg(is_v_7_1)]
    /// IOMMU type used in this VM.
    ///
    /// # Returns
    ///
    /// Returns [`IommuType`] success, or a [`VboxError`] on failure.
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
    /// let iommu_type = machine.get_iommu_type().unwrap();

    pub fn get_iommu_type(&self) -> Result<IommuType, VboxError> {
        self.get_platform()?.get_iommu_type()
    }

    #[cfg(is_v_7_1)]
    /// IOMMU type used in this VM.
    ///
    /// # Arguments
    ///
    /// * `iommu_type` - [`IommuType`].
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
    /// use virtualbox_rs::enums::{IommuType, SessionType};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let mut session = Session::init().unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    /// let machine_mut = session.get_machine().unwrap();
    /// machine_mut.set_iommu_type(IommuType::Intel).unwrap();
    /// machine_mut.save_settings().unwrap();

    pub fn set_iommu_type(&self, iommu_type: IommuType) -> Result<(), VboxError> {
        self.get_platform()?.set_iommu_type(iommu_type)
    }

    #[cfg(not(is_v_6_1))]
    /// Associated audio adapter, always present.
    ///
    /// # Returns
    ///
    /// Returns [`AudioAdapter`] on success, or a [`VboxError`] on failure.
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
    /// let adapter = machine.get_audio_adapter().unwrap();
    ///
    pub fn get_audio_adapter(&self) -> Result<AudioAdapter, VboxError> {
        let audio_settings = self.get_audio_settings()?;
        audio_settings.get_adapter()
    }

    /// The machine's audio settings.
    ///
    /// # Returns
    ///
    /// Returns [`AudioSettings`] on success, or a [`VboxError`] on failure.
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
    /// let audio_settings = machine.get_audio_settings().unwrap();
    ///
    #[cfg(not(is_v_6_1))]
    pub fn get_audio_settings(&self) -> Result<AudioSettings, VboxError> {
        let audio_settings_ptr =
            get_function_result_pointer!(self.object, GetAudioSettings, *mut IAudioSettings)?;
        Ok(AudioSettings::new(audio_settings_ptr))
    }

    /// Guest debugging configuration.
    ///
    /// # Returns
    ///
    /// Returns [`GuestDebugControl`] on success, or a [`VboxError`] on failure.
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
    /// let guest_debug_control = machine.get_guest_debug_control().unwrap();
    ///
    #[cfg(not(is_v_6_1))]
    pub fn get_guest_debug_control(&self) -> Result<GuestDebugControl, VboxError> {
        let guest_debug_control = get_function_result_pointer!(
            self.object,
            GetGuestDebugControl,
            *mut IGuestDebugControl
        )?;
        Ok(GuestDebugControl::new(guest_debug_control))
    }

    #[cfg(not(is_v_6_1))]
    /// Key Id of the password used for encrypting the state file.
    ///
    /// Internal use only for now.
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
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let state_key_id = machine.get_state_key_id().unwrap();

    pub fn get_state_key_id(&self) -> Result<&'static str, VboxError> {
        get_function_result_str!(self.object, GetStateKeyId)
    }

    #[cfg(not(is_v_6_1))]
    /// Key store used for encrypting the state file.
    ///
    /// Internal use only for now.
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
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let state_key_store = machine.get_state_key_store().unwrap();

    pub fn get_state_key_store(&self) -> Result<&'static str, VboxError> {
        get_function_result_str!(self.object, GetStateKeyStore)
    }

    #[cfg(not(is_v_6_1))]
    /// Key Id of the password used for encrypting the log files.
    ///
    /// Internal use only for now.
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
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let log_key_id = machine.get_log_key_id().unwrap();

    pub fn get_log_key_id(&self) -> Result<&'static str, VboxError> {
        get_function_result_str!(self.object, GetLogKeyId)
    }

    #[cfg(not(is_v_6_1))]
    /// Key store used for encrypting the log files.
    ///
    /// Internal use only for now.
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
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let log_key_id = machine.get_log_key_id().unwrap();

    pub fn get_log_key_store(&self) -> Result<&'static str, VboxError> {
        get_function_result_str!(self.object, GetLogKeyStore)
    }

    #[cfg(not(is_v_6_1))]
    /// Starts encryption of this VM.
    ///
    /// This means that the stored data of the VM is encrypted.
    ///
    /// Please note that the results can be either returned straight away, or later as the result of the background operation via the object returned via the progress parameter.
    ///
    /// # Arguments
    ///
    /// * `current_password` - &str. The current password the VM is protected with. Use an empty string to indicate that the VM isn't encrypted.
    /// * `cipher` - &str. The cipher to use for encryption. An empty string indicates no encryption for the result.
    /// * `new_password` - &str. The new password the VM should be protected with. An empty password and password ID will result in the VM being encrypted with the current password.
    /// * `new_password_id` - &str. The ID of the new password when unlocking the VM.
    /// * `force` - bool. Force reencryption of the data if just password is changed. Otherwise, if data already encrypted and cipher doesn't change only the password is changed.
    ///
    /// # Returns
    ///
    /// Returns a [`Progress`] object to track the operation completion, or a [`VboxError`] on failure.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use virtualbox_rs::{Session, VirtualBox};
    /// use virtualbox_rs::enums::SessionType;
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let machine = vbox.find_machines("Freebsd_14").unwrap();
    /// let mut session = Session::init().unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    ///
    /// let machine_mut = session.get_machine().unwrap();
    /// let progress = machine_mut.change_encryption(
    ///     "current_password",
    ///     "AES-XTS256-PLAIN64",
    ///     "new_password",
    ///     "new_password_id",
    ///     true).unwrap();
    /// progress.wait_for_completion(-1).unwrap();
    /// ```
    pub fn change_encryption(
        &self,
        current_password: &str,
        cipher: &str,
        new_password: &str,
        new_password_id: &str,
        force: bool,
    ) -> Result<Progress, VboxError> {
        let current_password = string_to_c_u64_str(current_password)?;
        let cipher = string_to_c_u64_str(cipher)?;
        let new_password = string_to_c_u64_str(new_password)?;
        let new_password_id = string_to_c_u64_str(new_password_id)?;
        let force = if force { 1 } else { 0 };
        let progress = get_function_result_pointer!(
            self.object,
            ChangeEncryption,
            *mut IProgress,
            current_password,
            cipher,
            new_password,
            new_password_id,
            force
        )?;
        Ok(Progress::new(progress))
    }

    #[cfg(not(is_v_6_1))]
    /// Returns the encryption settings for this VM.
    ///
    /// # Returns
    ///
    /// Returns a tuple containing:
    /// * `cipher` - The cipher used for encryption.
    /// * `passwordId` - The ID of the password when unlocking the VM.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use virtualbox_rs::{Session, VirtualBox};
    /// use virtualbox_rs::enums::SessionType;
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let mut session = Session::init().unwrap();
    /// let machine = vbox.find_machines("Freebsd_14").unwrap();
    ///     machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    ///
    /// let machine_mut = session.get_machine().unwrap();
    /// let (cipher, password_id) = machine_mut.get_encryption_settings().unwrap();
    /// ```
    pub fn get_encryption_settings(&self) -> Result<(&'static str, &'static str), VboxError> {
        let mut cipher: *mut u16 = std::ptr::null_mut();
        let password_id =
            get_function_result_str!(self.object, GetEncryptionSettings, &mut cipher)?;
        let cipher = c_u64_str_to_string(cipher)?;
        Ok((cipher, password_id))
    }

    #[cfg(not(is_v_6_1))]
    /// Checks whether the supplied password is correct for the VM.
    ///
    /// # Arguments
    /// * `password` - &str. The password to check.
    ///
    /// # Returns
    ///
    /// Returns () success, or a [`VboxError`] on failure.
    ///
    /// # Expected result codes:
    ///
    /// * `VBOX_E_NOT_SUPPORTED` - Encryption is not configured for this VM.
    /// * `VBOX_E_PASSWORD_INCORRECT` - The given password is incorrect.
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
    /// machine.check_encryption_password("111").unwrap();
    /// ```
    pub fn check_encryption_password(&self, password: &str) -> Result<(), VboxError> {
        let password = string_to_c_u64_str(password)?;
        get_function_result_unit!(self.object, CheckEncryptionPassword, password)
    }

    #[cfg(not(is_v_6_1))]
    /// Adds a password used for encryption.
    ///
    /// # Arguments
    /// * `id` - &str. The identifier used for the password. Must match the identifier used when the encrypted VM was created.
    /// * `password` - &str. The password.
    ///
    /// # Returns
    ///
    /// Returns () success, or a [`VboxError`] on failure.
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
    /// machine.add_encryption_password("Freebsd_14", "111").unwrap();
    /// ```
    pub fn add_encryption_password(&self, id: &str, password: &str) -> Result<(), VboxError> {
        let id = string_to_c_u64_str(id)?;
        let password = string_to_c_u64_str(password)?;
        get_function_result_unit!(self.object, AddEncryptionPassword, id, password)
    }

    #[cfg(not(is_v_6_1))]
    /// Adds passwords used for encryption.
    ///
    /// Updates the accesibility state if the list contains password used the VM encryption.
    ///
    /// # Arguments
    /// * `id` - &str. List of identifiers for the passwords. Must match the identifier used when the encrypted VM was created.
    /// * `password` - &str. List of passwords.
    ///
    /// # Returns
    ///
    /// Returns () success, or a [`VboxError`] on failure.
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
    /// machine.add_encryption_passwords(vec!["Freebsd_14"], vec!["111"]).unwrap();
    /// ```
    pub fn add_encryption_passwords(
        &self,
        id: Vec<&str>,
        password: Vec<&str>,
    ) -> Result<(), VboxError> {
        let (id_size, id_ptr) = str_vec_to_ptr(id)?;
        let (password_size, password_ptr) = str_vec_to_ptr(password)?;
        get_function_result_unit!(
            self.object,
            AddEncryptionPasswords,
            id_size,
            id_ptr,
            password_size,
            password_ptr
        )
    }

    #[cfg(not(is_v_6_1))]
    /// Removes a password used for the VM encryption/decryption.
    ///
    /// The password can be removed only if the VM is powered off. Removing the password causes the VM goes to the inaccessible state and the password must be provided again.
    ///
    /// # Arguments
    ///
    /// * `id` - &str. The identifier used for the password. Must match the identifier used when the encrypted VM was created.
    ///
    /// # Returns
    ///
    /// Returns () success, or a [`VboxError`] on failure.
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
    /// machine.remove_encryption_password("Freebsd_14").unwrap();
    /// ```
    pub fn remove_encryption_password(&self, id: &str) -> Result<(), VboxError> {
        let id = string_to_c_u64_str(id)?;
        get_function_result_unit!(self.object, RemoveEncryptionPassword, id)
    }

    #[cfg(not(is_v_6_1))]
    /// Clears all provided VM passwords.
    ///
    /// The passwords can be removed only if the VM is powered off. Removing the passwords causes the VM goes to the inaccessible state and the password must be provided again.
    ///
    /// # Returns
    ///
    /// Returns () success, or a [`VboxError`] on failure.
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
    /// machine.clear_all_encryption_passwords().unwrap();
    /// ```
    pub fn clear_all_encryption_passwords(&self) -> Result<(), VboxError> {
        get_function_result_unit!(self.object, ClearAllEncryptionPasswords)
    }
}
#[cfg(is_v_7_0)]
impl Machine {
    /// IOMMU type used in this VM.
    ///
    /// # Returns
    ///
    /// Returns [`IommuType`] success, or a [`VboxError`] on failure.
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
    /// let iommu_type = machine.get_iommu_type().unwrap();

    pub fn get_iommu_type(&self) -> Result<IommuType, VboxError> {
        let iommu_type = get_function_result_number!(self.object, GetIommuType, u32)?;
        Ok(IommuType::from(iommu_type))
    }

    /// IOMMU type used in this VM.
    ///
    /// # Arguments
    ///
    /// * `iommu_type` - [`IommuType`].
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
    /// use virtualbox_rs::enums::{IommuType, SessionType};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let mut session = Session::init().unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    /// let machine_mut = session.get_machine().unwrap();
    /// machine_mut.set_iommu_type(IommuType::Intel).unwrap();
    /// machine_mut.save_settings().unwrap();

    pub fn set_iommu_type(&self, iommu_type: IommuType) -> Result<(), VboxError> {
        let iommu_type = iommu_type.into();
        get_function_result_unit!(self.object, SetIommuType, iommu_type)
    }
}
#[cfg(not(is_v_7_1))]
impl Machine {
    /// Object containing all BIOS settings.
    ///
    /// # Returns
    ///
    /// Returns [`BIOSSettings`] success, or a [`VboxError`] on failure.
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
    /// let bios_settings = machine.get_bios_settings().unwrap();

    pub fn get_bios_settings(&self) -> Result<BIOSSettings, VboxError> {
        let bios_settings =
            get_function_result_pointer!(self.object, GetBIOSSettings, *mut IBIOSSettings)?;
        Ok(BIOSSettings::new(bios_settings))
    }

    /// Type of firmware (such as legacy BIOS or EFI), used for initial bootstrap in this VM.
    ///
    /// # Returns
    ///
    /// Returns [`FirmwareType`] success, or a [`VboxError`] on failure.
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
    /// let firmware_type = machine.get_firmware_type().unwrap();

    pub fn get_firmware_type(&self) -> Result<FirmwareType, VboxError> {
        let firmware_type = get_function_result_number!(self.object, GetFirmwareType, u32)?;
        Ok(FirmwareType::from(firmware_type))
    }

    /// Type of firmware (such as legacy BIOS or EFI), used for initial bootstrap in this VM.
    ///
    /// # Arguments
    ///
    /// * `firmware_type` - [`FirmwareType`].
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
    /// use virtualbox_rs::enums::{FirmwareType, SessionType};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let mut session = Session::init().unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    /// let machine_mut = session.get_machine().unwrap();
    /// machine_mut.set_firmware_type(FirmwareType::BIOS).unwrap();
    /// machine_mut.save_settings().unwrap();

    pub fn set_firmware_type(&self, firmware_type: FirmwareType) -> Result<(), VboxError> {
        let firmware_type = firmware_type.into();
        get_function_result_unit!(self.object, SetFirmwareType, firmware_type)
    }

    /// This attribute controls if High Precision Event Timer (HPET) is enabled in this VM.
    ///
    /// Use this property if you want to provide guests with additional time source, or if guest requires HPET to function correctly. Default is false.
    ///
    /// # Returns
    ///
    /// Returns bool success, or a [`VboxError`] on failure.
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
    /// let enabled = machine.get_hpet_enabled().unwrap();
    pub fn get_hpet_enabled(&self) -> Result<bool, VboxError> {
        get_function_result_bool!(self.object, GetHPETEnabled)
    }

    /// This attribute controls if High Precision Event Timer (HPET) is enabled in this VM.
    ///
    /// Use this property if you want to provide guests with additional time source, or if guest requires HPET to function correctly. Default is false.
    ///
    /// # Arguments
    ///
    /// * `enabled` - bool.
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
    /// use virtualbox_rs::enums::{SessionType};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let mut session = Session::init().unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    /// let machine_mut = session.get_machine().unwrap();
    /// machine_mut.set_hpet_enabled(true).unwrap();
    /// machine_mut.save_settings().unwrap();
    pub fn set_hpet_enabled(&self, enabled: bool) -> Result<(), VboxError> {
        let enabled = if enabled { 1 } else { 0 };
        get_function_result_unit!(self.object, SetHPETEnabled, enabled)
    }

    /// Chipset type used in this VM.
    ///
    /// # Returns
    ///
    /// Returns [`ChipsetType`] success, or a [`VboxError`] on failure.
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
    /// let chipset_type = machine.get_chipset_type().unwrap();

    pub fn get_chipset_type(&self) -> Result<ChipsetType, VboxError> {
        let chipset_type = get_function_result_number!(self.object, GetChipsetType, u32)?;
        Ok(ChipsetType::from(chipset_type))
    }

    /// Chipset type used in this VM.
    ///
    /// # Arguments
    ///
    /// * `chipset_type` - [`ChipsetType`].
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
    /// use virtualbox_rs::enums::{ChipsetType, SessionType};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let mut session = Session::init().unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    /// let machine_mut = session.get_machine().unwrap();
    /// machine_mut.set_chipset_type(ChipsetType::PIIX3).unwrap();
    /// machine_mut.save_settings().unwrap();

    pub fn set_chipset_type(&self, chipset_type: ChipsetType) -> Result<(), VboxError> {
        let chipset_type = chipset_type.into();
        get_function_result_unit!(self.object, SetChipsetType, chipset_type)
    }

    /// When set to true, the RTC device of the virtual machine will run in UTC time, otherwise in local time.
    ///
    /// Especially Unix guests prefer the time in UTC.
    ///
    /// # Returns
    ///
    /// Returns bool success, or a [`VboxError`] on failure.
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
    /// let rtc_use_utc = machine.get_rtc_use_utc().unwrap();
    pub fn get_rtc_use_utc(&self) -> Result<bool, VboxError> {
        get_function_result_bool!(self.object, GetRTCUseUTC)
    }

    /// When set to true, the RTC device of the virtual machine will run in UTC time, otherwise in local time.
    ///
    /// Especially Unix guests prefer the time in UTC.
    ///
    /// # Arguments
    ///
    /// * `rtc_use_utc` - bool. When set to true, the RTC device of the virtual machine will run in UTC time, otherwise in local time.
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
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let mut session = Session::init().unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    /// let machine_mut = session.get_machine().unwrap();
    /// machine_mut.set_rtc_use_utc(true).unwrap();
    pub fn set_rtc_use_utc(&self, rtc_use_utc: bool) -> Result<(), VboxError> {
        let rtc_use_utc = if rtc_use_utc { 1 } else { 0 };
        get_function_result_unit!(self.object, SetRTCUseUTC, rtc_use_utc)
    }

    /// Returns the virtual CPU boolean value of the specified property.
    ///
    /// # Arguments
    ///
    /// * `property` - [`CPUPropertyType`]. Property type to query.
    ///
    /// # Returns
    ///
    /// Returns bool success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{VirtualBox};
    /// use virtualbox_rs::enums::CPUPropertyType;
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let cpu_property = machine.get_cpu_property(CPUPropertyType::APIC).unwrap();
    pub fn get_cpu_property(&self, property: CPUPropertyType) -> Result<bool, VboxError> {
        let property = property.into();
        get_function_result_bool!(self.object, GetCPUProperty, property)
    }

    ///  Sets the virtual CPU boolean value of the specified property.
    ///
    /// # Arguments
    ///
    /// * `property` - [`CPUPropertyType`]. Property type to query.
    /// * `value` - bool. Property value.
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
    /// use virtualbox_rs::enums::{CPUPropertyType, SessionType};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let mut session = Session::init().unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    /// let machine_mut = session.get_machine().unwrap();
    /// machine_mut.set_cpu_property(CPUPropertyType::APIC, true).unwrap();
    pub fn set_cpu_property(
        &self,
        property: CPUPropertyType,
        value: bool,
    ) -> Result<(), VboxError> {
        let property = property.into();
        let value = if value { 1 } else { 0 };
        get_function_result_unit!(self.object, SetCPUProperty, property, value)
    }

    /// Returns the value of the specified hardware virtualization boolean property.
    ///
    /// # Arguments
    ///
    /// * `property` - [`HWVirtExPropertyType`]. Property type to query.
    ///
    /// # Returns
    ///
    /// Returns bool success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{VirtualBox};
    /// use virtualbox_rs::enums::{HWVirtExPropertyType};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let property = machine.get_hw_virt_ex_property(HWVirtExPropertyType::NestedPaging).unwrap();
    pub fn get_hw_virt_ex_property(
        &self,
        property: HWVirtExPropertyType,
    ) -> Result<bool, VboxError> {
        let property = property.into();
        get_function_result_bool!(self.object, GetHWVirtExProperty, property)
    }

    ///  Sets a new value for the specified hardware virtualization boolean property.
    ///
    /// # Arguments
    ///
    /// * `property` - [`HWVirtExPropertyType`]. Property type to set.
    /// * `value` - bool. New property value.
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
    /// use virtualbox_rs::enums::{HWVirtExPropertyType, SessionType};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// let mut session = Session::init().unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    /// let machine_mut = session.get_machine().unwrap();
    /// machine_mut.set_hw_virt_ex_property(HWVirtExPropertyType::NestedPaging, true).unwrap();
    pub fn set_hw_virt_ex_property(
        &self,
        property: HWVirtExPropertyType,
        value: bool,
    ) -> Result<(), VboxError> {
        let property = property.into();
        let value = if value { 1 } else { 0 };
        get_function_result_unit!(self.object, SetHWVirtExProperty, property, value)
    }

    /// Placeholder Method
    ///
    /// This method serves as a placeholder for versions of the API where the actual method is not available.
    /// Attempting to call this method will result in an `UnsupportedInCurrentApiVersion` error.
    /// This method is intended to ensure that the codebase can be compiled against multiple API versions
    /// without modification.
    ///
    /// Supported from API version: v7_1
    pub fn get_firmware_settings(&self) -> Result<FirmwareSettings, VboxError> {
        Err(VboxError::unsupported_in_current_api_version(
            "Machine::get_firmware_settings",
            "v7_1",
        ))
    }

    /// Placeholder Method
    ///
    /// This method serves as a placeholder for versions of the API where the actual method is not available.
    /// Attempting to call this method will result in an `UnsupportedInCurrentApiVersion` error.
    /// This method is intended to ensure that the codebase can be compiled against multiple API versions
    /// without modification.
    ///
    /// Supported from API version: v7_1
    pub fn get_platform(&self) -> Result<Platform, VboxError> {
        Err(VboxError::unsupported_in_current_api_version(
            "Machine::get_platform",
            "v7_1",
        ))
    }
}

#[cfg(is_v_6_1)]
impl Machine {
    /// Placeholder Method
    ///
    /// This method serves as a placeholder for versions of the API where the actual method is not available.
    /// Attempting to call this method will result in an `UnsupportedInCurrentApiVersion` error.
    /// This method is intended to ensure that the codebase can be compiled against multiple API versions
    /// without modification.
    ///
    /// Supported from API version: v7_0
    pub fn get_trusted_platform_module(&self) -> Result<TrustedPlatformModule, VboxError> {
        Err(VboxError::unsupported_in_current_api_version(
            "Machine::get_guest_debug_control",
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
    pub fn get_non_volatile_store(&self) -> Result<NvramStore, VboxError> {
        Err(VboxError::unsupported_in_current_api_version(
            "Machine::get_guest_debug_control",
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
    pub fn get_iommu_type(&self) -> Result<IommuType, VboxError> {
        Err(VboxError::unsupported_in_current_api_version(
            "Machine::get_guest_debug_control",
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
    pub fn set_iommu_type(&self, _iommu_type: IommuType) -> Result<(), VboxError> {
        Err(VboxError::unsupported_in_current_api_version(
            "Machine::get_guest_debug_control",
            "v7_0",
        ))
    }

    /// Associated audio adapter, always present.
    ///
    /// # Returns
    ///
    /// Returns [`AudioAdapter`] on success, or a [`VboxError`] on failure.
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
    /// let adapter = machine.get_audio_adapter().unwrap();
    ///
    pub fn get_audio_adapter(&self) -> Result<AudioAdapter, VboxError> {
        let audio_adapter_ptr =
            get_function_result_pointer!(self.object, GetAudioAdapter, *mut IAudioAdapter)?;
        Ok(AudioAdapter::new(audio_adapter_ptr))
    }

    /// Placeholder Method
    ///
    /// This method serves as a placeholder for versions of the API where the actual method is not available.
    /// Attempting to call this method will result in an `UnsupportedInCurrentApiVersion` error.
    /// This method is intended to ensure that the codebase can be compiled against multiple API versions
    /// without modification.
    ///
    /// Supported from API version: v7_0
    pub fn get_audio_settings(&self) -> Result<AudioSettings, VboxError> {
        Err(VboxError::unsupported_in_current_api_version(
            "Machine::get_audio_settings",
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
    pub fn get_guest_debug_control(&self) -> Result<GuestDebugControl, VboxError> {
        Err(VboxError::unsupported_in_current_api_version(
            "Machine::get_guest_debug_control",
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
    pub fn get_state_key_id(&self) -> Result<&'static str, VboxError> {
        Err(VboxError::unsupported_in_current_api_version(
            "Machine::get_state_key_id",
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
    pub fn get_state_key_store(&self) -> Result<&'static str, VboxError> {
        Err(VboxError::unsupported_in_current_api_version(
            "Machine::get_state_key_store",
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
    pub fn get_log_key_id(&self) -> Result<&'static str, VboxError> {
        Err(VboxError::unsupported_in_current_api_version(
            "Machine::get_log_key_id",
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
    pub fn get_log_key_store(&self) -> Result<&'static str, VboxError> {
        Err(VboxError::unsupported_in_current_api_version(
            "Machine::get_log_key_store",
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
    pub fn change_encryption(
        &self,
        _current_password: &str,
        _cipher: &str,
        _new_password: &str,
        _new_password_id: &str,
        _force: bool,
    ) -> Result<Progress, VboxError> {
        Err(VboxError::unsupported_in_current_api_version(
            "Machine::unsupported_in_current_api_version",
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
    pub fn get_encryption_settings(&self) -> Result<(&'static str, &'static str), VboxError> {
        Err(VboxError::unsupported_in_current_api_version(
            "Machine::get_encryption_settings",
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
    pub fn check_encryption_password(&self, _password: &str) -> Result<(), VboxError> {
        Err(VboxError::unsupported_in_current_api_version(
            "Machine::check_encryption_password",
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
    pub fn add_encryption_password(&self, _id: &str, _password: &str) -> Result<(), VboxError> {
        Err(VboxError::unsupported_in_current_api_version(
            "Machine::add_encryption_password",
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
    pub fn add_encryption_passwords(
        &self,
        _id: Vec<&str>,
        _password: Vec<&str>,
    ) -> Result<(), VboxError> {
        Err(VboxError::unsupported_in_current_api_version(
            "Machine::add_encryption_passwords",
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
    pub fn remove_encryption_password(&self, _id: &str) -> Result<(), VboxError> {
        Err(VboxError::unsupported_in_current_api_version(
            "Machine::remove_encryption_password",
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
    pub fn clear_all_encryption_passwords(&self) -> Result<(), VboxError> {
        Err(VboxError::unsupported_in_current_api_version(
            "Machine::clear_all_encryption_passwords",
            "v7_0",
        ))
    }
}
