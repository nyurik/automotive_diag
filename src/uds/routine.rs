use crate::enum_wrapper;
use enum2repr::EnumRepr;

enum_wrapper!(uds, RoutineControlType, RoutineControlTypeByte);

/// UDS Routine (0x31) service control types.
#[repr(u8)]
#[derive(EnumRepr, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum RoutineControlType {
    /// Launches a routine on the ECU
    StartRoutine = 0x01,

    /// Stops the routine executing on the ECU
    StopRoutine = 0x02,

    /// Gets the result of the routing from the ECU
    RequestRoutineResult = 0x03,
}
