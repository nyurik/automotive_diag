/// ECU Communication types
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum CommunicationType {
    /// Application layer communication (inter-signal exchanges)
    /// between ECUs
    NormalCommunication,
    /// Network management related communication
    NetworkManagement,
    /// Both application layer communication and network management communication
    All,
}

/// ECU communication subnet type
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Subnet {
    /// All subnets
    All,
    /// Custom Subnet ID. Values range from 0x01-0x0E
    Custom(u8),
    /// Only received subnets
    RxOnly,
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
