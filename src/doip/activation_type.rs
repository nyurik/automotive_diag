use crate::utils::{enum_wrapper, python_test};

enum_wrapper!(doip, ActivationType, ActivationTypeByte, display = @"14425352116563829080");
python_test!(doip, ActivationType, Default, WwhObd);

/// Represents routing types for activation requests.
///
/// Used to customize the routing type requested from the `DoIP` entity for different
/// scenarios.
#[repr(u8)]
#[derive(strum::FromRepr, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "display", derive(displaydoc::Display))]
#[cfg_attr(feature = "iter", derive(strum::EnumIter))]
#[cfg_attr(feature = "pyo3", pyo3::pyclass(eq, eq_int))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ActivationType {
    /// Default
    Default = 0x00,

    /// WWH-OBD
    WwhObd = 0x01,

    /// Central Security
    CentralSecurity = 0x02,
}
