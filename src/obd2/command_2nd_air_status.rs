use crate::utils::{enum_wrapper, python_test};

enum_wrapper!(
    obd2,
    CommandedSecondaryAirStatus,
    CommandedSecondaryAirStatusByte,
    display = @"4674215784794864501"
);
python_test!(obd2, CommandedSecondaryAirStatus, Upstream, DownstreamOfCat);

/// Commanded secondary air status for PID 12
#[repr(u8)]
#[derive(strum::FromRepr, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "display", derive(displaydoc::Display))]
#[cfg_attr(feature = "iter", derive(strum::EnumIter))]
#[cfg_attr(feature = "pyo3", pyo3::pyclass(eq, eq_int))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum CommandedSecondaryAirStatus {
    /// Upstream
    Upstream = 0x01,
    /// Downstream of catalytic converter
    DownstreamOfCat = 0x02,
    /// From the outside atmosphere or off
    FromOutsideOrOff = 0x04,
    /// Pump commanded on for diagnostics
    PumpCommandedForDiagnostics = 0x08,
}
