use crate::enums::BitmapFormat;
use crate::framebuffer::framebuffer_impl::IFramebufferImpl;
use crate::utility::macros::macros::{get_function_result_number, get_function_result_pointer_vec};
use crate::{Framebuffer, VboxError, VboxErrorType};
use image::{ImageBuffer, ImageEncoder, Rgba};
use std::slice;
use log::trace;

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
    //TODO

    pub fn get_capabilities(&self) -> Result<Vec<u32>, VboxError> {
        let capabilities = get_function_result_pointer_vec!(self.object, GetCapabilities, u32)?;
        Ok(capabilities)
    }

    pub fn get_image(&self) -> Result<Vec<u8>, VboxError> {
        trace!("get_image");
        match self.get_object_impl() {
            None => Err(VboxError::null_pointer_error("get_image1")),

            Some(obj) => unsafe {
                if (*obj).data.image.is_null() {
                    return Err(VboxError::null_pointer_error("get_image2"));
                }
                if (*obj).data.image_size == 0 {
                    return Err(VboxError::null_pointer_error("get_image3"));
                }
                // let screen_data_slice =
                //     slice::from_raw_parts((*obj).data.image, (*obj).data.image_size as usize);
                let screen_data_slice = raw_ptr_to_png(
                    (*obj).data.image,
                    (*obj).data.image_size as usize,
                    (*obj).data.width,
                    (*obj).data.height,
                );
                screen_data_slice
            },
        }
    }
}

unsafe fn raw_ptr_to_png(
    raw_ptr: *const u8,
    image_size: usize,
    width: u32,
    height: u32,
) -> Result<Vec<u8>, VboxError> {
    trace!("raw_ptr_to_png");
    // Проверяем указатель
    if raw_ptr.is_null() {
        return Err(VboxError::null_pointer_error("raw_ptr_to_png"));
    }

    // Ожидаемый размер массива (4 байта на пиксель для RGBA)
    let expected_size = (width * height * 4) as usize;
    if image_size != expected_size {
        VboxError::new(
            0,
            "raw_ptr_to_png",
            format!(
                "Invalid image size. Expected size: {}, Actual size: {}",
                expected_size, image_size
            ),
            Some(VboxErrorType::VectorsLengthMismatch),
        );
    }
    trace!("raw_ptr_to_png raw to slice");
    // Создаем слайс из указателя
    let image_data: &[u8] = unsafe { slice::from_raw_parts(raw_ptr, image_size) };
    trace!("raw_ptr_to_png slice to vec");
    // Преобразуем данные в ImageBuffer
    let image_buffer: ImageBuffer<Rgba<u8>, _> = ImageBuffer::from_raw(width, height, image_data)
        .ok_or("Failed to create ImageBuffer")
        .map_err(|error| VboxError::parse_error("raw_ptr_to_png", error))?;
    trace!("raw_ptr_to_png image buffer created");
    // Кодируем в PNG
    let mut png_data = Vec::new();
    {
        let mut encoder = image::codecs::png::PngEncoder::new(&mut png_data);
        encoder
            .write_image(&image_buffer, width, height, image::ColorType::Rgba8.into())
            .map_err(|e| {
                VboxError::parse_error(
                    "raw_ptr_to_png",
                    format!("PNG encoding failed: {}", e).as_str(),
                )
            })?;
    }
    trace!("raw_ptr_to_png png encoded. Size: {}", png_data.len());
    Ok(png_data)
}
