use crate::utility::macros::macros::call_function;
use crate::VboxError;
use log::{debug, error};
use vbox_raw::sys_lib::IGuest;

mod implementation;

/// The IGuest interface represents information about the operating system running inside the virtual machine.
///
/// **Reference to the official documentation:**
///
/// [https://www.virtualbox.org/sdkref/interface_i_guest.html](https://www.virtualbox.org/sdkref/interface_i_guest.html)

#[derive(Debug)]
pub struct Guest {
    object: *mut IGuest,
}

impl Guest {
    pub(crate) fn new(object: *mut IGuest) -> Self {
        Self { object }
    }

    fn release(&self) -> Result<i32, VboxError> {
        call_function!(self.object, Release)
    }
}

impl Drop for Guest {
    fn drop(&mut self) {
        match self.release() {
            Ok(count) => {
                debug!("Guest refcount: {}", count)
            }
            Err(err) => {
                error!("Failed drop Guest. Error: {:?}", err)
            }
        }
    }
}

#[derive(Debug)]
pub struct GuestInternalGetStatistics {
    /// Percentage of processor time spent in user mode as seen by the guest.
    pub cpu_user: u32,
    /// Percentage of processor time spent in kernel mode as seen by the guest.
    pub cpu_kernel: u32,
    /// Percentage of processor time spent idling as seen by the guest.
    pub cpu_idle: u32,
    /// Total amount of physical guest RAM.
    pub mem_total: u32,
    /// Free amount of physical guest RAM.
    pub mem_free: u32,
    /// Amount of ballooned physical guest RAM.
    pub mem_balloon: u32,
    /// Amount of shared physical guest RAM.
    pub mem_shared: u32,
    /// Total amount of guest (disk) cache memory.
    pub mem_cache: u32,
    /// Total amount of space in the page file.
    pub paged_total: u32,
    /// Total amount of memory allocated by the hypervisor.
    pub mem_alloc_total: u32,
    /// Total amount of free memory available in the hypervisor.
    pub mem_free_total: u32,
    /// Total amount of memory ballooned by the hypervisor.
    pub mem_balloon_total: u32,
    /// Total amount of shared memory in the hypervisor.
    pub mem_shared_total: u32,
}
