use crate::utils::{enum_wrapper, python_test};

enum_wrapper!(uds, UdsSessionType, UdsSessionTypeByte);
python_test!(uds, UdsSessionType, Default, Programming);

/// UDS Diagnostic session modes. Handled by SID 0x10
#[repr(u8)]
#[derive(strum::FromRepr, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "iter", derive(strum::EnumIter))]
#[cfg_attr(feature = "pyo3", pyo3::pyclass(eq, eq_int))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum UdsSessionType {
    /// Default diagnostic session mode (ECU is normally in this mode on startup)
    /// This session type does not require the diagnostic server to sent `TesterPresent` messages
    Default = 0x01,

    /// This diagnostic session mode enables all diagnostic services related to flashing or programming
    /// the ECU
    Programming = 0x02,

    /// This diagnostic session mode enabled all diagnostic services and allows adjusting
    /// ECU values
    Extended = 0x03,

    /// This diagnostic session enables all diagnostic services required to support safety system-related functions
    SafetySystem = 0x04,
}
