use crate::utils::{enum_wrapper, python_test};

enum_wrapper!(doip, PowerMode, PowerModeByte, display = @"6865494478269464291");
python_test!(doip, PowerMode, NotReady, Ready);

/// Used in `PowerInformationResponse`, `PowerMode` provides the power mode that
/// the `DoIP` entity can be.
#[repr(u8)]
#[derive(strum::FromRepr, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "display", derive(displaydoc::Display))]
#[cfg_attr(feature = "iter", derive(strum::EnumIter))]
#[cfg_attr(feature = "pyo3", pyo3::pyclass(eq, eq_int))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PowerMode {
    /// Not Ready
    NotReady = 0x00,

    /// Ready
    Ready = 0x01,

    /// Not Supported
    NotSupported = 0x02,
}
