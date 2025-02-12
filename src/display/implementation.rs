use crate::display::{Display, Resolution};
use crate::enums::BitmapFormat;
use crate::utility::macros::macros::{
    get_function_result_pointer, get_function_result_str, get_function_result_unit,
};
use crate::utility::string_to_c_u64_str;
use crate::{Framebuffer, VboxError};
use std::slice;
use vbox_raw::sys_lib::{IFramebuffer, PRUint8};

impl Display {
    /// Queries certain attributes such as display width, height, color depth and the X and Y origin for a given guest screen.
    ///
    /// # Arguments
    ///
    /// * `screen_id` -	The guest monitor to take screenshot from.
    ///
    /// # Returns
    ///
    /// Returns A [`Resolution`] success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use std::fs::File;
    /// use std::io::Write;
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
    /// let display = console.get_display().unwrap();
    ///
    /// let resolution = display.get_screen_resolution(0).unwrap();
    ///
    /// println!("{:?}", resolution);

    pub fn get_screen_resolution(&self, screen_id: u32) -> Result<Resolution, VboxError> {
        let mut width: u32 = 0;
        let mut height: u32 = 0;
        let mut bits_per_pixel: u32 = 0;
        let mut x_origin: i32 = 0;
        let mut y_origin: i32 = 0;
        let mut guest_monitor_status: u32 = 0;
        get_function_result_unit!(
            self.object,
            GetScreenResolution,
            screen_id,
            &mut width,
            &mut height,
            &mut bits_per_pixel,
            &mut x_origin,
            &mut y_origin,
            &mut guest_monitor_status
        )?;
        Ok(Resolution {
            width,
            height,
            bits_per_pixel,
            x_origin,
            y_origin,
            guest_monitor_status,
        })
    }

    /// Sets the graphics update target for a screen.
    ///
    /// # Arguments
    ///
    /// * `screen_id` - The guest monitor to take screenshot from.
    ///
    /// # Returns
    ///
    /// Returns ([`Framebuffer`], &str) on success, or a [`VboxError`] on failure.
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
    /// let framebuffer = display.attach_framebuffer(0);

    pub fn attach_framebuffer(
        &mut self,
        screen_id: u32,
    ) -> Result<(Framebuffer, &'static str), VboxError> {
        let framebuffer: *mut IFramebuffer = std::ptr::null_mut();

        let framebuffer_id_str =
            get_function_result_str!(self.object, AttachFramebuffer, screen_id, framebuffer)?;
        let mut framebuffer_ids = self
            .framebuffer_ids
            .lock()
            .map_err(|err| VboxError::new(0, "attach_framebuffer", err.to_string(), None))?;
        framebuffer_ids.insert(framebuffer_id_str, screen_id);
        Ok((Framebuffer::new(framebuffer), framebuffer_id_str))
    }

    /// Removes the graphics updates target for a screen.
    ///
    /// # Arguments
    ///
    /// * `screen_id` - The guest monitor to take screenshot from.
    ///
    /// * `framebuffer_id` - wstringUUID
    ///
    /// # Returns
    ///
    /// Returns Ok() on success, or a [`VboxError`] on failure.
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
    /// display.detach_framebuffer(0, "2d2e6264-169c-4151-9732-5989ba04ec81").unwrap();

    pub fn detach_framebuffer(
        &self,
        screen_id: u32,
        framebuffer_id: &str,
    ) -> Result<(), VboxError> {
        let framebuffer_id_ptr = string_to_c_u64_str(framebuffer_id)?;

        get_function_result_unit!(
            self.object,
            DetachFramebuffer,
            screen_id,
            framebuffer_id_ptr
        )
    }

    /// Takes a guest screenshot of the requested size and format and returns it as an array of bytes.
    ///.
    /// # Arguments
    ///
    /// * `screen_id` - The guest monitor to take screenshot from.
    ///
    /// * `width` - Desired image width.
    ///
    /// * `height` - Desired image height.
    ///
    /// * `bitmap_format` - The requested format.
    ///
    /// # Returns
    ///
    /// Returns array of bytes on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    ///```no_run
    ///
    /// use std::fs::File;
    /// use std::io::Write;
    /// use virtualbox_rs::{Session, VirtualBox};use virtualbox_rs::enums::{BitmapFormat, SessionType};
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
    /// let display = console.get_display().unwrap();
    /// let img = display.take_screen_shot_to_array(
    ///     0,
    ///     720,
    ///     400,
    ///     BitmapFormat::PNG.into()
    ///     ).unwrap();
    ///
    /// let mut file = File::create("output.png").unwrap();
    /// file.write_all(img).unwrap();

    pub fn take_screen_shot_to_array(
        &self,
        screen_id: u32,
        width: u32,
        height: u32,
        bitmap_format: BitmapFormat,
    ) -> Result<&[u8], VboxError> {
        let mut screen_data_size: u32 = 0;
        let bitmap_format: u32 = bitmap_format.into();
        let img_ptr = get_function_result_pointer!(
            self.object,
            TakeScreenShotToArray,
            *mut PRUint8,
            screen_id,
            width,
            height,
            bitmap_format,
            &mut screen_data_size
        )?;
        let screen_data_slice =
            unsafe { slice::from_raw_parts(img_ptr, screen_data_size as usize) };

        Ok(screen_data_slice)
    }
}
