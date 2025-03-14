//! `virtualbox_rs` is a Rust library for interacting with VirtualBox through its SDK.
//!
//! This library provides a comprehensive set of bindings to manage VirtualBox VMs, including
//! creating, deleting, and modifying virtual machines, handling snapshots, and configuring
//! various VM settings such as network adapters, audio settings, and encryption.
//!
//! # Features
//!
//! - Create and manage VirtualBox VMs
//! - Handle VM snapshots
//! - Configure VM settings (network, audio, encryption, etc.)
//! - Support for different VirtualBox API versions (6.1 and 7.0)
//!
//! # Example
//!
//! ```no_run
//! use virtualbox_rs::{Session, VirtualBox, VirtualBoxClient};
//! use virtualbox_rs::enums::SessionType;
//!
//! VirtualBoxClient::check_version().unwrap();
//! let vbox = VirtualBox::init().unwrap();
//! let mut session = Session::init().unwrap();
//! let machine = vbox.find_machines("Freebsd_14").unwrap();
//! machine.lock_machine(&mut session, SessionType::Shared).unwrap();
//!
//! let machine_mut = session.get_machine().unwrap();
//! let progress = machine_mut.delete_snapshot_and_all_children("278ef54a-2e75-4aba-b212-551af4c69725").unwrap();
//! progress.wait_for_completion(-1).unwrap();
//! ```
//!
//! # Supported Platforms
//!
//! This library has been tested on:
//! - FreeBSD 14.1
//! - Ubuntu 24.04
//!
//! # Supported VirtualBox Versions
//!
//! This library supports VirtualBox versions 6.1, 7.0, and 7.1. By default, the library is built for version 7.1.
//! To change the target version, enable the corresponding feature flag: `v7_1`, `v7_0`, or `v6_1`.
//! If multiple versions are specified, the library will be built for the highest version available.
//!
//! **Important:** Running the library with an incompatible VirtualBox version is likely to result in a `Segmentation fault`.
//! Before using the library, ensure compatibility by invoking `VirtualBoxClient::check_version().unwrap();`.
//! If the versions do not match, the program should terminate immediately.
//!
//! # Note
//!
//! Not everything has been implemented yet. The library emphasizes providing examples for each method, which should simplify its usage.
mod appliance;
mod audio_adapter;
#[cfg(not(is_v_6_1))]
mod audio_settings;
mod bandwidth_control;
mod bandwidth_group;
#[cfg(not(is_v_7_1))]
mod bios_settings;
mod cloud_network;
mod console;
mod core;
mod certificate;
#[cfg(not(is_v_6_1))]
mod cpu_profile;
mod data_stream;
mod dhcp_server;
mod display;
pub mod enums;
mod errors;
mod event;
pub mod event_detail;
mod event_listener;
mod event_source;
mod ext_pack_manager;
#[cfg(is_v_7_1)]
mod firmware_settings;
mod framebuffer;
mod graphics_adapter;
mod guest;
#[cfg(not(is_v_6_1))]
mod guest_debug_control;
mod guest_file;
mod guest_os_type;
mod guest_process;
mod guest_session;
mod host;
#[cfg(not(is_v_6_1))]
mod host_audio_device;
mod host_network_interface;
#[cfg(not(is_v_6_1))]
mod host_only_network;
mod keyboard;
mod machine;
mod medium;
mod medium_attachment;
mod medium_format;
mod medium_io;
mod mouse;
mod nat_network;
mod network_adapter;
mod not_impl;
#[cfg(not(is_v_6_1))]
mod nvram_store;
mod parallel_port;
mod pci_device_attachment;
#[cfg(is_v_7_1)]
mod platform;
#[cfg(is_v_7_1)]
mod platform_properties;
mod performance_collector;
mod progress;
mod recording_settings;
mod serial_port;
mod session;
mod shared_folder;
mod snapshot;
mod storage_controller;
mod system_properties;
mod token;
#[cfg(not(is_v_6_1))]
mod trusted_platform_module;
mod unattended;
#[cfg(not(is_v_6_1))]
mod update_agent;
mod usb_controller;
mod usb_device;
mod usb_device_filters;
pub(crate) mod utility;
mod virtual_system_description;
mod virtualbox;
mod virtualbox_client;
mod virtualbox_error_info;
mod vfs_explorer;
mod vrde_server;
#[cfg(is_v_7_1)]
mod platform_x86;
#[cfg(is_v_7_1)]
mod platform_arm;

