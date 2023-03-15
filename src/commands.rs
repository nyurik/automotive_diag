/// UDS Command Service IDs
#[allow(missing_docs)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Command {
    /// Diagnostic session control.
    DiagnosticSessionControl,
    /// ECU Reset.
    ECUReset,
    /// Security access.
    SecurityAccess,
    /// Controls communication functionality of the ECU.
    CommunicationControl,
    /// Tester present command. Used internally by UDS Server
    TesterPresent,
    /// Accesses ECU timing parameters.
    AccessTimingParameters,
    SecuredDataTransmission,
    ControlDTCSettings,
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
    ReadDTCInformation,
    InputOutputControlByIdentifier,
    RoutineControl,
    RequestDownload,
    RequestUpload,
    TransferData,
    RequestTransferExit,
    Other(u8),
}

impl From<u8> for Command {
    fn from(value: u8) -> Self {
        match value {
            0x10 => Command::DiagnosticSessionControl,
            0x11 => Command::ECUReset,
            0x27 => Command::SecurityAccess,
            0x28 => Command::CommunicationControl,
            0x3E => Command::TesterPresent,
            0x83 => Command::AccessTimingParameters,
            0x84 => Command::SecuredDataTransmission,
            0x85 => Command::ControlDTCSettings,
            0x86 => Command::ResponseOnEvent,
            0x87 => Command::LinkControl,
            0x22 => Command::ReadDataByIdentifier,
            0x23 => Command::ReadMemoryByAddress,
            0x24 => Command::ReadScalingDataByIdentifier,
            0x2A => Command::ReadDataByPeriodicIdentifier,
            0x2C => Command::DynamicallyDefineDataIdentifier,
            0x2E => Command::WriteDataByIdentifier,
            0x3D => Command::WriteMemoryByAddress,
            0x14 => Command::ClearDiagnosticInformation,
            0x19 => Command::ReadDTCInformation,
            0x2F => Command::InputOutputControlByIdentifier,
            0x31 => Command::RoutineControl,
            0x34 => Command::RequestDownload,
            0x35 => Command::RequestUpload,
            0x36 => Command::TransferData,
            0x37 => Command::RequestTransferExit,
            _ => Command::Other(value),
        }
    }
}

impl From<Command> for u8 {
    fn from(value: Command) -> Self {
        match value {
            Command::DiagnosticSessionControl => 0x10,
            Command::ECUReset => 0x11,
            Command::SecurityAccess => 0x27,
            Command::CommunicationControl => 0x28,
            Command::TesterPresent => 0x3E,
            Command::AccessTimingParameters => 0x83,
            Command::SecuredDataTransmission => 0x84,
            Command::ControlDTCSettings => 0x85,
            Command::ResponseOnEvent => 0x86,
            Command::LinkControl => 0x87,
            Command::ReadDataByIdentifier => 0x22,
            Command::ReadMemoryByAddress => 0x23,
            Command::ReadScalingDataByIdentifier => 0x24,
            Command::ReadDataByPeriodicIdentifier => 0x2A,
            Command::DynamicallyDefineDataIdentifier => 0x2C,
            Command::WriteDataByIdentifier => 0x2E,
            Command::WriteMemoryByAddress => 0x3D,
            Command::ClearDiagnosticInformation => 0x14,
            Command::ReadDTCInformation => 0x19,
            Command::InputOutputControlByIdentifier => 0x2F,
            Command::RoutineControl => 0x31,
            Command::RequestDownload => 0x34,
            Command::RequestUpload => 0x35,
            Command::TransferData => 0x36,
            Command::RequestTransferExit => 0x37,
            Command::Other(v) => v,
        }
    }
}
