crate::utils::enum_wrapper!(obd2, Obd2Error, Obd2ErrorByte);

/// OBD2 Error definitions
#[repr(u8)]
#[derive(strum::FromRepr, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "iter", derive(strum::EnumIter))]
#[cfg_attr(feature = "display", derive(displaydoc::Display))]
pub enum Obd2Error {
    /// ECU general reject
    GeneralReject = 0x10,
    /// Request message format was incorrect
    FormatIncorrect = 0x13,
    /// Requested data was out of range
    OutOfRange = 0x31,
    /// ECU is busy, repeat the request
    BusyRepeatRequest = 0x21,
    /// ECU is busy, but will respond to the original request shortly
    BusyResponsePending = 0x78,
    /// Conditions are not correct to execute the request
    ConditionsNotCorrect = 0x22,
    /// Out of order request in a sequence of request
    RequestSequenceError = 0x24,
    /// Security access is denied
    SecurityAccessDenied = 0x33,
    /// Invalid security key
    InvalidKey = 0x35,
    /// Exceeded the maximum number of attempts at authentication
    ExceedAttempts = 0x36,
}

/// FIXME: it seems there are more than one value for `ServiceNotSupportedInActiveSession` condition,
/// so for now we just have a test here. In the future, it should be clarified as individual constants above.
impl Obd2ErrorByte {
    /// Returns true if the error is one of the well known "not supported" errors
    #[must_use]
    pub fn is_not_supported(&self) -> bool {
        matches!(self, Obd2ErrorByte::Extended(v) if *v == 0x11 || *v == 0x12 || *v == 0x7E || *v == 0x7F)
    }
}
