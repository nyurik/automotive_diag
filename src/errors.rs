/// UDS Error definitions
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum UdsError {
    /// ECU rejected the request (No specific error)
    GeneralReject,
    /// Service is not supported by the ECU
    ServiceNotSupported,
    /// Sub function is not supported by the ECU
    SubFunctionNotSupported,
    /// Request message was an invalid length, or the format of the
    /// request was incorrect
    IncorrectMessageLengthOrInvalidFormat,
    /// The response message is too long for the transport protocol
    ResponseTooLong,
    /// The ECU is too busy to perform this request. Therefore, the request
    /// Should be sent again if this error occurs
    BusyRepeatRequest,
    /// The requested action could not be preformed due to the prerequisite conditions
    /// not being correct
    ConditionsNotCorrect,
    /// The ECU cannot perform the request as the request has been sent in the incorrect order.
    /// For example, if [UdsDiagnosticServer::send_key] is used before [UdsDiagnosticServer::request_seed],
    /// then the ECU will respond with this error.
    RequestSequenceError,
    /// The ECU cannot perform the request as it has timed out trying to communicate with another
    /// component within the vehicle.
    NoResponseFromSubnetComponent,
    /// The ECU cannot perform the requested action as there is currently a DTC
    /// or failure of a component that is preventing the execution of the request.
    FailurePreventsExecutionOfRequestedAction,
    /// The request message contains data outside of a valid range
    RequestOutOfRange,
    /// The request could not be completed due to security access being denied.
    SecurityAccessDenied,
    /// The key sent from [UdsDiagnosticServer::send_key] was invalid
    InvalidKey,
    /// The client has tried to obtain security access to the ECU too many times with
    /// incorrect keys
    ExceedNumberOfAttempts,
    /// The client has tried to request seed_key's too quickly, before the ECU timeout's period
    /// has expired
    RequiredTimeDelayNotExpired,
    /// The ECU cannot accept the requested upload/download request due to a fault condition
    UploadDownloadNotAccepted,
    /// The ECU has halted data transfer due to a fault condition
    TransferDataSuspended,
    /// The ECU has encountered an error during reprogramming (erasing / flashing)
    GeneralProgrammingFailure,
    /// The ECU has detected the reprogramming error as the blockSequenceCounter is incorrect.
    WrongBlockSequenceCounter,
    /// The ECU has accepted the request, but cannot reply right now. If this error occurs,
    /// the [UdsDiagnosticServer] will automatically stop sending tester present messages and
    /// will wait for the ECUs response. If after 2000ms, the ECU did not respond, then this error
    /// will get returned back to the function call.
    RequestCorrectlyReceivedResponsePending,
    /// The sub function is not supported in the current diagnostic session mode
    SubFunctionNotSupportedInActiveSession,
    /// The service is not supported in the current diagnostic session mode
    ServiceNotSupportedInActiveSession,
    /// Engine RPM is too high
    RpmTooHigh,
    /// Engine RPM is too low
    RpmTooLow,
    /// Engine is running
    EngineIsRunning,
    /// Engine is not running
    EngineIsNotRunning,
    /// Engine has not been running for long enough
    EngineRunTimeTooLow,
    /// Engine temperature (coolant) is too high
    TemperatureTooHigh,
    /// Engine temperature (coolant) is too low
    TemperatureTooLow,
    /// Vehicle speed is too high
    VehicleSpeedTooHigh,
    /// Vehicle speed is too low
    VehicleSpeedTooLow,
    /// Throttle or pedal value is too high
    ThrottleTooHigh,
    /// Throttle or pedal value is too low
    ThrottleTooLow,
    /// Transmission is not in neutral
    TransmissionRangeNotInNeutral,
    /// Transmission is not in gear
    TransmissionRangeNotInGear,
    /// Brake is not applied
    BrakeSwitchNotClosed,
    /// Shifter lever is not in park
    ShifterLeverNotInPark,
    /// Automatic/CVT transmission torque convert is locked
    TorqueConverterClutchLocked,
    /// Voltage is too high
    VoltageTooHigh,
    /// Voltage is too low
    VoltageTooLow,
    /// (0x94-0xFE) This range is reserved for future definition.
    ReservedForSpecificConditionsNotCorrect(u8),
    /// (0x38-0x4F) This range of values is reserved for ISO-15765 data link security
    ReservedByExtendedDataLinkSecurityDocumentation(u8),
    /// Other reserved error code
    IsoSAEReserved(u8),
}

