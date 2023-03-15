/// UDS Diagnostic session modes. Handled by SID 0x10
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum UdsSessionType {
    /// Default diagnostic session mode (ECU is normally in this mode on startup)
    /// This session type does not require the diagnostic server to sent TesterPresent messages
    Default,

    /// This diagnostic session mode enables all diagnostic services related to flashing or programming
    /// the ECU
    Programming,

    /// This diagnostic session mode enabled all diagnostic services and allows adjusting
    /// ECU values
    Extended,

    /// This diagnostic session enables all diagnostic services required to support safety system-related functions
    SafetySystem,

    /// Custom session type. This covers both vehicleManufacturerSpecific modes (0x40-0x5F) and systemSupplierSpecific modes (0x60-0x7E).
    Other(u8),
}

impl From<u8> for UdsSessionType {
    fn from(value: u8) -> Self {
        match value {
            0x01 => UdsSessionType::Default,
            0x02 => UdsSessionType::Programming,
            0x03 => UdsSessionType::Extended,
            0x04 => UdsSessionType::SafetySystem,
            v => UdsSessionType::Other(v),
        }
    }
}

impl From<UdsSessionType> for u8 {
    fn from(value: UdsSessionType) -> u8 {
        match value {
            UdsSessionType::Default => 0x01,
            UdsSessionType::Programming => 0x02,
            UdsSessionType::Extended => 0x03,
            UdsSessionType::SafetySystem => 0x04,
            UdsSessionType::Other(v) => v,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_decode_enum() {
        for value in 0_u8..=0xFF {
            let enum_val = UdsSessionType::from(value);
            let decoded = u8::from(enum_val);
            assert_eq!(value, decoded, "0x{value:x} → {enum_val:?} → 0x{decoded:x}");
        }
    }
}
