//! This service requests the ECU to perform a reset

use enum2repr::EnumRepr;

use crate::enum_wrapper;

enum_wrapper!(kwp2000, ResetType, ResetTypeByte);

/// ECU Reset types
///
/// Command support matrix
///
/// | `ResetType` | Support by ECUs |
/// |--|--|
/// |[`ResetType::PowerOnReset`]|Mandatory|
/// |[`ResetType::NonVolatileMemoryReset`]|Optional|
#[repr(u8)]
#[derive(EnumRepr, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "display", derive(displaydoc::Display))]
pub enum ResetType {
    /// Simulates a power off/on reset of the ECU.
    PowerOnReset = 0x01,
    /// Just resets Non volatile memory of the ECU, resetting it
    NonVolatileMemoryReset = 0x82,
}
