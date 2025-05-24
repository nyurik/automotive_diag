use crate::utils::{enum_wrapper, python_test};

enum_wrapper!(doip, DiagnosticNackCode, DiagnosticNackCodeByte, display = @"10387153187284031830");
python_test!(
    doip,
    DiagnosticNackCode,
    InvalidSourceAddress,
    UnknownTargetAddress
);

/// Available negative acknowledgement codes for `DiagnosticMessageAck`.
///
/// Negative acknowledgement codes from the result of a sent `DiagnosticMessage`.
#[repr(u8)]
#[derive(strum::FromRepr, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "display", derive(displaydoc::Display))]
#[cfg_attr(feature = "iter", derive(strum::EnumIter))]
#[cfg_attr(feature = "pyo3", pyo3::pyclass(eq, eq_int))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum DiagnosticNackCode {
    /// Invalid Source Address
    InvalidSourceAddress = 0x02,

    /// Unknown Target Address
    UnknownTargetAddress = 0x03,

    /// Diagnostic Message Too Large
    DiagnosticMessageTooLarge = 0x04,

    /// Out Of Memory
    OutOfMemory = 0x05,

    /// Target Unreachable
    TargetUnreachable = 0x06,

    /// Unknown Network
    UnknownNetwork = 0x07,

    /// Transport Protocol Error
    TransportProtocolError = 0x08,
}
