use crate::utils::{enum_wrapper, python_test};

enum_wrapper!(doip, SyncStatus, SyncStatusByte, display = @"14520419517183707886");
python_test!(doip, SyncStatus, VinGidSynchronized, VinGidNotSynchronized);

/// The synchronization status of the VIN and the GID for the entity
///
/// This gives the status that all `DoIP` entities have synchronized their information
/// about the VIN or GID of the vehicle.
#[repr(u8)]
#[derive(strum::FromRepr, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "display", derive(displaydoc::Display))]
#[cfg_attr(feature = "iter", derive(strum::EnumIter))]
#[cfg_attr(feature = "pyo3", pyo3::pyclass(eq, eq_int))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SyncStatus {
    /// VIN/GID Synchronized
    VinGidSynchronized = 0x00,

    /// VIN/GID are not synchronized
    VinGidNotSynchronized = 0x10,
}