impl From<UdsError> for u8 {
    fn from(v: UdsError) -> Self {
        match v {
            UdsError::GeneralReject => 0x10,
            UdsError::ServiceNotSupported => 0x11,
            UdsError::SubFunctionNotSupported => 0x12,
            UdsError::IncorrectMessageLengthOrInvalidFormat => 0x13,
            UdsError::ResponseTooLong => 0x14,
            UdsError::BusyRepeatRequest => 0x21,
            UdsError::ConditionsNotCorrect => 0x22,
            UdsError::RequestSequenceError => 0x24,
            UdsError::NoResponseFromSubnetComponent => 0x25,
            UdsError::FailurePreventsExecutionOfRequestedAction => 0x26,
            UdsError::RequestOutOfRange => 0x31,
            UdsError::SecurityAccessDenied => 0x33,
            UdsError::InvalidKey => 0x35,
            UdsError::ExceedNumberOfAttempts => 0x36,
            UdsError::RequiredTimeDelayNotExpired => 0x37,
            UdsError::UploadDownloadNotAccepted => 0x70,
            UdsError::TransferDataSuspended => 0x71,
            UdsError::GeneralProgrammingFailure => 0x72,
            UdsError::WrongBlockSequenceCounter => 0x73,
            UdsError::RequestCorrectlyReceivedResponsePending => 0x78,
            UdsError::SubFunctionNotSupportedInActiveSession => 0x7E,
            UdsError::ServiceNotSupportedInActiveSession => 0x7F,
            UdsError::RpmTooHigh => 0x81,
            UdsError::RpmTooLow => 0x82,
            UdsError::EngineIsRunning => 0x83,
            UdsError::EngineIsNotRunning => 0x84,
            UdsError::EngineRunTimeTooLow => 0x85,
            UdsError::TemperatureTooHigh => 0x86,
            UdsError::TemperatureTooLow => 0x87,
            UdsError::VehicleSpeedTooHigh => 0x88,
            UdsError::VehicleSpeedTooLow => 0x89,
            UdsError::ThrottleTooHigh => 0x8A,
            UdsError::ThrottleTooLow => 0x8B,
            UdsError::TransmissionRangeNotInNeutral => 0x8C,
            UdsError::TransmissionRangeNotInGear => 0x8D,
            UdsError::BrakeSwitchNotClosed => 0x8F,
            UdsError::ShifterLeverNotInPark => 0x90,
            UdsError::TorqueConverterClutchLocked => 0x91,
            UdsError::VoltageTooHigh => 0x92,
            UdsError::VoltageTooLow => 0x93,
            UdsError::ReservedForSpecificConditionsNotCorrect(v)
            | UdsError::ReservedByExtendedDataLinkSecurityDocumentation(v)
            | UdsError::IsoSAEReserved(v) => v,
        }
    }
}

impl From<u8> for UdsError {
    fn from(v: u8) -> Self {
        match v {
            0x10 => Self::GeneralReject,
            0x11 => Self::ServiceNotSupported,
            0x12 => Self::SubFunctionNotSupported,
            0x13 => Self::IncorrectMessageLengthOrInvalidFormat,
            0x14 => Self::ResponseTooLong,
            0x21 => Self::BusyRepeatRequest,
            0x22 => Self::ConditionsNotCorrect,
            0x24 => Self::RequestSequenceError,
            0x25 => Self::NoResponseFromSubnetComponent,
            0x26 => Self::FailurePreventsExecutionOfRequestedAction,
            0x31 => Self::RequestOutOfRange,
            0x33 => Self::SecurityAccessDenied,
            0x35 => Self::InvalidKey,
            0x36 => Self::ExceedNumberOfAttempts,
            0x37 => Self::RequiredTimeDelayNotExpired,
            0x70 => Self::UploadDownloadNotAccepted,
            0x71 => Self::TransferDataSuspended,
            0x72 => Self::GeneralProgrammingFailure,
            0x73 => Self::WrongBlockSequenceCounter,
            0x78 => Self::RequestCorrectlyReceivedResponsePending,
            0x7E => Self::SubFunctionNotSupportedInActiveSession,
            0x7F => Self::ServiceNotSupportedInActiveSession,
            0x81 => Self::RpmTooHigh,
            0x82 => Self::RpmTooLow,
            0x83 => Self::EngineIsRunning,
            0x84 => Self::EngineIsNotRunning,
            0x85 => Self::EngineRunTimeTooLow,
            0x86 => Self::TemperatureTooHigh,
            0x87 => Self::TemperatureTooLow,
            0x88 => Self::VehicleSpeedTooHigh,
            0x89 => Self::VehicleSpeedTooLow,
            0x8A => Self::ThrottleTooHigh,
            0x8B => Self::ThrottleTooLow,
            0x8C => Self::TransmissionRangeNotInNeutral,
            0x8D => Self::TransmissionRangeNotInGear,
            0x8F => Self::BrakeSwitchNotClosed,
            0x90 => Self::ShifterLeverNotInPark,
            0x91 => Self::TorqueConverterClutchLocked,
            0x92 => Self::VoltageTooHigh,
            0x93 => Self::VoltageTooLow,
            0x94..=0xFE => Self::ReservedForSpecificConditionsNotCorrect(v),
            0x38..=0x4F => Self::ReservedByExtendedDataLinkSecurityDocumentation(v),
            v => Self::IsoSAEReserved(v),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_decode_enum() {
        for value in 0_u8..=0xFF {
            let enum_val = UdsError::from(value);
            let decoded = u8::from(enum_val);
            assert_eq!(value, decoded, "0x{value:x} → {enum_val:?} → 0x{decoded:x}");
        }
    }
}
