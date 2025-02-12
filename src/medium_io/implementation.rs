use crate::enums::{MediumVariant, PartitionTableType};
use crate::utility::macros::macros::{
    get_function_result_bool, get_function_result_number, get_function_result_pointer,
    get_function_result_unit,
};
use crate::utility::string_to_c_u64_str;
use crate::{DataStream, Medium, MediumIO, Progress, VFSExplorer, VboxError};
use vbox_raw::sys_lib::{IMedium, IProgress, IVFSExplorer};

impl MediumIO {
    /// The open medium.
    ///
    /// # Returns
    ///
    /// Returns [`Medium`], or a [`VboxError`] on failure.
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
    /// let medium_io = medium.open_for_io(true, "").unwrap();
    /// let medium1 = medium_io.get_medium().unwrap();
    /// ```
    pub fn get_medium(&self) -> Result<Medium, VboxError> {
        let medium = get_function_result_pointer!(self.object, GetMedium, *mut IMedium)?;
        Ok(Medium::new(medium))
    }

    /// The open medium.
    ///
    /// # Returns
    ///
    /// Returns bool, or a [`VboxError`] on failure.
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
    /// let medium_io = medium.open_for_io(true, "").unwrap();
    /// let writable = medium_io.get_writable().unwrap();
    /// ```
    pub fn get_writable(&self) -> Result<bool, VboxError> {
        get_function_result_bool!(self.object, GetWritable)
    }

    /// The open medium.
    ///
    /// # Returns
    ///
    /// Returns [`Medium`], or a [`VboxError`] on failure.
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
    /// let medium_io = medium.open_for_io(true, "").unwrap();
    /// let explorer = medium_io.get_explorer().unwrap();
    /// ```
    pub fn get_explorer(&self) -> Result<VFSExplorer, VboxError> {
        let explorer = get_function_result_pointer!(self.object, GetExplorer, *mut IVFSExplorer)?;
        Ok(VFSExplorer::new(explorer))
    }

    /// Read data from the medium.
    ///
    /// # Arguments
    /// * `offset` - The offset in bytes from where to start reading.
    /// * `size` - The number of bytes to read.
    ///
    /// # Returns
    ///
    /// Returns [`&[u8]`], or a [`VboxError`] on failure.
    ///
    /// - Array of data read. This may be shorter than the specified size.
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
    /// let medium_io = medium.open_for_io(true, "").unwrap();
    /// let data = medium_io.read(0, 1024).unwrap();
    /// ```
    pub fn read(&self, offset: i64, size: u32) -> Result<&[u8], VboxError> {
        let mut data_size: u32 = 0;
        let data_ptr =
            get_function_result_pointer!(self.object, Read, *mut u8, offset, size, &mut data_size)?;
        let data_slice = unsafe { std::slice::from_raw_parts(data_ptr, size as usize) };
        Ok(data_slice)
    }

    /// Write data to the medium.
    ///
    /// # Arguments
    ///
    /// * `offset` - The byte offset into the medium to start reading at.
    ///
    /// * `data` - The data to write.
    ///
    /// # Returns
    ///
    /// Returns u32, or a [`VboxError`] on failure.
    /// - How many bytes were actually written.
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
    /// let medium_io = medium.open_for_io(true, "").unwrap();
    /// let written = medium_io.write(0, &mut [0,0,0,0,]).unwrap();
    /// ```
    pub fn write(&self, offset: i64, data: &mut [u8]) -> Result<u32, VboxError> {
        let data_size = data.len() as u32;
        let data_ptr = data.as_mut_ptr();
        get_function_result_number!(self.object, Write, u32, offset, data_size, data_ptr)
    }

    /// Formats the medium as FAT.
    ///
    /// Generally only useful for floppy images as no partition table will be created.
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
    /// let mediums = vbox.get_floppy_images().unwrap();
    /// let medium = mediums.get(0).unwrap();
    /// let medium_io = medium.open_for_io(true, "").unwrap();
    /// medium_io.format_fat(true).unwrap();
    /// ```
    pub fn format_fat(&self, quick: bool) -> Result<(), VboxError> {
        let quick = if quick { 1 } else { 0 };
        get_function_result_unit!(self.object, FormatFAT, quick)
    }

