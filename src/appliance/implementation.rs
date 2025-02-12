use crate::enums::{ExportOptions, ImportOptions};
use crate::utility::macros::macros::{get_function_result_number, get_function_result_pointer, get_function_result_pointer_vec, get_function_result_str, get_function_result_str_vec, get_function_result_unit};
use crate::utility::{str_vec_to_ptr, string_to_c_u64_str};
use crate::{Appliance, Certificate, Progress, VFSExplorer, VboxError, VirtualSystemDescription};
use vbox_raw::sys_lib::{ICertificate, IProgress, IVFSExplorer, IVirtualSystemDescription};
#[cfg(doc)]
use crate::Machine;

impl Appliance {
    /// Path to the main file of the OVF appliance, which is either the .ovf or the .ova file passed to [`Appliance::read`] (for import) or [`Appliance::write`] (for export).
    ///
    /// This attribute is empty until one of these methods has been called.
    ///
    /// # Returns
    ///
    /// Returns &str on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::Appliance;
    /// let appliance = Appliance::init().unwrap();
    /// let progress = appliance.read("/home/user/iso/FreeBSD_14.ova").unwrap();
    /// progress.wait_for_completion(-1).unwrap();
    /// let path = appliance.get_path().unwrap();
    pub fn get_path(&self) -> Result<&'static str, VboxError> {
        get_function_result_str!(self.object, GetPath)
    }

    /// Array of virtual disk definitions.
    ///
    /// One such description exists for each disk definition in the OVF; each string array item represents one such piece of disk information, with the information fields separated by tab (\t) characters.
    ///
    /// The caller should be prepared for additional fields being appended to this string in future versions of VirtualBox and therefore check for the number of tabs in the strings returned.
    ///
    /// In the current version, the following eight fields are returned per string in the array:
    ///
    /// 1. Disk ID (unique string identifier given to disk)
    /// 2. Capacity (unsigned integer indicating the maximum capacity of the disk)
    /// 3. Populated size (optional unsigned integer indicating the current size of the disk; can be approximate; -1 if unspecified)
    /// 4. Format (string identifying the disk format, typically ["<http://www.vmware.com/specifications/vmdk.html#sparse>"])
    /// 5. Reference (where to find the disk image, typically a file name; if empty, then the disk should be created on import)
    /// 6. Image size (optional unsigned integer indicating the size of the image, which need not necessarily be the same as the values specified above, since the image may be compressed or sparse; -1 if not specified)
    /// 7. Chunk size (optional unsigned integer if the image is split into chunks; presently unsupported and always -1)
    /// 8. Compression (optional string equaling "gzip" if the image is gzip-compressed)
    ///
    /// # Returns
    ///
    /// Returns &str on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::Appliance;
    /// let appliance = Appliance::init().unwrap();
    /// let progress = appliance.read("/home/user/iso/FreeBSD_14.ova").unwrap();
    /// progress.wait_for_completion(-1).unwrap();
    /// let disks = appliance.get_disks().unwrap();
    pub fn get_disks(&self) -> Result<Vec<&'static str>, VboxError> {
        get_function_result_str_vec!(self.object, GetDisks)
    }

    /// Array of virtual system descriptions.
    ///
    /// One such description is created for each virtual system (machine) found in the OVF. This array is empty until either [`Appliance::interpret`] (for import) or [`Machine::export_to`] (for export) has been called.
    ///
    /// # Returns
    ///
    /// Returns [`Vec<VirtualSystemDescription>`] on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::Appliance;
    /// let appliance = Appliance::init().unwrap();
    /// let progress = appliance.read("/home/user/iso/FreeBSD_14.ova").unwrap();
    /// progress.wait_for_completion(-1).unwrap();
    /// appliance.interpret().unwrap();
    /// let virtual_system_descriptions = appliance.get_virtual_system_descriptions().unwrap();
    pub fn get_virtual_system_descriptions(
        &self,
    ) -> Result<Vec<VirtualSystemDescription>, VboxError> {
        let descriptions = get_function_result_pointer_vec!(
            self.object,
            GetVirtualSystemDescriptions,
            *mut IVirtualSystemDescription
        )?;
        Ok(descriptions
            .iter()
            .map(|object| VirtualSystemDescription::new(*object))
            .collect())
    }
    /// Contains the UUIDs of the machines created from the information in this appliances.
    ///
    /// This is only relevant for the import case, and will only contain data after a call to [`Appliance::import_machines`] succeeded.
    ///
    /// # Returns
    ///
    /// Returns &str on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::Appliance;
    /// let appliance = Appliance::init().unwrap();
    /// let progress = appliance.read("/home/user/iso/FreeBSD_14.ova").unwrap();
    /// progress.wait_for_completion(-1).unwrap();
    /// let machines = appliance.get_machines().unwrap();
    pub fn get_machines(&self) -> Result<Vec<&'static str>, VboxError> {
        get_function_result_str_vec!(self.object, GetMachines)
    }

    /// The X.509 signing certificate, if the imported OVF was signed, null if not signed.
    ///
    /// This is available after calling [`Appliance::read`].
    ///
    /// # Returns
    ///
    /// Returns [`Certificate`] on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::Appliance;
    /// let appliance = Appliance::init().unwrap();
    /// let certificate = appliance.get_certificate().unwrap();
    pub fn get_certificate(&self) -> Result<Certificate, VboxError> {
        let certificate =
            get_function_result_pointer!(self.object, GetCertificate, *mut ICertificate)?;
        Ok(Certificate::new(certificate))
    }

    /// Reads an OVF file into the appliance object.
    ///
    /// This method succeeds if the OVF is syntactically valid and, by itself, without errors. The mere fact that this method returns successfully does not mean that VirtualBox supports all features requested by the appliance; this can only be examined after a call to [`Appliance::interpret`].
    ///
    /// # Arguments
    ///
    /// * `file` - Name of appliance file to open (either with an .ovf or .ova extension, depending on whether the appliance is distributed as a set of files or as a single file, respectively).
    ///
    /// # Returns
    ///
    /// Returns [`Progress`] on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::Appliance;
    /// let appliance = Appliance::init().unwrap();
    /// let progress = appliance.read("/home/user/iso/FreeBSD_14.ova").unwrap();
    /// progress.wait_for_completion(-1).unwrap();
    /// ```
    pub fn read(&self, file: &str) -> Result<Progress, VboxError> {
        let file = string_to_c_u64_str(file)?;
        let progress = get_function_result_pointer!(self.object, Read, *mut IProgress, file)?;
        Ok(Progress::new(progress))
    }

    /// Interprets the OVF data that was read when the appliance was constructed.
    ///
    /// After calling this method, one can inspect the [`Appliance::get_virtual_system_descriptions`] array attribute, which will then contain one [`VirtualSystemDescription`] for each virtual machine found in the appliance.
    ///
    /// Calling this method is the second step of importing an appliance into VirtualBox; see [`Appliance`] for an overview.
    ///
    /// After calling this method, one should call [`Appliance::get_warnings`] to find out if problems were encountered during the processing which might later lead to errors.
    ///
    /// # Returns
    ///
    /// Returns () on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::Appliance;
    /// let appliance = Appliance::init().unwrap();
    /// let progress = appliance.read("/home/user/iso/FreeBSD_14.ova").unwrap();
    /// progress.wait_for_completion(-1).unwrap();
    /// appliance.interpret().unwrap();
    /// ```
    pub fn interpret(&self) -> Result<(), VboxError> {
        get_function_result_unit!(self.object, Interpret)
    }

    /// Imports the appliance into VirtualBox by creating instances of [`Machine`] and other interfaces that match the information contained in the appliance as closely as possible, as represented by the import instructions in the [`Appliance::get_virtual_system_descriptions`] array.
    ///
    /// Calling this method is the final step of importing an appliance into VirtualBox.
    ///
    /// Since importing the appliance will most probably involve copying and converting disk images, which can take a long time, this method operates asynchronously and returns an [`Progress`] object to allow the caller to monitor the progress.
    ///
    /// After the import succeeded, the UUIDs of the [`Machine`] instances created can be retrieved from the machines array attribute.
    ///
    /// # Arguments
    ///
    /// * `options` - Options for the importing operation.
    ///
    /// # Returns
    ///
    /// Returns [`Progress`] on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::Appliance;
    /// let appliance = Appliance::init().unwrap();
    /// let progress = appliance.read("/home/user/iso/FreeBSD_14.ova").unwrap();
    /// progress.wait_for_completion(-1).unwrap();
    /// appliance.interpret().unwrap();
    /// ```
    pub fn import_machines(&self, options: Vec<ImportOptions>) -> Result<Progress, VboxError> {
        let mut options: Vec<u32> = options.iter().map(|flag| (*flag).into()).collect();
        let options_ptr = options.as_mut_ptr();
        let options_size = options.len() as u32;
        let progress = get_function_result_pointer!(
            self.object,
            ImportMachines,
            *mut IProgress,
            options_size,
            options_ptr
        )?;
        Ok(Progress::new(progress))
    }

    /// Returns a [`VFSExplorer`] object for the given URI.
    ///
    /// # Arguments
    ///
    /// * `uri` - The URI describing the file system to use.
    ///
    /// # Returns
    ///
    /// Returns [`VFSExplorer`] on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::Appliance;
    /// let appliance = Appliance::init().unwrap();
    /// let progress = appliance.read("/home/user/iso/FreeBSD_14.ova").unwrap();
    /// progress.wait_for_completion(-1).unwrap();
    /// let vfs_explorer = appliance.create_vfs_explorer("/home/user/iso/FreeBSD_14.ova").unwrap();
    /// ```

    pub fn create_vfs_explorer(&self, uri: &str) -> Result<VFSExplorer, VboxError> {
        let uri = string_to_c_u64_str(uri)?;
        let vfs_explorer =
            get_function_result_pointer!(self.object, CreateVFSExplorer, *mut IVFSExplorer, uri)?;
        Ok(VFSExplorer::new(vfs_explorer))
    }

    /// Writes the contents of the appliance exports into a new OVF file.
    ///
    /// Calling this method is the final step of exporting an appliance from VirtualBox; see [`Appliance`] for an overview.
    ///
    /// Since exporting the appliance will most probably involve copying and converting disk images, which can take a long time, this method operates asynchronously and returns an [`Progress`] object to allow the caller to monitor the progress.
    ///
    /// # Arguments
    ///
    /// * `format` - Output format, as a string. Currently supported formats are "ovf-0.9", "ovf-1.0", "ovf-2.0" and "opc-1.0"; future versions of VirtualBox may support additional formats. The "opc-1.0" format is for creating tarballs for the Oracle Public Cloud.
    /// * `options` - Options for the exporting operation.
    /// * `path` - Name of appliance file to create. There are certain restrictions with regard to the file name suffix. If the format parameter is "opc-1.0" a .tar.gz suffix is required. Otherwise the suffix must either be .ovf or .ova, depending on whether the appliance is distributed as a set of files or as a single file, respectively.
    ///
    /// # Returns
    ///
    /// Returns [`Progress`] on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{Appliance, VirtualBox};
    /// let appliance = Appliance::init().unwrap();
    /// let vbox = VirtualBox::init().unwrap();
    /// let machine = vbox.find_machines("FreeBSD_14").unwrap();
    /// let _ = machine.export_to(&appliance, "").unwrap();
    /// let progress = appliance.write("ovf-2.0",vec![],"/home/user/iso/FreeBSD_14.ova").unwrap();
    /// progress.wait_for_completion(-1).unwrap()
    /// ```
    pub fn write(
        &self,
        format: &str,
        options: Vec<ExportOptions>,
        path: &str,
    ) -> Result<Progress, VboxError> {
        let format = string_to_c_u64_str(format)?;
        let mut options: Vec<u32> = options.iter().map(|flag| (*flag).into()).collect();
        let options_ptr = options.as_mut_ptr();
        let options_size = options.len() as u32;
        let path = string_to_c_u64_str(path)?;
        let progress = get_function_result_pointer!(
            self.object,
            Write,
            *mut IProgress,
            format,
            options_size,
            options_ptr,
            path
        )?;
        Ok(Progress::new(progress))
    }


    /// Returns textual warnings which occurred during execution of [`Appliance::interpret`].
    ///
    /// # Returns
    ///
    /// Returns &str on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::Appliance;
    /// let appliance = Appliance::init().unwrap();
    /// let progress = appliance.read("/home/user/iso/FreeBSD_14.ova").unwrap();
    /// progress.wait_for_completion(-1).unwrap();
    /// appliance.interpret().unwrap();
    /// let warnings = appliance.get_warnings().unwrap();
    /// ```
    pub fn get_warnings(&self) -> Result<Vec<&'static str>, VboxError> {
        get_function_result_str_vec!(self.object, GetWarnings)
    }

    /// Returns a list of password identifiers which must be supplied to import or export encrypted virtual machines.
    ///
    /// # Returns
    ///
    /// Returns [`Vec<&str>`] on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::Appliance;
    /// let appliance = Appliance::init().unwrap();
    /// let progress = appliance.read("/home/user/iso/FreeBSD_14.ova").unwrap();
    /// progress.wait_for_completion(-1).unwrap();
    /// let password_ids = appliance.get_password_ids().unwrap();
    /// ```
    pub fn get_password_ids(&self) -> Result<Vec<&'static str>, VboxError> {
        get_function_result_str_vec!(self.object, GetPasswordIds)
    }

    /// Returns a list of medium identifiers which use the given password identifier.
    ///
    /// # Arguments
    ///
    /// * `password_id` - The password identifier to get the medium identifiers for.
    ///
    /// # Returns
    ///
    /// Returns [`Vec<&str>`] on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::Appliance;
    /// let appliance = Appliance::init().unwrap();
    /// let progress = appliance.read("/home/user/iso/FreeBSD_14.ova").unwrap();
    /// progress.wait_for_completion(-1).unwrap();
    /// let medium_ids = appliance.get_medium_ids_for_password_id("password_id1").unwrap();
    /// ```
    pub fn get_medium_ids_for_password_id(
        &self,
        password_id: &str,
    ) -> Result<Vec<&'static str>, VboxError> {
        let password_id = string_to_c_u64_str(password_id)?;
        get_function_result_str_vec!(self.object, GetMediumIdsForPasswordId, password_id)
    }


    /// Adds a list of passwords required to import or export encrypted virtual machines.
    ///
    /// # Arguments
    ///
    /// * `identifiers` - List of identifiers.
    /// * `passwords` - List of matching passwords.
    ///
    /// # Returns
    ///
    /// Returns () on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::Appliance;
    /// let appliance = Appliance::init().unwrap();
    /// let progress = appliance.read("/home/user/iso/FreeBSD_14.ova").unwrap();
    /// progress.wait_for_completion(-1).unwrap();
    /// appliance.add_passwords(vec!["password_id1", "password_id2"], vec!["password1", "password2"]).unwrap();
    /// ```
    pub fn add_passwords(
        &self,
        identifiers: Vec<&str>,
        passwords: Vec<&str>,
    ) -> Result<(), VboxError> {
        let (identifiers_size, identifiers_ptr) = str_vec_to_ptr(identifiers)?;
        let (passwords_size, passwords_ptr) = str_vec_to_ptr(passwords)?;
        get_function_result_unit!(
            self.object,
            AddPasswords,
            identifiers_size,
            identifiers_ptr,
            passwords_size,
            passwords_ptr
        )
    }

    /// Creates a number of [`VirtualSystemDescription`] objects and store them in the [`Appliance::get_virtual_system_descriptions`] array.
    ///
    /// # Arguments
    ///
    /// * `requested` - Requested number of new virtual system description objects
    ///
    /// # Returns
    ///
    /// Returns u32 on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::Appliance;
    /// let appliance = Appliance::init().unwrap();
    /// let progress = appliance.read("/home/user/iso/FreeBSD_14.ova").unwrap();
    /// progress.wait_for_completion(-1).unwrap();
    /// let count = appliance.create_virtual_system_descriptions(1).unwrap();
    /// ```
    pub fn create_virtual_system_descriptions(&self, requested: u32) -> Result<u32, VboxError> {
        get_function_result_number!(self.object, CreateVirtualSystemDescriptions,u32, requested)
    }
}
