use enum2repr::EnumRepr;

use crate::enum_wrapper;

enum_wrapper!(uds, ResetType, ResetTypeByte);

/// Reset ECU subcommand
#[repr(u8)]
#[derive(EnumRepr, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ResetType {
    /// Signals the ECU to perform a hard-reset,
    /// simulating a forceful power off/on cycle
    ///
    /// This might result in both non-volatile memory and volatile memory locations being re-initialized
    HardReset = 0x01,

    /// Signals the ECU to perform a simulated key off/on cycle,
    /// simulating the usual key-off/on cycle
    ///
    /// This typically results in the preservation of non-volatile memory,
    /// but volatile memory will be re-initialized
    KeyOffReset = 0x02,

    /// Signals the ECU to perform a soft reset, simply rebooting the current
    /// application running on it.
    ///
    /// This will result in the preservation of both non-volatile and volatile memory
    SoftReset = 0x03,

    /// Enables a rapid power shutdown on the ECU during a key-off cycle.
    ///
    /// IMPORTANT: Once this has been used, the diagnostic server **cannot** send
    /// any other messages other than `ECUReset` in order to not disturb the rapid power
    /// shutdown function.
    EnableRapidPowerShutDown = 0x04,

    /// Disables a rapid power shutdown on the ECU during a key-off cycle.
    DisableRapidPowerShutDown = 0x05,
}
