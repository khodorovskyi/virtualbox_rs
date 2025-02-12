use crate::utility::macros::macros::{get_function_result_number, get_function_result_pointer};
use crate::{DataStream, VboxError};

impl DataStream {
    /// Recommended size of a read.
    ///
    /// Requesting a larger read may be possible in certain situations, but it is not guaranteed.
    ///
    /// # Returns
    ///
    /// Returns u32, or a [`VboxError`] on failure.
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
    /// progress.wait_for_completion(-1).unwrap();
    /// let read_size = stream.get_read_size().unwrap();
    /// ```
    pub fn get_read_size(&self) -> Result<u32, VboxError> {
        get_function_result_number!(self.object, GetReadSize, u32)
    }

    /// Read data from the stream.
    ///
    /// # Arguments
    ///
    /// * `size` - How many bytes to try read.
    /// * `timeout_ms` - Timeout (in ms) for limiting the wait time for data to be available. Pass 0 for an infinite timeout.
    ///
    /// # Returns
    ///
    /// Returns [`&[u8]`], or a [`VboxError`] on failure.
    ///  -Array of data read. This may be shorter than the specified size. Returning a zero-sized array indicates the end of the stream, if the status is successful.
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
    /// progress.wait_for_completion(-1).unwrap();
    /// let read_size = stream.get_read_size().unwrap();
    /// let data = stream.read(read_size, 1000).unwrap();
    /// ```
    pub fn read(&self, size: u32, timeout_ms: u32) -> Result<&[u8], VboxError> {
        let mut data_size: u32 = 0;
        let data_ptr = get_function_result_pointer!(
            self.object,
            Read,
            *mut u8,
            size,
            timeout_ms,
            &mut data_size
        )?;
        let data_slice = unsafe { std::slice::from_raw_parts(data_ptr, size as usize) };
        Ok(data_slice)
    }
}
