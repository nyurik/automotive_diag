crate::enum_wrapper!(kwp2000, KwpError, KwpErrorByte);

/// KWP2000 Error definitions
#[repr(u8)]
#[derive(strum::FromRepr, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "iter", derive(strum::EnumIter))]
pub enum KwpError {
    /// ECU rejected the request for unknown reason
    GeneralReject = 0x10,
    /// ECU Does not support the requested service
    ServiceNotSupported = 0x11,
    /// ECU does not support arguments provided, or message format is incorrect
    SubFunctionNotSupportedInvalidFormat = 0x12,
    /// ECU is too busy to perform the request
    BusyRepeatRequest = 0x21,
    /// ECU prerequisite conditions are not met
    ConditionsNotCorrectRequestSequenceError = 0x22,
    /// **Deprecated in v2.2 of KWP2000**. Requested results of a routine that is not completed.
    RoutineNotComplete = 0x23,
    /// The request message contains data which is out of range
    RequestOutOfRange = 0x31,
    /// Security access is denied
    SecurityAccessDenied = 0x33,
    /// Invalid key provided to the ECU
    InvalidKey = 0x35,
    /// Exceeded the number of incorrect security access attempts
    ExceedNumberOfAttempts = 0x36,
    /// Time period for requesting a new seed not expired
    RequiredTimeDelayNotExpired = 0x37,
    /// ECU fault prevents data download
    DownloadNotAccepted = 0x40,
    /// ECU fault prevents data upload
    UploadNotAccepted = 0x50,
    /// ECU fault has stopped the transfer of data
    TransferSuspended = 0x71,
    /// The ECU has accepted the request, but cannot reply right now.
    RequestCorrectlyReceivedResponsePending = 0x78,
    /// Requested service is not supported in the current diagnostic session mode
    ServiceNotSupportedInActiveSession = 0x80,
    /// Data decompression failed
    DataDecompressionFailed = 0x9A,
    /// Data decryption failed
    DataDecryptionFailed = 0x9B,
    /// Sent by a gateway ECU. The requested ECU behind the gateway is not responding
    EcuNotResponding = 0xA0,
    /// Sent by a gateway ECU. The requested ECU address is unknown
    EcuAddressUnknown = 0xA1,
}
