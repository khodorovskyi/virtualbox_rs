use crate::enums::MouseButtonState;
use crate::utility::macros::macros::{
    get_function_result_bool, get_function_result_pointer, get_function_result_unit,
};
use crate::{EventSource, Mouse, VboxError};
use vbox_raw::sys_lib::IEventSource;

impl Mouse {
    /// Initiates a mouse event using relative pointer movements along x and y axis.
    ///
    /// # Arguments
    ///
    ///
    /// * `dx` - Amount of pixels the mouse should move to the right. Negative values move the mouse to the left.
    /// * `dy` - Amount of pixels the mouse should move downwards. Negative values move the mouse upwards.
    /// * `dz` - Amount of mouse wheel moves. Positive values describe clockwise wheel rotations, negative values describe counterclockwise rotations.
    /// * `dw` - Amount of horizontal mouse wheel moves. Positive values describe a movement to the left, negative values describe a movement to the right.
    /// * `button_state` - The current state of mouse buttons.
    ///
    /// # Returns
    ///
    /// Returns  Ok(()) success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{Session, VirtualBox};
    /// use virtualbox_rs::enums::{MouseButtonState, SessionType};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let mut session = Session::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    ///
    /// let console = session.get_console().unwrap();
    ///
    /// let mouse = console.get_mouse().unwrap();
    /// mouse.put_mouse_event(1,1,1,1,MouseButtonState::Left).unwrap();
    pub fn put_mouse_event(
        &self,
        dx: i32,
        dy: i32,
        dz: i32,
        dw: i32,
        button_state: MouseButtonState,
    ) -> Result<(), VboxError> {
        let button_state = button_state.into();
        get_function_result_unit!(self.object, PutMouseEvent, dx, dy, dz, dw, button_state)
    }

    /// Positions the mouse pointer using absolute x and y coordinates.
    ///
    /// # Arguments
    ///
    ///
    /// * `x` - X coordinate of the pointer in pixels, starting from 1.
    /// * `y` - Y coordinate of the pointer in pixels, starting from 1.
    /// * `dz` - Amount of mouse wheel moves. Positive values describe clockwise wheel rotations, negative values describe counterclockwise rotations.
    /// * `dw` - Amount of horizontal mouse wheel moves. Positive values describe a movement to the left, negative values describe a movement to the right.
    /// * `button_state` - The current state of mouse buttons.
    ///
    /// # Returns
    ///
    /// Returns  Ok(()) success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{Session, VirtualBox};
    /// use virtualbox_rs::enums::{MouseButtonState, SessionType};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let mut session = Session::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    ///
    /// let console = session.get_console().unwrap();
    ///
    /// let mouse = console.get_mouse().unwrap();
    /// mouse.put_mouse_event_absolute(1,1,1,1,MouseButtonState::Left).unwrap();
    pub fn put_mouse_event_absolute(
        &self,
        x: i32,
        y: i32,
        dz: i32,
        dw: i32,
        button_state: MouseButtonState,
    ) -> Result<(), VboxError> {
        let button_state = button_state.into();
        get_function_result_unit!(
            self.object,
            PutMouseEventAbsolute,
            x,
            y,
            dz,
            dw,
            button_state
        )
    }

    /// Whether the guest OS supports absolute mouse pointer positioning or not.
    ///
    /// # Returns
    ///
    /// Returns  bool success, or a [`VboxError`] on failure.
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
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    ///
    /// let console = session.get_console().unwrap();
    ///
    /// let mouse = console.get_mouse().unwrap();
    /// let supported = mouse.get_absolute_supported().unwrap();
    pub fn get_absolute_supported(&self) -> Result<bool, VboxError> {
        get_function_result_bool!(self.object, GetAbsoluteSupported)
    }

    /// Whether the guest OS supports relative mouse pointer positioning or not.
    ///
    /// # Returns
    ///
    /// Returns  bool success, or a [`VboxError`] on failure.
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
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    ///
    /// let console = session.get_console().unwrap();
    ///
    /// let mouse = console.get_mouse().unwrap();
    /// let supported = mouse.get_relative_supported().unwrap();
    pub fn get_relative_supported(&self) -> Result<bool, VboxError> {
        get_function_result_bool!(self.object, GetRelativeSupported)
    }

    /// Whether the guest OS has enabled the multi-touch reporting device, touchscreen variant.
    /// <div class="warning">
    ///  This flag only exists for versions 7 and above.
    ///  For compatibility with earlier versions, this field has been retained, but it always returns false
    /// </div>
    ///
    /// # Returns
    ///
    /// Returns  bool success, or a [`VboxError`] on failure.
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
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    ///
    /// let console = session.get_console().unwrap();
    ///
    /// let mouse = console.get_mouse().unwrap();
    /// let supported = mouse.get_touch_screen_supported().unwrap();
    pub fn get_touch_screen_supported(&self) -> Result<bool, VboxError> {
        #[cfg(not(is_v_6_1))]
        {
            get_function_result_bool!(self.object, GetTouchScreenSupported)
        }
        // TODO
        #[cfg(is_v_6_1)]
        {
            Ok(false)
        }
    }

    /// Whether the guest OS has enabled the multi-touch reporting device, touchpad variant.
    ///
    /// <div class="warning">
    ///  This flag only exists for versions 7 and above.
    ///  For compatibility with earlier versions, this field has been retained, but it always returns false
    /// </div>
    ///
    /// # Returns
    ///
    /// Returns  bool success, or a [`VboxError`] on failure.
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
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    ///
    /// let console = session.get_console().unwrap();
    ///
    /// let mouse = console.get_mouse().unwrap();
    /// let supported = mouse.get_touch_pad_supported().unwrap();
    pub fn get_touch_pad_supported(&self) -> Result<bool, VboxError> {
        #[cfg(not(is_v_6_1))]
        {
            get_function_result_bool!(self.object, GetTouchPadSupported)
        }
        #[cfg(is_v_6_1)]
        {
            Ok(false)
        }
    }

    /// Whether the guest OS can currently switch to drawing it's own mouse cursor on demand.
    ///
    /// # Returns
    ///
    /// Returns  bool success, or a [`VboxError`] on failure.
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
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    ///
    /// let console = session.get_console().unwrap();
    ///
    /// let mouse = console.get_mouse().unwrap();
    /// let supported = mouse.get_needs_host_cursor().unwrap();
    pub fn get_needs_host_cursor(&self) -> Result<bool, VboxError> {
        get_function_result_bool!(self.object, GetNeedsHostCursor)
    }

    /// Event source for mouse events.
    ///
    /// # Returns
    ///
    /// Returns a new instance of the [`EventSource`] class on success, or a [`VboxError`] on failure.
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
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    ///
    /// let console = session.get_console().unwrap();
    /// let mouse = console.get_mouse().unwrap();
    ///
    /// let event_source = mouse.get_event_source().unwrap();
    pub fn get_event_source(&self) -> Result<EventSource, VboxError> {
        let event_source =
            get_function_result_pointer!(self.object, GetEventSource, *mut IEventSource)?;
        Ok(EventSource::new(event_source))
    }
}
