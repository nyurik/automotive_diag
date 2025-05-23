use crate::utils::{enum_wrapper, python_test};

enum_wrapper!(uds, ResetType, ResetTypeByte);
python_test!(uds, ResetType, HardReset, SoftReset);

/// Reset ECU subcommand
#[repr(u8)]
#[derive(strum::FromRepr, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "iter", derive(strum::EnumIter))]
#[cfg_attr(feature = "pyo3", pyo3::pyclass(eq, eq_int))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
