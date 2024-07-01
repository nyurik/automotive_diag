use strum::{EnumIter, FromRepr};

crate::enum_wrapper!(kwp2000, KwpCommand, KwpCommandByte);

/// KWP Command Service IDs.
///
/// Note. This does not cover both the 'Reserved' range (0x87-0xB9) and
/// 'System supplier specific' range (0xBA-0xBF)
#[repr(u8)]
#[derive(FromRepr, EnumIter, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum KwpCommand {
    /// Start or change ECU diagnostic session mode.
    StartDiagnosticSession = 0x10,
    /// Reset the ECU.
    ECUReset = 0x11,
    /// Clears diagnostic information stored on the ECU.
    ClearDiagnosticInformation = 0x14,
    /// Reads snapshot data of DTCs stored on the ECU.
    ReadStatusOfDiagnosticTroubleCodes = 0x17,
    /// Reads DTCs stored on the ECU.
    ReadDiagnosticTroubleCodesByStatus = 0x18,
    /// Reads ECU identification data.
    ReadECUIdentification = 0x1A,
    /// Reads data from the ECU using a local identifier.
    ReadDataByLocalIdentifier = 0x21,
    /// Reads data from the ECU using a unique identifier.
    ReadDataByIdentifier = 0x22,
    /// Reads memory from the ECU by address.
    ReadMemoryByAddress = 0x23,
    /// Security access functions.
    SecurityAccess = 0x27,
    /// Disables normal CAN message transmission from an ECU.
    DisableNormalMessageTransmission = 0x28,
    /// Enables normal CAN message transmission from an ECU.
    EnableNormalMessageTransmission = 0x29,
    DynamicallyDefineLocalIdentifier = 0x2C,
    WriteDataByIdentifier = 0x2E,
    InputOutputControlByLocalIdentifier = 0x30,
    /// Starts a ECU routine given a local identifier.
    StartRoutineByLocalIdentifier = 0x31,
    /// Stops a ECU routine given a local identifier.
    StopRoutineByLocalIdentifier = 0x32,
    /// requests results of an executed routine given a local identifier.
    RequestRoutineResultsByLocalIdentifier = 0x33,
    RequestDownload = 0x34,
    RequestUpload = 0x35,
    TransferData = 0x36,
    RequestTransferExit = 0x37,
    WriteDataByLocalIdentifier = 0x3B,
    WriteMemoryByAddress = 0x3D,
    /// Tester present message.
    TesterPresent = 0x3E,
    ControlDTCSettings = 0x85,
    ResponseOnEvent = 0x86,
}
