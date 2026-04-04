use log::error;
use std::fmt::Display;
use vbox_raw::sys_lib as raw;
#[cfg(doc)]
use crate::virtual_system_description::VirtualSystemDescription;

/// Used with [`VirtualSystemDescription`] to describe the type of a configuration value.
#[derive(Debug, Eq, PartialEq)]
pub enum VirtualSystemDescriptionType {
    Ignore,
    /// the guest operating system type. There must be exactly one such array item on import.
    /// The corresponding item in aVBoxValues[] contains the suggested guest operating system for VirtualBox.
    /// This will be one of the values listed in IVirtualBox::guestOSTypes. The corresponding item in OVFValues[]
    /// will contain a numerical value that described the operating system in the OVF.
    OS,
    /// the name to give to the new virtual machine. There can be at most one such array item; if none is
    /// present on import, then an automatic name will be created from the operating system type. The corresponding
    /// item in OVFValues[] will contain the suggested virtual machine name from the OVF file, and aVBoxValues[]
    /// will contain a suggestion for a unique VirtualBox IMachine name that does not exist yet.
    Name,
    Product,
    Vendor,
    Version,
    ProductUrl,
    VendorUrl,
    /// an arbitrary description
    Description,
    /// the EULA section from the OVF, if present. It is the responsibility of the calling code to display such a
    /// license for agreement; the Main API does not enforce any such policy.
    License,
    /// reserved for future use.
    Miscellaneous,
    /// the number of CPUs. There can be at most one such item, which will presently be ignored.
    CPU,
    /// the amount of guest RAM, in bytes. There can be at most one such array item; if none is present on import,
    /// then VirtualBox will set a meaningful default based on the operating system type.
    Memory,
    /// an IDE hard disk controller. There can be at most two such items. An optional value in OVFValues[] and
    /// aVBoxValues[] can be "PIIX3" or "PIIX4" to specify the type of IDE controller; this corresponds to the
    /// ResourceSubType element which VirtualBox writes into the OVF. The matching item in the aRefs[] array will
    /// contain an integer that items of the "Harddisk" type can use to specify which hard disk controller a virtual
    /// disk should be connected to. Note that in OVF, an IDE controller has two channels, corresponding to "master"
    /// and "slave" in traditional terminology, whereas the IDE storage controller that VirtualBox supports in its
    /// virtual machines supports four channels (primary master, primary slave, secondary master, secondary slave)
    /// and thus maps to two IDE controllers in the OVF sense.
    HardDiskControllerIDE,
    /// an SATA hard disk controller. There can be at most one such item. This has no value in OVFValues[] or
    /// aVBoxValues[]. The matching item in the aRefs[] array will be used as with IDE controllers (see above).
    HardDiskControllerSATA,
    /// a SCSI hard disk controller. There can be at most one such item. The items in OVFValues[] and aVBoxValues[]
    /// will either be "LsiLogic", "BusLogic" or "LsiLogicSas". (Note that in OVF, the LsiLogicSas controller is
    /// treated as a SCSI controller whereas VirtualBox considers it a class of storage controllers of its own; see
    /// StorageControllerType). The matching item in the aRefs[] array will be used as with IDE controllers (see above).
    HardDiskControllerSCSI,
    /// a SCSI controller using the SAS (Serial Attached SCSI) variant. There can be at most one such item.
    /// Items in OVFValues[] and aVBoxValues[] use the same values as for SCSI controllers (e.g. "LsiLogicSas").
    HardDiskControllerSAS,
    /// a virtual hard disk, most probably as a reference to an image file. There can be an arbitrary number of these
    /// items, one for each virtual disk image that accompanies the OVF. The array item in OVFValues[] will contain the
    /// file specification from the OVF file (without a path since the image file should be in the same location as the
    /// OVF file itself), whereas the item in aVBoxValues[] will contain a qualified path specification to where
    /// VirtualBox uses the hard disk image. This means that on import the image will be copied and converted from the
    /// "ovf" location to the "vbox" location; on export, this will be handled the other way round. The matching item
    /// in the aExtraConfigValues[] array must contain a string of the following format: "controller=&lt;index&gt;;channel=&lt;c&gt;".
    /// In this string, &lt;index&gt; must be an integer specifying the hard disk controller to connect the image to. That
    /// number must be the index of an array item with one of the hard disk controller types (HardDiskControllerSCSI,
    /// HardDiskControllerSATA, HardDiskControllerIDE). In addition, &lt;c&gt; must specify the channel to use on that controller.
    /// For IDE controllers, this can be 0 or 1 for master or slave, respectively. For compatibility with VirtualBox
    /// versions before 3.2, the values 2 and 3 (for secondary master and secondary slave) are also supported, but no
    /// longer exported. For SATA and SCSI controllers, the channel can range from 0-29.
    HardDiskImage,
    /// a virtual CD-ROM drive. The matching item in aExtraConfigValue[] contains the same attachment information as
    /// with "HardDiskImage" items.
    CDROM,
    /// a virtual floppy drive. The matching item in aExtraConfigValue[] contains the same attachment information as
    /// with "HardDiskImage" items.
    Floppy,
    /// a network adapter. The array item in aVBoxValues[] will specify the hardware for the network adapter, whereas
    /// the array item in aExtraConfigValues[] will have a string of the "type=&lt;X&gt;" format, where &lt;X&gt; must be
    /// either "NAT" or "Bridged".
    NetworkAdapter,
    /// a USB controller. There can be at most one such item. If, and only if, such an item is present, USB support will
    /// be enabled for the new virtual machine.
    USBController,
    /// a sound card. There can be at most one such item. If and only if such an item is present, sound support will be
    /// enabled for the new virtual machine. Note that the virtual machine in VirtualBox will always be presented with
    /// the standard VirtualBox soundcard, which may be different from the virtual soundcard expected by the appliance.
    SoundCard,
    /// the file name of the settings file, relative to the base folder. There can be at most one such item.
    SettingsFile,
    /// the base folder for the virtual machine. This is an absolute path on the host system. There can be at most one
    /// such item; if none is present on import, then a default location will be used. The corresponding item in
    /// OVFValues[] will contain the suggested base folder from the OVF file, and aVBoxValues[] will contain a
    /// suggestion for a unique base folder that does not exist yet.
    BaseFolder,
    /// the primary group for the virtual machine. This is an arbitrary string. There can be at most one such item.
    PrimaryGroup,
    /// the shape of the cloud instance. This is an arbitrary string. There can be at most one such item.
    CloudInstanceShape,
    /// the domain for the cloud instance. This is an arbitrary string. There can be at most one such item.
    CloudDomain,
    /// the size of the cloud boot disk, in bytes. There can be at most one such item.
    CloudBootDiskSize,
    /// the cloud storage bucket. This is an arbitrary string. There can be at most one such item.
    CloudBucket,
    /// the OCI VCN for the cloud instance. This is an arbitrary string. There can be at most one such item.
    CloudOCIVCN,
    /// the public IP for the cloud instance. This is an arbitrary string. There can be at most one such item.
    CloudPublicIP,
    /// the profile name for the cloud instance. This is an arbitrary string. There can be at most one such item.
    CloudProfileName,
    /// the OCI subnet for the cloud instance. This is an arbitrary string. There can be at most one such item.
    CloudOCISubnet,
    /// whether to keep the object in the cloud after use. There can be at most one such item.
    CloudKeepObject,
    /// whether to launch the instance in the cloud. There can be at most one such item.
    CloudLaunchInstance,
    /// the ID of the cloud instance. This is an arbitrary string. There can be at most one such item.
    CloudInstanceId,
    /// the ID of the image used to create the cloud instance. This is an arbitrary string. There can be at most one
    /// such item.
    CloudImageId,
    /// the state of the cloud instance. This is an arbitrary string. There can be at most one such item.
    CloudInstanceState,
    /// the state of the image used to create the cloud instance. This is an arbitrary string. There can be at most
    /// one such item.
    CloudImageState,
    /// the display name of the cloud instance. This is an arbitrary string. There can be at most one such item.
    CloudInstanceDisplayName,
    /// the display name of the image used to create the cloud instance. This is an arbitrary string. There can be at
    /// most one such item.
    CloudImageDisplayName,
    /// the launch mode for the cloud instance. This is an arbitrary string. There can be at most one such item.
    CloudOCILaunchMode,
    /// the private IP for the cloud instance. This is an arbitrary string. There can be at most one such item.
    CloudPrivateIP,
    /// the ID of the boot volume in the cloud. This is an arbitrary string. There can be at most one such item.
    CloudBootVolumeId,
    /// the compartment for the OCI VCN in the cloud. This is an arbitrary string. There can be at most one such item.
    CloudOCIVCNCompartment,
    /// the compartment for the OCI subnet in the cloud. This is an arbitrary string. There can be at most one such item.
    CloudOCISubnetCompartment,
    /// the public SSH key for the cloud instance. This is an arbitrary string. There can be at most one such item.
    CloudPublicSSHKey,
    /// the firmware used for booting the virtual machine. There can be at most one such item.
    BootingFirmware,
    /// the path to the cloud-init script. This is an arbitrary string. There can be at most one such item.
    CloudInitScriptPath,
    /// the compartment ID for the cloud instance. This is an arbitrary string. There can be at most one such item.
    CloudCompartmentId,
    /// the number of CPUs for the cloud shape. There can be at most one such item.
    CloudShapeCpus,
    /// the amount of memory for the cloud shape, in bytes. There can be at most one such item.
    CloudShapeMemory,
    /// metadata for the cloud instance. This is an arbitrary string. There can be at most one such item.
    CloudInstanceMetadata,
    /// free-form tags for the cloud instance. This is an arbitrary string. There can be at most one such item.
    CloudInstanceFreeFormTags,
    /// free-form tags for the image used to create the cloud instance. This is an arbitrary string. There can be at
    /// most one such item.
    CloudImageFreeFormTags,
    /// a virtual hard disk controller using the Virtio SCSI interface. There can be at most one such item. The item
    /// in OVFValues[] will be ignored.
    HardDiskControllerVirtioSCSI,
    /// a virtual hard disk controller using the NVMe interface. There can be at most one such item. The item in
    /// OVFValues[] will be ignored.
    HardDiskControllerNVMe,
    /// the NVRAM file for the virtual machine. There can be at most one such item. The item in OVFValues[] will be
    /// ignored.
    NVRAM,
}
impl Into<u32> for VirtualSystemDescriptionType {
    fn into(self) -> u32 {
        match self {
            VirtualSystemDescriptionType::OS => raw::VirtualSystemDescriptionType_VirtualSystemDescriptionType_OS,
            VirtualSystemDescriptionType::Name => raw::VirtualSystemDescriptionType_VirtualSystemDescriptionType_Name,
            VirtualSystemDescriptionType::Product => raw::VirtualSystemDescriptionType_VirtualSystemDescriptionType_Product,
            VirtualSystemDescriptionType::Vendor => raw::VirtualSystemDescriptionType_VirtualSystemDescriptionType_Vendor,
            VirtualSystemDescriptionType::Version => raw::VirtualSystemDescriptionType_VirtualSystemDescriptionType_Version,
            VirtualSystemDescriptionType::ProductUrl => raw::VirtualSystemDescriptionType_VirtualSystemDescriptionType_ProductUrl,
            VirtualSystemDescriptionType::VendorUrl => raw::VirtualSystemDescriptionType_VirtualSystemDescriptionType_VendorUrl,
            VirtualSystemDescriptionType::Description => raw::VirtualSystemDescriptionType_VirtualSystemDescriptionType_Description,
            VirtualSystemDescriptionType::License => raw::VirtualSystemDescriptionType_VirtualSystemDescriptionType_License,
            VirtualSystemDescriptionType::Miscellaneous => raw::VirtualSystemDescriptionType_VirtualSystemDescriptionType_Miscellaneous,
            VirtualSystemDescriptionType::CPU => raw::VirtualSystemDescriptionType_VirtualSystemDescriptionType_CPU,
            VirtualSystemDescriptionType::Memory => raw::VirtualSystemDescriptionType_VirtualSystemDescriptionType_Memory,
            VirtualSystemDescriptionType::HardDiskControllerIDE => raw::VirtualSystemDescriptionType_VirtualSystemDescriptionType_HardDiskControllerIDE,
            VirtualSystemDescriptionType::HardDiskControllerSATA => raw::VirtualSystemDescriptionType_VirtualSystemDescriptionType_HardDiskControllerSATA,
            VirtualSystemDescriptionType::HardDiskControllerSCSI => raw::VirtualSystemDescriptionType_VirtualSystemDescriptionType_HardDiskControllerSCSI,
            VirtualSystemDescriptionType::HardDiskControllerSAS => raw::VirtualSystemDescriptionType_VirtualSystemDescriptionType_HardDiskControllerSAS,
            VirtualSystemDescriptionType::HardDiskImage => raw::VirtualSystemDescriptionType_VirtualSystemDescriptionType_HardDiskImage,
            VirtualSystemDescriptionType::Floppy => raw::VirtualSystemDescriptionType_VirtualSystemDescriptionType_Floppy,
            VirtualSystemDescriptionType::CDROM => raw::VirtualSystemDescriptionType_VirtualSystemDescriptionType_CDROM,
            VirtualSystemDescriptionType::NetworkAdapter => raw::VirtualSystemDescriptionType_VirtualSystemDescriptionType_NetworkAdapter,
            VirtualSystemDescriptionType::USBController => raw::VirtualSystemDescriptionType_VirtualSystemDescriptionType_USBController,
            VirtualSystemDescriptionType::SoundCard => raw::VirtualSystemDescriptionType_VirtualSystemDescriptionType_SoundCard,
            VirtualSystemDescriptionType::SettingsFile => raw::VirtualSystemDescriptionType_VirtualSystemDescriptionType_SettingsFile,
            VirtualSystemDescriptionType::BaseFolder => raw::VirtualSystemDescriptionType_VirtualSystemDescriptionType_BaseFolder,
            VirtualSystemDescriptionType::PrimaryGroup => raw::VirtualSystemDescriptionType_VirtualSystemDescriptionType_PrimaryGroup,
            VirtualSystemDescriptionType::CloudInstanceShape => raw::VirtualSystemDescriptionType_VirtualSystemDescriptionType_CloudInstanceShape,
            VirtualSystemDescriptionType::CloudDomain => raw::VirtualSystemDescriptionType_VirtualSystemDescriptionType_CloudDomain,
            VirtualSystemDescriptionType::CloudBootDiskSize => raw::VirtualSystemDescriptionType_VirtualSystemDescriptionType_CloudBootDiskSize,
            VirtualSystemDescriptionType::CloudBucket => raw::VirtualSystemDescriptionType_VirtualSystemDescriptionType_CloudBucket,
            VirtualSystemDescriptionType::CloudOCIVCN => raw::VirtualSystemDescriptionType_VirtualSystemDescriptionType_CloudOCIVCN,
            VirtualSystemDescriptionType::CloudPublicIP => raw::VirtualSystemDescriptionType_VirtualSystemDescriptionType_CloudPublicIP,
            VirtualSystemDescriptionType::CloudProfileName => raw::VirtualSystemDescriptionType_VirtualSystemDescriptionType_CloudProfileName,
            VirtualSystemDescriptionType::CloudOCISubnet => raw::VirtualSystemDescriptionType_VirtualSystemDescriptionType_CloudOCISubnet,
            VirtualSystemDescriptionType::CloudKeepObject => raw::VirtualSystemDescriptionType_VirtualSystemDescriptionType_CloudKeepObject,
            VirtualSystemDescriptionType::CloudLaunchInstance => raw::VirtualSystemDescriptionType_VirtualSystemDescriptionType_CloudLaunchInstance,
            VirtualSystemDescriptionType::CloudInstanceId => raw::VirtualSystemDescriptionType_VirtualSystemDescriptionType_CloudInstanceId,
            VirtualSystemDescriptionType::CloudImageId => raw::VirtualSystemDescriptionType_VirtualSystemDescriptionType_CloudImageId,
            VirtualSystemDescriptionType::CloudInstanceState => raw::VirtualSystemDescriptionType_VirtualSystemDescriptionType_CloudInstanceState,
            VirtualSystemDescriptionType::CloudImageState => raw::VirtualSystemDescriptionType_VirtualSystemDescriptionType_CloudImageState,
            VirtualSystemDescriptionType::CloudInstanceDisplayName => raw::VirtualSystemDescriptionType_VirtualSystemDescriptionType_CloudInstanceDisplayName,
            VirtualSystemDescriptionType::CloudImageDisplayName => raw::VirtualSystemDescriptionType_VirtualSystemDescriptionType_CloudImageDisplayName,
            VirtualSystemDescriptionType::CloudOCILaunchMode => raw::VirtualSystemDescriptionType_VirtualSystemDescriptionType_CloudOCILaunchMode,
            VirtualSystemDescriptionType::CloudPrivateIP => raw::VirtualSystemDescriptionType_VirtualSystemDescriptionType_CloudPrivateIP,
            VirtualSystemDescriptionType::CloudBootVolumeId => raw::VirtualSystemDescriptionType_VirtualSystemDescriptionType_CloudBootVolumeId,
            VirtualSystemDescriptionType::CloudOCIVCNCompartment => raw::VirtualSystemDescriptionType_VirtualSystemDescriptionType_CloudOCIVCNCompartment,
            VirtualSystemDescriptionType::CloudOCISubnetCompartment => raw::VirtualSystemDescriptionType_VirtualSystemDescriptionType_CloudOCISubnetCompartment,
            VirtualSystemDescriptionType::CloudPublicSSHKey => raw::VirtualSystemDescriptionType_VirtualSystemDescriptionType_CloudPublicSSHKey,
            VirtualSystemDescriptionType::BootingFirmware => raw::VirtualSystemDescriptionType_VirtualSystemDescriptionType_BootingFirmware,
            VirtualSystemDescriptionType::CloudInitScriptPath => raw::VirtualSystemDescriptionType_VirtualSystemDescriptionType_CloudInitScriptPath,
            #[cfg(not(is_v_6_1))]
            VirtualSystemDescriptionType::CloudCompartmentId => raw::VirtualSystemDescriptionType_VirtualSystemDescriptionType_CloudCompartmentId,
            #[cfg(not(is_v_6_1))]
            VirtualSystemDescriptionType::CloudShapeCpus => raw::VirtualSystemDescriptionType_VirtualSystemDescriptionType_CloudShapeCpus,
            #[cfg(not(is_v_6_1))]
            VirtualSystemDescriptionType::CloudShapeMemory => raw::VirtualSystemDescriptionType_VirtualSystemDescriptionType_CloudShapeMemory,
            #[cfg(not(is_v_6_1))]
            VirtualSystemDescriptionType::CloudInstanceMetadata => raw::VirtualSystemDescriptionType_VirtualSystemDescriptionType_CloudInstanceMetadata,
            #[cfg(not(is_v_6_1))]
            VirtualSystemDescriptionType::CloudInstanceFreeFormTags => raw::VirtualSystemDescriptionType_VirtualSystemDescriptionType_CloudInstanceFreeFormTags,
            #[cfg(not(is_v_6_1))]
            VirtualSystemDescriptionType::CloudImageFreeFormTags => raw::VirtualSystemDescriptionType_VirtualSystemDescriptionType_CloudImageFreeFormTags,
            VirtualSystemDescriptionType::HardDiskControllerVirtioSCSI => raw::VirtualSystemDescriptionType_VirtualSystemDescriptionType_HardDiskControllerVirtioSCSI,
            VirtualSystemDescriptionType::HardDiskControllerNVMe => raw::VirtualSystemDescriptionType_VirtualSystemDescriptionType_HardDiskControllerNVMe,
            #[cfg(is_v_7_1_or_newer)]
            VirtualSystemDescriptionType::NVRAM => raw::VirtualSystemDescriptionType_VirtualSystemDescriptionType_NVRAM,
            _ => raw::VirtualSystemDescriptionType_VirtualSystemDescriptionType_Ignore
        }
    }
}

