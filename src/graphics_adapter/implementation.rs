use crate::enums::{GraphicsControllerType, GraphicsFeature};
use crate::VboxError;
use crate::GraphicsAdapter;
use crate::utility::macros::macros::{ get_function_result_number, get_function_result_unit, get_function_result_bool};

impl GraphicsAdapter {
    /// Graphics controller type.
    ///
    /// # Returns
    ///
    /// Returns [`GraphicsControllerType`] on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{Session, VirtualBox};
    /// use virtualbox_rs::enums::SessionType;
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    ///
    /// let graphics_adapter = machine.get_graphics_adapter().unwrap();
    /// let controller_type = graphics_adapter.get_graphics_controller_type().unwrap();
    /// ```
    pub fn get_graphics_controller_type(&self) -> Result<GraphicsControllerType, VboxError> {
        let controller_type = get_function_result_number!(self.object, GetGraphicsControllerType, u32)?;
        Ok(GraphicsControllerType::from(controller_type))
    }

    /// Graphics controller type.
    ///
    /// # Arguments
    ///
    /// * `controller_type` - [`GraphicsControllerType`]. Graphics controller type.
    ///
    /// # Returns
    ///
    /// Returns u32 on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{Session, VirtualBox};
    /// use virtualbox_rs::enums::{GraphicsControllerType, SessionType};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let mut session = Session::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    /// let machine_mut = session.get_machine().unwrap();
    /// let graphics_adapter = machine_mut.get_graphics_adapter().unwrap();
    /// graphics_adapter.set_graphics_controller_type(GraphicsControllerType::VBoxSVGA).unwrap();
    /// machine_mut.save_settings().unwrap();
    /// ```
    pub fn set_graphics_controller_type(&self, controller_type: GraphicsControllerType) -> Result<(), VboxError> {
        let controller_type: u32 = controller_type.into();
        get_function_result_unit!(self.object, SetGraphicsControllerType, controller_type)
    }

    /// Video memory size in megabytes.
    ///
    /// # Returns
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
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    ///
    /// let graphics_adapter = machine.get_graphics_adapter().unwrap();
    /// let vram_size = graphics_adapter.get_vram_size().unwrap();
    /// ```
    pub fn get_vram_size(&self) -> Result<u32, VboxError> {
        get_function_result_number!(self.object, GetVRAMSize, u32)
    }

    /// Video memory size in megabytes.
    ///
    /// # Arguments
    ///
    /// * `vram_size` - u32. Video memory size in megabytes.
    ///
    /// # Returns
    ///
    /// Returns () on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{Session, VirtualBox};
    /// use virtualbox_rs::enums::{GraphicsControllerType, SessionType};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let mut session = Session::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    /// let machine_mut = session.get_machine().unwrap();
    /// let graphics_adapter = machine_mut.get_graphics_adapter().unwrap();
    /// graphics_adapter.set_vram_size(16).unwrap();
    /// machine_mut.save_settings().unwrap();
    /// ```
    pub fn set_vram_size(&self, vram_size: u32) -> Result<(), VboxError> {
        get_function_result_unit!(self.object, SetVRAMSize, vram_size)
    }

    #[cfg(is_v_7_1)]
    /// This setting determines whether VirtualBox allows this machine to make use of the 3D graphics support available on the host.
    ///
    /// # Returns
    ///
    /// Returns bool on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{Session, VirtualBox};
    /// use virtualbox_rs::enums::SessionType;
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    ///
    /// let graphics_adapter = machine.get_graphics_adapter().unwrap();
    /// let enabled = graphics_adapter.get_accelerate3d_enabled().unwrap();
    /// ```
    pub fn get_accelerate3d_enabled(&self) -> Result<bool, VboxError> {
        self.is_feature_enabled(GraphicsFeature::Acceleration3D)
    }

    #[cfg(is_v_7_1)]
    /// This setting determines whether VirtualBox allows this machine to make use of the 3D graphics support available on the host.
    ///
    /// # Arguments
    ///
    /// * `enabled` - bool.
    ///
    /// # Returns
    ///
    /// Returns () on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{Session, VirtualBox};
    /// use virtualbox_rs::enums::{GraphicsControllerType, SessionType};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let mut session = Session::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    /// let machine_mut = session.get_machine().unwrap();
    /// let graphics_adapter = machine_mut.get_graphics_adapter().unwrap();
    /// graphics_adapter.set_accelerate3d_enabled(true).unwrap();
    /// machine_mut.save_settings().unwrap();
    /// ```
    pub fn set_accelerate3d_enabled(&self, enabled: bool) -> Result<(), VboxError> {
        self.set_feature(GraphicsFeature::Acceleration3D, enabled)
    }

    #[cfg(is_v_7_1)]
    /// This setting determines whether VirtualBox allows this machine to make use of the 2D video acceleration support available on the host.
    ///
    /// # Returns
    ///
    /// Returns bool on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{Session, VirtualBox};
    /// use virtualbox_rs::enums::SessionType;
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    ///
    /// let graphics_adapter = machine.get_graphics_adapter().unwrap();
    /// let enabled = graphics_adapter.get_accelerate2dvideo_enabled().unwrap();
    /// ```
    pub fn get_accelerate2dvideo_enabled(&self) -> Result<bool, VboxError> {
        self.is_feature_enabled(GraphicsFeature::Acceleration2DVideo)
    }