    /// Writes an empty partition table to the disk.
    ///
    ///  # Arguments
    ///
    /// * `format` - The partition table format.
    /// * `whole_disk_in_one_entry` - When true a partition table entry for the whole disk is created. Otherwise the partition table is empty.
    ///
    /// # Returns
    ///
    /// Returns (), or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::enums::PartitionTableType;
    /// use virtualbox_rs::VirtualBox;
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let mediums = vbox.get_floppy_images().unwrap();
    /// let medium = mediums.get(0).unwrap();
    /// let medium_io = medium.open_for_io(true, "").unwrap();
    /// medium_io.initialize_partition_table(PartitionTableType::MBR, true).unwrap();
    /// ```
    pub fn initialize_partition_table(
        &self,
        format: PartitionTableType,
        whole_disk_in_one_entry: bool,
    ) -> Result<(), VboxError> {
        let format = format.into();
        let whole_disk_in_one_entry = if whole_disk_in_one_entry { 1 } else { 0 };
        get_function_result_unit!(
            self.object,
            InitializePartitionTable,
            format,
            whole_disk_in_one_entry
        )
    }

    /// Converts the currently opened image into a stream of the specified image type/variant.
    ///
    /// It is sufficient to open the image in read-only mode. Only few types and variants are supported due to the inherent restrictions of the output style.
    ///
    /// # Arguments
    ///
    /// * `format` - Identifier of the storage format to use for output.
    /// * `variants` - The partition table format.
    /// * `buffer_size` - Requested buffer size (in bytes) for efficient conversion. Sizes which are too small or too large are silently truncated to suitable values. Tens to hundreds of Megabytes are a good choice.
    ///
    /// # Returns
    ///
    /// Returns (stream, progress), or a [`VboxError`] on failure.
    /// - stream: The stream object representing the converted image.
    /// - progress: Progress object to track the conversion progress.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::enums::PartitionTableType;
    /// use virtualbox_rs::VirtualBox;
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let mediums = vbox.get_floppy_images().unwrap();
    /// let medium = mediums.get(0).unwrap();
    /// let medium_io = medium.open_for_io(true, "").unwrap();
    /// let (stream, progress) = medium_io.convert_to_stream("VDI", vec![], 1024).unwrap();
    /// progress.wait_for_completion(-1).unwrap()
    /// ```
    pub fn convert_to_stream(
        &self,
        format: &str,
        variants: Vec<MediumVariant>,
        buffer_size: u32,
    ) -> Result<(DataStream, Progress), VboxError> {
        let format = string_to_c_u64_str(format)?;
        let mut variant: Vec<u32> = variants.iter().map(|f| (*f).into()).collect();
        let variant_size = variant.len() as u32;
        let variant_ptr = variant.as_mut_ptr();
        let mut stream = std::ptr::null_mut();
        let progress = get_function_result_pointer!(
            self.object,
            ConvertToStream,
            *mut IProgress,
            format,
            variant_size,
            variant_ptr,
            buffer_size,
            &mut stream
        )?;
        Ok((DataStream::new(stream), Progress::new(progress)))
    }


    /// Explictly close the medium I/O rather than waiting for garbage collection and the destructor.
    ///
    /// This will wait for any pending reads and writes to complete and then close down the I/O access without regard for open explorer instances or anything like that.
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
    /// let mediums = vbox.get_floppy_images().unwrap();
    /// let medium = mediums.get(0).unwrap();
    /// let medium_io = medium.open_for_io(true, "").unwrap();
    /// medium_io.close().unwrap();
    /// ```
    pub fn close(&self) -> Result<(), VboxError> {
        get_function_result_unit!(self.object, Close)
    }
}
