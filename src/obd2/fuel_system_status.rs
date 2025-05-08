crate::utils::enum_wrapper!(obd2, FuelSystemStatus, FuelSystemStatusByte, display = @"14417397174224904691");

/// Fuel system status enumeration for PID 03
#[repr(u8)]
#[derive(strum::FromRepr, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "display", derive(displaydoc::Display))]
#[cfg_attr(feature = "iter", derive(strum::EnumIter))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FuelSystemStatus {
    /// The motor is off
    Off = 0x00,
    /// Open loop due to insufficient engine temperature
    OpenLoopLowTemp = 0x01, // FIXME: 0x01 was double-mapped to OpenLoopEngineLoad
    /// Closed loop, using oxygen sensor feedback to determine fuel mix
    ClosedLoopO2Feedback = 0x02,
    /// Open loop due to engine load / fuel cut due to deceleration
    OpenLoopEngineLoad = 0x04,
    /// Open loop due to system failure
    OpenLoopSystemFailure = 0x08,
    /// Closed loop, using at least one oxygen sensor but there is a fault in the feedback system
    ClosedLoopWithFault = 0x10,
}
