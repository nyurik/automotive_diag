crate::utils::enum_wrapper!(obd2, ObdStandard, ObdStandardByte, display = @"16384231687073919054");

/// OBD Standard for PID 1C
#[allow(non_camel_case_types)]
#[repr(u8)]
#[derive(strum::FromRepr, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "iter", derive(strum::EnumIter))]
#[cfg_attr(feature = "display", derive(displaydoc::Display))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[allow(clippy::doc_markdown, clippy::upper_case_acronyms)]
pub enum ObdStandard {
    /// OBD-II as defined by the CARB
    OBD_II_CARB = 1,
    /// OBD as defined by the EPA
    OBD_EPA = 2,
    /// OBD and OBD-II
    OBD_OBD_II = 3,
    /// OBD-I
    OBD_I = 4,
    /// Not OBD Compliant
    NON_COMPLIANT = 5,
    /// Europe OBD
    EOBD = 6,
    /// Europe OBD and OBD-II
    EOBD_OBD_II = 7,
    /// Europe OBD and OBD
    EOBD_OBD = 8,
    /// Europe OBD, OBD and OBD-II
    EOBD_OBD_OBD_II = 9,
    /// Japan OBD
    JOBD = 10,
    /// Japan OBD and OBD-II
    JOBD_OBD_II = 11,
    /// Japan OBD and Europe OBD
    JOBD_EOBD = 12,
    /// Japan OBD, Europe OBD and OBD-II
    JOBD_EOBD_OBD_II = 13,
    /// Engine Manufacturer Diagnostics
    EMD = 17,
    /// Engine Manufacturer Diagnostics Enhanced
    EMD_PLUS = 18,
    /// Heavy Duty OBD (Child/Partial)
    HD_OBD_C = 19,
    /// Heavy duty OBD
    HD_OBD = 20,
    /// World wide harmonized OBD
    WWH_OBD = 21,
    /// Heavy duty OBD Stage I without NOx control
    HD_EOBD_I = 23,
    /// Heavy duty OBD Stage I with NOx control
    HD_EOBD_I_N = 24,
    /// Heavy duty OBD Stage II without NOx control
    HD_EOBD_II = 25,
    /// Heavy duty OBD Stage II with NOx control
    HD_EOBD_II_N = 26,
    /// Brazil OBD Phase 1
    OBDBR_1 = 28,
    /// Brazil OBD Phase 2
    OBDBR_2 = 29,
    /// Korean OBD
    KOBD = 30,
    /// Indian OBD-I
    IOBD_I = 31,
    /// Indian OBD-II
    IOBD_II = 32,
    /// Heavy duty Euro OBD Stage VI
    HD_EOBD_IV = 33,
}
