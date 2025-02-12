#[cfg(is_v_6_1)]
pub mod audio_settings_not_impl;
#[cfg(is_v_6_1)]
pub mod cpu_profile;
#[cfg(is_v_6_1)]
pub mod guest_debug_control_not_impl;
#[cfg(is_v_6_1)]
pub mod host_audio_device;
#[cfg(is_v_6_1)]
pub mod host_only_network;
#[cfg(is_v_6_1)]
pub mod nvram_store;
#[cfg(is_v_6_1)]
pub mod trusted_platform_module;
#[cfg(is_v_6_1)]
pub mod update_agent;
#[cfg(is_v_7_1)]
pub mod bios_settings;
#[cfg(not(is_v_7_1))]
pub mod firmware_settings;
#[cfg(not(is_v_7_1))]
pub mod platform_x86;
#[cfg(not(is_v_7_1))]
pub mod platform_arm;
#[cfg(not(is_v_7_1))]
pub mod platform;
