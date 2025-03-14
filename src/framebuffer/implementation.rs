use crate::enums::BitmapFormat;
use crate::utility::macros::macros::get_function_result_number;
use crate::{Framebuffer, VboxError};

impl Framebuffer {
    /// Frame buffer width, in pixels.
    ///
    /// Returns u32 on success, or a [`VboxError`] on failure.
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
    ///
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    ///
    /// let console = session.get_console().unwrap();
    ///
    /// let mut display = console.get_display().unwrap();
    /// let framebuffer = display.query_framebuffer(0).unwrap();
    /// let width = framebuffer.get_width().unwrap();
    pub fn get_width(&self) -> Result<u32, VboxError> {
        get_function_result_number!(self.object, GetWidth, u32)
    }

    /// Frame buffer height, in pixels.
    ///
    /// Returns u32 on success, or a [`VboxError`] on failure.
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
    ///
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    ///
    /// let console = session.get_console().unwrap();
    ///
    /// let mut display = console.get_display().unwrap();
    /// let framebuffer = display.query_framebuffer(0).unwrap();
    /// let height = framebuffer.get_height().unwrap();
    pub fn get_height(&self) -> Result<u32, VboxError> {
        get_function_result_number!(self.object, GetHeight, u32)
    }

    /// Color depth, in bits per pixel.
    ///
    /// Returns u32 on success, or a [`VboxError`] on failure.
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
    ///
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    ///
    /// let console = session.get_console().unwrap();
    ///
    /// let mut display = console.get_display().unwrap();
    /// let framebuffer = display.query_framebuffer(0).unwrap();
    /// let bits_per_pixel = framebuffer.get_bits_per_pixel().unwrap();
    pub fn get_bits_per_pixel(&self) -> Result<u32, VboxError> {
        get_function_result_number!(self.object, GetBitsPerPixel, u32)
    }

    /// Scan line size, in bytes.
    ///
    /// Returns u32 on success, or a [`VboxError`] on failure.
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
    ///
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    ///
    /// let console = session.get_console().unwrap();
    ///
    /// let mut display = console.get_display().unwrap();
    /// let framebuffer = display.query_framebuffer(0).unwrap();
    /// let bytes_per_line = framebuffer.get_bytes_per_line().unwrap();
    pub fn get_bytes_per_line(&self) -> Result<u32, VboxError> {
        get_function_result_number!(self.object, GetBytesPerLine, u32)
    }

    /// Frame buffer pixel format.
    ///
    /// Returns [`BitmapFormat`] on success, or a [`VboxError`] on failure.
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
    ///
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    ///
    /// let console = session.get_console().unwrap();
    ///
    /// let mut display = console.get_display().unwrap();
    /// let framebuffer = display.query_framebuffer(0).unwrap();
    /// let pixel_format = framebuffer.get_pixel_format().unwrap();
    pub fn get_pixel_format(&self) -> Result<BitmapFormat, VboxError> {
        let pixel_format = get_function_result_number!(self.object, GetPixelFormat, u32)?;
        Ok(BitmapFormat::from(pixel_format))
    }

    /// Hint from the frame buffer about how much of the standard screen height it wants to use for itself.
    ///
    /// This information is exposed to the guest through the VESA BIOS and VMMDev interface so that it can use it for determining its video mode table. It is not guaranteed that the guest respects the value.
    ///
    /// Returns u32 on success, or a [`VboxError`] on failure.
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
    ///
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    ///
    /// let console = session.get_console().unwrap();
    ///
    /// let mut display = console.get_display().unwrap();
    /// let framebuffer = display.query_framebuffer(0).unwrap();
    /// let height_reduction = framebuffer.get_height_reduction().unwrap();
    pub fn get_height_reduction(&self) -> Result<u32, VboxError> {
        get_function_result_number!(self.object, GetHeightReduction, u32)
    }

    /// Hint from the frame buffer about how much of the standard screen height it wants to use for itself.
    ///
    /// This information is exposed to the guest through the VESA BIOS and VMMDev interface so that it can use it for determining its video mode table. It is not guaranteed that the guest respects the value.
    ///
    /// Returns i64 on success, or a [`VboxError`] on failure.
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
    ///
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    ///
    /// let console = session.get_console().unwrap();
    ///
    /// let mut display = console.get_display().unwrap();
    /// let framebuffer = display.query_framebuffer(0).unwrap();
    /// let win_id = framebuffer.get_win_id().unwrap();

    pub fn get_win_id(&self) -> Result<i64, VboxError> {
        get_function_result_number!(self.object, GetWinId, i64)
    }
}
