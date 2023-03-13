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

impl From<SecurityOperation> for u8 {
    fn from(from: SecurityOperation) -> Self {
        match from {
            SecurityOperation::RequestSeed => 0x01,
            SecurityOperation::SendKey => 0x02,
        }
    }
}