pub use appliance::Appliance;
pub use audio_adapter::AudioAdapter;
#[cfg(not(is_v_6_1))]
pub use audio_settings::AudioSettings;
pub use bandwidth_control::BandwidthControl;
pub use bandwidth_group::BandwidthGroup;
#[cfg(not(is_v_7_1))]
pub use bios_settings::BIOSSettings;
pub use certificate::Certificate;
pub use cloud_network::CloudNetwork;
pub use console::Console;
#[cfg(not(is_v_6_1))]
pub use cpu_profile::CPUProfile;
pub use data_stream::DataStream;
pub use dhcp_server::DHCPServer;
pub use display::Display;
pub use display::Resolution;
pub use display::VideoModeHint;
pub use errors::vbox_error_type::VboxErrorType;
pub use errors::VboxError;
pub use event::Event;
pub use event_listener::EventListener;
pub use event_source::EventSource;
pub use ext_pack_manager::ExtPackManager;
#[cfg(is_v_7_1)]
pub use firmware_settings::FirmwareSettings;
pub use framebuffer::Framebuffer;
pub use graphics_adapter::GraphicsAdapter;
pub use guest::Guest;
pub use guest::GuestInternalGetStatistics;
#[cfg(not(is_v_6_1))]
pub use guest_debug_control::GuestDebugControl;
pub use guest_file::GuestFile;
pub use guest_os_type::GuestOSType;
pub use guest_process::GuestProcess;
pub use guest_session::GuestSession;
pub use host::Host;
#[cfg(not(is_v_6_1))]
pub use host_audio_device::HostAudioDevice;
pub use host_network_interface::HostNetworkInterface;
#[cfg(not(is_v_6_1))]
pub use host_only_network::HostOnlyNetwork;
pub use keyboard::Keyboard;
pub use machine::Machine;
pub use medium::Medium;
pub use medium_attachment::MediumAttachment;
pub use medium_format::MediumFormat;
pub use medium_io::MediumIO;
pub use mouse::Mouse;
pub use nat_network::NATNetwork;
pub use network_adapter::NetworkAdapter;
#[cfg(is_v_7_1)]
pub use not_impl::bios_settings::BIOSSettings;
#[cfg(is_v_6_1)]
pub use not_impl::audio_settings_not_impl::AudioSettings;
#[cfg(is_v_6_1)]
pub use not_impl::cpu_profile::CPUProfile;
#[cfg(is_v_6_1)]
pub use not_impl::guest_debug_control_not_impl::GuestDebugControl;
#[cfg(not(is_v_7_1))]
pub use not_impl::firmware_settings::FirmwareSettings;
#[cfg(is_v_6_1)]
pub use not_impl::host_audio_device::HostAudioDevice;
#[cfg(is_v_6_1)]
pub use not_impl::host_only_network::HostOnlyNetwork;
#[cfg(is_v_6_1)]
pub use not_impl::nvram_store::NvramStore;
#[cfg(is_v_6_1)]
pub use not_impl::trusted_platform_module::TrustedPlatformModule;
#[cfg(is_v_6_1)]
pub use not_impl::update_agent::UpdateAgent;
#[cfg(not(is_v_6_1))]
pub use nvram_store::NvramStore;
pub use parallel_port::ParallelPort;
#[cfg(is_v_7_1)]
pub use platform::Platform;
#[cfg(not(is_v_7_1))]
pub use not_impl::platform::Platform;
#[cfg(is_v_7_1)]
pub use platform_properties::PlatformProperties;
pub use pci_device_attachment::PCIDeviceAttachment;
pub use performance_collector::PerformanceCollector;
pub use progress::Progress;
pub use recording_settings::RecordingSettings;
pub use serial_port::SerialPort;
pub use session::Session;
pub use shared_folder::SharedFolder;
pub use snapshot::Snapshot;
pub use storage_controller::StorageController;
pub use system_properties::SystemProperties;
pub use token::Token;
#[cfg(not(is_v_6_1))]
pub use trusted_platform_module::TrustedPlatformModule;
pub use unattended::Unattended;
#[cfg(not(is_v_6_1))]
pub use update_agent::UpdateAgent;
pub use usb_controller::USBController;
pub use usb_device::USBDevice;
pub use usb_device_filters::USBDeviceFilters;
pub use virtual_system_description::VirtualSystemDescription;
pub use virtualbox::VirtualBox;
pub use virtualbox_client::VirtualBoxClient;
pub use virtualbox_error_info::VirtualBoxErrorInfo;
pub use vfs_explorer::VFSExplorer;
pub use vrde_server::VRDEServer;
#[cfg(is_v_7_1)]
pub use platform_x86::PlatformX86;
#[cfg(not(is_v_7_1))]
pub use not_impl::platform_x86::PlatformX86;
#[cfg(is_v_7_1)]
pub use platform_arm::PlatformARM;
#[cfg(not(is_v_7_1))]
pub use not_impl::platform_arm::PlatformARM;

#[allow(dead_code)]
fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
