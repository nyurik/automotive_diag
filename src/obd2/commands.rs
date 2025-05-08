crate::utils::enum_wrapper!(obd2, Obd2Command, Obd2CommandByte, display = @"200205385734257990");

/// OBD2 Command Service IDs
#[repr(u8)]
#[derive(strum::FromRepr, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "display", derive(displaydoc::Display))]
#[cfg_attr(feature = "iter", derive(strum::EnumIter))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Obd2Command {
    /// Service 01 - Show current data
    Service01 = 0x01,
    /// Service 02 - Show freeze frame data
    Service02 = 0x02,
    /// Service 03 - Show stored DTCs
    Service03 = 0x03,
    /// Service 04 - Clear stored DTCs
    Service04 = 0x04,
    /// Test results, O2 sensor monitoring (non CAN)
    Service05 = 0x05,
    /// Test results, O2 sensor monitoring (CAN)
    Service06 = 0x06,
    /// Show pending DTCs
    Service07 = 0x07,
    /// Control operation of on-board components
    Service08 = 0x08,
    /// Service 09 - Request vehicle information
    Service09 = 0x09,
    /// Service 0A - Read permanent DTCs
    Service0A = 0x0A,
}