    #[cfg(is_v_7_1)]
    /// This setting determines whether VirtualBox allows this machine to make use of the 2D video acceleration support available on the host.
    ///
    /// # Arguments
    ///
    /// * `enabled` - bool.
    ///
    /// # Returns
    ///
    /// Returns () on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{Session, VirtualBox};
    /// use virtualbox_rs::enums::{GraphicsControllerType, SessionType};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let mut session = Session::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    /// let machine_mut = session.get_machine().unwrap();
    /// let graphics_adapter = machine_mut.get_graphics_adapter().unwrap();
    /// graphics_adapter.set_accelerate2dvideo_enabled(true).unwrap();
    /// machine_mut.save_settings().unwrap();
    /// ```
    pub fn set_accelerate2dvideo_enabled(&self, enabled: bool) -> Result<(), VboxError> {
        self.set_feature(GraphicsFeature::Acceleration2DVideo, enabled)
    }

    /// Number of virtual monitors.
    ///
    /// # Returns
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
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    ///
    /// let graphics_adapter = machine.get_graphics_adapter().unwrap();
    /// let monitor_count = graphics_adapter.get_monitor_count().unwrap();
    /// ```
    pub fn get_monitor_count(&self) -> Result<u32, VboxError> {
        get_function_result_number!(self.object, GetMonitorCount, u32)
    }

    /// Number of virtual monitors.
    ///
    /// # Arguments
    ///
    /// * `vram_size` - u32. Number of virtual monitors.
    ///
    /// # Returns
    ///
    /// Returns () on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{Session, VirtualBox};
    /// use virtualbox_rs::enums::{GraphicsControllerType, SessionType};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let mut session = Session::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    /// let machine_mut = session.get_machine().unwrap();
    /// let graphics_adapter = machine_mut.get_graphics_adapter().unwrap();
    /// graphics_adapter.set_monitor_count(2).unwrap();
    /// machine_mut.save_settings().unwrap();
    /// ```
    pub fn set_monitor_count(&self, monitor_count: u32) -> Result<(), VboxError> {
        get_function_result_unit!(self.object, SetMonitorCount, monitor_count)
    }
    #[cfg(is_v_7_1)]
    /// Sets a graphics controller feature.
    ///
    /// # Arguments
    ///
    /// * `feature` - Graphics controller feature to set.
    /// * `enabled` - Whether to enable or disable the feature.
    ///
    /// # Returns
    ///
    /// Returns () on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{Session, VirtualBox};
    /// use virtualbox_rs::enums::{GraphicsControllerType, GraphicsFeature, SessionType};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let mut session = Session::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    /// let machine_mut = session.get_machine().unwrap();
    /// let graphics_adapter = machine_mut.get_graphics_adapter().unwrap();
    /// graphics_adapter.set_feature(GraphicsFeature::Acceleration3D, true).unwrap();
    /// machine_mut.save_settings().unwrap();
    /// ```
    pub fn set_feature(&self, feature: GraphicsFeature, enabled: bool) -> Result<(), VboxError> {
        let feature: u32 = feature.into();
        let enabled: i32 = if enabled { 1 } else { 0 };
        get_function_result_unit!(self.object, SetFeature, feature, enabled)
    }

    #[cfg(is_v_7_1)]
    /// Returns whether a particular feature is enabled for this adapter or not.
    ///
    /// # Arguments
    ///
    /// * `feature` - Feature to check for.
    ///
    /// # Returns
    ///
    /// Returns bool on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{Session, VirtualBox};
    /// use virtualbox_rs::enums::{GraphicsControllerType, GraphicsFeature, SessionType};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let mut session = Session::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    /// let machine_mut = session.get_machine().unwrap();
    /// let graphics_adapter = machine_mut.get_graphics_adapter().unwrap();
    /// let enabled = graphics_adapter.is_feature_enabled(GraphicsFeature::Acceleration3D).unwrap();
    /// ```
    pub fn is_feature_enabled(&self, feature: GraphicsFeature) -> Result<bool, VboxError> {
        let feature: u32 = feature.into();
        get_function_result_bool!(self.object, IsFeatureEnabled, feature)
    }
}

#[cfg(not(is_v_7_1))]
impl GraphicsAdapter {

    /// This setting determines whether VirtualBox allows this machine to make use of the 3D graphics support available on the host.
    ///
    /// # Returns
    ///
    /// Returns bool on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{Session, VirtualBox};
    /// use virtualbox_rs::enums::SessionType;
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    ///
    /// let graphics_adapter = machine.get_graphics_adapter().unwrap();
    /// let enabled = graphics_adapter.get_accelerate3d_enabled().unwrap();
    /// ```
    pub fn get_accelerate3d_enabled(&self) -> Result<bool, VboxError> {
        get_function_result_bool!(self.object, GetAccelerate3DEnabled)
    }

