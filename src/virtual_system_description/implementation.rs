use crate::enums::{VirtualSystemDescriptionType, VirtualSystemDescriptionValueType};
use crate::utility::c_u64_str_to_string;
use crate::utility::macros::macros::{get_function_result_number, get_function_result_str_vec, get_function_result_unit};
use crate::virtual_system_description::VirtualSystemDescriptionInfo;
use crate::{VboxError, VirtualSystemDescription};

impl VirtualSystemDescription {
    //TODO:
    ///
    ///
    ///
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::VirtualBox;
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let vm = vbox.find_machines("Freebsd_15").unwrap();
    /// let appliance = vbox.create_appliance().unwrap();
    /// let description = vm.export_to(&appliance, "").unwrap();
    /// let count = description.get_count().unwrap();
    /// ```

    pub fn get_count(&self) -> Result<u32, VboxError> {
        get_function_result_number!(self.object, GetCount, u32)
    }

    /// Retrieves the description of the virtual system, which includes the mapping between OVF elements and VirtualBox configuration.
    ///
    /// # Returns
    ///
    /// On success, returns a vector of [`VirtualSystemDescriptionInfo`] containing the description details. On failure, returns a [`VboxError`].
    ///
    /// # Example
    ///
    /// ```no_run
    /// use virtualbox_rs::{Appliance, VirtualBox};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let vm = vbox.find_machines("Freebsd_15").unwrap();
    /// let appliance = Appliance::init().unwrap();
    /// let description = vm.export_to(&appliance, "").unwrap();
    /// let description_info = description.get_description().unwrap();
    /// ```
    pub fn get_description(&self) -> Result<Vec<VirtualSystemDescriptionInfo>, VboxError> {
        let mut types_size: u32 = 0;
        let mut types_ptr: *mut u32 = std::ptr::null_mut();
        let mut refs_size: u32 = 0;
        let mut refs_ptr: *mut *mut u16 = std::ptr::null_mut();
        let mut ovf_values_size: u32 = 0;
        let mut ovf_values_ptr: *mut *mut u16 = std::ptr::null_mut();
        let mut vbox_values_size: u32 = 0;
        let mut vbox_values_ptr: *mut *mut u16 = std::ptr::null_mut();
        let mut extra_config_values_size: u32 = 0;
        let mut extra_config_values_ptr: *mut *mut u16 = std::ptr::null_mut();
        get_function_result_unit!(
            self.object,
            GetDescription,
            &mut types_size,
            &mut types_ptr,
            &mut refs_size,
            &mut refs_ptr,
            &mut ovf_values_size,
            &mut ovf_values_ptr,
            &mut vbox_values_size,
            &mut vbox_values_ptr,
            &mut extra_config_values_size,
            &mut extra_config_values_ptr
        )?;

        convert_ptr_to_virtual_system_description_info(
            types_size,
            types_ptr,
            refs_size,
            refs_ptr,
            ovf_values_size,
            ovf_values_ptr,
            vbox_values_size,
            vbox_values_ptr,
            extra_config_values_size,
            extra_config_values_ptr,
        )
    }

    /// Retrieves the description of the virtual system filtered by a specific type, which includes the mapping between OVF elements and VirtualBox configuration for that type.
    ///
    /// # Arguments
    /// * `type_` - The type of the description to filter by, represented as a [`VirtualSystemDescriptionType`].
    ///
    /// # Returns
    ///
    /// On success, returns a vector of [`VirtualSystemDescriptionInfo`] containing the description details. On failure, returns a [`VboxError`].
    ///
    /// # Example
    ///
    /// ```no_run
    /// use virtualbox_rs::enums::VirtualSystemDescriptionType;
    /// use virtualbox_rs::{Appliance, VirtualBox};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let vm = vbox.find_machines("Freebsd_15").unwrap();
    /// let appliance = Appliance::init().unwrap();
    /// let description = vm.export_to(&appliance, "").unwrap();
    /// let description_info = description.get_description_by_type(VirtualSystemDescriptionType::HardDiskImage).unwrap();
    /// ```
    pub fn get_description_by_type(
        &self,
        type_: VirtualSystemDescriptionType,
    ) -> Result<Vec<VirtualSystemDescriptionInfo>, VboxError> {
        let type_: u32 = type_.into();
        let mut types_size: u32 = 0;
        let mut types_ptr: *mut u32 = std::ptr::null_mut();
        let mut refs_size: u32 = 0;
        let mut refs_ptr: *mut *mut u16 = std::ptr::null_mut();
        let mut ovf_values_size: u32 = 0;
        let mut ovf_values_ptr: *mut *mut u16 = std::ptr::null_mut();
        let mut vbox_values_size: u32 = 0;
        let mut vbox_values_ptr: *mut *mut u16 = std::ptr::null_mut();
        let mut extra_config_values_size: u32 = 0;
        let mut extra_config_values_ptr: *mut *mut u16 = std::ptr::null_mut();
        get_function_result_unit!(
            self.object,
            GetDescriptionByType,
            type_,
            &mut types_size,
            &mut types_ptr,
            &mut refs_size,
            &mut refs_ptr,
            &mut ovf_values_size,
            &mut ovf_values_ptr,
            &mut vbox_values_size,
            &mut vbox_values_ptr,
            &mut extra_config_values_size,
            &mut extra_config_values_ptr
        )?;

        convert_ptr_to_virtual_system_description_info(
            types_size,
            types_ptr,
            refs_size,
            refs_ptr,
            ovf_values_size,
            ovf_values_ptr,
            vbox_values_size,
            vbox_values_ptr,
            extra_config_values_size,
            extra_config_values_ptr,
        )
    }

