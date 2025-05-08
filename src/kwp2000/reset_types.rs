//! This service requests the ECU to perform a reset

crate::utils::enum_wrapper!(kwp2000, ResetType, ResetTypeByte, display = @"157395241436022347");

/// ECU Reset types
///
/// Command support matrix
///
/// | `ResetType` | Support by ECUs |
/// |--|--|
/// |[`ResetType::PowerOnReset`]|Mandatory|
/// |[`ResetType::NonVolatileMemoryReset`]|Optional|
#[repr(u8)]
#[derive(strum::FromRepr, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "display", derive(displaydoc::Display))]
#[cfg_attr(feature = "iter", derive(strum::EnumIter))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ResetType {
    /// Simulates a power off/on reset of the ECU.
    PowerOnReset = 0x01,
    /// Just resets Non volatile memory of the ECU, resetting it
    NonVolatileMemoryReset = 0x82,
}
