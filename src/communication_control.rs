/// Communication level toggle
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum CommunicationLevel {
    /// Enable both Rx and Tx communication
    EnableRxAndTx,
    /// Enable Rx communication and disable Tx communication
    EnableRxDisableTx,
    /// Disable Rx communication and enable Tx communication
    DisableRxEnableTx,
    /// Disable both Rx and Tx communication
    DisableRxAndTx,
}

impl From<CommunicationLevel> for u8 {
    fn from(value: CommunicationLevel) -> Self {
        match value {
            CommunicationLevel::EnableRxAndTx => 0x00,
            CommunicationLevel::EnableRxDisableTx => 0x01,
            CommunicationLevel::DisableRxEnableTx => 0x02,
            CommunicationLevel::DisableRxAndTx => 0x03,
        }
    }
}

/// ECU Communication types
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Subnet {
    /// All subnets
    All,
    /// Custom Subnet ID. Values range from 0x01-0x0E
    Custom(u8),
    /// Only received subnets
    RxOnly,
}

/// Encode communication type and subnet into a single byte
pub fn encode_communication_type(typ: CommunicationType, subnet: Subnet) -> u8 {
    // Encode communication_Type
    (match typ {
        CommunicationType::NormalCommunication => 0x01,
        CommunicationType::NetworkManagement => 0x02,
        CommunicationType::All => 0x03,
    }) | (match subnet {
        Subnet::All => 0x00,
        Subnet::Custom(x) => x << 4,
        Subnet::RxOnly => 0x0F,
    } << 4)
}
