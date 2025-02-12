use crate::enums::{DeviceType, MediumState, MediumType, MediumVariant};
use crate::utility::macros::macros::{
    get_function_result_bool, get_function_result_number, get_function_result_pointer,
    get_function_result_pointer_vec, get_function_result_str, get_function_result_str_vec,
    get_function_result_unit,
};
use crate::utility::{c_u64_str_to_string, str_vec_to_ptr, string_to_c_u64_str};
use crate::{Medium, MediumFormat, MediumIO, Progress, Token, VboxError};
use log::debug;
use vbox_raw::sys_lib::{IMedium, IMediumFormat, IMediumIO, IProgress, IToken};

#[cfg(doc)]
use crate::{SystemProperties, VirtualBox};

impl Medium {
    /// UUID of the medium.
    ///
    /// For a newly created medium, this value is a randomly generated UUID.
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
    /// let mediums = vbox.get_dvd_images().unwrap();
    /// let medium = mediums.get(0).unwrap();
    /// let id = medium.get_id().unwrap();
    /// ```
    pub fn get_id(&self) -> Result<&'static str, VboxError> {
        get_function_result_str!(self.object, GetId)
    }

    /// Optional description of the medium.
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
    /// let mediums = vbox.get_dvd_images().unwrap();
    /// let medium = mediums.get(0).unwrap();
    /// let description = medium.get_description().unwrap();
    /// ```
    pub fn get_description(&self) -> Result<&'static str, VboxError> {
        get_function_result_str!(self.object, GetDescription)
    }

    /// Optional description of the medium.
    ///
    /// # Arguments
    ///
    /// * `description` - Optional description of the medium.
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
    /// let mediums = vbox.get_dvd_images().unwrap();
    /// let medium = mediums.get(0).unwrap();
    /// medium.set_description("description").unwrap();
    /// ```
    pub fn set_description(&self, description: &str) -> Result<(), VboxError> {
        let description = string_to_c_u64_str(description)?;
        get_function_result_unit!(self.object, SetDescription, description)
    }

    /// Name of the storage unit holding medium data.
    ///
    /// # Returns
    ///
    /// Returns [`MediumState`] on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::VirtualBox;
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let mediums = vbox.get_dvd_images().unwrap();
    /// let medium = mediums.get(0).unwrap();
    /// let state = medium.get_state().unwrap();
    /// ```

    pub fn get_state(&self) -> Result<MediumState, VboxError> {
        let state = get_function_result_number!(self.object, GetState, u32)?;
        Ok(MediumState::from(state))
    }

    /// Returns the storage format variant information for this medium as an array of the flags described at MediumVariant.
    ///
    /// Before [`Medium::refresh_state`] is called this method returns an undefined value.
    ///
    /// # Returns
    ///
    /// Returns [`MediumVariant`] on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::VirtualBox;
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let mediums = vbox.get_dvd_images().unwrap();
    /// let medium = mediums.get(0).unwrap();
    /// let variant = medium.get_variant().unwrap();
    /// ```

    pub fn get_variant(&self) -> Result<Vec<MediumVariant>, VboxError> {
        let state = get_function_result_pointer_vec!(self.object, GetVariant, u32)?;
        let mut result = Vec::new();
        for item in state {
            result.push(MediumVariant::from(item));
        }
        Ok(result)
    }

    /// OLocation of the storage unit holding medium data.
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
    /// let mediums = vbox.get_dvd_images().unwrap();
    /// let medium = mediums.get(0).unwrap();
    /// let location = medium.get_location().unwrap();
    /// ```
    pub fn get_location(&self) -> Result<&'static str, VboxError> {
        get_function_result_str!(self.object, GetLocation)
    }

    /// Location of the storage unit holding medium data.
    ///
    /// The format of the location string is medium type specific. For medium types using regular files in a host's file system, the location string is the full file name.
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
    /// let mediums = vbox.get_dvd_images().unwrap();
    /// let medium = mediums.get(0).unwrap();
    /// let location = medium.get_location().unwrap();
    /// ```
    pub fn set_location(&self, location: &str) -> Result<(), VboxError> {
        let location = string_to_c_u64_str(location)?;
        get_function_result_unit!(self.object, SetLocation, location)
    }

    /// Name of the storage unit holding medium data.
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
    /// let mediums = vbox.get_dvd_images().unwrap();
    /// let medium = mediums.get(0).unwrap();
    /// let name = medium.get_name().unwrap();
    /// ```
    pub fn get_name(&self) -> Result<&'static str, VboxError> {
        get_function_result_str!(self.object, GetName)
    }

    /// Kind of device (DVD/Floppy/HardDisk) which is applicable to this medium.
    ///
    /// # Returns
    ///
    /// Returns [`DeviceType`] on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::VirtualBox;
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let mediums = vbox.get_dvd_images().unwrap();
    /// let medium = mediums.get(0).unwrap();
    /// let device_type = medium.get_device_type().unwrap();
    /// ```

    pub fn get_device_type(&self) -> Result<DeviceType, VboxError> {
        let state = get_function_result_number!(self.object, GetDeviceType, u32)?;
        Ok(DeviceType::from(state))
    }

    /// True if this corresponds to a drive on the host.
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
    /// let mediums = vbox.get_dvd_images().unwrap();
    /// let medium = mediums.get(0).unwrap();
    /// let host_drive = medium.get_host_drive().unwrap();
    /// ```

    pub fn get_host_drive(&self) -> Result<bool, VboxError> {
        get_function_result_bool!(self.object, GetHostDrive)
    }

    /// Physical size of the storage unit used to hold medium data (in bytes).
    ///
    /// # Returns
    ///
    /// Returns i64 on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::VirtualBox;
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let mediums = vbox.get_dvd_images().unwrap();
    /// let medium = mediums.get(0).unwrap();
    /// let size = medium.get_size().unwrap();
    /// ```

    pub fn get_size(&self) -> Result<i64, VboxError> {
        get_function_result_number!(self.object, GetSize, i64)
    }

    /// Storage format of this medium.
    ///
    /// The value of this attribute is a string that specifies a backend used to store medium data. The storage format is defined when you create a new medium or automatically detected when you open an existing medium, and cannot be changed later.
    ///
    /// The list of all storage formats supported by this VirtualBox installation can be obtained using [`SystemProperties::get_medium_formats`].
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
    /// let mediums = vbox.get_dvd_images().unwrap();
    /// let medium = mediums.get(0).unwrap();
    /// let format = medium.get_format().unwrap();
    /// ```

    pub fn get_format(&self) -> Result<&'static str, VboxError> {
        get_function_result_str!(self.object, GetFormat)
    }

    /// Storage format of this medium.
    ///
    /// The value of this attribute is a string that specifies a backend used to store medium data. The storage format is defined when you create a new medium or automatically detected when you open an existing medium, and cannot be changed later.
    ///
    /// The list of all storage formats supported by this VirtualBox installation can be obtained using [`SystemProperties::get_medium_formats`].
    ///
    /// # Returns
    ///
    /// Returns [`MediumFormat`] on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::VirtualBox;
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let mediums = vbox.get_dvd_images().unwrap();
    /// let medium = mediums.get(0).unwrap();
    /// let medium_format = medium.get_medium_format().unwrap();
    /// ```

    pub fn get_medium_format(&self) -> Result<MediumFormat, VboxError> {
        let medium_format =
            get_function_result_pointer!(self.object, GetMediumFormat, *mut IMediumFormat)?;
        Ok(MediumFormat::new(medium_format))
    }

    /// Storage format of this medium.
    ///
    /// The value of this attribute is a string that specifies a backend used to store medium data. The storage format is defined when you create a new medium or automatically detected when you open an existing medium, and cannot be changed later.
    ///
    /// The list of all storage formats supported by this VirtualBox installation can be obtained using [`SystemProperties::get_medium_formats`].
    ///
    /// # Returns
    ///
    /// Returns [`MediumType`] on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::VirtualBox;
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let mediums = vbox.get_dvd_images().unwrap();
    /// let medium = mediums.get(0).unwrap();
    /// let type_ = medium.get_type().unwrap();
    /// ```

    pub fn get_type(&self) -> Result<MediumType, VboxError> {
        let type_ = get_function_result_number!(self.object, GetType, u32)?;
        Ok(MediumType::from(type_))
    }

    /// Storage format of this medium.
    ///
    /// The value of this attribute is a string that specifies a backend used to store medium data. The storage format is defined when you create a new medium or automatically detected when you open an existing medium, and cannot be changed later.
    ///
    /// The list of all storage formats supported by this VirtualBox installation can be obtained using [`SystemProperties::get_medium_formats`].
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
    /// let mediums = vbox.get_dvd_images().unwrap();
    /// let medium = mediums.get(0).unwrap();
    /// let format = medium.get_format().unwrap();
    /// ```

    pub fn set_type(&self, type_: MediumType) -> Result<(), VboxError> {
        let type_ = type_.into();
        get_function_result_unit!(self.object, SetType, type_)
    }

    /// Returns which medium types can selected for this medium.
    ///
    /// # Returns
    ///
    /// Returns [`Vec<MediumType>`] on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::VirtualBox;
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let mediums = vbox.get_dvd_images().unwrap();
    /// let medium = mediums.get(0).unwrap();
    /// let types = medium.get_allowed_types().unwrap();
    /// ```

    pub fn get_allowed_types(&self) -> Result<Vec<MediumType>, VboxError> {
        let types = get_function_result_pointer_vec!(self.object, GetAllowedTypes, u32)?;
        Ok(types.iter().map(|x| MediumType::from(*x)).collect())
    }

    /// Parent of this medium (the medium this medium is directly based on).
    ///
    /// Only differencing media have parents. For base (non-differencing) media, [`None`] is returned.
    ///
    /// # Returns
    ///
    /// Returns [`Option<Medium>`] on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::VirtualBox;
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let mediums = vbox.get_dvd_images().unwrap();
    /// let medium = mediums.get(0).unwrap();
    /// let parent = medium.get_parent().unwrap();
    /// ```

    pub fn get_parent(&self) -> Result<Option<Medium>, VboxError> {
        let parent = get_function_result_pointer!(self.object, GetParent, *mut IMedium);
        match parent {
            Ok(parent) => Ok(Some(Medium::new(parent))),
            Err(err) => {
                if err.is_null() {
                    Ok(None)
                } else {
                    Err(err)
                }
            }
        }
    }

    /// Children of this medium (all differencing media directly based on this medium).
    ///
    /// A null array is returned if this medium does not have any children.
    ///
    /// # Returns
    ///
    /// Returns [`Vec<Medium>`] on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::VirtualBox;
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let mediums = vbox.get_dvd_images().unwrap();
    /// let medium = mediums.get(0).unwrap();
    /// let children = medium.get_children().unwrap();
    ///```
    pub fn get_children(&self) -> Result<Vec<Medium>, VboxError> {
        let children = get_function_result_pointer_vec!(self.object, GetChildren, *mut IMedium)?;
        Ok(children.iter().map(|x| Medium::new(*x)).collect())
    }

    /// Base medium of this medium.
    ///
    /// If this is a differencing medium, its base medium is the medium the given medium branch starts from. For all other types of media, this property returns the medium object itself (i.e. the same object this property is read on).
    ///
    /// # Returns
    ///
    /// Returns [`Medium`] on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::VirtualBox;
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let mediums = vbox.get_dvd_images().unwrap();
    /// let medium = mediums.get(0).unwrap();
    /// let base = medium.get_base().unwrap();
    ///```
    pub fn get_base(&self) -> Result<Medium, VboxError> {
        let base = get_function_result_pointer!(self.object, GetBase, *mut IMedium)?;
        Ok(Medium::new(base))
    }

    /// Returns true if this medium is read-only and false otherwise.
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
    /// let mediums = vbox.get_dvd_images().unwrap();
    /// let medium = mediums.get(0).unwrap();
    /// let read_only = medium.get_read_only().unwrap();
    ///```
    pub fn get_read_only(&self) -> Result<bool, VboxError> {
        get_function_result_bool!(self.object, GetReadOnly)
    }

    /// Logical size of this medium (in bytes), as reported to the guest OS running inside the virtual machine this medium is attached to.
    ///
    /// The logical size is defined when the medium is created and cannot be changed later.
    ///
    /// # Returns
    ///
    /// Returns i64 on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::VirtualBox;
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let mediums = vbox.get_dvd_images().unwrap();
    /// let medium = mediums.get(0).unwrap();
    /// let logical_size = medium.get_logical_size().unwrap();
    ///```
    pub fn get_logical_size(&self) -> Result<i64, VboxError> {
        get_function_result_number!(self.object, GetLogicalSize, i64)
    }

    /// Whether this differencing medium will be automatically reset each time a virtual machine it is attached to is powered up.
    ///
    /// This attribute is automatically set to true for the last differencing image of an "immutable" medium (see MediumType).
    ///
    /// See [`Medium::reset`] for more information about resetting differencing media.
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
    /// let mediums = vbox.get_dvd_images().unwrap();
    /// let medium = mediums.get(0).unwrap();
    /// let auto_reset = medium.get_auto_reset().unwrap();
    ///```
    pub fn get_auto_reset(&self) -> Result<bool, VboxError> {
        get_function_result_bool!(self.object, GetAutoReset)
    }
    /// Whether this differencing medium will be automatically reset each time a virtual machine it is attached to is powered up.
    ///
    /// This attribute is automatically set to true for the last differencing image of an "immutable" medium (see MediumType).
    ///
    /// See [`Medium::reset`] for more information about resetting differencing media.
    ///
    /// # Arguments
    ///
    /// * `auto_reset` - Whether this differencing medium will be automatically reset each time a virtual machine it is attached to is powered up.
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
    /// let mediums = vbox.get_dvd_images().unwrap();
    /// let medium = mediums.get(0).unwrap();
    /// medium.set_auto_reset(true).unwrap();
    ///```
    pub fn set_auto_reset(&self, auto_reset: bool) -> Result<(), VboxError> {
        let auto_reset = if auto_reset { 1 } else { 0 };
        get_function_result_unit!(self.object, SetAutoReset, auto_reset)
    }

    /// Text message that represents the result of the last accessibility check performed by [`Medium::refresh_state`].
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
    /// let mediums = vbox.get_dvd_images().unwrap();
    /// let medium = mediums.get(0).unwrap();
    /// let last_access_error = medium.get_last_access_error().unwrap();
    ///```
    pub fn get_last_access_error(&self) -> Result<&'static str, VboxError> {
        get_function_result_str!(self.object, GetLastAccessError)
    }

    /// Array of UUIDs of all machines this medium is attached to.
    ///
    /// A null array is returned if this medium is not attached to any machine or to any machine's snapshot.
    ///
    /// # Returns
    ///
    /// Returns [`Vec<&str>`] on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::VirtualBox;
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let mediums = vbox.get_dvd_images().unwrap();
    /// let medium = mediums.get(0).unwrap();
    /// let machine_ids = medium.get_machine_ids().unwrap();
    ///```
    pub fn get_machine_ids(&self) -> Result<Vec<&'static str>, VboxError> {
        get_function_result_str_vec!(self.object, GetMachineIds)
    }

    /// Changes the UUID and parent UUID for a hard disk medium.
    ///
    /// # Arguments
    ///
    /// * `set_image_id` - Select whether a new image UUID is set or not.
    /// * `image_id` - New UUID for the image. If an empty string is passed, then a new UUID is automatically created, provided that setImageId is true. Specifying a zero UUID is not allowed.
    /// * `set_parent_id` - Select whether a new parent UUID is set or not.
    /// * `parent_id` - New parent UUID for the image. If an empty string is passed, then a new UUID is automatically created, provided setParentId is true. A zero UUID is valid.
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
    /// let mediums = vbox.get_dvd_images().unwrap();
    /// let medium = mediums.get(0).unwrap();
    /// medium.set_ids(false, "", false, "").unwrap();
    ///```
    pub fn set_ids(
        &self,
        set_image_id: bool,
        image_id: &str,
        set_parent_id: bool,
        parent_id: &str,
    ) -> Result<(), VboxError> {
        let set_image_id = if set_image_id { 1 } else { 0 };
        let image_id = string_to_c_u64_str(image_id)?;
        let set_parent_id = if set_parent_id { 1 } else { 0 };
        let parent_id = string_to_c_u64_str(parent_id)?;
        get_function_result_unit!(
            self.object,
            SetIds,
            set_image_id,
            image_id,
            set_parent_id,
            parent_id
        )
    }

    /// If the current medium state (see [`MediumState`]) is one of "Created", "Inaccessible" or "LockedRead", then this performs an accessibility check on the medium and sets the value of the [`Medium::get_state`] attribute accordingly; that value is also returned for convenience.
    ///
    /// For all other state values, this does not perform a refresh but returns the state only.
    ///
    /// The refresh, if performed, may take a long time (several seconds or even minutes, depending on the storage unit location and format) because it performs an accessibility check of the storage unit. This check may cause a significant delay if the storage unit of the given medium is, for example, a file located on a network share which is not currently accessible due to connectivity problems. In that case, the call will not return until a timeout interval defined by the host OS for this operation expires. For this reason, it is recommended to never read this attribute on the main UI thread to avoid making the UI unresponsive.
    ///
    /// If the last known state of the medium is "Created" and the accessibility check fails, then the state would be set to "Inaccessible", and [`Medium::get_last_access_error`] may be used to get more details about the failure. If the state of the medium is "LockedRead", then it remains the same, and a non-empty value of [`Medium::get_last_access_error`] will indicate a failed accessibility check in this case.
    ///
    /// Note that not all medium states are applicable to all medium types.
    ///
    /// # Returns
    ///
    /// Returns [`MediumState`] on success, or a [`VboxError`] on failure.
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
    /// medium.set_ids(false, "", false, "").unwrap();
    ///```
    pub fn refresh_state(&self) -> Result<MediumState, VboxError> {
        let state = get_function_result_number!(self.object, RefreshState, u32)?;
        Ok(MediumState::from(state))
    }

    /// Returns an array of UUIDs of all snapshots of the given machine where this medium is attached to.
    ///
    /// If the medium is attached to the machine in the current state, then the first element in the array will always be the ID of the queried machine (i.e. the value equal to the machineId argument), followed by snapshot IDs (if any).
    ///
    /// If the medium is not attached to the machine in the current state, then the array will contain only snapshot IDs.
    ///
    /// The returned array may be null if this medium is not attached to the given machine at all, neither in the current state nor in one of the snapshots.
    ///
    /// # Returns
    ///
    /// Returns [`Vec<&str>`] on success, or a [`VboxError`] on failure.
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
    /// let snapshot_ids = medium.get_snapshot_ids("27877b9c-da58-4778-850e-0440f4b0dad7").unwrap();
    ///```
    pub fn get_snapshot_ids(&self, machine_id: &str) -> Result<Vec<&'static str>, VboxError> {
        let machine_id = string_to_c_u64_str(machine_id)?;
        get_function_result_str_vec!(self.object, GetSnapshotIds, machine_id)
    }

    /// Locks this medium for reading.
    ///
    /// # Returns
    ///
    /// Returns [`Token`] on success, or a [`VboxError`] on failure.
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
    /// let token = medium.lock_read().unwrap();
    ///```
    pub fn lock_read(&self) -> Result<Token, VboxError> {
        let token = get_function_result_pointer!(self.object, LockRead, *mut IToken)?;
        Ok(Token::new(token))
    }

    /// Locks this medium for writing.
    ///
    /// # Returns
    ///
    /// Returns [`Token`] on success, or a [`VboxError`] on failure.
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
    ///```
    pub fn lock_write(&self) -> Result<Token, VboxError> {
        let token = get_function_result_pointer!(self.object, LockWrite, *mut IToken)?;
        Ok(Token::new(token))
    }

    /// Closes this medium.
    ///
    /// The medium must not be attached to any known virtual machine and must not have any known child media, otherwise the operation will fail.
    ///
    /// When the medium is successfully closed, it is removed from the list of registered media, but its storage unit is not deleted. In particular, this means that this medium can later be opened again using the [`VirtualBox::open_medium`] call.
    ///
    /// Note that after this method successfully returns, the given medium object becomes uninitialized. This means that any attempt to call any of its methods or attributes will fail with the "Object not ready" (E_ACCESSDENIED) error.
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
    /// medium.close().unwrap();
    ///```
    pub fn close(&self) -> Result<(), VboxError> {
        get_function_result_unit!(self.object, Close)
    }

    /// Returns the value of the custom medium property with the given name.
    ///
    /// The list of all properties supported by the given medium format can be obtained with [`MediumFormat::describe_properties`].
    ///
    /// #Arguments
    ///
    /// * `name` - Name of the property to get.
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
    /// let mediums = vbox.get_hard_disks().unwrap();
    /// let medium = mediums.get(0).unwrap();
    /// let property = medium.get_property("AllocationBlockSize").unwrap();
    ///```
    pub fn get_property(&self, name: &str) -> Result<&'static str, VboxError> {
        let name = string_to_c_u64_str(name)?;
        get_function_result_str!(self.object, GetProperty, name)
    }

    /// Sets the value of the custom medium property with the given name.
    ///
    /// The list of all properties supported by the given medium format can be obtained with [`MediumFormat::describe_properties`].
    ///
    /// #Arguments
    ///
    /// * `name` - Name of the property to get.
    /// * `value` - Property value to set.
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
    /// medium.set_property("AllocationBlockSize", "1048576").unwrap();
    ///```
    pub fn set_property(&self, name: &str, value: &str) -> Result<(), VboxError> {
        let name = string_to_c_u64_str(name)?;
        let value = string_to_c_u64_str(value)?;
        get_function_result_unit!(self.object, SetProperty, name, value)
    }

    /// Returns values for a group of properties in one call.
    ///
    /// The names of the properties to get are specified using the names argument which is a list of comma-separated property names or an empty string if all properties are to be returned.
    ///
    /// The list of all properties supported by the given medium format can be obtained with [`MediumFormat::describe_properties`].
    ///
    /// #Arguments
    ///
    /// * `name` - Name of the property to get.
    ///
    /// # Returns
    ///
    /// Returns Vec<(return_name, return_values) on success, or a [`VboxError`] on failure.
    ///
    /// * `return_name` - Names of returned properties.
    /// * `return_values` - Values of returned properties.
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
    /// let property = medium.get_properties("AllocationBlockSize").unwrap();
    ///```
    pub fn get_properties(
        &self,
        name: &str,
    ) -> Result<Vec<(&'static str, &'static str)>, VboxError> {
        let name = string_to_c_u64_str(name)?;
        let mut return_names_size = 0;
        let mut return_names_ptr = std::ptr::null_mut();
        let mut return_values_size = 0;
        let mut return_values_ptr = std::ptr::null_mut();
        get_function_result_unit!(
            self.object,
            GetProperties,
            name,
            &mut return_names_size,
            &mut return_names_ptr,
            &mut return_values_size,
            &mut return_values_ptr
        )?;
        if return_names_ptr.is_null() || return_values_ptr.is_null() {
            debug!("result.is_null. fn_name: Medium::get_properties");
            return Err(VboxError::null_pointer_error("Medium::get_properties"));
        };
        if return_names_size != return_values_size {
            return Err(VboxError::vectors_length_mismatch("Medium::get_properties"));
        };
        let mut result = Vec::new();
        for i in 0..return_names_size {
            let name = c_u64_str_to_string(unsafe { *return_names_ptr.offset(i as isize) })?;
            let value = c_u64_str_to_string(unsafe { *return_values_ptr.offset(i as isize) })?;
            result.push((name, value));
        }
        Ok(result)
    }

    /// Sets values for a group of properties in one call.
    ///
    /// The list of all properties supported by the given medium format can be obtained with [`MediumFormat::describe_properties`].
    ///
    /// # Arguments
    ///
    /// - `names_values`: A vector of tuples, where each tuple contains the property name and its corresponding value as strings.
    ///     - `&str`: The name of the property to be set.
    ///     - `&str`: The value to assign to the property.
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
    /// medium.set_properties(vec![("AllocationBlockSize", "1048576")]).unwrap();
    ///```
    pub fn set_properties(&self, names_values: Vec<(&str, &str)>) -> Result<(), VboxError> {
        let mut names = Vec::new();
        let mut values = Vec::new();
        for (name, value) in names_values {
            names.push(name);
            values.push(value);
        }
        let (names_size, names_ptr) = str_vec_to_ptr(names)?;
        let (values_size, values_ptr) = str_vec_to_ptr(values)?;
        get_function_result_unit!(
            self.object,
            SetProperties,
            names_size,
            names_ptr,
            values_size,
            values_ptr
        )
    }

    /// Starts creating a hard disk storage unit (fixed/dynamic, according to the variant flags) in the background.
    ///
    /// The previous storage unit created for this object, if any, must first be deleted using [`Medium::delete_storage`], otherwise the operation will fail.
    ///
    /// Before the operation starts, the medium is placed in [`MediumState::Creating`] state. If the create operation fails, the medium will be placed back in [`MediumState::NotCreated`] state.
    ///
    /// After the returned progress object reports that the operation has successfully completed, the medium state will be set to [`MediumState::Created`], the medium will be remembered by this VirtualBox installation and may be attached to virtual machines.
    ///
    /// # Arguments
    ///
    /// * `logical_size` - Maximum logical size of the medium in bytes.
    ///
    /// * `variant` - Exact image variant which should be created (as a combination of [`MediumVariant`] flags).
    ///
    /// # Returns
    ///
    /// Returns [`Progress`] on success, or a [`VboxError`] on failure.
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
    /// let progress = medium.create_base_storage(10000000, vec![]).unwrap();
    /// progress.wait_for_completion(-1).unwrap();
    ///```
    pub fn create_base_storage(
        &self,
        logical_size: i64,
        variant: Vec<MediumVariant>,
    ) -> Result<Progress, VboxError> {
        let mut variant: Vec<u32> = variant.iter().map(|f| (*f).into()).collect();
        let variant_size = variant.len() as u32;
        let variant_ptr = variant.as_mut_ptr();
        let progress = get_function_result_pointer!(
            self.object,
            CreateBaseStorage,
            *mut IProgress,
            logical_size,
            variant_size,
            variant_ptr
        )?;
        Ok(Progress::new(progress))
    }

    /// Starts deleting the storage unit of this medium.
    ///
    /// After the returned progress object reports that the operation is complete, the medium state will be set to [`MediumState::NotCreated`] and you will be able to use one of the storage creation methods to create it again.
    ///
    /// # Returns
    ///
    /// Returns [`Progress`] on success, or a [`VboxError`] on failure.
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
    /// let progress = medium.delete_storage().unwrap();
    /// progress.wait_for_completion(-1).unwrap();
    ///```
    pub fn delete_storage(&self) -> Result<Progress, VboxError> {
        let progress = get_function_result_pointer!(self.object, DeleteStorage, *mut IProgress)?;
        Ok(Progress::new(progress))
    }

    /// Starts creating an empty differencing storage unit based on this medium in the format and at the location defined by the target argument.
    /// The target medium must be in [`MediumState::NotCreated`] state (i.e. must not have an existing storage unit).
    ///
    /// # Arguments
    ///
    /// * `target` - Target medium.
    /// * `variants` - Exact image variant which should be created (as a combination of [`MediumVariant`] flags).
    ///
    /// # Returns
    ///
    /// Returns [`Progress`] on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::enums::{AccessMode, DeviceType};
    /// use virtualbox_rs::VirtualBox;
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let mediums = vbox.get_hard_disks().unwrap();
    /// let medium = mediums.get(0).unwrap();
    /// let medium1 = vbox
    ///     .create_medium("VDI",
    ///                    "/home/user/iso/medium1.vdi",
    ///                    AccessMode::ReadWrite,
    ///                    DeviceType::HardDisk).unwrap();
    /// let progress = medium.create_diff_storage(&medium1, vec![]).unwrap();
    /// progress.wait_for_completion(-1).unwrap();
    ///```
    pub fn create_diff_storage(
        &self,
        target: &Medium,
        variants: Vec<MediumVariant>,
    ) -> Result<Progress, VboxError> {
        let target = target.object;
        let mut variant: Vec<u32> = variants.iter().map(|f| (*f).into()).collect();
        let variant_size = variant.len() as u32;
        let variant_ptr = variant.as_mut_ptr();
        let progress = get_function_result_pointer!(
            self.object,
            CreateDiffStorage,
            *mut IProgress,
            target,
            variant_size,
            variant_ptr
        )?;
        Ok(Progress::new(progress))
    }

    /// Starts merging the contents of this medium and all intermediate differencing media in the chain to the given target medium.
    ///
    /// # Arguments
    ///
    /// * `target` - Target medium.
    ///
    /// # Returns
    ///
    /// Returns [`Progress`] on success, or a [`VboxError`] on failure.
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
    /// let medium1 = medium.get_parent().unwrap().unwrap();
    /// let progress = medium1.merge_to(&medium).unwrap();
    /// progress.wait_for_completion(-1).unwrap();
    /// ```
    pub fn merge_to(&self, target: &Medium) -> Result<Progress, VboxError> {
        let target = target.object;
        let progress = get_function_result_pointer!(self.object, MergeTo, *mut IProgress, target)?;
        Ok(Progress::new(progress))
    }

    /// Starts creating a clone of this medium in the format and at the location defined by the target argument.
    ///
    /// The target medium must be either in [`MediumState::NotCreated`] state (i.e. must not have an existing storage unit) or in [`MediumState::Created`] state (i.e. created and not locked, and big enough to hold the data or else the copy will be partial). Upon successful completion, the cloned medium will contain exactly the same sector data as the medium being cloned, except that in the first case a new UUID for the clone will be randomly generated, and in the second case the UUID will remain unchanged.
    ///
    /// The parent argument defines which medium will be the parent of the clone. Passing a [`None`] reference indicates that the clone will be a base image, i.e. completely independent. It is possible to specify an arbitrary medium for this parameter, including the parent of the medium which is being cloned. Even cloning to a child of the source medium is possible. Note that when cloning to an existing image, the parent argument is ignored.
    ///
    /// After the returned progress object reports that the operation is successfully complete, the target medium gets remembered by this VirtualBox installation and may be attached to virtual machines.
    ///
    /// # Arguments
    ///
    /// * `target` - Target medium.
    /// * `variants` - Exact image variant which should be created (as a combination of [`MediumVariant`] flags).
    /// * `parent` - Parent of the cloned medium.
    ///
    /// # Returns
    ///
    /// Returns [`Progress`] on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::enums::{AccessMode, DeviceType};
    /// use virtualbox_rs::VirtualBox;
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let mediums = vbox.get_hard_disks().unwrap();
    /// let medium = mediums.get(0).unwrap();
    /// let medium1 = vbox
    ///     .create_medium("VDI",
    ///                    "/home/user/iso/medium1.vdi",
    ///                    AccessMode::ReadWrite,
    ///                    DeviceType::HardDisk).unwrap();
    /// let progress = medium.clone_to(&medium1, vec![], None).unwrap();
    /// progress.wait_for_completion(-1).unwrap();
    /// ```
    pub fn clone_to(
        &self,
        target: &Medium,
        variants: Vec<MediumVariant>,
        parent: Option<&Medium>,
    ) -> Result<Progress, VboxError> {
        let target = target.object;
        let mut variant: Vec<u32> = variants.iter().map(|f| (*f).into()).collect();
        let variant_size = variant.len() as u32;
        let variant_ptr = variant.as_mut_ptr();
        let parent = match parent {
            Some(parent) => parent.object,
            None => std::ptr::null_mut(),
        };
        let progress = get_function_result_pointer!(
            self.object,
            CloneTo,
            *mut IProgress,
            target,
            variant_size,
            variant_ptr,
            parent
        )?;
        Ok(Progress::new(progress))
    }

    /// Starts creating a clone of this medium in the format and at the location defined by the target argument.
    ///
    /// # Arguments
    ///
    /// * `target` - Target medium.
    /// * `variants` - Exact image variant which should be created (as a combination of [`MediumVariant`] flags).
    ///
    /// # Returns
    ///
    /// Returns [`Progress`] on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::enums::{AccessMode, DeviceType};
    /// use virtualbox_rs::VirtualBox;
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let mediums = vbox.get_hard_disks().unwrap();
    /// let medium = mediums.get(0).unwrap();
    /// let medium1 = vbox
    ///     .create_medium("VDI",
    ///                    "/home/user/iso/medium1.vdi",
    ///                    AccessMode::ReadWrite,
    ///                    DeviceType::HardDisk).unwrap();
    /// let progress = medium.clone_to_base(&medium1, vec![]).unwrap();
    /// progress.wait_for_completion(-1).unwrap();
    /// ```
    pub fn clone_to_base(
        &self,
        target: &Medium,
        variants: Vec<MediumVariant>,
    ) -> Result<Progress, VboxError> {
        let target = target.object;
        let mut variant: Vec<u32> = variants.iter().map(|f| (*f).into()).collect();
        let variant_size = variant.len() as u32;
        let variant_ptr = variant.as_mut_ptr();

        let progress = get_function_result_pointer!(
            self.object,
            CloneToBase,
            *mut IProgress,
            target,
            variant_size,
            variant_ptr
        )?;
        Ok(Progress::new(progress))
    }

    /// Changes the location of this medium.
    ///
    /// # Arguments
    ///
    /// * `location` - New location.
    ///
    /// # Returns
    ///
    /// Returns [`Progress`] on success, or a [`VboxError`] on failure.
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
    /// let progress = medium.move_to("/home/user/iso/medium2.vdi").unwrap();
    /// progress.wait_for_completion(-1).unwrap();
    /// ```
    pub fn move_to(&self, location: &str) -> Result<Progress, VboxError> {
        let location = string_to_c_u64_str(location)?;
        let progress = get_function_result_pointer!(self.object, MoveTo, *mut IProgress, location)?;
        Ok(Progress::new(progress))
    }

    /// Starts compacting of this medium.
    ///
    /// This means that the medium is transformed into a possibly more compact storage representation. This potentially creates temporary images, which can require a substantial amount of additional disk space.
    ///
    /// This medium will be placed to [`MediumState::LockedWrite`] state and all its parent media (if any) will be placed to [`MediumState::LockedRead`] state for the duration of this operation.
    ///
    /// Please note that the results can be either returned straight away, or later as the result of the background operation via the object returned via the progress parameter.
    ///
    /// # Returns
    ///
    /// Returns [`Progress`] on success, or a [`VboxError`] on failure.
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
    /// let progress = medium.compact().unwrap();
    /// progress.wait_for_completion(-1).unwrap();
    /// ```
    pub fn compact(&self) -> Result<Progress, VboxError> {
        let progress = get_function_result_pointer!(self.object, Compact, *mut IProgress)?;
        Ok(Progress::new(progress))
    }

    /// Starts resizing this medium.
    ///
    /// This means that the nominal size of the medium is set to the new value. Both increasing and decreasing the size is possible, and there are no safety checks, since VirtualBox does not make any assumptions about the medium contents.
    ///
    /// Resizing usually needs additional disk space, and possibly also some temporary disk space. Note that resize does not create a full temporary copy of the medium, so the additional disk space requirement is usually much lower than using the clone operation.
    ///
    /// This medium will be placed to [`MediumState::LockedWrite`] state for the duration of this operation.
    ///
    /// Please note that the results can be either returned straight away, or later as the result of the background operation via the object returned via the progress parameter.
    /// # Arguments
    ///
    /// * `logical_size` - New nominal capacity of the medium in bytes..
    ///
    /// # Returns
    ///
    /// Returns [`Progress`] on success, or a [`VboxError`] on failure.
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
    /// let progress = medium.resize(1000000000).unwrap();
    /// progress.wait_for_completion(-1).unwrap();
    /// ```
    pub fn resize(&self, logical_size: i64) -> Result<Progress, VboxError> {
        let progress =
            get_function_result_pointer!(self.object, Resize, *mut IProgress, logical_size)?;
        Ok(Progress::new(progress))
    }

    /// Starts erasing the contents of this differencing medium.
    ///
    /// This operation will reset the differencing medium to its initial state when it does not contain any sector data and any read operation is redirected to its parent medium. This automatically gets called during VM power-up for every medium whose autoReset attribute is true.
    ///
    /// The medium will be write-locked for the duration of this operation (see [`Medium::lock_write`]).
    ///
    /// # Returns
    ///
    /// Returns [`Progress`] on success, or a [`VboxError`] on failure.
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
    /// let progress = medium.reset().unwrap();
    /// progress.wait_for_completion(-1).unwrap();
    /// ```
    pub fn reset(&self) -> Result<Progress, VboxError> {
        let progress = get_function_result_pointer!(self.object, Reset, *mut IProgress)?;
        Ok(Progress::new(progress))
    }

    /// Starts encryption of this medium.
    ///
    /// This means that the stored data in the medium is encrypted.
    ///
    /// This medium will be placed to [`MediumState::LockedWrite`] state.
    ///
    /// Please note that the results can be either returned straight away, or later as the result of the background operation via the object returned via the progress parameter.
    ///
    /// # Arguments
    ///
    /// * `current_password` - The current password the medium is protected with. Use an empty string to indicate that the medium isn't encrypted.
    /// * `cipher` - The cipher to use for encryption. An empty string indicates no encryption for the result.
    /// * `password` - The new password the medium should be protected with. An empty password and password ID will result in the medium being encrypted with the current password.
    /// * `password_id` - The ID of the new password when unlocking the medium.
    ///
    /// # Returns
    ///
    /// Returns [`Progress`] on success, or a [`VboxError`] on failure.
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
    /// let progress = medium.change_encryption(
    ///     "current_password",
    ///     "AES-XTS256-PLAIN64",
    ///     "new_password",
    ///     "new_password_id").unwrap();
    /// progress.wait_for_completion(-1).unwrap();
    /// ```
    pub fn change_encryption(
        &self,
        current_password: &str,
        cipher: &str,
        new_password: &str,
        new_password_id: &str,
    ) -> Result<Progress, VboxError> {
        let current_password = string_to_c_u64_str(current_password)?;
        let cipher = string_to_c_u64_str(cipher)?;
        let new_password = string_to_c_u64_str(new_password)?;
        let new_password_id = string_to_c_u64_str(new_password_id)?;

        let progress = get_function_result_pointer!(
            self.object,
            ChangeEncryption,
            *mut IProgress,
            current_password,
            cipher,
            new_password,
            new_password_id
        )?;
        Ok(Progress::new(progress))
    }

    /// Returns the encryption settings for this medium.
    ///
    /// # Returns
    ///
    /// Returns a tuple containing:
    /// * `cipher` - The cipher used for encryption.
    /// * `passwordId` - The ID of the password when unlocking the medium.
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
    /// let (cipher, password_id) = medium.get_encryption_settings().unwrap();
    /// ```
    pub fn get_encryption_settings(&self) -> Result<(&'static str, &'static str), VboxError> {
        let mut cipher: *mut u16 = std::ptr::null_mut();
        let password_id =
            get_function_result_str!(self.object, GetEncryptionSettings, &mut cipher)?;
        let cipher = c_u64_str_to_string(cipher)?;
        Ok((cipher, password_id))
    }

    /// Checks whether the supplied password is correct for the medium.
    ///
    ///  # Arguments
    ///
    /// * `password` - The password to check.
    ///
    /// # Returns
    ///
    /// Returns (), or a [`VboxError`] on failure.
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
    /// let (cipher, password_id) = medium.get_encryption_settings().unwrap();
    /// ```
    pub fn check_encryption_password(&self, password: &str) -> Result<(), VboxError> {
        let password = string_to_c_u64_str(password)?;
        get_function_result_unit!(self.object, CheckEncryptionPassword, password)
    }

    /// Open the medium for I/O.
    ///
    ///  # Arguments
    ///
    /// * `writable` - Set this to open the medium for both reading and writing. When not set the medium is opened readonly.
    /// * `password` - Password for accessing an encrypted medium. Must be empty if not encrypted.
    ///
    /// # Returns
    ///
    /// Returns [`MediumIO`], or a [`VboxError`] on failure.
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
    /// let medium_io = medium.open_for_io(true, "password").unwrap();
    /// ```
    pub fn open_for_io(&self, writable: bool, password: &str) -> Result<MediumIO, VboxError> {
        let writable = if writable { 1 } else { 0 };
        let password = string_to_c_u64_str(password)?;
        let medium_io = get_function_result_pointer!(
            self.object,
            OpenForIO,
            *mut IMediumIO,
            writable,
            password
        )?;
        Ok(MediumIO::new(medium_io))
    }

    #[cfg(not(is_v_6_1))]
    /// This is a helper function that combines the functionality of [`Medium::clone_to`] and [`Medium::resize`].
    ///
    /// The target medium will take the contents of the calling medium.
    ///
    ///  # Arguments
    ///
    /// * `target` - Target medium.
    /// * `logical_size` - New nominal capacity of the medium in bytes.
    /// * `variants` - Exact image variant which should be created (as a combination of [`MediumVariant`] flags).
    /// * `parent` - Parent of the cloned medium.
    ///
    /// # Returns
    ///
    /// Returns [`Progress`], or a [`VboxError`] on failure.
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
    /// let medium_io = medium.open_for_io(true, "password").unwrap();
    /// ```
    pub fn resize_and_clone_to(
        &self,
        target: Medium,
        logical_size: i64,
        variants: Vec<MediumVariant>,
        parent: Option<&Medium>,
    ) -> Result<Progress, VboxError> {
        let target = target.object;
        let mut variant: Vec<u32> = variants.iter().map(|f| (*f).into()).collect();
        let variant_size = variant.len() as u32;
        let variant_ptr = variant.as_mut_ptr();
        let parent = match parent {
            Some(parent) => parent.object,
            None => std::ptr::null_mut(),
        };

        let progress = get_function_result_pointer!(
            self.object,
            ResizeAndCloneTo,
            *mut IProgress,
            target,
            logical_size,
            variant_size,
            variant_ptr,
            parent
        )?;
        Ok(Progress::new(progress))
    }
}
#[cfg(is_v_6_1)]
impl Medium {
    /// Placeholder Method
    ///
    /// This method serves as a placeholder for versions of the API where the actual method is not available.
    /// Attempting to call this method will result in an `UnsupportedInCurrentApiVersion` error.
    /// This method is intended to ensure that the codebase can be compiled against multiple API versions
    /// without modification.
    ///
    /// Supported from API version: v7_0
    pub fn resize_and_clone_to(
        &self,
        _target: Medium,
        _logical_size: i64,
        _variants: Vec<MediumVariant>,
        _parent: Option<&Medium>,
    ) -> Result<Progress, VboxError> {
        Err(VboxError::unsupported_in_current_api_version(
            "Medium::resize_and_clone_to",
            "v7_0",
        ))
    }
}
