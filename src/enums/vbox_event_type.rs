#[cfg(doc)]
use crate::event_detail::{
    AdditionsStateChangedEvent, AudioAdapterChangedEvent, BandwidthGroupChangedEvent,
    CPUChangedEvent, CPUExecutionCapChangedEvent, CanShowWindowEvent,
    ClipboardFileTransferModeChangedEvent, ClipboardModeChangedEvent, CloudProfileChangedEvent,
    CloudProfileRegisteredEvent, CloudProviderListChangedEvent, CloudProviderRegisteredEvent,
    CloudProviderUninstallEvent, CursorPositionChangedEvent, DnDModeChangedEvent,
    EventSourceChangedEvent, ExtraDataCanChangeEvent, ExtraDataChangedEvent,
    GuestAdditionsStatusChangedEvent, GuestDebugControlChangedEvent, GuestFileOffsetChangedEvent,
    GuestFileReadEvent, GuestFileRegisteredEvent, GuestFileSizeChangedEvent,
    GuestFileStateChangedEvent, GuestFileWriteEvent, GuestKeyboardEvent,
    GuestMonitorInfoChangedEvent, GuestMouseEvent, GuestMultiTouchEvent,
    GuestProcessInputNotifyEvent, GuestProcessOutputEvent, GuestProcessRegisteredEvent,
    GuestProcessStateChangedEvent, GuestPropertyChangedEvent, GuestSessionRegisteredEvent,
    GuestSessionStateChangedEvent, GuestUserStateChangedEvent, HostAudioDeviceChangedEvent,
    HostNameResolutionConfigurationChangeEvent, HostPCIDevicePlugEvent, KeyboardLedsChangedEvent,
    LanguageChangedEvent, MachineDataChangedEvent, MachineGroupsChangedEvent,
    MachineRegisteredEvent, MachineStateChangedEvent, MediumChangedEvent, MediumConfigChangedEvent,
    MediumRegisteredEvent, MouseCapabilityChangedEvent, MousePointerShapeChangedEvent,
    NATNetworkAlterEvent, NATNetworkChangedEvent, NATNetworkCreationDeletionEvent,
    NATNetworkPortForwardEvent, NATNetworkSettingEvent, NATNetworkStartStopEvent, NATRedirectEvent,
    NetworkAdapterChangedEvent, ParallelPortChangedEvent, ProgressCreatedEvent,
    ProgressPercentageChangedEvent, ProgressTaskCompletedEvent, RecordingChangedEvent,
    RuntimeErrorEvent, SerialPortChangedEvent, SessionStateChangedEvent, SharedFolderChangedEvent,
    ShowWindowEvent, SnapshotChangedEvent, SnapshotDeletedEvent, SnapshotRestoredEvent,
    SnapshotTakenEvent, StateChangedEvent, StorageControllerChangedEvent,
    StorageDeviceChangedEvent, USBControllerChangedEvent, USBDeviceStateChangedEvent,
    UpdateAgentAvailableEvent, UpdateAgentErrorEvent, UpdateAgentSettingsChangedEvent,
    UpdateAgentStateChangedEvent, VBoxSVCAvailabilityChangedEvent, VRDEServerChangedEvent,
    VRDEServerInfoChangedEvent,
};
#[cfg(doc)]
use crate::EventSource;
use std::fmt::Display;
use vbox_raw::sys_lib as raw;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum VBoxEventType {
    /// Invalid event, must be first.
    Invalid,
    /// Wildcard for all events.
    ///
    /// Events of this type are never delivered, and only used in [`EventSource::register_listener`] call to simplify registration.
    Any,
    /// Wildcard for all vetoable events.
    ///
    /// Events of this type are never delivered, and only used in [`EventSource::register_listener`] call to simplify registration.
    Vetoable,
    /// Wildcard for all machine events.
    ///
    /// Events of this type are never delivered, and only used in [`EventSource::register_listener`] call to simplify registration.
    MachineEvent,
    /// Wildcard for all snapshot events.
    ///
    /// Events of this type are never delivered, and only used in [`EventSource::register_listener`] call to simplify registration.
    SnapshotEvent,
    /// Wildcard for all input device (keyboard, mouse) events.
    ///
    /// Events of this type are never delivered, and only used in [`EventSource::register_listener`] call to simplify registration.
    InputEvent,
    /// Last wildcard.
    LastWildcard,
    /// See [`MachineStateChangedEvent`].
    OnMachineStateChanged,
    /// See [`MachineDataChangedEvent`].
    OnMachineDataChanged,
    /// See [`ExtraDataChangedEvent`].
    OnExtraDataChanged,
    /// See [`ExtraDataCanChangeEvent`].
    OnExtraDataCanChange,
    /// See [`MediumRegisteredEvent`].
    OnMediumRegistered,
    /// See [`MachineRegisteredEvent`].
    OnMachineRegistered,
    /// See [`SessionStateChangedEvent`].
    OnSessionStateChanged,
    /// See [`SnapshotTakenEvent`].
    OnSnapshotTaken,
    /// See [`SnapshotDeletedEvent`].
    OnSnapshotDeleted,
    /// See [`SnapshotChangedEvent`].
    OnSnapshotChanged,
    /// See [`GuestPropertyChangedEvent`].
    OnGuestPropertyChanged,
    /// See [`MousePointerShapeChangedEvent`].
    OnMousePointerShapeChanged,
    /// See [`MouseCapabilityChangedEvent`].
    OnMouseCapabilityChanged,
    /// See [`KeyboardLedsChangedEvent`].
    OnKeyboardLedsChanged,
    /// See [`StateChangedEvent`].
    OnStateChanged,
    /// See [`AdditionsStateChangedEvent`].
    OnAdditionsStateChanged,
    /// See [`NetworkAdapterChangedEvent`].
    OnNetworkAdapterChanged,
    /// See [`SerialPortChangedEvent`].
    OnSerialPortChanged,
    /// See [`ParallelPortChangedEvent`].
    OnParallelPortChanged,
    /// See [`StorageControllerChangedEvent`].
    OnStorageControllerChanged,
    /// See [`MediumChangedEvent`].
    OnMediumChanged,
    /// See [`VRDEServerChangedEvent`].
    OnVRDEServerChanged,
    /// See [`USBControllerChangedEvent`].
    OnUSBControllerChanged,
    /// See [`USBDeviceStateChangedEvent`].
    OnUSBDeviceStateChanged,
    /// See [`SharedFolderChangedEvent`].
    OnSharedFolderChanged,
    /// See [`RuntimeErrorEvent`].
    OnRuntimeError,
    /// See [`CanShowWindowEvent`].
    OnCanShowWindow,
    /// See [`ShowWindowEvent`].
    OnShowWindow,
    /// See [`CPUChangedEvent`].
    OnCPUChanged,
    /// See [`VRDEServerInfoChangedEvent`].
    OnVRDEServerInfoChanged,
    /// See [`EventSourceChangedEvent`].
    OnEventSourceChanged,
    /// See [`CPUExecutionCapChangedEvent`].
    OnCPUExecutionCapChanged,
    /// See [`GuestKeyboardEvent`].
    OnGuestKeyboard,
    /// See [`GuestMouseEvent`].
    OnGuestMouse,
    /// See [`NATRedirectEvent`].
    OnNATRedirect,
    /// See [`HostPCIDevicePlugEvent`].
    OnHostPCIDevicePlug,
    /// See [`VBoxSVCAvailabilityChangedEvent`].
    OnVBoxSVCAvailabilityChanged,
    /// See [`BandwidthGroupChangedEvent`].
    OnBandwidthGroupChanged,
    /// See GuestMonitorChangedEvent.
    OnGuestMonitorChanged,
    /// See [`StorageDeviceChangedEvent`].
    OnStorageDeviceChanged,
    /// See [`ClipboardModeChangedEvent`].
    OnClipboardModeChanged,
    /// See [`DnDModeChangedEvent`].
    OnDnDModeChanged,
    /// See [`NATNetworkChangedEvent`].
    OnNATNetworkChanged,
    /// See [`NATNetworkStartStopEvent`].
    OnNATNetworkStartStop,
    /// See [`NATNetworkAlterEvent`].
    OnNATNetworkAlter,
    /// See [`NATNetworkCreationDeletionEvent`].
    OnNATNetworkCreationDeletion,
    /// See [`NATNetworkSettingEvent`].
    OnNATNetworkSetting,
    /// See [`NATNetworkPortForwardEvent`].
    OnNATNetworkPortForward,
    /// See [`GuestSessionStateChangedEvent`].
    OnGuestSessionStateChanged,
    /// See [`GuestSessionRegisteredEvent`].
    OnGuestSessionRegistered,
    /// See [`GuestProcessRegisteredEvent`].
    OnGuestProcessRegistered,
    /// See [`GuestProcessStateChangedEvent`].
    OnGuestProcessStateChanged,
    /// See [`GuestProcessInputNotifyEvent`].
    OnGuestProcessInputNotify,
    /// See [`GuestProcessOutputEvent`].
    OnGuestProcessOutput,
    /// See [`GuestFileRegisteredEvent`].
    OnGuestFileRegistered,
    /// See [`GuestFileStateChangedEvent`].
    OnGuestFileStateChanged,
    /// See [`GuestFileOffsetChangedEvent`].
    OnGuestFileOffsetChanged,
    /// See [`GuestFileReadEvent`].
    OnGuestFileRead,
    /// See [`GuestFileWriteEvent`].
    OnGuestFileWrite,
    /// See [`RecordingChangedEvent`].
    OnRecordingChanged,
    /// See [`GuestUserStateChangedEvent`].
    OnGuestUserStateChanged,
    /// See [`GuestMultiTouchEvent`].
    OnGuestMultiTouch,
    /// See [`HostNameResolutionConfigurationChangeEvent`].
    OnHostNameResolutionConfigurationChange,
    /// See [`SnapshotRestoredEvent`].
    OnSnapshotRestored,
    /// See [`MediumConfigChangedEvent`].
    OnMediumConfigChanged,
    /// See [`AudioAdapterChangedEvent`].
    OnAudioAdapterChanged,
    /// See [`ProgressPercentageChangedEvent`].
    OnProgressPercentageChanged,
    /// See [`ProgressTaskCompletedEvent`].
    OnProgressTaskCompleted,
    /// See [`CursorPositionChangedEvent`].
    OnCursorPositionChanged,
    /// See [`GuestAdditionsStatusChangedEvent`].
    OnGuestAdditionsStatusChanged,
    /// See [`GuestMonitorInfoChangedEvent`].
    OnGuestMonitorInfoChanged,
    /// See [`GuestFileSizeChangedEvent`].
    OnGuestFileSizeChanged,
    /// See [`ClipboardFileTransferModeChangedEvent`].
    OnClipboardFileTransferModeChanged,
    /// See [`CloudProviderListChangedEvent`].
    /// <div class="warning">
    /// This EventType only exists for versions 7 and above.
    /// To ensure compatibility with earlier versions, this field has been retained, but this type will never be returned.
    /// </div>
    OnCloudProviderListChanged,
    /// See [`CloudProviderRegisteredEvent`].
    /// <div class="warning">
    /// This EventType only exists for versions 7 and above.
    /// To ensure compatibility with earlier versions, this field has been retained, but this type will never be returned.
    /// </div>
    OnCloudProviderRegistered,
    /// See [`CloudProviderUninstallEvent`].
    /// <div class="warning">
    /// This EventType only exists for versions 7 and above.
    /// To ensure compatibility with earlier versions, this field has been retained, but this type will never be returned.
    /// </div>
    OnCloudProviderUninstall,
    /// See [`CloudProfileRegisteredEvent`].
    /// <div class="warning">
    /// This EventType only exists for versions 7 and above.
    /// To ensure compatibility with earlier versions, this field has been retained, but this type will never be returned.
    /// </div>
    OnCloudProfileRegistered,
    /// See [`CloudProfileChangedEvent`].
    /// <div class="warning">
    /// This EventType only exists for versions 7 and above.
    /// To ensure compatibility with earlier versions, this field has been retained, but this type will never be returned.
    /// </div>
    OnCloudProfileChanged,
    /// See [`ProgressCreatedEvent`].
    /// <div class="warning">
    /// This EventType only exists for versions 7 and above.
    /// To ensure compatibility with earlier versions, this field has been retained, but this type will never be returned.
    /// </div>
    OnProgressCreated,
    /// See [`LanguageChangedEvent`].
    /// <div class="warning">
    /// This EventType only exists for versions 7 and above.
    /// To ensure compatibility with earlier versions, this field has been retained, but this type will never be returned.
    /// </div>
    OnLanguageChanged,
    /// See [`UpdateAgentAvailableEvent`].
    /// <div class="warning">
    /// This EventType only exists for versions 7 and above.
    /// To ensure compatibility with earlier versions, this field has been retained, but this type will never be returned.
    /// </div>
    OnUpdateAgentAvailable,
    /// See [`UpdateAgentErrorEvent`].
    /// <div class="warning">
    /// This EventType only exists for versions 7 and above.
    /// To ensure compatibility with earlier versions, this field has been retained, but this type will never be returned.
    /// </div>
    OnUpdateAgentError,
    /// See [`UpdateAgentSettingsChangedEvent`].
    /// <div class="warning">
    /// This EventType only exists for versions 7 and above.
    /// To ensure compatibility with earlier versions, this field has been retained, but this type will never be returned.
    /// </div>
    OnUpdateAgentSettingsChanged,
    /// See [`UpdateAgentStateChangedEvent`].
    /// <div class="warning">
    /// This EventType only exists for versions 7 and above.
    /// To ensure compatibility with earlier versions, this field has been retained, but this type will never be returned.
    /// </div>
    OnUpdateAgentStateChanged,
    /// See [`HostAudioDeviceChangedEvent`].
    /// <div class="warning">
    /// This EventType only exists for versions 7 and above.
    /// To ensure compatibility with earlier versions, this field has been retained, but this type will never be returned.
    /// </div>
    OnHostAudioDeviceChanged,
    /// See [`GuestDebugControlChangedEvent`].
    /// <div class="warning">
    /// This EventType only exists for versions 7 and above.
    /// To ensure compatibility with earlier versions, this field has been retained, but this type will never be returned.
    /// </div>
    OnGuestDebugControlChanged,
    /// See [`MachineGroupsChangedEvent`].
    /// <div class="warning">
    /// This EventType only exists for versions 7 and above.
    /// To ensure compatibility with earlier versions, this field has been retained, but this type will never be returned.
    /// </div>
    OnMachineGroupsChanged,
    /// Must be last event, used for iterations and structures relying on numerical event values.
    /// <div class="warning">
    /// This EventType only exists for versions 7 and above.
    /// To ensure compatibility with earlier versions, this field has been retained, but this type will never be returned.
    /// </div>
    End,
}