    /// This setting determines whether VirtualBox allows this machine to make use of the 3D graphics support available on the host.
    ///
    /// # Arguments
    ///
    /// * `enabled` - bool.
    ///
    /// # Returns
    ///
    /// Returns () on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{Session, VirtualBox};
    /// use virtualbox_rs::enums::{GraphicsControllerType, SessionType};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let mut session = Session::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    /// let machine_mut = session.get_machine().unwrap();
    /// let graphics_adapter = machine_mut.get_graphics_adapter().unwrap();
    /// graphics_adapter.set_accelerate3d_enabled(true).unwrap();
    /// machine_mut.save_settings().unwrap();
    /// ```
    pub fn set_accelerate3d_enabled(&self, enabled: bool) -> Result<(), VboxError> {
        let enabled: i32 = if enabled { 1 } else { 0 };
        get_function_result_unit!(self.object, SetAccelerate3DEnabled, enabled)
    }

    /// This setting determines whether VirtualBox allows this machine to make use of the 2D video acceleration support available on the host.
    ///
    /// # Returns
    ///
    /// Returns bool on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{Session, VirtualBox};
    /// use virtualbox_rs::enums::SessionType;
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    ///
    /// let graphics_adapter = machine.get_graphics_adapter().unwrap();
    /// let enabled = graphics_adapter.get_accelerate2dvideo_enabled().unwrap();
    /// ```
    pub fn get_accelerate2dvideo_enabled(&self) -> Result<bool, VboxError> {
        get_function_result_bool!(self.object, GetAccelerate2DVideoEnabled)
    }

    /// This setting determines whether VirtualBox allows this machine to make use of the 2D video acceleration support available on the host.
    ///
    /// # Arguments
    ///
    /// * `enabled` - bool.
    ///
    /// # Returns
    ///
    /// Returns () on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{Session, VirtualBox};
    /// use virtualbox_rs::enums::{GraphicsControllerType, SessionType};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let mut session = Session::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    /// let machine_mut = session.get_machine().unwrap();
    /// let graphics_adapter = machine_mut.get_graphics_adapter().unwrap();
    /// graphics_adapter.set_accelerate2dvideo_enabled(true).unwrap();
    /// machine_mut.save_settings().unwrap();
    /// ```
    pub fn set_accelerate2dvideo_enabled(&self, enabled: bool) -> Result<(), VboxError> {
        let enabled: i32 = if enabled { 1 } else { 0 };
        get_function_result_unit!(self.object, SetAccelerate2DVideoEnabled, enabled)
    }
    /// Sets a graphics controller feature.
    ///
    /// # Arguments
    ///
    /// * `feature` - Graphics controller feature to set.
    /// * `enabled` - Whether to enable or disable the feature.
    ///
    /// # Returns
    ///
    /// Returns () on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{Session, VirtualBox};
    /// use virtualbox_rs::enums::{GraphicsControllerType, GraphicsFeature, SessionType};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let mut session = Session::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    /// let machine_mut = session.get_machine().unwrap();
    /// let graphics_adapter = machine_mut.get_graphics_adapter().unwrap();
    /// graphics_adapter.set_feature(GraphicsFeature::Acceleration3D, true).unwrap();
    /// machine_mut.save_settings().unwrap();
    /// ```
    pub fn set_feature(&self, feature: GraphicsFeature, enabled: bool) -> Result<(), VboxError> {
        if feature == GraphicsFeature::Acceleration3D {
            self.set_accelerate3d_enabled(enabled)
        } else {
            self.set_accelerate2dvideo_enabled(enabled)
        }
    }

    /// Returns whether a particular feature is enabled for this adapter or not.
    ///
    /// # Arguments
    ///
    /// * `feature` - Feature to check for.
    ///
    /// # Returns
    ///
    /// Returns bool on success, or a [`VboxError`] on failure.
    ///
    ///  # Example
    ///
    /// ```no_run
    ///
    /// use virtualbox_rs::{Session, VirtualBox};
    /// use virtualbox_rs::enums::{GraphicsControllerType, GraphicsFeature, SessionType};
    ///
    /// let vbox = VirtualBox::init().unwrap();
    /// let mut session = Session::init().unwrap();
    /// let machine = vbox.
    ///         find_machines("Freebsd_14").unwrap();
    /// machine.lock_machine(&mut session, SessionType::Shared).unwrap();
    /// let machine_mut = session.get_machine().unwrap();
    /// let graphics_adapter = machine_mut.get_graphics_adapter().unwrap();
    /// let enabled = graphics_adapter.is_feature_enabled(GraphicsFeature::Acceleration3D).unwrap();
    /// ```
    pub fn is_feature_enabled(&self, feature: GraphicsFeature) -> Result<bool, VboxError> {
        if feature == GraphicsFeature::Acceleration3D {
            self.get_accelerate3d_enabled()
        } else {
            self.get_accelerate2dvideo_enabled()
        }
    }
}