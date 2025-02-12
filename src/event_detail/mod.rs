use crate::Event;
use std::fmt::Display;

mod extra_data_can_change_event;
mod extra_data_changed_event;
mod machine_data_changed_event;
mod machine_state_changed_even;
mod session_state_changed_event;
//  ExtraDataCanChangeEvent
mod additions_state_changed_event;
mod audio_adapter_changed_event;
mod bandwidth_group_changed_event;
mod can_show_window_event;
mod clipboard_file_transfer_mode_changed_event;
mod clipboard_mode_changed_event;
mod cloud_profile_changed_event;
mod cloud_profile_registered_event;
mod cloud_provider_list_changed_event;
mod cloud_provider_registered_event;
mod cloud_provider_uninstall_event;
mod cpu_changed_event;
mod cpu_execution_cap_changed_event;
mod cursor_position_changed_event;
mod dnd_mode_changed_event;
mod event_source_changed_event;
mod guest_additions_status_changed_event;
mod guest_debug_control_changed_event;
mod guest_file_offset_changed_event;
mod guest_file_read_event;
mod guest_file_registered_event;
mod guest_file_size_changed_event;
mod guest_file_state_changed_event;
mod guest_file_write_event;
mod guest_keyboard_event;
mod guest_monitor_changed_event;
mod guest_monitor_info_changed_event;
mod guest_mouse_event;
mod guest_multi_touch_event;
mod guest_process_input_notify_event;
mod guest_process_output_event;
mod guest_process_registered_event;
mod guest_process_state_changed_event;
mod guest_property_changed_event;
mod guest_session_registered_event;
mod guest_session_state_changed_event;
mod guest_user_state_changed_event;
mod host_audio_device_changed_event;
mod host_name_resolution_configuration_change_event;
mod host_pcidevice_plug_event;
mod keyboard_leds_changed_event;
mod language_changed_event;
mod machine_registered_event;
mod medium_changed_event;
mod medium_config_changed_event;
mod medium_registered_event;
mod mouse_capability_changed_event;
mod mouse_pointer_shape_changed_event;
mod nat_network_alter_event;
mod nat_network_changed_event;
mod nat_network_creation_deletion_event;
mod nat_network_port_forward_event;
mod nat_network_setting_event;
mod nat_network_start_stop_event;
mod nat_redirect_event;
mod network_adapter_changed_event;
mod parallel_port_changed_event;
mod progress_created_event;
mod progress_percentage_changed_event;
mod progress_task_completed_event;
mod recording_changed_event;
mod runtime_error_event;
mod serial_port_changed_event;
mod shared_folder_changed_event;
mod show_window_event;
mod snapshot_changed_event;
mod snapshot_deleted_event;
mod snapshot_restored_event;
mod snapshot_taken_event;
mod state_changed_event;
mod storage_controller_changed_event;
mod storage_device_changed_event;
mod update_agent_available_event;
mod update_agent_error_event;
mod update_agent_settings_changed_event;
mod update_agent_state_changed_event;
mod usb_controller_changed_event;
mod usb_device_state_changed_event;
mod vbox_svc_availability_changed_event;
mod vrde_server_changed_event;
mod vrde_server_info_changed_event;
// GuestDebugControlChangedEvent.
mod machine_groups_changed_event;

