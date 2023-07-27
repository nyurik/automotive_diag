use enum2repr::EnumRepr;

use crate::enum_wrapper;

enum_wrapper!(kwp2000, RoutineExitStatus, RoutineExitStatusByte);

#[repr(u8)]
#[derive(EnumRepr, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum RoutineExitStatus {
    /// Normal exit with results available
    NormalExitWithResults = 0x61,
    /// Normal exit, the routine does not return any result data
    NormalExitWithoutResults = 0x62,
    /// Abnormal or premature exit. No results available.
    AbnormalExitWithoutResults = 0x64,
}
