/// Scaling byte high nibble encoding
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
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

impl From<ScalingByteHigh> for u8 {
    fn from(value: ScalingByteHigh) -> Self {
        match value {
            ScalingByteHigh::UnsignedNumeric { num_bytes } => num_bytes,
            ScalingByteHigh::SignedNumeric { num_bytes } => 0x10 | num_bytes,
            ScalingByteHigh::BitMappingWithoutMask => 0x20,
            ScalingByteHigh::BitMappingWithMask => 0x30,
            ScalingByteHigh::BCD => 0x40,
            ScalingByteHigh::StateEncodedVariable => 0x50,
            ScalingByteHigh::ASCII { num_bytes } => 0x60 | num_bytes,
            ScalingByteHigh::SignedFloatingPoint => 0x70,
            ScalingByteHigh::Packet => 0x80,
            ScalingByteHigh::Formula => 0x90,
            ScalingByteHigh::UnitOrFormat => 0xA0,
            ScalingByteHigh::StateAndConnectionType => 0xB0,
            ScalingByteHigh::Reserved => 0xC0,
        }
    }
}

impl From<u8> for ScalingByteHigh {
    fn from(value: u8) -> Self {
        match value & 0xF0 {
            0x00 => Self::UnsignedNumeric {
                num_bytes: value & 0x0F,
            },
            0x01 => Self::SignedNumeric {
                num_bytes: value & 0x0F,
            },
            0x02 => Self::BitMappingWithoutMask,
            0x03 => Self::BitMappingWithMask,
            0x04 => Self::BCD,
            0x05 => Self::StateEncodedVariable,
            0x06 => Self::ASCII {
                num_bytes: value & 0x0F,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_decode_enum() {
        for value in 0_u8..=0xFF {
            let enum_val = ScalingByteHigh::from(value);
            if enum_val != ScalingByteHigh::Reserved {
                let decoded = u8::from(enum_val);
                assert_eq!(value, decoded, "0x{value:x} → {enum_val:?} → 0x{decoded:x}");
            }
        }
    }
}
