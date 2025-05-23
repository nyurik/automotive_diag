use crate::utils::{enum_wrapper, python_test};

enum_wrapper!(obd2, Service09Pid, Service09PidByte, display = @"7936397335745330157");
python_test!(obd2, Service09Pid, VinMsgCount, Vin);

/// OBD2 service 09 (Request vehicle information) PIDs
#[repr(u8)]
#[derive(strum::FromRepr, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "display", derive(displaydoc::Display))]
#[cfg_attr(feature = "iter", derive(strum::EnumIter))]
#[cfg_attr(feature = "pyo3", pyo3::pyclass(eq, eq_int))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Service09Pid {
    /// VIN message count (Only for LIN)
    VinMsgCount = 0x01,
    /// VIN
    Vin = 0x02,
    /// Calibration ID message count (Only for LIN)
    CalibrationIDMsgCount = 0x03,
    /// Calibration ID
    CalibrationID = 0x04,
    /// CVN message count (Only for LIN)
    CvnMsgCount = 0x05,
    /// CVN
    Cvn = 0x06,
    /// In use performance tracking for spark ignition engines
    InUsePerfTracking = 0x08,
    ///ECU name message count (Only for LIN)
    EcuNameMsgCount = 0x09,
    /// ECU name
    EcuName = 0x0A,
}
