/// Scaling byte high nibble encoding
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum ScalingByteHigh {
    /// Unsigned numeric integer
    UnsignedNumeric {
        /// Number of bytes making the integer, usually 1-4
        num_bytes: u8,
    },
    /// Signed numeric integer
    SignedNumeric {
        /// Number of bytes making the integer, usually 1-4
        num_bytes: u8,
    },
    /// Bit mapping encoding to set statuses, without mask
    BitMappingWithoutMask,
    /// Bit mapping encoding to set statuses, with mask
    BitMappingWithMask,
    /// Binary coded decimal encoding
    BCD,
    /// State encoded variable (Enum)
    StateEncodedVariable,
    /// ASCII Text
    ASCII {
        /// Number of bytes stored as ASCII Text
        num_bytes: u8,
    },
    /// ANSI 754 signed floating point
    SignedFloatingPoint,
    /// Multiple values data packet
    Packet,
    /// Conversion formula
    Formula,
    /// Unit of presentation format
    UnitOrFormat,
    /// Input / Output state encoding
    StateAndConnectionType,
    /// Reserved or Vehicle manufacturer specific (Unknown)
    Reserved,
}

impl From<u8> for ScalingByteHigh {
    fn from(x: u8) -> Self {
        match x & 0xF0 {
            0x00 => Self::UnsignedNumeric {
                num_bytes: x & 0x0F,
            },
            0x01 => Self::SignedNumeric {
                num_bytes: x & 0x0F,
            },
            0x02 => Self::BitMappingWithoutMask,
            0x03 => Self::BitMappingWithMask,
            0x04 => Self::BCD,
            0x05 => Self::StateEncodedVariable,
            0x06 => Self::ASCII {
                num_bytes: x & 0x0F,
            },
            0x07 => Self::SignedFloatingPoint,
            0x08 => Self::Packet,
            0x09 => Self::Formula,
            0x0A => Self::UnitOrFormat,
            0x0B => Self::StateAndConnectionType,
            _ => Self::Reserved,
        }
    }
}
