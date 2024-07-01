use strum::{EnumIter, FromRepr};

crate::enum_wrapper!(kwp2000, RoutineExitStatus, RoutineExitStatusByte);

#[repr(u8)]
#[derive(FromRepr, EnumIter, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum RoutineExitStatus {
    /// Normal exit with results available
    NormalExitWithResults = 0x61,
    /// Normal exit, the routine does not return any result data
    NormalExitWithoutResults = 0x62,
    /// Abnormal or premature exit. No results available.
    AbnormalExitWithoutResults = 0x64,
}
