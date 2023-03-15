/// Communication level toggle
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum CommunicationLevel {
    /// Enable both Rx and Tx communication
    EnableRxAndTx,
    /// Enable Rx communication and disable Tx communication
    EnableRxDisableTx,
    /// Disable Rx communication and enable Tx communication
    DisableRxEnableTx,
    /// Disable both Rx and Tx communication
    DisableRxAndTx,
}

impl From<CommunicationLevel> for u8 {
    fn from(value: CommunicationLevel) -> Self {
        match value {
            CommunicationLevel::EnableRxAndTx => 0x00,
            CommunicationLevel::EnableRxDisableTx => 0x01,
            CommunicationLevel::DisableRxEnableTx => 0x02,
            CommunicationLevel::DisableRxAndTx => 0x03,
        }
    }
}
