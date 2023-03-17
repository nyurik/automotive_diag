use crate::byte_enum;
use crate::utils::ByteWrapper;
use bytenum::Bytenum;

/// Communication level toggle
#[allow(clippy::enum_variant_names)]
#[repr(u8)]
#[derive(Bytenum, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum CommunicationLevel {
    /// Enable both Rx and Tx communication
    EnableRxAndTx = 0x00,
    /// Enable Rx communication and disable Tx communication
    EnableRxDisableTx = 0x01,
    /// Disable Rx communication and enable Tx communication
    DisableRxEnableTx = 0x02,
    /// Disable both Rx and Tx communication
    DisableRxAndTx = 0x03,
}

byte_enum!(CommunicationLevel, CommunicationLevelByte);