use crate::enums::VBoxEventType;
pub use additions_state_changed_event::AdditionsStateChangedEvent;
pub use audio_adapter_changed_event::AudioAdapterChangedEvent;
pub use bandwidth_group_changed_event::BandwidthGroupChangedEvent;
pub use can_show_window_event::CanShowWindowEvent;
pub use clipboard_file_transfer_mode_changed_event::ClipboardFileTransferModeChangedEvent;
pub use clipboard_mode_changed_event::ClipboardModeChangedEvent;
pub use cloud_profile_changed_event::CloudProfileChangedEvent;
pub use cloud_profile_registered_event::CloudProfileRegisteredEvent;
pub use cloud_provider_list_changed_event::CloudProviderListChangedEvent;
pub use cloud_provider_registered_event::CloudProviderRegisteredEvent;
pub use cloud_provider_uninstall_event::CloudProviderUninstallEvent;
pub use cpu_changed_event::CPUChangedEvent;
pub use cpu_execution_cap_changed_event::CPUExecutionCapChangedEvent;
pub use cursor_position_changed_event::CursorPositionChangedEvent;
pub use dnd_mode_changed_event::DnDModeChangedEvent;
pub use event_source_changed_event::EventSourceChangedEvent;
pub use extra_data_can_change_event::ExtraDataCanChangeEvent;
pub use extra_data_changed_event::ExtraDataChangedEvent;
pub use guest_additions_status_changed_event::GuestAdditionsStatusChangedEvent;
pub use guest_debug_control_changed_event::GuestDebugControlChangedEvent;
pub use guest_file_offset_changed_event::GuestFileOffsetChangedEvent;
pub use guest_file_read_event::GuestFileReadEvent;
pub use guest_file_registered_event::GuestFileRegisteredEvent;
pub use guest_file_size_changed_event::GuestFileSizeChangedEvent;
pub use guest_file_state_changed_event::GuestFileStateChangedEvent;
pub use guest_file_write_event::GuestFileWriteEvent;
pub use guest_keyboard_event::GuestKeyboardEvent;
pub use guest_monitor_changed_event::GuestMonitorChangedEvent;
pub use guest_monitor_info_changed_event::GuestMonitorInfoChangedEvent;
pub use guest_mouse_event::GuestMouseEvent;
pub use guest_multi_touch_event::GuestMultiTouchEvent;
pub use guest_process_input_notify_event::GuestProcessInputNotifyEvent;
pub use guest_process_output_event::GuestProcessOutputEvent;
pub use guest_process_registered_event::GuestProcessRegisteredEvent;
pub use guest_process_state_changed_event::GuestProcessStateChangedEvent;
pub use guest_property_changed_event::GuestPropertyChangedEvent;
pub use guest_session_registered_event::GuestSessionRegisteredEvent;
pub use guest_session_state_changed_event::GuestSessionStateChangedEvent;
pub use guest_user_state_changed_event::GuestUserStateChangedEvent;
pub use host_audio_device_changed_event::HostAudioDeviceChangedEvent;
pub use host_name_resolution_configuration_change_event::HostNameResolutionConfigurationChangeEvent;
pub use host_pcidevice_plug_event::HostPCIDevicePlugEvent;
pub use keyboard_leds_changed_event::KeyboardLedsChangedEvent;
pub use language_changed_event::LanguageChangedEvent;
pub use machine_data_changed_event::MachineDataChangedEvent;
pub use machine_groups_changed_event::MachineGroupsChangedEvent;
pub use machine_registered_event::MachineRegisteredEvent;
pub use machine_state_changed_even::MachineStateChangedEvent;
pub use medium_changed_event::MediumChangedEvent;
pub use medium_config_changed_event::MediumConfigChangedEvent;
pub use medium_registered_event::MediumRegisteredEvent;
pub use mouse_capability_changed_event::MouseCapabilityChangedEvent;
pub use mouse_pointer_shape_changed_event::MousePointerShapeChangedEvent;
pub use nat_network_alter_event::NATNetworkAlterEvent;
pub use nat_network_changed_event::NATNetworkChangedEvent;
pub use nat_network_creation_deletion_event::NATNetworkCreationDeletionEvent;
pub use nat_network_port_forward_event::NATNetworkPortForwardEvent;
pub use nat_network_setting_event::NATNetworkSettingEvent;
pub use nat_network_start_stop_event::NATNetworkStartStopEvent;
pub use nat_redirect_event::NATRedirectEvent;
pub use network_adapter_changed_event::NetworkAdapterChangedEvent;
pub use parallel_port_changed_event::ParallelPortChangedEvent;
pub use progress_created_event::ProgressCreatedEvent;
pub use progress_percentage_changed_event::ProgressPercentageChangedEvent;
pub use progress_task_completed_event::ProgressTaskCompletedEvent;
pub use recording_changed_event::RecordingChangedEvent;
pub use runtime_error_event::RuntimeErrorEvent;
pub use serial_port_changed_event::SerialPortChangedEvent;
pub use session_state_changed_event::SessionStateChangedEvent;
pub use shared_folder_changed_event::SharedFolderChangedEvent;
pub use show_window_event::ShowWindowEvent;
pub use snapshot_changed_event::SnapshotChangedEvent;
pub use snapshot_deleted_event::SnapshotDeletedEvent;
pub use snapshot_restored_event::SnapshotRestoredEvent;
pub use snapshot_taken_event::SnapshotTakenEvent;
pub use state_changed_event::StateChangedEvent;
pub use storage_controller_changed_event::StorageControllerChangedEvent;
pub use storage_device_changed_event::StorageDeviceChangedEvent;
pub use update_agent_available_event::UpdateAgentAvailableEvent;
pub use update_agent_error_event::UpdateAgentErrorEvent;
pub use update_agent_settings_changed_event::UpdateAgentSettingsChangedEvent;
pub use update_agent_state_changed_event::UpdateAgentStateChangedEvent;
pub use usb_controller_changed_event::USBControllerChangedEvent;
pub use usb_device_state_changed_event::USBDeviceStateChangedEvent;
pub use vbox_svc_availability_changed_event::VBoxSVCAvailabilityChangedEvent;
pub use vrde_server_changed_event::VRDEServerChangedEvent;
pub use vrde_server_info_changed_event::VRDEServerInfoChangedEvent;

