/// Options for resetting the ECU
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum UdsReset {
    /// Signals the ECU to perform a hard-reset,
    /// simulating a forceful power off/on cycle
    ///
    /// This might result in both non-volatile memory and volatile memory locations being re-initialized
    HardReset,

    /// Signals the ECU to perform a simulated key off/on cycle,
    /// simulating the usual key-off/on cycle
    ///
    /// This typically results in the preservation of non-volatile memory,
    /// but volatile memory will be re-initialized
    KeyOffReset,

    /// Signals the ECU to perform a soft reset, simply rebooting the current
    /// application running on it.
    ///
    /// This will result in the preservation of both non-volatile and volatile memory
    SoftReset,

    /// Enables a rapid power shutdown on the ECU during a key-off cycle.
    ///
    /// IMPORTANT: Once this has been used, the diagnostic server **cannot** send
    /// any other messages other than ECUReset in order to not disturb the rapid power
    /// shutdown function.
    EnableRapidPowerShutDown,

    /// Disables a rapid power shutdown on the ECU during a key-off cycle.
    DisableRapidPowerShutDown,

    /// Other OEM defined power mode
    Other(u8),
}

impl From<UdsReset> for u8 {
    fn from(from: UdsReset) -> Self {
        match from {
            UdsReset::HardReset => 0x01,
            UdsReset::KeyOffReset => 0x02,
            UdsReset::SoftReset => 0x03,
            UdsReset::EnableRapidPowerShutDown => 0x04,
            UdsReset::DisableRapidPowerShutDown => 0x05,
            UdsReset::Other(x) => x,
        }
    }
}
