use crate::utils::{enum_wrapper, python_test};

enum_wrapper!(uds, RoutineControlType, RoutineControlTypeByte);
python_test!(uds, RoutineControlType, StartRoutine, StopRoutine);

/// UDS Routine (0x31) service control types.
/// See chapter `14.2 RoutineControl service` in the ISO 14229 spec.
#[repr(u8)]
#[derive(strum::FromRepr, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "iter", derive(strum::EnumIter))]
#[cfg_attr(feature = "pyo3", pyo3::pyclass(eq, eq_int))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum RoutineControlType {
    /// Launches a routine on the ECU
    StartRoutine = 0x01,

    /// Stops the routine executing on the ECU
    StopRoutine = 0x02,

    /// Gets the result of the routing from the ECU
    RequestRoutineResult = 0x03,
}