impl From<u32> for VirtualSystemDescriptionType {
    fn from(value: u32) -> Self {
        match value {
            raw::VirtualSystemDescriptionType_VirtualSystemDescriptionType_Ignore => VirtualSystemDescriptionType::Ignore,
            raw::VirtualSystemDescriptionType_VirtualSystemDescriptionType_OS => VirtualSystemDescriptionType::OS,
            raw::VirtualSystemDescriptionType_VirtualSystemDescriptionType_Name => VirtualSystemDescriptionType::Name,
            raw::VirtualSystemDescriptionType_VirtualSystemDescriptionType_Product => VirtualSystemDescriptionType::Product,
            raw::VirtualSystemDescriptionType_VirtualSystemDescriptionType_Vendor => VirtualSystemDescriptionType::Vendor,
            raw::VirtualSystemDescriptionType_VirtualSystemDescriptionType_Version => VirtualSystemDescriptionType::Version,
            raw::VirtualSystemDescriptionType_VirtualSystemDescriptionType_ProductUrl => VirtualSystemDescriptionType::ProductUrl,
            raw::VirtualSystemDescriptionType_VirtualSystemDescriptionType_VendorUrl => VirtualSystemDescriptionType::VendorUrl,
            raw::VirtualSystemDescriptionType_VirtualSystemDescriptionType_Description => VirtualSystemDescriptionType::Description,
            raw::VirtualSystemDescriptionType_VirtualSystemDescriptionType_License => VirtualSystemDescriptionType::License,
            raw::VirtualSystemDescriptionType_VirtualSystemDescriptionType_Miscellaneous => VirtualSystemDescriptionType::Miscellaneous,
            raw::VirtualSystemDescriptionType_VirtualSystemDescriptionType_CPU => VirtualSystemDescriptionType::CPU,
            raw::VirtualSystemDescriptionType_VirtualSystemDescriptionType_Memory => VirtualSystemDescriptionType::Memory,
            raw::VirtualSystemDescriptionType_VirtualSystemDescriptionType_HardDiskControllerIDE => VirtualSystemDescriptionType::HardDiskControllerIDE,
            raw::VirtualSystemDescriptionType_VirtualSystemDescriptionType_HardDiskControllerSATA => VirtualSystemDescriptionType::HardDiskControllerSATA,
            raw::VirtualSystemDescriptionType_VirtualSystemDescriptionType_HardDiskControllerSCSI => VirtualSystemDescriptionType::HardDiskControllerSCSI,
            raw::VirtualSystemDescriptionType_VirtualSystemDescriptionType_HardDiskControllerSAS => VirtualSystemDescriptionType::HardDiskControllerSAS,
            raw::VirtualSystemDescriptionType_VirtualSystemDescriptionType_HardDiskImage => VirtualSystemDescriptionType::HardDiskImage,
            raw::VirtualSystemDescriptionType_VirtualSystemDescriptionType_Floppy => VirtualSystemDescriptionType::Floppy,
            raw::VirtualSystemDescriptionType_VirtualSystemDescriptionType_CDROM => VirtualSystemDescriptionType::CDROM,
            raw::VirtualSystemDescriptionType_VirtualSystemDescriptionType_NetworkAdapter => VirtualSystemDescriptionType::NetworkAdapter,
            raw::VirtualSystemDescriptionType_VirtualSystemDescriptionType_USBController => VirtualSystemDescriptionType::USBController,
            raw::VirtualSystemDescriptionType_VirtualSystemDescriptionType_SoundCard => VirtualSystemDescriptionType::SoundCard,
            raw::VirtualSystemDescriptionType_VirtualSystemDescriptionType_SettingsFile => VirtualSystemDescriptionType::SettingsFile,
            raw::VirtualSystemDescriptionType_VirtualSystemDescriptionType_BaseFolder => VirtualSystemDescriptionType::BaseFolder,
            raw::VirtualSystemDescriptionType_VirtualSystemDescriptionType_PrimaryGroup => VirtualSystemDescriptionType::PrimaryGroup,
            raw::VirtualSystemDescriptionType_VirtualSystemDescriptionType_CloudInstanceShape => VirtualSystemDescriptionType::CloudInstanceShape,
            raw::VirtualSystemDescriptionType_VirtualSystemDescriptionType_CloudDomain => VirtualSystemDescriptionType::CloudDomain,
            raw::VirtualSystemDescriptionType_VirtualSystemDescriptionType_CloudBootDiskSize => VirtualSystemDescriptionType::CloudBootDiskSize,
            raw::VirtualSystemDescriptionType_VirtualSystemDescriptionType_CloudBucket => VirtualSystemDescriptionType::CloudBucket,
            raw::VirtualSystemDescriptionType_VirtualSystemDescriptionType_CloudOCIVCN => VirtualSystemDescriptionType::CloudOCIVCN,
            raw::VirtualSystemDescriptionType_VirtualSystemDescriptionType_CloudPublicIP => VirtualSystemDescriptionType::CloudPublicIP,
            raw::VirtualSystemDescriptionType_VirtualSystemDescriptionType_CloudProfileName => VirtualSystemDescriptionType::CloudProfileName,
            raw::VirtualSystemDescriptionType_VirtualSystemDescriptionType_CloudOCISubnet => VirtualSystemDescriptionType::CloudOCISubnet,
            raw::VirtualSystemDescriptionType_VirtualSystemDescriptionType_CloudKeepObject => VirtualSystemDescriptionType::CloudKeepObject,
            raw::VirtualSystemDescriptionType_VirtualSystemDescriptionType_CloudLaunchInstance => VirtualSystemDescriptionType::CloudLaunchInstance,
            raw::VirtualSystemDescriptionType_VirtualSystemDescriptionType_CloudInstanceId => VirtualSystemDescriptionType::CloudInstanceId,
            raw::VirtualSystemDescriptionType_VirtualSystemDescriptionType_CloudImageId => VirtualSystemDescriptionType::CloudImageId,
            raw::VirtualSystemDescriptionType_VirtualSystemDescriptionType_CloudInstanceState => VirtualSystemDescriptionType::CloudInstanceState,
            raw::VirtualSystemDescriptionType_VirtualSystemDescriptionType_CloudImageState => VirtualSystemDescriptionType::CloudImageState,
            raw::VirtualSystemDescriptionType_VirtualSystemDescriptionType_CloudInstanceDisplayName => VirtualSystemDescriptionType::CloudInstanceDisplayName,
            raw::VirtualSystemDescriptionType_VirtualSystemDescriptionType_CloudImageDisplayName => VirtualSystemDescriptionType::CloudImageDisplayName,
            raw::VirtualSystemDescriptionType_VirtualSystemDescriptionType_CloudOCILaunchMode => VirtualSystemDescriptionType::CloudOCILaunchMode,
            raw::VirtualSystemDescriptionType_VirtualSystemDescriptionType_CloudPrivateIP => VirtualSystemDescriptionType::CloudPrivateIP,
            raw::VirtualSystemDescriptionType_VirtualSystemDescriptionType_CloudBootVolumeId => VirtualSystemDescriptionType::CloudBootVolumeId,
            raw::VirtualSystemDescriptionType_VirtualSystemDescriptionType_CloudOCIVCNCompartment => VirtualSystemDescriptionType::CloudOCIVCNCompartment,
            raw::VirtualSystemDescriptionType_VirtualSystemDescriptionType_CloudOCISubnetCompartment => VirtualSystemDescriptionType::CloudOCISubnetCompartment,
            raw::VirtualSystemDescriptionType_VirtualSystemDescriptionType_CloudPublicSSHKey => VirtualSystemDescriptionType::CloudPublicSSHKey,
            raw::VirtualSystemDescriptionType_VirtualSystemDescriptionType_BootingFirmware => VirtualSystemDescriptionType::BootingFirmware,
            raw::VirtualSystemDescriptionType_VirtualSystemDescriptionType_CloudInitScriptPath => VirtualSystemDescriptionType::CloudInitScriptPath,
            #[cfg(not(is_v_6_1))]
            raw::VirtualSystemDescriptionType_VirtualSystemDescriptionType_CloudCompartmentId => VirtualSystemDescriptionType::CloudCompartmentId,
            #[cfg(not(is_v_6_1))]
            raw::VirtualSystemDescriptionType_VirtualSystemDescriptionType_CloudShapeCpus => VirtualSystemDescriptionType::CloudShapeCpus,
            #[cfg(not(is_v_6_1))]
            raw::VirtualSystemDescriptionType_VirtualSystemDescriptionType_CloudShapeMemory => VirtualSystemDescriptionType::CloudShapeMemory,
            #[cfg(not(is_v_6_1))]
            raw::VirtualSystemDescriptionType_VirtualSystemDescriptionType_CloudInstanceMetadata => VirtualSystemDescriptionType::CloudInstanceMetadata,
            #[cfg(not(is_v_6_1))]
            raw::VirtualSystemDescriptionType_VirtualSystemDescriptionType_CloudInstanceFreeFormTags => VirtualSystemDescriptionType::CloudInstanceFreeFormTags,
            #[cfg(not(is_v_6_1))]
            raw::VirtualSystemDescriptionType_VirtualSystemDescriptionType_CloudImageFreeFormTags => VirtualSystemDescriptionType::CloudImageFreeFormTags,
            raw::VirtualSystemDescriptionType_VirtualSystemDescriptionType_HardDiskControllerVirtioSCSI => VirtualSystemDescriptionType::HardDiskControllerVirtioSCSI,
            raw::VirtualSystemDescriptionType_VirtualSystemDescriptionType_HardDiskControllerNVMe => VirtualSystemDescriptionType::HardDiskControllerNVMe,
            #[cfg(is_v_7_1_or_newer)]
            raw::VirtualSystemDescriptionType_VirtualSystemDescriptionType_NVRAM => VirtualSystemDescriptionType::NVRAM,
            _ => {
                error!(
                    "VirtualSystemDescriptionType::from. Unknown type: {}",
                    value
                );
                VirtualSystemDescriptionType::Ignore
            }
        }
    }
}

impl Display for VirtualSystemDescriptionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self))
    }
}
