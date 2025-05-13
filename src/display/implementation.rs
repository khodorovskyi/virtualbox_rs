use crate::display::{Display, Resolution, VideoModeHint};
use crate::enums::{BitmapFormat, GuestMonitorStatus};
use crate::utility::macros::macros::{
    get_function_result_pointer, get_function_result_str, get_function_result_unit,
};
use crate::utility::string_to_c_u64_str;
use crate::{Framebuffer, VboxError};
use std::slice;
use log::trace;
use vbox_raw::sys_lib::{IFramebuffer, PRBool, PRUint8};

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
            guest_monitor_status: GuestMonitorStatus::from(guest_monitor_status),
        })
    }

    /// Sets the graphics update target for a screen.
    ///
    /// # Arguments
    ///
    /// * `screen_id` - The guest monitor to take screenshot from.
    /// * `framebuffer` - The framebuffer to attach.
    ///
    /// # Returns
    ///
    /// Returns &str on success, or a [`VboxError`] on failure.
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
    /// let framebuffer_id_str = display.attach_framebuffer(0, &framebuffer).unwrap();

    pub fn attach_framebuffer(
        &mut self,
        screen_id: u32,
        framebuffer: &Framebuffer
    ) -> Result<&'static str, VboxError> {
        let framebuffer: *mut IFramebuffer = framebuffer.object;

        let framebuffer_id_str =
            get_function_result_str!(self.object, AttachFramebuffer, screen_id, framebuffer)?;
        // let mut framebuffer_ids = self
        //     .framebuffer_ids
        //     .lock()
        //     .map_err(|err| VboxError::new(0, "attach_framebuffer", err.to_string(), None))?;
        // framebuffer_ids.insert(framebuffer_id_str, screen_id);
        Ok(framebuffer_id_str)
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

    /// Queries the graphics updates targets for a screen.
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
    /// display.query_framebuffer(0).unwrap();

    pub fn query_framebuffer(&self, screen_id: u32) -> Result<Framebuffer, VboxError> {
        let framebuffer = get_function_result_pointer!(
            self.object,
            QueryFramebuffer,
            *mut IFramebuffer,
            screen_id
        )?;
        Ok(Framebuffer::new(framebuffer))
    }

    /// Changes the monitor information reported by a given output of the guest graphics device.
    ///
    /// This information can be read by the guest if suitable drivers and driver tools are available, including but not limited to those in the Guest Additions. The guest will receive monitor hotplug notification when the monitor information is changed, and the information itself will be available to the guest until the next change. The information should not be resent if the guest does not resize in response. The guest might have chosen to ignore the change, or the resize might happen later when a suitable driver is started.
    ///
    /// Specifying 0 for either width, height or bitsPerPixel parameters means that the corresponding values should be taken from the current video mode (i.e. left unchanged).
    ///
    /// # Arguments
    ///
    /// * `video_mode_hint` - The new monitor information.
    /// * `notify` - Whether the guest should be notified of the change. Normally this is wished, but it might not be when re-setting monitor information from the last session (no hotplug happened, as it is still the same virtual monitor). Might also be useful if several monitors are to be changed at once, but this would not reflect physical hardware well, and we also have `set_screen_layout` for that.
    ///
    /// # Returns
    ///
    /// Returns () on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{Session, VideoModeHint, VirtualBox};
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
    /// let video_mode_hint = VideoModeHint{
    ///     display: 0,
    ///     enabled: false,
    ///     change_origin: false,
    ///     origin_x: 0,
    ///     origin_y: 0,
    ///     width: 0,
    ///     height: 0,
    ///     bits_per_pixel: 0
    /// };
    ///
    /// display.set_video_mode_hint(video_mode_hint, true).unwrap();
    /// ```
    pub fn set_video_mode_hint(
        &self,
        video_mode_hint: VideoModeHint,
        notify: bool,
    ) -> Result<(), VboxError> {
        get_function_result_unit!(
            self.object,
            SetVideoModeHint,
            video_mode_hint.display,
            PRBool::from(video_mode_hint.enabled),
            PRBool::from(video_mode_hint.change_origin),
            video_mode_hint.origin_x,
            video_mode_hint.origin_y,
            video_mode_hint.width,
            video_mode_hint.height,
            video_mode_hint.bits_per_pixel,
            PRBool::from(notify)
        )
    }

    /// Queries the monitor information for a given guest output.
    ///
    /// See [`Display::set_video_mode_hint`] for more information.  If no monitor information has been set yet by a front-end the preferred mode values returned will be zero.
    ///
    /// # Arguments
    ///
    /// * `screen_id` - The guest monitor to take screenshot from.
    ///
    /// # Returns
    ///
    /// Returns ([`VideoModeHint`], &str) on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{Session, VideoModeHint, VirtualBox};
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
    /// let video_mode_hint = display.get_video_mode_hint(0).unwrap();
    /// ```
    pub fn get_video_mode_hint(&self, screen_id: u32) -> Result<VideoModeHint, VboxError> {
        let mut enabled: i32 = 0;
        let mut change_origin: i32 = 0;
        let mut origin_x: i32 = 0;
        let mut origin_y: i32 = 0;
        let mut width: u32 = 0;
        let mut height: u32 = 0;
        let mut bits_per_pixel: u32 = 0;
        get_function_result_unit!(
            self.object,
            GetVideoModeHint,
            screen_id,
            &mut enabled,
            &mut change_origin,
            &mut origin_x,
            &mut origin_y,
            &mut width,
            &mut height,
            &mut bits_per_pixel
        )?;
        Ok(VideoModeHint {
            display: screen_id,
            enabled: enabled == 1,
            change_origin: change_origin == 1,
            origin_x,
            origin_y,
            width,
            height,
            bits_per_pixel,
        })
    }
    /// Enables or disables seamless guest display rendering (seamless desktop integration) mode.
    ///.
    /// # Arguments
    ///
    /// * `enabled` - Whether to enable or disable seamless mode.
    ///
    /// # Returns
    ///
    /// Returns () on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    ///```no_run
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
    /// let display = console.get_display().unwrap();
    /// display.set_seamless_mode(true).unwrap();
    pub fn set_seamless_mode(&self, enabled: bool) -> Result<(), VboxError> {
        get_function_result_unit!(self.object, SetSeamlessMode, PRBool::from(enabled))
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
        trace!("screen_data_slice {}", screen_data_size);

        Ok(screen_data_slice)
    }

    /// Does a full invalidation of the VM display and instructs the VM to update it.
    ///
    ///
    /// # Returns
    ///
    /// Returns () on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    ///```no_run
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
    /// let display = console.get_display().unwrap();
    /// display.invalidate_and_update().unwrap();
    /// ```

    pub fn invalidate_and_update(&self) -> Result<(), VboxError> {
        get_function_result_unit!(self.object, InvalidateAndUpdate)
    }

    /// Does a full invalidation of the VM display and instructs the VM to update it.
    ///
    /// # Arguments
    ///
    /// * `screen_id` - The guest monitor to take screenshot from.
    ///
    /// # Returns
    ///
    /// Returns () on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    ///```no_run
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
    /// let display = console.get_display().unwrap();
    /// display.invalidate_and_update().unwrap();
    /// ```

    pub fn invalidate_and_update_screen(&self, screen_id: u32) -> Result<(), VboxError> {
        get_function_result_unit!(self.object, InvalidateAndUpdateScreen, screen_id)
    }

    /// Signals that framebuffer window viewport has changed.
    ///
    /// # Arguments
    ///
    /// * `screen_id` - Monitor to take the screenshot from.
    /// * `x` - Framebuffer x offset.
    /// * `y` - Framebuffer y offset.
    /// * `width` - Viewport width.
    /// * `height` - Viewport height.
    ///
    /// Returns () on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    ///```no_run
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
    /// let display = console.get_display().unwrap();
    /// display.viewport_changed(0, 0, 0, 720, 400).unwrap();
    /// ```
    pub fn viewport_changed(
        &self,
        screen_id: u32,
        x: u32,
        y: u32,
        width: u32,
        height: u32,
    ) -> Result<(), VboxError> {
        get_function_result_unit!(self.object, ViewportChanged, screen_id, x, y, width, height)
    }
}
