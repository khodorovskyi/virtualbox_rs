use crate::console::Console;
use crate::display::Display;
use crate::progress::Progress;
use crate::utility::macros::macros::{
    get_function_result_pointer, get_function_result_pointer_vec, get_function_result_unit,
};
use crate::{EventSource, Guest, Keyboard, Mouse, PCIDeviceAttachment, VboxError};
use log::debug;
use vbox_raw::sys_lib::{
    IDisplay, IEventSource, IGuest, IKeyboard, IMouse, IPCIDeviceAttachment, IProgress,
};

impl Console {
    /// Starts the virtual machine execution using the current machine state
    /// (that is, its current execution state, current settings and current storage devices).
    ///
    /// # Returns
    ///
    /// Returns a new instance of the [`Progress`] class on success, or a [`VboxError`] on failure.
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
    /// let progress = console.power_up().unwrap();
    pub fn power_up(&self) -> Result<Progress, VboxError> {
        let progress = get_function_result_pointer!(self.object, PowerUp, *mut IProgress)?;
        Ok(Progress::new(progress))
    }

    /// Identical to powerUp except that the VM will enter the [`MachineState::Paused`](crate::enums::MachineState::Paused) state,
    /// instead of [`MachineState::Running`](crate::enums::MachineState::Running).
    ///
    /// # Returns
    ///
    /// Returns a new instance of the [`Progress`] class on success, or a [`VboxError`] on failure.
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
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    ///
    /// let console = session.get_console().unwrap();
    ///
    /// let progress = console.power_up_paused().unwrap();
    pub fn power_up_paused(&self) -> Result<Progress, VboxError> {
        let progress = get_function_result_pointer!(self.object, PowerUpPaused, *mut IProgress)?;
        Ok(Progress::new(progress))
    }
    /// Initiates the power down procedure to stop the virtual machine execution.
    ///
    /// # Returns
    ///
    /// Returns a new instance of the [`Progress`] class on success, or a [`VboxError`] on failure.
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
    /// let progress = console.power_down();

    pub fn power_down(&self) -> Result<Progress, VboxError> {
        let progress = get_function_result_pointer!(self.object, PowerDown, *mut IProgress)?;
        Ok(Progress::new(progress))
    }
    /// Resets the virtual machine.
    ///
    /// # Returns
    ///
    /// Returns Ok(()), or a [`VboxError`] on failure.
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
    /// console.reset().unwrap();
    pub fn reset(&self) -> Result<(), VboxError> {
        get_function_result_unit!(self.object, Reset)
    }

    /// Pauses the virtual machine execution.
    ///
    /// # Returns
    ///
    /// Returns Ok(()), or a [`VboxError`] on failure.
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
    /// console.pause().unwrap();
    pub fn pause(&self) -> Result<(), VboxError> {
        get_function_result_unit!(self.object, Pause)
    }
    /// Resumes the virtual machine execution.
    ///
    /// # Returns
    ///
    /// Returns Ok(()), or a [`VboxError`] on failure.
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
    /// console.resume().unwrap();
    pub fn resume(&self) -> Result<(), VboxError> {
        get_function_result_unit!(self.object, Resume)
    }
    /// Sends the ACPI power button event to the guest.
    ///
    /// # Returns
    ///
    /// Returns Ok(()), or a [`VboxError`] on failure.
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
    /// console.power_button().unwrap();
    pub fn power_button(&self) -> Result<(), VboxError> {
        get_function_result_unit!(self.object, PowerButton)
    }

    /// Sends the ACPI sleep button event to the guest.
    ///
    /// # Returns
    ///
    /// Returns Ok(()), or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    ///
    /// use virtualbox_rs::enums::SessionType;
    /// use virtualbox_rs::{Session, VirtualBox};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let mut session = Session::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    ///
    /// let console = session.get_console().unwrap();
    ///
    /// console.sleep_button().unwrap();
    pub fn sleep_button(&self) -> Result<(), VboxError> {
        get_function_result_unit!(self.object, SleepButton)
    }

    /// Virtual keyboard object.
    ///
    /// # Returns
    ///
    /// Returns a new instance of the [`Keyboard`] class on success, or a [`VboxError`] on failure.
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
    /// let keyboard = console.get_keyboard().unwrap();
    pub fn get_keyboard(&self) -> Result<Keyboard, VboxError> {
        let keyboard = get_function_result_pointer!(self.object, GetKeyboard, *mut IKeyboard)?;
        Ok(Keyboard::new(keyboard))
    }

    /// Virtual mouse object.
    ///
    /// # Returns
    ///
    /// Returns a new instance of the [`Mouse`] class on success, or a [`VboxError`] on failure.
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
    /// let keyboard = console.get_keyboard().unwrap();
    pub fn get_mouse(&self) -> Result<Mouse, VboxError> {
        let mouse = get_function_result_pointer!(self.object, GetMouse, *mut IMouse)?;
        Ok(Mouse::new(mouse))
    }

    /// Virtual display object.
    ///
    /// # Returns
    ///
    /// Returns a new instance of the [`Display`] class on success, or a [`VboxError`] on failure.
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
    /// let display = console.get_display().unwrap();
    pub fn get_display(&self) -> Result<Display, VboxError> {
        let display = get_function_result_pointer!(self.object, GetDisplay, *mut IDisplay)?;
        Ok(Display::new(display))
    }

    /// Event source for console events.
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
    ///
    /// let event_source = console.get_event_source().unwrap();
    pub fn get_event_source(&self) -> Result<EventSource, VboxError> {
        let event_source =
            get_function_result_pointer!(self.object, GetEventSource, *mut IEventSource)?;
        Ok(EventSource::new(event_source))
    }

    ///Guest object.
    ///
    /// # Returns
    ///
    /// Returns a new instance of the [`Guest`] class on success, or a [`VboxError`] on failure.
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
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    ///
    /// let console = session.get_console().unwrap();
    ///
    /// let guest = console.get_guest().unwrap();
    pub fn get_guest(&self) -> Result<Guest, VboxError> {
        let guest = get_function_result_pointer!(self.object, GetGuest, *mut IGuest)?;
        Ok(Guest::new(guest))
    }

    /// Array of PCI devices attached to this machine.
    ///
    /// # Returns
    ///
    /// Returns [`Vec<PCIDeviceAttachment>`] on success, or a [`VboxError`] on failure.
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
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    /// let console = session.get_console().unwrap();
    /// let attached_pci_devices = console.get_attached_pci_devices().unwrap();
    pub fn get_attached_pci_devices(&self) -> Result<Vec<PCIDeviceAttachment>, VboxError> {
        let attached_pci_devices = get_function_result_pointer_vec!(
            self.object,
            GetAttachedPCIDevices,
            *mut IPCIDeviceAttachment
        )?;
        Ok(attached_pci_devices
            .iter()
            .map(|object| PCIDeviceAttachment::new(object.clone()))
            .collect())
    }
}
