crate::utils::enum_wrapper!(uds, UdsNrc, UdsNrcByte);

/// UDS Negative Response Code
#[repr(u8)]
#[derive(strum::FromRepr, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "iter", derive(strum::EnumIter))]
#[cfg_attr(feature = "display", derive(displaydoc::Display))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum UdsNrc {
    /// General rejection, typically for unspecified errors.
    GeneralReject = 0x10,

    /// The requested service is not supported by the ECU.
    ServiceNotSupported = 0x11,

    /// The requested sub-function is not supported.
    SubFunctionNotSupported = 0x12,

    /// The message length or format is invalid.
    InvalidMessageLengthOrFormat = 0x13,

    /// The response message is too long for the ECU to handle.
    ResponseTooLong = 0x14,

    /// The ECU is busy, and the client should repeat the request later.
    BusyRepeatRequest = 0x21,

    /// Conditions are not correct to perform the requested operation.
    ConditionsNotCorrect = 0x22,

    /// The request sequence is incorrect (e.g., missing a required step).
    RequestSequenceError = 0x24,

    /// No response received from a required subnet component.
    NoResponseFromSubnetComponent = 0x25,

    /// A failure prevents the ECU from executing the requested action.
    FailurePreventsExecution = 0x26,

    /// The requested parameter is out of the acceptable range.
    RequestOutOfRange = 0x31,

    /// Security access is denied due to authentication failure.
    SecurityAccessDenied = 0x33,

    /// The provided key for security access is invalid.
    InvalidKey = 0x35,

    /// The number of allowed security access attempts has been exceeded.
    ExceededNumberOfAttempts = 0x36,

    /// The required time delay before retrying has not expired.
    RequiredTimeDelayNotExpired = 0x37,

    /// Upload/download operations are not accepted by the ECU.
    UploadDownloadNotAccepted = 0x70,

    /// Data transfer has been temporarily suspended.
    TransferDataSuspended = 0x71,

    /// A failure occurred during programming.
    ProgrammingFailure = 0x72,

    /// The block sequence counter for data transfer is incorrect.
    WrongBlockSequenceCounter = 0x73,

    /// The request has been received, but the response is pending.
    RequestReceivedResponsePending = 0x78,

    /// The requested sub-function is not supported in the active session.
    SubFunctionNotSupportedInActiveSession = 0x7E,

    /// The requested service is not supported in the active session.
    ServiceNotSupportedInActiveSession = 0x7F,

    /// Engine RPM is too high or too low.
    RpmTooHighOrLow = 0x81,

    /// The engine is running when it should be off, or vice versa.
    EngineRunningOrNotRunning = 0x83,

    /// Engine runtime is too low to perform the requested action.
    EngineRunTimeTooLow = 0x85,

    /// The temperature is outside the acceptable range.
    TemperatureTooHighOrLow = 0x86,

    /// The speed is too high or too low to perform the requested operation.
    SpeedTooHighOrLow = 0x88,

    /// The throttle pedal position is too high or too low.
    ThrottlePedalTooHighOrLow = 0x89,

    /// The transmission is not in neutral or in a valid gear.
    TransmissionNotInNeutralOrGear = 0x8C,

    /// Brake switches are not closed, preventing execution.
    BrakeSwitchesNotClosed = 0x8F,

    /// The shifter lever is not in the park position.
    ShifterLeverNotInPark = 0x90,

    /// The torque converter clutch is locked.
    TorqueConverterClutchLocked = 0x91,

    /// Voltage is either too high or too low.
    VoltageTooHighOrLow = 0x92,

    /// Manufacturer-specific error codes (0xF0–0xFE range).
    ManufacturerSpecific = 0xF0,
}

#[cfg(all(test, feature = "serde"))]
mod tests {
    use super::*;

    #[derive(serde::Serialize, serde::Deserialize)]
    struct TestStruct {
        nrc: UdsNrc,
        nrc_byte: UdsNrcByte,
    }

    #[test]
    fn test_serde() {
        let test = TestStruct {
            nrc: UdsNrc::ConditionsNotCorrect,
            nrc_byte: UdsNrcByte::from(UdsNrc::ConditionsNotCorrect),
        };

        let json = serde_json::to_string(&test).unwrap();
        assert_eq!(
            json,
            r###"{"nrc":"ConditionsNotCorrect","nrc_byte":{"Standard":"ConditionsNotCorrect"}}"###
        );

        let deserialized: TestStruct = serde_json::from_str(&json).unwrap();

        assert_eq!(test.nrc, deserialized.nrc);
        assert_eq!(test.nrc_byte, deserialized.nrc_byte);
    }
}
