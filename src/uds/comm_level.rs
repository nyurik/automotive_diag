use crate::utils::{enum_wrapper, python_test};

enum_wrapper!(uds, CommunicationLevel, CommunicationLevelByte);
python_test!(uds, CommunicationLevel, EnableRxAndTx, EnableRxDisableTx);

/// Communication level toggle
#[repr(u8)]
#[derive(strum::FromRepr, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "iter", derive(strum::EnumIter))]
#[cfg_attr(feature = "pyo3", pyo3::pyclass(eq, eq_int))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum CommunicationLevel {
    /// This value indicates that the reception and transmission of messages
    /// shall be enabled for the specified communicationType.
    EnableRxAndTx = 0x00,
    /// This value indicates that the reception of messages shall be enabled and
    /// the transmission shall be disabled for the specified communicationType.
    EnableRxDisableTx = 0x01,
    /// This value indicates that the reception of messages shall be disabled and
    /// the transmission shall be enabled for the specified communicationType.
    DisableRxEnableTx = 0x02,
    /// This value indicates that the reception and transmission of messages
    /// shall be disabled for the specified communicationType.
    DisableRxAndTx = 0x03,
    /// This value indicates that the addressed bus master shall switch
    /// the related sub-bus segment to the diagnostic-only scheduling mode.
    EnableRxAndDisableTxWithEnhancedAddressInformation = 0x04,
    /// This value indicates that the addressed bus master shall switch
    /// the related sub-bus segment to the application scheduling mode.
    EnableRxAndTxWithEnhancedAddressInformation = 0x05,
}
