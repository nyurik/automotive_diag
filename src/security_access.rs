//! Provides methods for security seed/key access to the ECU in order to unlock functions which
//! are considered secure such as writing or reading to specific memory regions on the ECU
//!
//! Currently, only default seed/key (0x01/0x02) are supported

use crate::enum_wrapper;
use enum2repr::EnumRepr;

/// Security operation request
#[repr(u8)]
#[derive(EnumRepr, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[deprecated(
    note = "The `auto_uds` crate has been renamed to `automotive_diag`. Update your Cargo.toml, and use this enum from the `uds` namespace."
)]
pub enum SecurityOperation {
    /// Asks the ECU for a security seed
    RequestSeed = 0x01,
    /// Sends the computed key to the ECU
    SendKey = 0x02,
}

enum_wrapper!(SecurityOperation, SecurityOperationByte);