mod utility;

pub(crate) fn create_event_detail(event: &&Event) -> DetailEvent {
    let event_type = event.get_type();
    let event_type = match event_type {
        Ok(event_type) => event_type,
        Err(_) => return DetailEvent::Null,
    };
    match event_type {
        VBoxEventType::OnMachineStateChanged => MachineStateChangedEvent::new(event.object),
        VBoxEventType::OnSessionStateChanged => SessionStateChangedEvent::new(event.object),
        VBoxEventType::OnMachineDataChanged => MachineDataChangedEvent::new(event.object),
        VBoxEventType::OnExtraDataCanChange => ExtraDataCanChangeEvent::new(event.object),
        VBoxEventType::OnExtraDataChanged => ExtraDataChangedEvent::new(event.object),
        VBoxEventType::OnMediumRegistered => MediumRegisteredEvent::new(event.object),
        VBoxEventType::OnSnapshotTaken => SnapshotTakenEvent::new(event.object),
        VBoxEventType::OnSnapshotDeleted => SnapshotDeletedEvent::new(event.object),
        VBoxEventType::OnSnapshotChanged => SnapshotChangedEvent::new(event.object),
        VBoxEventType::OnSnapshotRestored => SnapshotRestoredEvent::new(event.object),
        VBoxEventType::OnGuestPropertyChanged => GuestPropertyChangedEvent::new(event.object),
        VBoxEventType::OnGuestKeyboard => GuestKeyboardEvent::new(event.object),
        VBoxEventType::OnGuestMouse => GuestMouseEvent::new(event.object),
        VBoxEventType::OnGuestMonitorChanged => GuestMonitorChangedEvent::new(event.object),
        VBoxEventType::OnMousePointerShapeChanged => {
            MousePointerShapeChangedEvent::new(event.object)
        }
        VBoxEventType::OnMouseCapabilityChanged => MouseCapabilityChangedEvent::new(event.object),
        VBoxEventType::OnKeyboardLedsChanged => KeyboardLedsChangedEvent::new(event.object),
        VBoxEventType::OnStateChanged => StateChangedEvent::new(event.object),
        VBoxEventType::OnAdditionsStateChanged => AdditionsStateChangedEvent::new(event.object),
        VBoxEventType::OnNetworkAdapterChanged => NetworkAdapterChangedEvent::new(event.object),
        VBoxEventType::OnSerialPortChanged => SerialPortChangedEvent::new(event.object),
        VBoxEventType::OnParallelPortChanged => ParallelPortChangedEvent::new(event.object),
        VBoxEventType::OnStorageControllerChanged => {
            StorageControllerChangedEvent::new(event.object)
        }
        VBoxEventType::OnMediumChanged => MediumChangedEvent::new(event.object),
        VBoxEventType::OnVRDEServerChanged => VRDEServerChangedEvent::new(event.object),
        VBoxEventType::OnUSBControllerChanged => USBControllerChangedEvent::new(event.object),
        VBoxEventType::OnUSBDeviceStateChanged => USBDeviceStateChangedEvent::new(event.object),
        VBoxEventType::OnSharedFolderChanged => SharedFolderChangedEvent::new(event.object),
        VBoxEventType::OnRuntimeError => RuntimeErrorEvent::new(event.object),
        VBoxEventType::OnCanShowWindow => CanShowWindowEvent::new(event.object),
        VBoxEventType::OnShowWindow => ShowWindowEvent::new(event.object),
        VBoxEventType::OnCPUChanged => CPUChangedEvent::new(event.object),
        VBoxEventType::OnVRDEServerInfoChanged => VRDEServerInfoChangedEvent::new(event.object),
        VBoxEventType::OnEventSourceChanged => EventSourceChangedEvent::new(event.object),
        VBoxEventType::OnCPUExecutionCapChanged => CPUExecutionCapChangedEvent::new(event.object),
        VBoxEventType::OnNATRedirect => NATRedirectEvent::new(event.object),
        VBoxEventType::OnHostPCIDevicePlug => HostPCIDevicePlugEvent::new(event.object),
        VBoxEventType::OnVBoxSVCAvailabilityChanged => {
            VBoxSVCAvailabilityChangedEvent::new(event.object)
        }
        VBoxEventType::OnBandwidthGroupChanged => BandwidthGroupChangedEvent::new(event.object),
        VBoxEventType::OnStorageDeviceChanged => StorageDeviceChangedEvent::new(event.object),
        VBoxEventType::OnClipboardModeChanged => ClipboardModeChangedEvent::new(event.object),
        VBoxEventType::OnDnDModeChanged => DnDModeChangedEvent::new(event.object),
        VBoxEventType::OnNATNetworkChanged => NATNetworkChangedEvent::new(event.object),
        VBoxEventType::OnNATNetworkStartStop => NATNetworkStartStopEvent::new(event.object),
        VBoxEventType::OnNATNetworkAlter => NATNetworkAlterEvent::new(event.object),
        VBoxEventType::OnNATNetworkCreationDeletion => {
            NATNetworkCreationDeletionEvent::new(event.object)
        }
        VBoxEventType::OnNATNetworkSetting => NATNetworkSettingEvent::new(event.object),
        VBoxEventType::OnNATNetworkPortForward => NATNetworkPortForwardEvent::new(event.object),
        VBoxEventType::OnGuestSessionStateChanged => {
            GuestSessionStateChangedEvent::new(event.object)
        }
        VBoxEventType::OnGuestSessionRegistered => GuestSessionRegisteredEvent::new(event.object),
        VBoxEventType::OnGuestProcessRegistered => GuestProcessRegisteredEvent::new(event.object),
        VBoxEventType::OnGuestProcessStateChanged => {
            GuestProcessStateChangedEvent::new(event.object)
        }
        VBoxEventType::OnGuestProcessInputNotify => GuestProcessInputNotifyEvent::new(event.object),
        VBoxEventType::OnGuestFileStateChanged => GuestFileStateChangedEvent::new(event.object),
        VBoxEventType::OnGuestFileOffsetChanged => GuestFileOffsetChangedEvent::new(event.object),
        VBoxEventType::OnGuestFileRead => GuestFileReadEvent::new(event.object),
        VBoxEventType::OnGuestFileWrite => GuestFileWriteEvent::new(event.object),
        VBoxEventType::OnRecordingChanged => RecordingChangedEvent::new(event.object),
        VBoxEventType::OnGuestUserStateChanged => GuestUserStateChangedEvent::new(event.object),
        VBoxEventType::OnGuestMultiTouch => GuestMultiTouchEvent::new(event.object),
        VBoxEventType::OnHostNameResolutionConfigurationChange => {
            HostNameResolutionConfigurationChangeEvent::new(event.object)
        }
        VBoxEventType::OnMediumConfigChanged => MediumConfigChangedEvent::new(event.object),
        VBoxEventType::OnAudioAdapterChanged => AudioAdapterChangedEvent::new(event.object),
        VBoxEventType::OnProgressPercentageChanged => {
            ProgressPercentageChangedEvent::new(event.object)
        }
        VBoxEventType::OnProgressTaskCompleted => ProgressTaskCompletedEvent::new(event.object),
        VBoxEventType::OnCursorPositionChanged => CursorPositionChangedEvent::new(event.object),
        VBoxEventType::OnMachineRegistered => MachineRegisteredEvent::new(event.object),
        VBoxEventType::OnGuestAdditionsStatusChanged => {
            GuestAdditionsStatusChangedEvent::new(event.object)
        }
        VBoxEventType::OnGuestMonitorInfoChanged => GuestMonitorInfoChangedEvent::new(event.object),
        VBoxEventType::OnGuestFileSizeChanged => GuestFileSizeChangedEvent::new(event.object),
        VBoxEventType::OnClipboardFileTransferModeChanged => {
            ClipboardFileTransferModeChangedEvent::new(event.object)
        }
        VBoxEventType::OnCloudProviderListChanged => {
            CloudProviderListChangedEvent::new(event.object)
        }
        VBoxEventType::OnCloudProviderRegistered => CloudProviderRegisteredEvent::new(event.object),
        VBoxEventType::OnCloudProviderUninstall => CloudProviderUninstallEvent::new(event.object),
        VBoxEventType::OnCloudProfileRegistered => CloudProfileRegisteredEvent::new(event.object),
        VBoxEventType::OnCloudProfileChanged => CloudProfileChangedEvent::new(event.object),
        VBoxEventType::OnProgressCreated => ProgressCreatedEvent::new(event.object),
        VBoxEventType::OnLanguageChanged => LanguageChangedEvent::new(event.object),
        VBoxEventType::OnUpdateAgentAvailable => UpdateAgentAvailableEvent::new(event.object),
        VBoxEventType::OnUpdateAgentError => UpdateAgentErrorEvent::new(event.object),
        VBoxEventType::OnUpdateAgentSettingsChanged => {
            UpdateAgentSettingsChangedEvent::new(event.object)
        }
        VBoxEventType::OnUpdateAgentStateChanged => UpdateAgentStateChangedEvent::new(event.object),
        VBoxEventType::OnMachineGroupsChanged => MachineGroupsChangedEvent::new(event.object),
        VBoxEventType::OnHostAudioDeviceChanged => HostAudioDeviceChangedEvent::new(event.object),
        VBoxEventType::OnGuestDebugControlChanged => {
            GuestDebugControlChangedEvent::new(event.object)
        }

        _ => DetailEvent::Null,
    }
}
#[derive(Debug)]
pub enum DetailEvent {
    MachineStateChangedEvent(MachineStateChangedEvent),
    SessionStateChangedEvent(SessionStateChangedEvent),
    MachineDataChangedEvent(MachineDataChangedEvent),
    ExtraDataCanChangeEvent(ExtraDataCanChangeEvent),
    ExtraDataChangedEvent(ExtraDataChangedEvent),
    MediumRegisteredEvent(MediumRegisteredEvent),
    SnapshotTakenEvent(SnapshotTakenEvent),
    SnapshotDeletedEvent(SnapshotDeletedEvent),
    SnapshotChangedEvent(SnapshotChangedEvent),
    SnapshotRestoredEvent(SnapshotRestoredEvent),
    GuestPropertyChangedEvent(GuestPropertyChangedEvent),
    GuestKeyboardEvent(GuestKeyboardEvent),
    GuestMouseEvent(GuestMouseEvent),
    GuestMonitorChangedEvent(GuestMonitorChangedEvent),
    MousePointerShapeChangedEvent(MousePointerShapeChangedEvent),
    MouseCapabilityChangedEvent(MouseCapabilityChangedEvent),
    KeyboardLedsChangedEvent(KeyboardLedsChangedEvent),
    StateChangedEvent(StateChangedEvent),
    AdditionsStateChangedEvent(AdditionsStateChangedEvent),
    NetworkAdapterChangedEvent(NetworkAdapterChangedEvent),
    SerialPortChangedEvent(SerialPortChangedEvent),
    ParallelPortChangedEvent(ParallelPortChangedEvent),
    StorageControllerChangedEvent(StorageControllerChangedEvent),
    MediumChangedEvent(MediumChangedEvent),
    VRDEServerChangedEvent(VRDEServerChangedEvent),
    USBControllerChangedEvent(USBControllerChangedEvent),
    USBDeviceStateChangedEvent(USBDeviceStateChangedEvent),
    SharedFolderChangedEvent(SharedFolderChangedEvent),
    RuntimeErrorEvent(RuntimeErrorEvent),
    CanShowWindowEvent(CanShowWindowEvent),
    ShowWindowEvent(ShowWindowEvent),
    CPUChangedEvent(CPUChangedEvent),
    VRDEServerInfoChangedEvent(VRDEServerInfoChangedEvent),
    EventSourceChangedEvent(EventSourceChangedEvent),
    CPUExecutionCapChangedEvent(CPUExecutionCapChangedEvent),
    NATRedirectEvent(NATRedirectEvent),
    HostPCIDevicePlugEvent(HostPCIDevicePlugEvent),
    VBoxSVCAvailabilityChangedEvent(VBoxSVCAvailabilityChangedEvent),
    BandwidthGroupChangedEvent(BandwidthGroupChangedEvent),
    StorageDeviceChangedEvent(StorageDeviceChangedEvent),
    ClipboardModeChangedEvent(ClipboardModeChangedEvent),
    DnDModeChangedEvent(DnDModeChangedEvent),
    NATNetworkChangedEvent(NATNetworkChangedEvent),
    NATNetworkStartStopEvent(NATNetworkStartStopEvent),
    NATNetworkAlterEvent(NATNetworkAlterEvent),
    NATNetworkCreationDeletionEvent(NATNetworkCreationDeletionEvent),
    NATNetworkSettingEvent(NATNetworkSettingEvent),
    NATNetworkPortForwardEvent(NATNetworkPortForwardEvent),
    GuestSessionStateChangedEvent(GuestSessionStateChangedEvent),
    GuestSessionRegisteredEvent(GuestSessionRegisteredEvent),
    GuestProcessRegisteredEvent(GuestProcessRegisteredEvent),
    GuestProcessStateChangedEvent(GuestProcessStateChangedEvent),
    GuestProcessInputNotifyEvent(GuestProcessInputNotifyEvent),
    GuestProcessOutputEvent(GuestProcessOutputEvent),
    GuestFileRegisteredEvent(GuestFileRegisteredEvent),
    GuestFileStateChangedEvent(GuestFileStateChangedEvent),
    GuestFileOffsetChangedEvent(GuestFileOffsetChangedEvent),
    GuestFileReadEvent(GuestFileReadEvent),
    GuestFileWriteEvent(GuestFileWriteEvent),
    RecordingChangedEvent(RecordingChangedEvent),
    GuestUserStateChangedEvent(GuestUserStateChangedEvent),
    GuestMultiTouchEvent(GuestMultiTouchEvent),
    HostNameResolutionConfigurationChangeEvent(HostNameResolutionConfigurationChangeEvent),
    MediumConfigChangedEvent(MediumConfigChangedEvent),
    AudioAdapterChangedEvent(AudioAdapterChangedEvent),
    ProgressPercentageChangedEvent(ProgressPercentageChangedEvent),
    ProgressTaskCompletedEvent(ProgressTaskCompletedEvent),
    CursorPositionChangedEvent(CursorPositionChangedEvent),
    MachineRegisteredEvent(MachineRegisteredEvent),
    GuestAdditionsStatusChangedEvent(GuestAdditionsStatusChangedEvent),
    GuestMonitorInfoChangedEvent(GuestMonitorInfoChangedEvent),
    GuestFileSizeChangedEvent(GuestFileSizeChangedEvent),
    ClipboardFileTransferModeChangedEvent(ClipboardFileTransferModeChangedEvent),
    CloudProviderListChangedEvent(CloudProviderListChangedEvent),
    CloudProviderRegisteredEvent(CloudProviderRegisteredEvent),
    CloudProviderUninstallEvent(CloudProviderUninstallEvent),
    CloudProfileRegisteredEvent(CloudProfileRegisteredEvent),
    CloudProfileChangedEvent(CloudProfileChangedEvent),
    ProgressCreatedEvent(ProgressCreatedEvent),
    LanguageChangedEvent(LanguageChangedEvent),
    UpdateAgentAvailableEvent(UpdateAgentAvailableEvent),
    UpdateAgentErrorEvent(UpdateAgentErrorEvent),
    UpdateAgentSettingsChangedEvent(UpdateAgentSettingsChangedEvent),
    UpdateAgentStateChangedEvent(UpdateAgentStateChangedEvent),
    MachineGroupsChangedEvent(MachineGroupsChangedEvent),
    HostAudioDeviceChangedEvent(HostAudioDeviceChangedEvent),
    GuestDebugControlChangedEvent(GuestDebugControlChangedEvent),
    Null,
}