impl From<u32> for VBoxEventType {
    fn from(value: u32) -> Self {
        match value {
            raw::VBoxEventType_VBoxEventType_Invalid => VBoxEventType::Invalid,
            raw::VBoxEventType_VBoxEventType_Any => VBoxEventType::Any,
            raw::VBoxEventType_VBoxEventType_Vetoable => VBoxEventType::Vetoable,
            raw::VBoxEventType_VBoxEventType_MachineEvent => VBoxEventType::MachineEvent,
            raw::VBoxEventType_VBoxEventType_SnapshotEvent => VBoxEventType::SnapshotEvent,
            raw::VBoxEventType_VBoxEventType_InputEvent => VBoxEventType::InputEvent,
            raw::VBoxEventType_VBoxEventType_LastWildcard => VBoxEventType::LastWildcard,
            raw::VBoxEventType_VBoxEventType_OnMachineStateChanged => {
                VBoxEventType::OnMachineStateChanged
            }
            raw::VBoxEventType_VBoxEventType_OnMachineDataChanged => {
                VBoxEventType::OnMachineDataChanged
            }
            raw::VBoxEventType_VBoxEventType_OnExtraDataChanged => {
                VBoxEventType::OnExtraDataChanged
            }
            raw::VBoxEventType_VBoxEventType_OnExtraDataCanChange => {
                VBoxEventType::OnExtraDataCanChange
            }
            raw::VBoxEventType_VBoxEventType_OnMediumRegistered => {
                VBoxEventType::OnMediumRegistered
            }
            raw::VBoxEventType_VBoxEventType_OnMachineRegistered => {
                VBoxEventType::OnMachineRegistered
            }
            raw::VBoxEventType_VBoxEventType_OnSessionStateChanged => {
                VBoxEventType::OnSessionStateChanged
            }
            raw::VBoxEventType_VBoxEventType_OnSnapshotTaken => VBoxEventType::OnSnapshotTaken,
            raw::VBoxEventType_VBoxEventType_OnSnapshotDeleted => VBoxEventType::OnSnapshotDeleted,
            raw::VBoxEventType_VBoxEventType_OnSnapshotChanged => VBoxEventType::OnSnapshotChanged,
            raw::VBoxEventType_VBoxEventType_OnGuestPropertyChanged => {
                VBoxEventType::OnGuestPropertyChanged
            }
            raw::VBoxEventType_VBoxEventType_OnMousePointerShapeChanged => {
                VBoxEventType::OnMousePointerShapeChanged
            }
            raw::VBoxEventType_VBoxEventType_OnMouseCapabilityChanged => {
                VBoxEventType::OnMouseCapabilityChanged
            }
            raw::VBoxEventType_VBoxEventType_OnKeyboardLedsChanged => {
                VBoxEventType::OnKeyboardLedsChanged
            }
            raw::VBoxEventType_VBoxEventType_OnStateChanged => VBoxEventType::OnStateChanged,
            raw::VBoxEventType_VBoxEventType_OnAdditionsStateChanged => {
                VBoxEventType::OnAdditionsStateChanged
            }
            raw::VBoxEventType_VBoxEventType_OnNetworkAdapterChanged => {
                VBoxEventType::OnNetworkAdapterChanged
            }
            raw::VBoxEventType_VBoxEventType_OnSerialPortChanged => {
                VBoxEventType::OnSerialPortChanged
            }
            raw::VBoxEventType_VBoxEventType_OnParallelPortChanged => {
                VBoxEventType::OnParallelPortChanged
            }
            raw::VBoxEventType_VBoxEventType_OnStorageControllerChanged => {
                VBoxEventType::OnStorageControllerChanged
            }
            raw::VBoxEventType_VBoxEventType_OnMediumChanged => VBoxEventType::OnMediumChanged,
            raw::VBoxEventType_VBoxEventType_OnVRDEServerChanged => {
                VBoxEventType::OnVRDEServerChanged
            }
            raw::VBoxEventType_VBoxEventType_OnUSBControllerChanged => {
                VBoxEventType::OnUSBControllerChanged
            }
            raw::VBoxEventType_VBoxEventType_OnUSBDeviceStateChanged => {
                VBoxEventType::OnUSBDeviceStateChanged
            }
            raw::VBoxEventType_VBoxEventType_OnSharedFolderChanged => {
                VBoxEventType::OnSharedFolderChanged
            }
            raw::VBoxEventType_VBoxEventType_OnRuntimeError => VBoxEventType::OnRuntimeError,
            raw::VBoxEventType_VBoxEventType_OnCanShowWindow => VBoxEventType::OnCanShowWindow,
            raw::VBoxEventType_VBoxEventType_OnShowWindow => VBoxEventType::OnShowWindow,
            raw::VBoxEventType_VBoxEventType_OnCPUChanged => VBoxEventType::OnCPUChanged,
            raw::VBoxEventType_VBoxEventType_OnVRDEServerInfoChanged => {
                VBoxEventType::OnVRDEServerInfoChanged
            }
            raw::VBoxEventType_VBoxEventType_OnEventSourceChanged => {
                VBoxEventType::OnEventSourceChanged
            }
            raw::VBoxEventType_VBoxEventType_OnCPUExecutionCapChanged => {
                VBoxEventType::OnCPUExecutionCapChanged
            }
            raw::VBoxEventType_VBoxEventType_OnGuestKeyboard => VBoxEventType::OnGuestKeyboard,
            raw::VBoxEventType_VBoxEventType_OnGuestMouse => VBoxEventType::OnGuestMouse,
            raw::VBoxEventType_VBoxEventType_OnNATRedirect => VBoxEventType::OnNATRedirect,
            raw::VBoxEventType_VBoxEventType_OnHostPCIDevicePlug => {
                VBoxEventType::OnHostPCIDevicePlug
            }
            raw::VBoxEventType_VBoxEventType_OnVBoxSVCAvailabilityChanged => {
                VBoxEventType::OnVBoxSVCAvailabilityChanged
            }
            raw::VBoxEventType_VBoxEventType_OnBandwidthGroupChanged => {
                VBoxEventType::OnBandwidthGroupChanged
            }
            raw::VBoxEventType_VBoxEventType_OnGuestMonitorChanged => {
                VBoxEventType::OnGuestMonitorChanged
            }
            raw::VBoxEventType_VBoxEventType_OnStorageDeviceChanged => {
                VBoxEventType::OnStorageDeviceChanged
            }
            raw::VBoxEventType_VBoxEventType_OnClipboardModeChanged => {
                VBoxEventType::OnClipboardModeChanged
            }
            raw::VBoxEventType_VBoxEventType_OnDnDModeChanged => VBoxEventType::OnDnDModeChanged,
            raw::VBoxEventType_VBoxEventType_OnNATNetworkChanged => {
                VBoxEventType::OnNATNetworkChanged
            }
            raw::VBoxEventType_VBoxEventType_OnNATNetworkStartStop => {
                VBoxEventType::OnNATNetworkStartStop
            }
            raw::VBoxEventType_VBoxEventType_OnNATNetworkAlter => VBoxEventType::OnNATNetworkAlter,
            raw::VBoxEventType_VBoxEventType_OnNATNetworkCreationDeletion => {
                VBoxEventType::OnNATNetworkCreationDeletion
            }
            raw::VBoxEventType_VBoxEventType_OnNATNetworkSetting => {
                VBoxEventType::OnNATNetworkSetting
            }
            raw::VBoxEventType_VBoxEventType_OnNATNetworkPortForward => {
                VBoxEventType::OnNATNetworkPortForward
            }
            raw::VBoxEventType_VBoxEventType_OnGuestSessionStateChanged => {
                VBoxEventType::OnGuestSessionStateChanged
            }
            raw::VBoxEventType_VBoxEventType_OnGuestSessionRegistered => {
                VBoxEventType::OnGuestSessionRegistered
            }
            raw::VBoxEventType_VBoxEventType_OnGuestProcessRegistered => {
                VBoxEventType::OnGuestProcessRegistered
            }
            raw::VBoxEventType_VBoxEventType_OnGuestProcessStateChanged => {
                VBoxEventType::OnGuestProcessStateChanged
            }
            raw::VBoxEventType_VBoxEventType_OnGuestProcessInputNotify => {
                VBoxEventType::OnGuestProcessInputNotify
            }
            raw::VBoxEventType_VBoxEventType_OnGuestProcessOutput => {
                VBoxEventType::OnGuestProcessOutput
            }
            raw::VBoxEventType_VBoxEventType_OnGuestFileRegistered => {
                VBoxEventType::OnGuestFileRegistered
            }
            raw::VBoxEventType_VBoxEventType_OnGuestFileStateChanged => {
                VBoxEventType::OnGuestFileStateChanged
            }
            raw::VBoxEventType_VBoxEventType_OnGuestFileOffsetChanged => {
                VBoxEventType::OnGuestFileOffsetChanged
            }
            raw::VBoxEventType_VBoxEventType_OnGuestFileRead => VBoxEventType::OnGuestFileRead,
            raw::VBoxEventType_VBoxEventType_OnGuestFileWrite => VBoxEventType::OnGuestFileWrite,
            #[cfg(not(is_v_7_1))]
            raw::VBoxEventType_VBoxEventType_OnRecordingChanged => {
                VBoxEventType::OnRecordingChanged
            }
            raw::VBoxEventType_VBoxEventType_OnGuestUserStateChanged => {
                VBoxEventType::OnGuestUserStateChanged
            }
            raw::VBoxEventType_VBoxEventType_OnGuestMultiTouch => VBoxEventType::OnGuestMultiTouch,
            raw::VBoxEventType_VBoxEventType_OnHostNameResolutionConfigurationChange => {
                VBoxEventType::OnHostNameResolutionConfigurationChange
            }
            raw::VBoxEventType_VBoxEventType_OnSnapshotRestored => {
                VBoxEventType::OnSnapshotRestored
            }
            raw::VBoxEventType_VBoxEventType_OnMediumConfigChanged => {
                VBoxEventType::OnMediumConfigChanged
            }
            raw::VBoxEventType_VBoxEventType_OnAudioAdapterChanged => {
                VBoxEventType::OnAudioAdapterChanged
            }
            raw::VBoxEventType_VBoxEventType_OnProgressPercentageChanged => {
                VBoxEventType::OnProgressPercentageChanged
            }
            raw::VBoxEventType_VBoxEventType_OnProgressTaskCompleted => {
                VBoxEventType::OnProgressTaskCompleted
            }
            raw::VBoxEventType_VBoxEventType_OnCursorPositionChanged => {
                VBoxEventType::OnCursorPositionChanged
            }
            raw::VBoxEventType_VBoxEventType_OnGuestAdditionsStatusChanged => {
                VBoxEventType::OnGuestAdditionsStatusChanged
            }
            raw::VBoxEventType_VBoxEventType_OnGuestMonitorInfoChanged => {
                VBoxEventType::OnGuestMonitorInfoChanged
            }
            raw::VBoxEventType_VBoxEventType_OnGuestFileSizeChanged => {
                VBoxEventType::OnGuestFileSizeChanged
            }
            raw::VBoxEventType_VBoxEventType_OnClipboardFileTransferModeChanged => {
                VBoxEventType::OnClipboardFileTransferModeChanged
            }
            #[cfg(not(is_v_6_1))]
            raw::VBoxEventType_VBoxEventType_OnCloudProviderListChanged => {
                VBoxEventType::OnCloudProviderListChanged
            }
            #[cfg(not(is_v_6_1))]
            raw::VBoxEventType_VBoxEventType_OnCloudProviderRegistered => {
                VBoxEventType::OnCloudProviderRegistered
            }
            #[cfg(not(is_v_6_1))]
            raw::VBoxEventType_VBoxEventType_OnCloudProviderUninstall => {
                VBoxEventType::OnCloudProviderUninstall
            }
            #[cfg(not(is_v_6_1))]
            raw::VBoxEventType_VBoxEventType_OnCloudProfileRegistered => {
                VBoxEventType::OnCloudProfileRegistered
            }
            #[cfg(not(is_v_6_1))]
            raw::VBoxEventType_VBoxEventType_OnCloudProfileChanged => {
                VBoxEventType::OnCloudProfileChanged
            }
            #[cfg(not(is_v_6_1))]
            raw::VBoxEventType_VBoxEventType_OnProgressCreated => VBoxEventType::OnProgressCreated,
            #[cfg(not(is_v_6_1))]
            raw::VBoxEventType_VBoxEventType_OnLanguageChanged => VBoxEventType::OnLanguageChanged,
            #[cfg(not(is_v_6_1))]
            raw::VBoxEventType_VBoxEventType_OnUpdateAgentAvailable => {
                VBoxEventType::OnUpdateAgentAvailable
            }
            #[cfg(not(is_v_6_1))]
            raw::VBoxEventType_VBoxEventType_OnUpdateAgentError => {
                VBoxEventType::OnUpdateAgentError
            }
            #[cfg(not(is_v_6_1))]
            raw::VBoxEventType_VBoxEventType_OnUpdateAgentSettingsChanged => {
                VBoxEventType::OnUpdateAgentSettingsChanged
            }
            #[cfg(not(is_v_6_1))]
            raw::VBoxEventType_VBoxEventType_OnUpdateAgentStateChanged => {
                VBoxEventType::OnUpdateAgentStateChanged
            }
            #[cfg(not(is_v_6_1))]
            raw::VBoxEventType_VBoxEventType_OnHostAudioDeviceChanged => {
                VBoxEventType::OnHostAudioDeviceChanged
            }
            #[cfg(not(is_v_6_1))]
            raw::VBoxEventType_VBoxEventType_OnGuestDebugControlChanged => {
                VBoxEventType::OnGuestDebugControlChanged
            }
            #[cfg(not(is_v_6_1))]
            raw::VBoxEventType_VBoxEventType_OnMachineGroupsChanged => {
                VBoxEventType::OnMachineGroupsChanged
            }
            #[cfg(not(is_v_6_1))]
            raw::VBoxEventType_VBoxEventType_End => VBoxEventType::End,
            _ => VBoxEventType::Invalid,
        }
    }
}

