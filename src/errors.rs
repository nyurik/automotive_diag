use crate::byte_enum;
use crate::utils::ByteWrapper;
use bytenum::Bytenum;

/// UDS Error definitions
#[repr(u8)]
#[derive(Bytenum, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum UdsError {
    /// ECU rejected the request (No specific error)
    GeneralReject = 0x10,
    /// Service is not supported by the ECU
    ServiceNotSupported = 0x11,
    /// Sub function is not supported by the ECU
    SubFunctionNotSupported = 0x12,
    /// Request message was an invalid length, or the format of the
    /// request was incorrect
    IncorrectMessageLengthOrInvalidFormat = 0x13,
    /// The response message is too long for the transport protocol
    ResponseTooLong = 0x14,
    /// The ECU is too busy to perform this request. Therefore, the request
    /// Should be sent again if this error occurs
    BusyRepeatRequest = 0x21,
    /// The requested action could not be preformed due to the prerequisite conditions
    /// not being correct
    ConditionsNotCorrect = 0x22,
    /// The ECU cannot perform the request as the request has been sent in the incorrect order.
    /// For example, if [UdsDiagnosticServer::send_key] is used before [UdsDiagnosticServer::request_seed],
    /// then the ECU will respond with this error.
    RequestSequenceError = 0x24,
    /// The ECU cannot perform the request as it has timed out trying to communicate with another
    /// component within the vehicle.
    NoResponseFromSubnetComponent = 0x25,
    /// The ECU cannot perform the requested action as there is currently a DTC
    /// or failure of a component that is preventing the execution of the request.
    FailurePreventsExecutionOfRequestedAction = 0x26,
    /// The request message contains data outside of a valid range
    RequestOutOfRange = 0x31,
    /// The request could not be completed due to security access being denied.
    SecurityAccessDenied = 0x33,
    /// The key sent from [UdsDiagnosticServer::send_key] was invalid
    InvalidKey = 0x35,
    /// The client has tried to obtain security access to the ECU too many times with
    /// incorrect keys
    ExceedNumberOfAttempts = 0x36,
    /// The client has tried to request seed_key's too quickly, before the ECU timeout's period
    /// has expired
    RequiredTimeDelayNotExpired = 0x37,
    /// The ECU cannot accept the requested upload/download request due to a fault condition
    UploadDownloadNotAccepted = 0x70,
    /// The ECU has halted data transfer due to a fault condition
    TransferDataSuspended = 0x71,
    /// The ECU has encountered an error during reprogramming (erasing / flashing)
    GeneralProgrammingFailure = 0x72,
    /// The ECU has detected the reprogramming error as the blockSequenceCounter is incorrect.
    WrongBlockSequenceCounter = 0x73,
    /// The ECU has accepted the request, but cannot reply right now. If this error occurs,
    /// the [UdsDiagnosticServer] will automatically stop sending tester present messages and
    /// will wait for the ECUs response. If after 2000ms, the ECU did not respond, then this error
    /// will get returned back to the function call.
    RequestCorrectlyReceivedResponsePending = 0x78,
    /// The sub function is not supported in the current diagnostic session mode
    SubFunctionNotSupportedInActiveSession = 0x7E,
    /// The service is not supported in the current diagnostic session mode
    ServiceNotSupportedInActiveSession = 0x7F,
    /// Engine RPM is too high
    RpmTooHigh = 0x81,
    /// Engine RPM is too low
    RpmTooLow = 0x82,
    /// Engine is running
    EngineIsRunning = 0x83,
    /// Engine is not running
    EngineIsNotRunning = 0x84,
    /// Engine has not been running for long enough
    EngineRunTimeTooLow = 0x85,
    /// Engine temperature (coolant) is too high
    TemperatureTooHigh = 0x86,
    /// Engine temperature (coolant) is too low
    TemperatureTooLow = 0x87,
    /// Vehicle speed is too high
    VehicleSpeedTooHigh = 0x88,
    /// Vehicle speed is too low
    VehicleSpeedTooLow = 0x89,
    /// Throttle or pedal value is too high
    ThrottleTooHigh = 0x8A,
    /// Throttle or pedal value is too low
    ThrottleTooLow = 0x8B,
    /// Transmission is not in neutral
    TransmissionRangeNotInNeutral = 0x8C,
    /// Transmission is not in gear
    TransmissionRangeNotInGear = 0x8D,
    /// Brake is not applied
    BrakeSwitchNotClosed = 0x8F,
    /// Shifter lever is not in park
    ShifterLeverNotInPark = 0x90,
    /// Automatic/CVT transmission torque convert is locked
    TorqueConverterClutchLocked = 0x91,
    /// Voltage is too high
    VoltageTooHigh = 0x92,
    /// Voltage is too low
    VoltageTooLow = 0x93,
}

byte_enum!(UdsError, UdsErrorByte);