    /// Delete all records which are equal to the passed type from the list.
    ///
    /// # Arguments
    /// * `type_` - The type of the description to remove, represented as a [`VirtualSystemDescriptionType`].
    ///
    /// # Returns
    ///
    /// On success, returns `Ok(())`. On failure, returns a [`VboxError`].
    ///
    /// # Example
    ///
    /// ```no_run
    /// use virtualbox_rs::enums::VirtualSystemDescriptionType;
    /// use virtualbox_rs::{Appliance, VirtualBox};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let vm = vbox.find_machines("Freebsd_15").unwrap();
    /// let appliance = Appliance::init().unwrap();
    /// let description = vm.export_to(&appliance, "").unwrap();
    /// description.remove_description_by_type(VirtualSystemDescriptionType::SoundCard).unwrap();
    /// ```
    pub fn remove_description_by_type(&self, type_: VirtualSystemDescriptionType) -> Result<(), VboxError> {
        let type_ = type_.into();
        get_function_result_unit!(self.object, RemoveDescriptionByType, type_)
    }

    /// This is the same as [`VirtualSystemDescription::get_description_by_type`] except that you can specify which value types should be returned.
    ///
    /// See [`VirtualSystemDescriptionValueType`] for possible value
    ///
    /// #  Arguments
    /// * `type_` - The type of the description to filter by, represented as a [`VirtualSystemDescriptionType`].
    /// * `which` - The type of values to return, represented as a [`VirtualSystemDescriptionValueType`].
    ///
    /// # Returns
    ///
    /// On success, returns a vector of strings containing the requested values. On failure, returns a [`VboxError`].
    ///
    /// # Example
    ///
    /// ```no_run
    /// use virtualbox_rs::enums::{VirtualSystemDescriptionType, VirtualSystemDescriptionValueType};
    /// use virtualbox_rs::{Appliance, VirtualBox};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let vm = vbox.find_machines("Freebsd_15").unwrap();
    /// let appliance = Appliance::init().unwrap();
    /// let description = vm.export_to(&appliance, "").unwrap();
    /// let values = description.get_values_by_type(
    ///     VirtualSystemDescriptionType::NetworkAdapter,
    ///     VirtualSystemDescriptionValueType::ExtraConfig
    /// ).unwrap();
    /// ```
    pub fn get_values_by_type(&self, type_: VirtualSystemDescriptionType, which: VirtualSystemDescriptionValueType) -> Result<Vec<&'static str>, VboxError> {
        let type_:u32 = type_.into();
        let which: u32 = which.into();
        get_function_result_str_vec!(self.object, GetValuesByType, type_, which)
    }
}

fn convert_ptr_to_virtual_system_description_info(
    types_size: u32,
    types_ptr: *mut u32,
    refs_size: u32,
    refs_ptr: *mut *mut u16,
    ovf_values_size: u32,
    ovf_values_ptr: *mut *mut u16,
    vbox_values_size: u32,
    vbox_values_ptr: *mut *mut u16,
    extra_config_values_size: u32,
    extra_config_values_ptr: *mut *mut u16,
) -> Result<Vec<VirtualSystemDescriptionInfo>, VboxError> {
    if types_ptr.is_null()
        || refs_ptr.is_null()
        || ovf_values_ptr.is_null()
        || vbox_values_ptr.is_null()
        || extra_config_values_ptr.is_null()
    {
        return Err(VboxError::null_pointer_error(
            "VirtualSystemDescription::get_description",
        ));
    }
    if types_size != refs_size
        || types_size != ovf_values_size
        || types_size != vbox_values_size
        || types_size != extra_config_values_size
        || types_size == 0
    {
        return Err(VboxError::vectors_length_mismatch(
            "VirtualSystemDescription::get_description",
        ));
    }
    let types = unsafe { Vec::from_raw_parts(types_ptr, types_size as usize, types_size as usize) };
    let mut result = Vec::new();

    for i in 0..types_size {
        let ref_ = c_u64_str_to_string(unsafe { *refs_ptr.offset(i as isize) })?;
        let ovf_value = c_u64_str_to_string(unsafe { *ovf_values_ptr.offset(i as isize) })?;
        let vbox_value = c_u64_str_to_string(unsafe { *vbox_values_ptr.offset(i as isize) })?;
        let extra_config_value =
            c_u64_str_to_string(unsafe { *extra_config_values_ptr.offset(i as isize) })?;
        result.push(VirtualSystemDescriptionInfo {
            type_: VirtualSystemDescriptionType::from(types[i as usize]),
            ref_,
            ovf_value,
            vbox_value,
            extra_config_value,
        });
    }
    Ok(result)
}
