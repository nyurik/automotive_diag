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

impl TryFrom<u8> for CommunicationLevel {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(CommunicationLevel::EnableRxAndTx),
            0x01 => Ok(CommunicationLevel::EnableRxDisableTx),
            0x02 => Ok(CommunicationLevel::DisableRxEnableTx),
            0x03 => Ok(CommunicationLevel::DisableRxAndTx),
            _ => Err(value),
        }
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_decode_enum() {
        for value in 0_u8..=0xFF {
            match CommunicationLevel::try_from(value) {
                Ok(v) => {
                    let enc: u8 = v.into();
                    assert_eq!(value, enc, "0x{value:x} → {v:?} → 0x{enc:x}");
                }
                Err(v) => assert_eq!(value, v),
            }
        }
    }
}
