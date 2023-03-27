use crate::enum_wrapper;
use crate::utils::ByteWrapper;
use enum2repr::EnumRepr;

/// UDS Command Service IDs
#[repr(u8)]
#[derive(EnumRepr, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum UdsCommand {
    /// The client requests to control a diagnostic session with a server(s).
    DiagnosticSessionControl = 0x10,
    /// The client forces the server(s) to perform a reset.
    ECUReset = 0x11,
    /// The client requests to unlock a secured server(s).
    SecurityAccess = 0x27,
    /// The client controls the setting of communication parameters in the server (e.g. communication baud-rate).
    CommunicationControl = 0x28,
    /// The client indicates to the server(s) that it is still present.
    TesterPresent = 0x3E,
    /// The client requests to access the server(s) with a specific access mode.
    Authentication = 0x29,
    /// The client uses this service to perform data transmission with an extended data link security.
    SecuredDataTransmission = 0x84,
    /// The client controls the setting of DTCs in the server.
    ControlDTCSetting = 0x85,
    /// The client requests to set up and/or control an event mechanism in the server.
    ResponseOnEvent = 0x86,
    /// The client requests control of the communication baud-rate.
    LinkControl = 0x87,
    /// The client requests to read the current value of a record identified by a provided dataIdentifier.
    ReadDataByIdentifier = 0x22,
    /// The client requests to read the current value of the provided memory range.
    ReadMemoryByAddress = 0x23,
    /// The client requests to read the scaling information of a record identified by a provided dataIdentifier.
    ReadScalingDataByIdentifier = 0x24,
    /// The client requests to schedule data in the server for periodic transmission.
    ReadDataByPeriodicIdentifier = 0x2A,
    /// The client requests to dynamically define data Identifiers that may subsequently be read by the [`Self::ReadDataByIdentifier`] service.
    DynamicallyDefineDataIdentifier = 0x2C,
    /// The client requests to write a record specified by a provided dataIdentifier.
    WriteDataByIdentifier = 0x2E,
    /// The client requests to overwrite a provided memory range.
    WriteMemoryByAddress = 0x3D,
    /// Allows the client to clear diagnostic information from the server (including DTCs, captured data, etc.).
    ClearDiagnosticInformation = 0x14,
    /// Allows the client to request diagnostic information from the server (including DTCs, captured data, etc.).
    ReadDTCInformation = 0x19,
    ///The client requests the control of an input/output specific to the server.
    InputOutputControlByIdentifier = 0x2F,
    /// The client requests to start, stop a routine in the server(s) or requests the routine results.
    RoutineControl = 0x31,
    ///The client requests the negotiation of a data transfer from the client to the server.
    RequestDownload = 0x34,
    /// The client requests the negotiation of a data transfer from the server to the client.
    RequestUpload = 0x35,
    /// The client transmits data to the server (download) or requests data from the server (upload).
    TransferData = 0x36,
    /// The client requests the termination of a data transfer.
    RequestTransferExit = 0x37,
    /// The client requests the negotiation of a file transfer between server and client.
    RequestFileTransfer = 0x38,
}

enum_wrapper!(UdsCommand, UdsCommandByte);
