crate::enum_wrapper!(kwp2000, KwpSessionType, KwpSessionTypeByte);

/// KWP2000 diagnostic session type
///
/// Session support matrix
///
/// | `KwpSessionType` | Support by ECUs |
/// |--|--|
/// |[`KwpSessionType::Normal`] | Mandatory |
/// |[`KwpSessionType::Reprogramming`] | Optional (Only ECUs which implement the ECU-Flash reprogramming specification) |
/// |[`KwpSessionType::Standby`] | Optional |
/// |[`KwpSessionType::Passive`] | Optional (Only intended for ECU development) |
/// |[`KwpSessionType::ExtendedDiagnostics`] | Mandatory |
#[repr(u8)]
#[derive(strum::FromRepr, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "iter", derive(strum::EnumIter))]
pub enum KwpSessionType {
    /// Normal session. The ECU will typically boot in this state.
    /// In this mode, only non-intrusive functions are supported.
    Normal = 0x81,
    /// Reprogramming session. Used for flashing an ECU. Only functions
    /// for reading/writing to memory are allowed in this mode
    Reprogramming = 0x85,
    /// In stand-by mode, the ECU will be in a low-power state,
    /// acting as a slave to other ECUs and only able to perform actuation tests
    /// at the request of a tester. If a request is made to the ECU which can disrupt
    /// its low power state, the ECU will reject the request.
    Standby = 0x89,
    /// In this mode, the ECU will remain active, but will disable
    /// all normal CAN/LIN communication with the vehicle, effectively putting
    /// the ECU to sleep. IMPORTANT. If the ECU is power cycled, it will
    /// reboot in this mode.
    Passive = 0x90,
    /// Extended diagnostics mode. Every service is available here
    ExtendedDiagnostics = 0x92,
}
