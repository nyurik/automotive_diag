use crate::{byte_enum, ByteWrapper};
use bytenum::Bytenum;

/// Scaling high nibble, representing the type of data without its size. The size is given by the low nibble.
#[repr(u8)]
#[derive(Bytenum, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ScalingType {
    /// Unsigned numeric integer. Must be followed by 1..4 bytes, given as a low nibble of the byte.
    UnsignedNumeric = 0x00,
    /// Signed numeric integer. Must be followed by 1..4 bytes, given as a low nibble of the byte.
    SignedNumeric = 0x10,
    /// Bit mapping encoding to set statuses, without mask
    BitMappingWithoutMask = 0x20,
    /// Bit mapping encoding to set statuses, with mask
    BitMappingWithMask = 0x30,
    /// Binary coded decimal encoding
    Bcd = 0x40,
    /// State encoded variable (Enum)
    StateEncodedVariable = 0x50,
    /// ASCII Text. Must be followed by 1..15 bytes, given as a low nibble of the byte.
    Ascii = 0x60,
    /// ANSI 754 signed floating point
    SignedFloatingPoint = 0x70,
    /// Multiple values data packet
    Packet = 0x80,
    /// Conversion formula
    Formula = 0x90,
    /// Unit of presentation format
    UnitOrFormat = 0xA0,
    /// Input / Output state encoding
    StateAndConnectionType = 0xB0,
}

/// Scaling value with both the [`ScalingType`] and the size of the data.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Scaling {
    typ: ScalingType,
    size: u8,
}

byte_enum!(
    Scaling,
    ScalingByte,
    impl From<Scaling> for u8 {
        fn from(value: Scaling) -> Self {
            value.typ as u8 | value.size
        }
    }
);

impl Scaling {
    pub fn new(typ: ScalingType, size: u8) -> Result<Self, &'static str> {
        if size > 0x0F {
            return Err("Invalid size, expecting between 0 and 15.");
        }
        (typ as u8 | size).try_into()
    }
}

impl TryFrom<u8> for Scaling {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        let typ: ScalingType = (value & 0xF0).try_into()?;
        let size = value & 0x0F;
        match typ {
            ScalingType::UnsignedNumeric | ScalingType::SignedNumeric => {
                if !(1..=4).contains(&size) {
                    return Err("Invalid number of data bytes for a numeric type, expecting between 1 and 4.");
                }
            }
            ScalingType::Ascii => {
                if !(1..=15).contains(&size) {
                    return Err(
                        "Invalid number of data bytes for ASCII type, expecting between 1 and 15.",
                    );
                }
            }
            _ => {
                if size != 0 {
                    return Err("No data bytes are expected for this type.");
                }
            }
        };
        Ok(Self { typ, size })
    }
}

#[cfg(test)]
mod tests2 {
    use super::*;

    #[test]
    fn test_scaling_byte() {
        use ScalingType::*;
        assert_eq!(u8::from(Scaling::new(SignedNumeric, 4).unwrap()), 0x14);
        assert_eq!(u8::from(Scaling::new(SignedNumeric, 1).unwrap()), 0x11);
        assert!(Scaling::new(SignedNumeric, 0).is_err());
        assert!(Scaling::new(Bcd, 1).is_err());
    }
}
