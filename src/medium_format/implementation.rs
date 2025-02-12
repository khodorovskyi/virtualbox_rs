use crate::enums::{DeviceType, MediumFormatCapabilities};
use crate::utility::c_u64_str_to_string;
use crate::utility::macros::macros::{
    get_function_result_pointer_vec, get_function_result_str, get_function_result_unit,
};
use crate::{MediumFormat, VboxError};
use log::debug;

#[cfg(doc)]
use crate::VirtualBox;

impl MediumFormat {
    /// Identifier of this format.
    ///
    /// The format identifier is a non-null non-empty ASCII string. Note that this string is case-insensitive.
    ///
    /// This string is used in methods of other interfaces where it is necessary to specify a medium format, such as [`VirtualBox::create_medium`].
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
    /// let medium_format = medium.get_medium_format().unwrap();
    /// let id = medium_format.get_id().unwrap();
    ///```
    pub fn get_id(&self) -> Result<&'static str, VboxError> {
        get_function_result_str!(self.object, GetId)
    }

    /// Human-readable description of this format.
    ///
    /// Mainly for use in file open dialogs.
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
    /// let medium_format = medium.get_medium_format().unwrap();
    /// let id = medium_format.get_id().unwrap();
    ///```
    pub fn get_name(&self) -> Result<&'static str, VboxError> {
        get_function_result_str!(self.object, GetName)
    }

    /// Capabilities of the format as an array of the flags.
    ///
    /// For the meaning of individual capability flags see [`MediumFormatCapabilities`].
    ///
    /// # Returns
    ///
    /// Returns [`Vec<MediumFormatCapabilities>`] on success, or a [`VboxError`] on failure.
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
    /// let medium_format = medium.get_medium_format().unwrap();
    /// let capabilities = medium_format.get_capabilities().unwrap();
    ///```
    pub fn get_capabilities(&self) -> Result<Vec<MediumFormatCapabilities>, VboxError> {
        let capabilities = get_function_result_pointer_vec!(self.object, GetCapabilities, u32)?;
        Ok(capabilities
            .iter()
            .map(|&x| MediumFormatCapabilities::from(x))
            .collect())
    }

    /// returns the list of supported file extensions and device types.
    ///
    /// # Returns
    ///
    /// Returns Vec<(extension, device_type) on success, or a [`VboxError`] on failure.
    ///
    ///  * `extension` - supported extensions.
    ///  * `device_type` - device type for extension.
    ///
    /// # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::VirtualBox;
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let mediums = vbox.get_hard_disks().unwrap();
    /// let medium = mediums.get(0).unwrap();
    /// let medium_format = medium.get_medium_format().unwrap();
    /// let file_extensions = medium_format.describe_file_extensions().unwrap();
    ///```
    pub fn describe_file_extensions(&self) -> Result<Vec<(&'static str, DeviceType)>, VboxError> {
        let mut extensions_ptr: *mut *mut u16 = std::ptr::null_mut();
        let mut extensions_size = 0;
        let types = get_function_result_pointer_vec!(
            self.object,
            DescribeFileExtensions,
            u32,
            &mut extensions_size,
            &mut extensions_ptr
        )?;
        if extensions_ptr.is_null() {
            debug!("result.is_null. fn_name: MediumFormat::describe_file_extensions");
            return Err(VboxError::null_pointer_error(
                "MediumFormat::describe_file_extensions",
            ));
        }
        if extensions_size != types.len() as u32 {
            return Err(VboxError::vectors_length_mismatch(
                "MediumFormat::describe_file_extensions",
            ));
        }
        let mut result = Vec::new();
        let extensions = unsafe {
            Vec::from_raw_parts(
                extensions_ptr,
                extensions_size as usize,
                extensions_size as usize,
            )
        };
        for i in 0..extensions_size {
            let extension = c_u64_str_to_string(extensions.get(i as usize).unwrap().clone())?;
            let device_type = DeviceType::from(types.get(i as usize).unwrap().clone());
            result.push((extension, device_type));
        }
        Ok(result)
    }

    /// Returns array describing the properties supported by this format.
    ///
    /// # Returns
    ///
    /// Returns Vec<(names, descriptions, device_type, flags, defaults) on success, or a [`VboxError`] on failure.
    ///
    ///  * `names` - supported extensions.
    ///  * `descriptions` - device type for extension.
    ///  * `device_type` - device type for extension.
    ///  * `flags` - device type for extension.
    ///  * `defaults` - device type for extension.
    ///
    /// # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::VirtualBox;
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let mediums = vbox.get_hard_disks().unwrap();
    /// let medium = mediums.get(0).unwrap();
    /// let medium_format = medium.get_medium_format().unwrap();
    /// let describe_properties = medium_format.describe_properties().unwrap();
    ///```
    pub fn describe_properties(
        &self,
    ) -> Result<
        Vec<(
            &'static str,
            &'static str,
            DeviceType,
            MediumFormatCapabilities,
            &'static str,
        )>,
        VboxError,
    > {
        let mut names_ptr: *mut *mut u16 = std::ptr::null_mut();
        let mut names_size = 0;
        let mut descriptions_ptr: *mut *mut u16 = std::ptr::null_mut();
        let mut descriptions_size = 0;
        let mut types_ptr: *mut u32 = std::ptr::null_mut();
        let mut types_size = 0;
        let mut flags_ptr: *mut u32 = std::ptr::null_mut();
        let mut flags_size = 0;
        let mut defaults_ptr: *mut *mut u16 = std::ptr::null_mut();
        let mut defaults_size = 0;

        get_function_result_unit!(
            self.object,
            DescribeProperties,
            &mut names_size,
            &mut names_ptr,
            &mut descriptions_size,
            &mut descriptions_ptr,
            &mut types_size,
            &mut types_ptr,
            &mut flags_size,
            &mut flags_ptr,
            &mut defaults_size,
            &mut defaults_ptr
        )?;
        if names_ptr.is_null()
            || descriptions_ptr.is_null()
            || types_ptr.is_null()
            || flags_ptr.is_null()
            || defaults_ptr.is_null()
        {
            debug!("result.is_null. fn_name: MediumFormat::describe_properties");
            return Err(VboxError::null_pointer_error(
                "MediumFormat::describe_properties",
            ));
        }
        if names_size != defaults_size
            || descriptions_size != defaults_size
            || types_size != defaults_size
            || flags_size != defaults_size
        {
            return Err(VboxError::vectors_length_mismatch(
                "MediumFormat::describe_file_extensions",
            ));
        }
        let mut result = Vec::new();
        let names =
            unsafe { Vec::from_raw_parts(names_ptr, names_size as usize, names_size as usize) };
        let descriptions = unsafe {
            Vec::from_raw_parts(
                descriptions_ptr,
                descriptions_size as usize,
                descriptions_size as usize,
            )
        };
        let types =
            unsafe { Vec::from_raw_parts(types_ptr, types_size as usize, types_size as usize) };
        let flags =
            unsafe { Vec::from_raw_parts(flags_ptr, flags_size as usize, flags_size as usize) };

        let defaults_ = unsafe {
            Vec::from_raw_parts(defaults_ptr, defaults_size as usize, defaults_size as usize)
        };

        for i in 0..names_size {
            let name = c_u64_str_to_string(names.get(i as usize).unwrap().clone())?;
            let description = c_u64_str_to_string(descriptions.get(i as usize).unwrap().clone())?;
            let device_type = DeviceType::from(types.get(i as usize).unwrap().clone());
            let flag = MediumFormatCapabilities::from(flags.get(i as usize).unwrap().clone());
            let default_ = c_u64_str_to_string(defaults_.get(i as usize).unwrap().clone())?;
            result.push((name, description, device_type, flag, default_));
        }
        Ok(result)
    }
}
