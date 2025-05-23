use crate::utils::{enum_wrapper, python_test};

enum_wrapper!(kwp2000, RoutineExitStatus, RoutineExitStatusByte);
python_test!(
    kwp2000,
    RoutineExitStatus,
    NormalExitWithResults,
    AbnormalExitWithoutResults
);

#[repr(u8)]
#[derive(strum::FromRepr, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "iter", derive(strum::EnumIter))]
#[cfg_attr(feature = "pyo3", pyo3::pyclass(eq, eq_int))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[allow(clippy::enum_variant_names)]
pub enum RoutineExitStatus {
    /// Normal exit with results available
    NormalExitWithResults = 0x61,
    /// Normal exit, the routine does not return any result data
    NormalExitWithoutResults = 0x62,
    /// Abnormal or premature exit. No results available.
    AbnormalExitWithoutResults = 0x64,
}
