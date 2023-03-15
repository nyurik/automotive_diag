//! Provides methods for security seed/key access to the ECU in order to unlock functions which
//! are considered secure such as writing or reading to specific memory regions on the ECU
//!
//! Currently, only default seed/key (0x01/0x02) are supported

/// Security operation request
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum SecurityOperation {
    /// Asks the ECU for a security seed
    RequestSeed,
    /// Sends the computed key to the ECU
    SendKey,
}

impl TryFrom<u8> for SecurityOperation {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(SecurityOperation::RequestSeed),
            0x02 => Ok(SecurityOperation::SendKey),
            _ => Err(value),
        }
    }
}

impl From<SecurityOperation> for u8 {
    fn from(from: SecurityOperation) -> Self {
        match from {
            SecurityOperation::RequestSeed => 0x01,
            SecurityOperation::SendKey => 0x02,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_decode_enum() {
        for value in 0_u8..=0xFF {
            match SecurityOperation::try_from(value) {
                Ok(v) => {
                    let enc: u8 = v.into();
                    assert_eq!(value, enc, "0x{value:x} → {v:?} → 0x{enc:x}");
                }
                Err(v) => assert_eq!(value, v),
            }
        }
    }
}
