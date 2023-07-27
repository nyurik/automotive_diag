//! Provides methods for security seed/key access to the ECU in order to unlock functions which
//! are considered secure such as writing or reading to specific memory regions on the ECU
//!
//! Currently, only default seed/key (0x01/0x02) are supported

use enum2repr::EnumRepr;

use crate::enum_wrapper;

enum_wrapper!(uds, SecurityOperation, SecurityOperationByte);

/// Security operation request
#[repr(u8)]
#[derive(EnumRepr, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum SecurityOperation {
    /// Asks the ECU for a security seed
    RequestSeed = 0x01,
    /// Sends the computed key to the ECU
    SendKey = 0x02,
}
