#[cfg(doc)]
use crate::uds::UdsCommand;
use crate::utils::enum_wrapper;

enum_wrapper!(uds, DtcSubFunction, DtcSubFunctionByte);

/// [`UdsCommand::ReadDTCInformation`] sub-function definitions
#[repr(u8)]
#[derive(strum::FromRepr, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "iter", derive(strum::EnumIter))]
#[cfg_attr(feature = "display", derive(displaydoc::Display))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[allow(clippy::doc_markdown, clippy::enum_variant_names)]
pub enum DtcSubFunction {
    /// This function takes a 1-byte DTCStatusMask
    ReportNumberOfDtcByStatusMask = 0x01,
    /// This function takes a 1-byte DTCStatusMask
    ReportDtcByStatusMask = 0x02,
    /// This function takes a 1-byte DTCStatusMask
    ReportMirrorMemoryDtcByStatusMask = 0x0F,
    /// This function takes a 1-byte DTCStatusMask
    ReportNumberOfMirrorMemoryDtcByStatusMask = 0x11,
    /// This function takes a 1-byte DTCStatusMask
    ReportNumberOfEmissionsRelatedObdDtcByStatusMask = 0x12,
    /// This function takes a 1-byte DTCStatusMask
    ReportEmissionsRelatedObdDtcByStatusMask = 0x13,

    /// This function takes a 3-byte DTCMaskRecord and a 1-byte DTCSnapshotRecordNumber
    ReportDtcSnapshotIdentifier = 0x03,
    /// This function takes a 3-byte DTCMaskRecord and a 1-byte DTCSnapshotRecordNumber
    ReportDtcSnapshotRecordByDtcNumber = 0x04,

    /// This function takes a 1-byte DTCSnapshotRecordNumber
    ReportDtcSnapshotRecordByRecordNumber = 0x05,

    /// This function takes a 3-byte DTCMaskRecord and a 1-byte DTCExtendedDataRecordNumber
    ReportDtcExtendedDataRecordByDtcNumber = 0x06,
    /// This function takes a 3-byte DTCMaskRecord and a 1-byte DTCExtendedDataRecordNumber
    ReportMirrorMemoryDtcExtendedDataRecordByDtcNumber = 0x10,

    /// This function takes a 1-byte DTCSeverityMask and a 1-byte DTCStatusMask
    ReportNumberOfDtcBySeverityMaskRecord = 0x07,
    /// This function takes a 1-byte DTCSeverityMask and a 1-byte DTCStatusMask
    ReportDtcBySeverityMaskRecord = 0x08,

    /// This function takes a 3-byte DTCMaskRecord
    ReportSeverityInformationOfDtc = 0x09,

    /// This function takes no additional arguments
    ReportSupportedDtc = 0x0A,
    /// This function takes no additional arguments
    ReportFirstTestFailedDtc = 0x0B,
    /// This function takes no additional arguments
    ReportFirstConfirmedDtc = 0x0C,
    /// This function takes no additional arguments
    ReportMostRecentTestFailedDtc = 0x0D,
    /// This function takes no additional arguments
    ReportMostRecentConfirmedDtc = 0x0E,
    /// This function takes no additional arguments
    ReportDtcFaultDetectionCounter = 0x14,
    /// This function takes no additional arguments
    ReportDtcWithPermanentStatus = 0x15,
}
