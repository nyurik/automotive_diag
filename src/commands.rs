use crate::byte_enum;
use crate::utils::ByteWrapper;
use bytenum::Bytenum;

/// UDS Command Service IDs
#[allow(missing_docs)]
#[repr(u8)]
#[derive(Bytenum, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum UdsCommand {
    /// Diagnostic session control.
    DiagnosticSessionControl = 0x10,
    /// ECU Reset.
    ResetEcu = 0x11,
    /// Security access.
    SecurityAccess = 0x27,
    /// Controls communication functionality of the ECU.
    CommunicationControl = 0x28,
    /// Tester present command. Used internally by UDS Server
    TesterPresent = 0x3E,
    /// Accesses ECU timing parameters.
    AccessTimingParameters = 0x83,
    SecuredDataTransmission = 0x84,
    ControlDtcSettings = 0x85,
    ResponseOnEvent = 0x86,
    LinkControl = 0x87,
    ReadDataByIdentifier = 0x22,
    ReadMemoryByAddress = 0x23,
    ReadScalingDataByIdentifier = 0x24,
    ReadDataByPeriodicIdentifier = 0x2A,
    DynamicallyDefineDataIdentifier = 0x2C,
    WriteDataByIdentifier = 0x2E,
    WriteMemoryByAddress = 0x3D,
    ClearDiagnosticInformation = 0x14,
    /// Reading and querying diagnostic trouble codes stored on the ECU.
    ReadDtcInformation = 0x19,
    InputOutputControlByIdentifier = 0x2F,
    RoutineControl = 0x31,
    RequestDownload = 0x34,
    RequestUpload = 0x35,
    TransferData = 0x36,
    RequestTransferExit = 0x37,
}

byte_enum!(UdsCommand, UdsCommandByte);
