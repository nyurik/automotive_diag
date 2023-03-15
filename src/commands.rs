/// UDS Command Service IDs
#[allow(missing_docs)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum UdsCommand {
    /// Diagnostic session control.
    DiagnosticSessionControl,
    /// ECU Reset.
    ResetEcu,
    /// Security access.
    SecurityAccess,
    /// Controls communication functionality of the ECU.
    CommunicationControl,
    /// Tester present command. Used internally by UDS Server
    TesterPresent,
    /// Accesses ECU timing parameters.
    AccessTimingParameters,
    SecuredDataTransmission,
    ControlDtcSettings,
    ResponseOnEvent,
    LinkControl,
    ReadDataByIdentifier,
    ReadMemoryByAddress,
    ReadScalingDataByIdentifier,
    ReadDataByPeriodicIdentifier,
    DynamicallyDefineDataIdentifier,
    WriteDataByIdentifier,
    WriteMemoryByAddress,
    ClearDiagnosticInformation,
    /// Reading and querying diagnostic trouble codes stored on the ECU.
    ReadDtcInformation,
    InputOutputControlByIdentifier,
    RoutineControl,
    RequestDownload,
    RequestUpload,
    TransferData,
    RequestTransferExit,
    Other(u8),
}

impl From<u8> for UdsCommand {
    fn from(value: u8) -> Self {
        match value {
            0x10 => UdsCommand::DiagnosticSessionControl,
            0x11 => UdsCommand::ResetEcu,
            0x27 => UdsCommand::SecurityAccess,
            0x28 => UdsCommand::CommunicationControl,
            0x3E => UdsCommand::TesterPresent,
            0x83 => UdsCommand::AccessTimingParameters,
            0x84 => UdsCommand::SecuredDataTransmission,
            0x85 => UdsCommand::ControlDtcSettings,
            0x86 => UdsCommand::ResponseOnEvent,
            0x87 => UdsCommand::LinkControl,
            0x22 => UdsCommand::ReadDataByIdentifier,
            0x23 => UdsCommand::ReadMemoryByAddress,
            0x24 => UdsCommand::ReadScalingDataByIdentifier,
            0x2A => UdsCommand::ReadDataByPeriodicIdentifier,
            0x2C => UdsCommand::DynamicallyDefineDataIdentifier,
            0x2E => UdsCommand::WriteDataByIdentifier,
            0x3D => UdsCommand::WriteMemoryByAddress,
            0x14 => UdsCommand::ClearDiagnosticInformation,
            0x19 => UdsCommand::ReadDtcInformation,
            0x2F => UdsCommand::InputOutputControlByIdentifier,
            0x31 => UdsCommand::RoutineControl,
            0x34 => UdsCommand::RequestDownload,
            0x35 => UdsCommand::RequestUpload,
            0x36 => UdsCommand::TransferData,
            0x37 => UdsCommand::RequestTransferExit,
            _ => UdsCommand::Other(value),
        }
    }
}

impl From<UdsCommand> for u8 {
    fn from(value: UdsCommand) -> Self {
        match value {
            UdsCommand::DiagnosticSessionControl => 0x10,
            UdsCommand::ResetEcu => 0x11,
            UdsCommand::SecurityAccess => 0x27,
            UdsCommand::CommunicationControl => 0x28,
            UdsCommand::TesterPresent => 0x3E,
            UdsCommand::AccessTimingParameters => 0x83,
            UdsCommand::SecuredDataTransmission => 0x84,
            UdsCommand::ControlDtcSettings => 0x85,
            UdsCommand::ResponseOnEvent => 0x86,
            UdsCommand::LinkControl => 0x87,
            UdsCommand::ReadDataByIdentifier => 0x22,
            UdsCommand::ReadMemoryByAddress => 0x23,
            UdsCommand::ReadScalingDataByIdentifier => 0x24,
            UdsCommand::ReadDataByPeriodicIdentifier => 0x2A,
            UdsCommand::DynamicallyDefineDataIdentifier => 0x2C,
            UdsCommand::WriteDataByIdentifier => 0x2E,
            UdsCommand::WriteMemoryByAddress => 0x3D,
            UdsCommand::ClearDiagnosticInformation => 0x14,
            UdsCommand::ReadDtcInformation => 0x19,
            UdsCommand::InputOutputControlByIdentifier => 0x2F,
            UdsCommand::RoutineControl => 0x31,
            UdsCommand::RequestDownload => 0x34,
            UdsCommand::RequestUpload => 0x35,
            UdsCommand::TransferData => 0x36,
            UdsCommand::RequestTransferExit => 0x37,
            UdsCommand::Other(v) => v,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_decode_enum() {
        for value in 0_u8..=0xFF {
            let enum_val = UdsCommand::from(value);
            let decoded = u8::from(enum_val);
            assert_eq!(value, decoded, "0x{value:x} → {enum_val:?} → 0x{decoded:x}");
        }
    }
}
