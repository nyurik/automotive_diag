/// ECU Communication types
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "display", derive(displaydoc::Display))]
pub enum CommunicationType {
    /// Application layer communication (inter-signal exchanges) between ECUs
    NormalCommunication,
    /// Network management related communication
    NetworkManagement,
    /// Both application layer communication and network management communication
    All,
}

/// ECU communication subnet type
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "display", derive(displaydoc::Display))]
pub enum Subnet {
    /// All subnets
    All,
    /// Custom Subnet ID. Values range from 0x01-0x0E
    Custom(u8),
    /// Only received subnets
    RxOnly,
}

/// Decode communication type and subnet from a single byte. If the low 4 bits are not a valid
/// communication type, the low 4 bits are returned as an error.
pub fn decode_communication_type(value: u8) -> Result<(CommunicationType, Subnet), u8> {
    let typ = match value & 0x0F {
        0x01 => CommunicationType::NormalCommunication,
        0x02 => CommunicationType::NetworkManagement,
        0x03 => CommunicationType::All,
        value => return Err(value),
    };

    let subnet = match value >> 4 {
        0x00 => Subnet::All,
        0x0F => Subnet::RxOnly,
        x => Subnet::Custom(x),
    };

    Ok((typ, subnet))
}

/// Encode communication type and subnet into a single byte
#[must_use]
pub fn encode_communication_type(typ: CommunicationType, subnet: Subnet) -> u8 {
    let typ = match typ {
        CommunicationType::NormalCommunication => 0x01,
        CommunicationType::NetworkManagement => 0x02,
        CommunicationType::All => 0x03,
    };

    let subnet = match subnet {
        Subnet::All => 0x00,
        Subnet::Custom(x) => x,
        Subnet::RxOnly => 0x0F,
    };

    typ | (subnet << 4)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_decode_enum() {
        for value in 0_u8..=0xFF {
            match decode_communication_type(value) {
                Ok((typ, sub)) => {
                    let enc = encode_communication_type(typ, sub);
                    assert_eq!(value, enc, "{value:#02X} → ({typ:?}, {sub:?}) → {enc:#02X}");
                }
                Err(err) => {
                    assert_eq!(value & 0x0F, err);
                }
            }
        }
    }
}
