use crate::enum_wrapper;
use enum2repr::EnumRepr;

/// Commanded secondary air status for PID 12
#[repr(u8)]
#[derive(EnumRepr, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "display", derive(displaydoc::Display))]
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

enum_wrapper!(
    obd2,
    CommandedSecondaryAirStatus,
    CommandedSecondaryAirStatusByte
);
