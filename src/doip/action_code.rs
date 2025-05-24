use crate::utils::{enum_wrapper, python_test};

enum_wrapper!(doip, ActionCode, ActionCodeByte, display = @"11975332154751864497");
python_test!(
    doip,
    ActionCode,
    NoFurtherActionRequired,
    RoutingActivationRequired
);

/// Used in Vehicle Announcement Messages to give next steps.
///
/// Used to inform the client of further actions which need to be taken on a
/// `DoIP` server.
#[repr(u8)]
#[derive(strum::FromRepr, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "display", derive(displaydoc::Display))]
#[cfg_attr(feature = "iter", derive(strum::EnumIter))]
#[cfg_attr(feature = "pyo3", pyo3::pyclass(eq, eq_int))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ActionCode {
    /// No Further Action Required
    NoFurtherActionRequired = 0x00,

    /// Routing Activation Required
    RoutingActivationRequired = 0x10,
}
