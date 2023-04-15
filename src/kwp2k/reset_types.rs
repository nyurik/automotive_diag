//! This service requests the ECU to perform a reset

use crate::enum_wrapper;
use enum2repr::EnumRepr;

/// ECU Reset types
///
/// Command support matrix
///
/// | ResetMode | Support by ECUs |
/// |--|--|
/// |[ResetMode::PowerOnReset]|Mandatory|
/// |[ResetMode::NonVolatileMemoryReset]|Optional|
#[repr(u8)]
#[derive(EnumRepr, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "display", derive(displaydoc::Display))]
pub enum ResetType {
    /// Simulates a power off/on reset of the ECU.
    PowerOnReset = 0x01,
    /// Just resets Non volatile memory of the ECU, resetting it
    NonVolatileMemoryReset = 0x82,
}

enum_wrapper!(kwp2k, ResetType, ResetTypeByte);
