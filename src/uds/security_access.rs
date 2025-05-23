//! Provides methods for security seed/key access to the ECU in order to unlock functions which
//! are considered secure such as writing or reading to specific memory regions on the ECU
//!
//! Currently, only default seed/key (0x01/0x02) are supported

use crate::utils::{enum_wrapper, python_test};

enum_wrapper!(uds, SecurityOperation, SecurityOperationByte);
python_test!(uds, SecurityOperation, RequestSeed, SendKey);

/// Security operation request
#[repr(u8)]
#[derive(strum::FromRepr, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "iter", derive(strum::EnumIter))]
#[cfg_attr(feature = "pyo3", pyo3::pyclass(eq, eq_int))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SecurityOperation {
    /// Asks the ECU for a security seed
    RequestSeed = 0x01,
    /// Sends the computed key to the ECU
    SendKey = 0x02,
}