impl Into<u32> for VBoxEventType {
    fn into(self) -> u32 {
        match self {
            VBoxEventType::Any => raw::VBoxEventType_VBoxEventType_Any,
            VBoxEventType::Vetoable => raw::VBoxEventType_VBoxEventType_Vetoable,
            VBoxEventType::MachineEvent => raw::VBoxEventType_VBoxEventType_MachineEvent,
            VBoxEventType::SnapshotEvent => raw::VBoxEventType_VBoxEventType_SnapshotEvent,
            VBoxEventType::InputEvent => raw::VBoxEventType_VBoxEventType_InputEvent,
            VBoxEventType::LastWildcard => raw::VBoxEventType_VBoxEventType_LastWildcard,
            VBoxEventType::OnMachineStateChanged => {
                raw::VBoxEventType_VBoxEventType_OnMachineStateChanged
            }
            VBoxEventType::OnMachineDataChanged => {
                raw::VBoxEventType_VBoxEventType_OnMachineDataChanged
            }
            VBoxEventType::OnExtraDataChanged => {
                raw::VBoxEventType_VBoxEventType_OnExtraDataChanged
            }
            VBoxEventType::OnExtraDataCanChange => {
                raw::VBoxEventType_VBoxEventType_OnExtraDataCanChange
            }
            VBoxEventType::OnMediumRegistered => {
                raw::VBoxEventType_VBoxEventType_OnMediumRegistered
            }
            VBoxEventType::OnMachineRegistered => {
                raw::VBoxEventType_VBoxEventType_OnMachineRegistered
            }
            VBoxEventType::OnSessionStateChanged => {
                raw::VBoxEventType_VBoxEventType_OnSessionStateChanged
            }
            VBoxEventType::OnSnapshotTaken => raw::VBoxEventType_VBoxEventType_OnSnapshotTaken,
            VBoxEventType::OnSnapshotDeleted => raw::VBoxEventType_VBoxEventType_OnSnapshotDeleted,
            VBoxEventType::OnSnapshotChanged => raw::VBoxEventType_VBoxEventType_OnSnapshotChanged,
            VBoxEventType::OnGuestPropertyChanged => {
                raw::VBoxEventType_VBoxEventType_OnGuestPropertyChanged
            }
            VBoxEventType::OnMousePointerShapeChanged => {
                raw::VBoxEventType_VBoxEventType_OnMousePointerShapeChanged
            }
            VBoxEventType::OnMouseCapabilityChanged => {
                raw::VBoxEventType_VBoxEventType_OnMouseCapabilityChanged
            }
            VBoxEventType::OnKeyboardLedsChanged => {
                raw::VBoxEventType_VBoxEventType_OnKeyboardLedsChanged
            }
            VBoxEventType::OnStateChanged => raw::VBoxEventType_VBoxEventType_OnStateChanged,
            VBoxEventType::OnAdditionsStateChanged => {
                raw::VBoxEventType_VBoxEventType_OnAdditionsStateChanged
            }
            VBoxEventType::OnNetworkAdapterChanged => {
                raw::VBoxEventType_VBoxEventType_OnNetworkAdapterChanged
            }
            VBoxEventType::OnSerialPortChanged => {
                raw::VBoxEventType_VBoxEventType_OnSerialPortChanged
            }
            VBoxEventType::OnParallelPortChanged => {
                raw::VBoxEventType_VBoxEventType_OnParallelPortChanged
            }
            VBoxEventType::OnStorageControllerChanged => {
                raw::VBoxEventType_VBoxEventType_OnStorageControllerChanged
            }
            VBoxEventType::OnMediumChanged => raw::VBoxEventType_VBoxEventType_OnMediumChanged,
            VBoxEventType::OnVRDEServerChanged => {
                raw::VBoxEventType_VBoxEventType_OnVRDEServerChanged
            }
            VBoxEventType::OnUSBControllerChanged => {
                raw::VBoxEventType_VBoxEventType_OnUSBControllerChanged
            }
            VBoxEventType::OnUSBDeviceStateChanged => {
                raw::VBoxEventType_VBoxEventType_OnUSBDeviceStateChanged
            }
            VBoxEventType::OnSharedFolderChanged => {
                raw::VBoxEventType_VBoxEventType_OnSharedFolderChanged
            }
            VBoxEventType::OnRuntimeError => raw::VBoxEventType_VBoxEventType_OnRuntimeError,
            VBoxEventType::OnCanShowWindow => raw::VBoxEventType_VBoxEventType_OnCanShowWindow,
            VBoxEventType::OnShowWindow => raw::VBoxEventType_VBoxEventType_OnShowWindow,
            VBoxEventType::OnCPUChanged => raw::VBoxEventType_VBoxEventType_OnCPUChanged,
            VBoxEventType::OnVRDEServerInfoChanged => {
                raw::VBoxEventType_VBoxEventType_OnVRDEServerInfoChanged
            }
            VBoxEventType::OnEventSourceChanged => {
                raw::VBoxEventType_VBoxEventType_OnEventSourceChanged
            }
            VBoxEventType::OnCPUExecutionCapChanged => {
                raw::VBoxEventType_VBoxEventType_OnCPUExecutionCapChanged
            }
            VBoxEventType::OnGuestKeyboard => raw::VBoxEventType_VBoxEventType_OnGuestKeyboard,
            VBoxEventType::OnGuestMouse => raw::VBoxEventType_VBoxEventType_OnGuestMouse,
            VBoxEventType::OnNATRedirect => raw::VBoxEventType_VBoxEventType_OnNATRedirect,
            VBoxEventType::OnHostPCIDevicePlug => {
                raw::VBoxEventType_VBoxEventType_OnHostPCIDevicePlug
            }
            VBoxEventType::OnVBoxSVCAvailabilityChanged => {
                raw::VBoxEventType_VBoxEventType_OnVBoxSVCAvailabilityChanged
            }
            VBoxEventType::OnBandwidthGroupChanged => {
                raw::VBoxEventType_VBoxEventType_OnBandwidthGroupChanged
            }
            VBoxEventType::OnGuestMonitorChanged => {
                raw::VBoxEventType_VBoxEventType_OnGuestMonitorChanged
            }
            VBoxEventType::OnStorageDeviceChanged => {
                raw::VBoxEventType_VBoxEventType_OnStorageDeviceChanged
            }
            VBoxEventType::OnClipboardModeChanged => {
                raw::VBoxEventType_VBoxEventType_OnClipboardModeChanged
            }
            VBoxEventType::OnDnDModeChanged => raw::VBoxEventType_VBoxEventType_OnDnDModeChanged,
            VBoxEventType::OnNATNetworkChanged => {
                raw::VBoxEventType_VBoxEventType_OnNATNetworkChanged
            }
            VBoxEventType::OnNATNetworkStartStop => {
                raw::VBoxEventType_VBoxEventType_OnNATNetworkStartStop
            }
            VBoxEventType::OnNATNetworkAlter => raw::VBoxEventType_VBoxEventType_OnNATNetworkAlter,
            VBoxEventType::OnNATNetworkCreationDeletion => {
                raw::VBoxEventType_VBoxEventType_OnNATNetworkCreationDeletion
            }
            VBoxEventType::OnNATNetworkSetting => {
                raw::VBoxEventType_VBoxEventType_OnNATNetworkSetting
            }
            VBoxEventType::OnNATNetworkPortForward => {
                raw::VBoxEventType_VBoxEventType_OnNATNetworkPortForward
            }
            VBoxEventType::OnGuestSessionStateChanged => {
                raw::VBoxEventType_VBoxEventType_OnGuestSessionStateChanged
            }
            VBoxEventType::OnGuestSessionRegistered => {
                raw::VBoxEventType_VBoxEventType_OnGuestSessionRegistered
            }
            VBoxEventType::OnGuestProcessRegistered => {
                raw::VBoxEventType_VBoxEventType_OnGuestProcessRegistered
            }
            VBoxEventType::OnGuestProcessStateChanged => {
                raw::VBoxEventType_VBoxEventType_OnGuestProcessStateChanged
            }
            VBoxEventType::OnGuestProcessInputNotify => {
                raw::VBoxEventType_VBoxEventType_OnGuestProcessInputNotify
            }
            VBoxEventType::OnGuestProcessOutput => {
                raw::VBoxEventType_VBoxEventType_OnGuestProcessOutput
            }
            VBoxEventType::OnGuestFileRegistered => {
                raw::VBoxEventType_VBoxEventType_OnGuestFileRegistered
            }
            VBoxEventType::OnGuestFileStateChanged => {
                raw::VBoxEventType_VBoxEventType_OnGuestFileStateChanged
            }
            VBoxEventType::OnGuestFileOffsetChanged => {
                raw::VBoxEventType_VBoxEventType_OnGuestFileOffsetChanged
            }
            VBoxEventType::OnGuestFileRead => raw::VBoxEventType_VBoxEventType_OnGuestFileRead,
            VBoxEventType::OnGuestFileWrite => raw::VBoxEventType_VBoxEventType_OnGuestFileWrite,
            #[cfg(not(is_v_7_1))]
            VBoxEventType::OnRecordingChanged => {
                raw::VBoxEventType_VBoxEventType_OnRecordingChanged
            }
            VBoxEventType::OnGuestUserStateChanged => {
                raw::VBoxEventType_VBoxEventType_OnGuestUserStateChanged
            }
            VBoxEventType::OnGuestMultiTouch => raw::VBoxEventType_VBoxEventType_OnGuestMultiTouch,
            VBoxEventType::OnHostNameResolutionConfigurationChange => {
                raw::VBoxEventType_VBoxEventType_OnHostNameResolutionConfigurationChange
            }
            VBoxEventType::OnSnapshotRestored => {
                raw::VBoxEventType_VBoxEventType_OnSnapshotRestored
            }
            VBoxEventType::OnMediumConfigChanged => {
                raw::VBoxEventType_VBoxEventType_OnMediumConfigChanged
            }
            VBoxEventType::OnAudioAdapterChanged => {
                raw::VBoxEventType_VBoxEventType_OnAudioAdapterChanged
            }
            VBoxEventType::OnProgressPercentageChanged => {
                raw::VBoxEventType_VBoxEventType_OnProgressPercentageChanged
            }
            VBoxEventType::OnProgressTaskCompleted => {
                raw::VBoxEventType_VBoxEventType_OnProgressTaskCompleted
            }
            VBoxEventType::OnCursorPositionChanged => {
                raw::VBoxEventType_VBoxEventType_OnCursorPositionChanged
            }
            VBoxEventType::OnGuestAdditionsStatusChanged => {
                raw::VBoxEventType_VBoxEventType_OnGuestAdditionsStatusChanged
            }
            VBoxEventType::OnGuestMonitorInfoChanged => {
                raw::VBoxEventType_VBoxEventType_OnGuestMonitorInfoChanged
            }
            VBoxEventType::OnGuestFileSizeChanged => {
                raw::VBoxEventType_VBoxEventType_OnGuestFileSizeChanged
            }
            VBoxEventType::OnClipboardFileTransferModeChanged => {
                raw::VBoxEventType_VBoxEventType_OnClipboardFileTransferModeChanged
            }
            #[cfg(not(is_v_6_1))]
            VBoxEventType::OnCloudProviderListChanged => {
                raw::VBoxEventType_VBoxEventType_OnCloudProviderListChanged
            }
            #[cfg(not(is_v_6_1))]
            VBoxEventType::OnCloudProviderRegistered => {
                raw::VBoxEventType_VBoxEventType_OnCloudProviderRegistered
            }
            #[cfg(not(is_v_6_1))]
            VBoxEventType::OnCloudProviderUninstall => {
                raw::VBoxEventType_VBoxEventType_OnCloudProviderUninstall
            }
            #[cfg(not(is_v_6_1))]
            VBoxEventType::OnCloudProfileRegistered => {
                raw::VBoxEventType_VBoxEventType_OnCloudProfileRegistered
            }
            #[cfg(not(is_v_6_1))]
            VBoxEventType::OnCloudProfileChanged => {
                raw::VBoxEventType_VBoxEventType_OnCloudProfileChanged
            }
            #[cfg(not(is_v_6_1))]
            VBoxEventType::OnProgressCreated => raw::VBoxEventType_VBoxEventType_OnProgressCreated,
            #[cfg(not(is_v_6_1))]
            VBoxEventType::OnLanguageChanged => raw::VBoxEventType_VBoxEventType_OnLanguageChanged,
            #[cfg(not(is_v_6_1))]
            VBoxEventType::OnUpdateAgentAvailable => {
                raw::VBoxEventType_VBoxEventType_OnUpdateAgentAvailable
            }
            #[cfg(not(is_v_6_1))]
            VBoxEventType::OnUpdateAgentError => {
                raw::VBoxEventType_VBoxEventType_OnUpdateAgentError
            }
            #[cfg(not(is_v_6_1))]
            VBoxEventType::OnUpdateAgentSettingsChanged => {
                raw::VBoxEventType_VBoxEventType_OnUpdateAgentSettingsChanged
            }
            #[cfg(not(is_v_6_1))]
            VBoxEventType::OnUpdateAgentStateChanged => {
                raw::VBoxEventType_VBoxEventType_OnUpdateAgentStateChanged
            }
            #[cfg(not(is_v_6_1))]
            VBoxEventType::OnHostAudioDeviceChanged => {
                raw::VBoxEventType_VBoxEventType_OnHostAudioDeviceChanged
            }
            #[cfg(not(is_v_6_1))]
            VBoxEventType::OnGuestDebugControlChanged => {
                raw::VBoxEventType_VBoxEventType_OnGuestDebugControlChanged
            }
            #[cfg(not(is_v_6_1))]
            VBoxEventType::OnMachineGroupsChanged => {
                raw::VBoxEventType_VBoxEventType_OnMachineGroupsChanged
            }
            #[cfg(not(is_v_6_1))]
            VBoxEventType::End => raw::VBoxEventType_VBoxEventType_End,
            _ => raw::VBoxEventType_VBoxEventType_Invalid,
        }
    }
}
impl Display for VBoxEventType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self))
    }
}
