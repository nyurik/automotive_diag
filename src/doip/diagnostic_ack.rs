use crate::utils::{enum_wrapper, python_test};

enum_wrapper!(doip, DiagnosticAckCode, DiagnosticAckCodeByte, display = @"10001725068943791844");
python_test!(doip, DiagnosticAckCode, Acknowledged);

/// Available positive acknowledgement codes for `DiagnosticMessageAck`.
///
/// Positive acknowledgement codes from the result of a sent `DiagnosticMessage`.
#[repr(u8)]
#[derive(strum::FromRepr, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "display", derive(displaydoc::Display))]
#[cfg_attr(feature = "iter", derive(strum::EnumIter))]
#[cfg_attr(feature = "pyo3", pyo3::pyclass(eq, eq_int))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum DiagnosticAckCode {
    /// Acknowledged
    Acknowledged = 0x00,
}
