use crate::utils::{enum_wrapper, python_test};

enum_wrapper!(doip, ActivationCode, ActivationCodeByte, display = @"2282232336611129619");
python_test!(
    doip,
    ActivationCode,
    DeniedUnknownSourceAddress,
    DeniedTcpSocketsFull
);

/// Used in Routing Activation Response for results from a Routing Activation
/// Request.
///
/// Used to understand the result of a Routing Activation Request to understand
/// which logical route to take.
#[repr(u8)]
#[derive(strum::FromRepr, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "display", derive(displaydoc::Display))]
#[cfg_attr(feature = "iter", derive(strum::EnumIter))]
#[cfg_attr(feature = "pyo3", pyo3::pyclass(eq, eq_int))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ActivationCode {
    /// Denied Unknown Source Address
    DeniedUnknownSourceAddress = 0x00,

    /// Denied TCP Sockets Full
    DeniedTcpSocketsFull = 0x01,

    /// Denied TCP Socket Already Connected
    DeniedTcpSocketAlreadyConnected = 0x02,

    /// Denied Source Is Already Active
    DeniedSourceIsAlreadyActive = 0x03,

    /// Denied Missing Authentication
    DeniedMissingAuthentication = 0x04,

    /// Denied Rejected Confirmation
    DeniedRejectedConfirmation = 0x05,

    /// Denied Unsupported Routing `ActivationType`
    DeniedUnsupportedRoutingActivationType = 0x06,

    /// Denied Request Encrypted TLS Connection
    DeniedRequestEncryptedTlsConnection = 0x07,

    /// Successfully Activated
    SuccessfullyActivated = 0x10,

    /// Activated Confirmation Required
    ActivatedConfirmationRequired = 0x11,
}