impl Display for DetailEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DetailEvent::Null => write!(f, "{}", "None".to_string()),
            DetailEvent::MachineStateChangedEvent(event) => write!(f, "{}", event),
            DetailEvent::SessionStateChangedEvent(event) => write!(f, "{}", event),
            DetailEvent::MachineDataChangedEvent(event) => write!(f, "{}", event),
            DetailEvent::ExtraDataCanChangeEvent(event) => write!(f, "{}", event),
            DetailEvent::ExtraDataChangedEvent(event) => write!(f, "{}", event),
            DetailEvent::MediumRegisteredEvent(event) => write!(f, "{}", event),
            DetailEvent::SnapshotTakenEvent(event) => write!(f, "{}", event),
            DetailEvent::SnapshotDeletedEvent(event) => write!(f, "{}", event),
            DetailEvent::SnapshotChangedEvent(event) => write!(f, "{}", event),
            DetailEvent::SnapshotRestoredEvent(event) => write!(f, "{}", event),
            DetailEvent::GuestPropertyChangedEvent(event) => write!(f, "{}", event),
            DetailEvent::GuestKeyboardEvent(event) => write!(f, "{}", event),
            DetailEvent::GuestMouseEvent(event) => write!(f, "{}", event),
            DetailEvent::GuestMonitorChangedEvent(event) => write!(f, "{}", event),
            DetailEvent::MousePointerShapeChangedEvent(event) => write!(f, "{}", event),
            DetailEvent::MouseCapabilityChangedEvent(event) => write!(f, "{}", event),
            DetailEvent::KeyboardLedsChangedEvent(event) => write!(f, "{}", event),
            DetailEvent::StateChangedEvent(event) => write!(f, "{}", event),
            DetailEvent::AdditionsStateChangedEvent(event) => write!(f, "{}", event),
            DetailEvent::NetworkAdapterChangedEvent(event) => write!(f, "{}", event),
            DetailEvent::SerialPortChangedEvent(event) => write!(f, "{}", event),
            DetailEvent::ParallelPortChangedEvent(event) => write!(f, "{}", event),
            DetailEvent::StorageControllerChangedEvent(event) => write!(f, "{}", event),
            DetailEvent::MediumChangedEvent(event) => write!(f, "{}", event),
            DetailEvent::VRDEServerChangedEvent(event) => write!(f, "{}", event),
            DetailEvent::USBControllerChangedEvent(event) => write!(f, "{}", event),
            DetailEvent::USBDeviceStateChangedEvent(event) => write!(f, "{}", event),
            DetailEvent::SharedFolderChangedEvent(event) => write!(f, "{}", event),
            DetailEvent::RuntimeErrorEvent(event) => write!(f, "{}", event),
            DetailEvent::CanShowWindowEvent(event) => write!(f, "{}", event),
            DetailEvent::ShowWindowEvent(event) => write!(f, "{}", event),
            DetailEvent::CPUChangedEvent(event) => write!(f, "{}", event),
            DetailEvent::VRDEServerInfoChangedEvent(event) => write!(f, "{}", event),
            DetailEvent::EventSourceChangedEvent(event) => write!(f, "{}", event),
            DetailEvent::CPUExecutionCapChangedEvent(event) => write!(f, "{}", event),
            DetailEvent::NATRedirectEvent(event) => write!(f, "{}", event),
            DetailEvent::HostPCIDevicePlugEvent(event) => write!(f, "{}", event),
            DetailEvent::VBoxSVCAvailabilityChangedEvent(event) => write!(f, "{}", event),
            DetailEvent::BandwidthGroupChangedEvent(event) => write!(f, "{}", event),
            DetailEvent::StorageDeviceChangedEvent(event) => write!(f, "{}", event),
            DetailEvent::ClipboardModeChangedEvent(event) => write!(f, "{}", event),
            DetailEvent::DnDModeChangedEvent(event) => write!(f, "{}", event),
            DetailEvent::NATNetworkChangedEvent(event) => write!(f, "{}", event),
            DetailEvent::NATNetworkStartStopEvent(event) => write!(f, "{}", event),
            DetailEvent::NATNetworkAlterEvent(event) => write!(f, "{}", event),
            DetailEvent::NATNetworkCreationDeletionEvent(event) => write!(f, "{}", event),
            DetailEvent::NATNetworkSettingEvent(event) => write!(f, "{}", event),
            DetailEvent::NATNetworkPortForwardEvent(event) => write!(f, "{}", event),
            DetailEvent::GuestSessionStateChangedEvent(event) => write!(f, "{}", event),
            DetailEvent::GuestSessionRegisteredEvent(event) => write!(f, "{}", event),
            DetailEvent::GuestProcessRegisteredEvent(event) => write!(f, "{}", event),
            DetailEvent::GuestProcessStateChangedEvent(event) => write!(f, "{}", event),
            DetailEvent::GuestProcessInputNotifyEvent(event) => write!(f, "{}", event),
            DetailEvent::GuestProcessOutputEvent(event) => write!(f, "{}", event),
            DetailEvent::GuestFileRegisteredEvent(event) => write!(f, "{}", event),
            DetailEvent::GuestFileStateChangedEvent(event) => write!(f, "{}", event),
            DetailEvent::GuestFileOffsetChangedEvent(event) => write!(f, "{}", event),
            DetailEvent::GuestFileReadEvent(event) => write!(f, "{}", event),
            DetailEvent::GuestFileWriteEvent(event) => write!(f, "{}", event),
            DetailEvent::RecordingChangedEvent(event) => write!(f, "{}", event),
            DetailEvent::GuestUserStateChangedEvent(event) => write!(f, "{}", event),
            DetailEvent::GuestMultiTouchEvent(event) => write!(f, "{}", event),
            DetailEvent::HostNameResolutionConfigurationChangeEvent(event) => {
                write!(f, "{}", event)
            }
            DetailEvent::MediumConfigChangedEvent(event) => write!(f, "{}", event),
            DetailEvent::AudioAdapterChangedEvent(event) => write!(f, "{}", event),
            DetailEvent::ProgressPercentageChangedEvent(event) => write!(f, "{}", event),
            DetailEvent::ProgressTaskCompletedEvent(event) => write!(f, "{}", event),
            DetailEvent::CursorPositionChangedEvent(event) => write!(f, "{}", event),
            DetailEvent::MachineRegisteredEvent(event) => write!(f, "{}", event),
            DetailEvent::GuestAdditionsStatusChangedEvent(event) => write!(f, "{}", event),
            DetailEvent::GuestMonitorInfoChangedEvent(event) => write!(f, "{}", event),
            DetailEvent::GuestFileSizeChangedEvent(event) => write!(f, "{}", event),
            DetailEvent::ClipboardFileTransferModeChangedEvent(event) => write!(f, "{}", event),
            DetailEvent::CloudProviderListChangedEvent(event) => write!(f, "{}", event),
            DetailEvent::CloudProviderRegisteredEvent(event) => write!(f, "{}", event),
            DetailEvent::CloudProviderUninstallEvent(event) => write!(f, "{}", event),
            DetailEvent::CloudProfileRegisteredEvent(event) => write!(f, "{}", event),
            DetailEvent::CloudProfileChangedEvent(event) => write!(f, "{}", event),
            DetailEvent::ProgressCreatedEvent(event) => write!(f, "{}", event),
            DetailEvent::LanguageChangedEvent(event) => write!(f, "{}", event),
            DetailEvent::UpdateAgentAvailableEvent(event) => write!(f, "{}", event),
            DetailEvent::UpdateAgentErrorEvent(event) => write!(f, "{}", event),
            DetailEvent::UpdateAgentSettingsChangedEvent(event) => write!(f, "{}", event),
            DetailEvent::UpdateAgentStateChangedEvent(event) => write!(f, "{}", event),
            DetailEvent::MachineGroupsChangedEvent(event) => write!(f, "{}", event),
            DetailEvent::HostAudioDeviceChangedEvent(event) => write!(f, "{}", event),
            DetailEvent::GuestDebugControlChangedEvent(event) => write!(f, "{}", event),
        }
    }
}
