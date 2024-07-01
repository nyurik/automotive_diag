use strum::{EnumIter, FromRepr};

use crate::enum_wrapper;
#[cfg(doc)]
use crate::uds::UdsCommand;

enum_wrapper!(uds, DtcSubFunction, DtcSubFunctionByte);

/// [`UdsCommand::ReadDTCInformation`] sub-function definitions
#[repr(u8)]
#[derive(FromRepr, EnumIter, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "display", derive(displaydoc::Display))]
#[allow(clippy::doc_markdown)]
pub enum DtcSubFunction {
    /// This function takes a 1 byte DTCStatusMask
    ReportNumberOfDtcByStatusMask = 0x01,
    /// This function takes a 1 byte DTCStatusMask
    ReportDtcByStatusMask = 0x02,
    /// This function takes a 1 byte DTCStatusMask
    ReportMirrorMemoryDtcByStatusMask = 0x0F,
    /// This function takes a 1 byte DTCStatusMask
    ReportNumberOfMirrorMemoryDtcByStatusMask = 0x11,
    /// This function takes a 1 byte DTCStatusMask
    ReportNumberOfEmissionsRelatedObdDtcByStatusMask = 0x12,
    /// This function takes a 1 byte DTCStatusMask
    ReportEmissionsRelatedObdDtcByStatusMask = 0x13,

    /// This function takes a 3 byte DTCMaskRecord and a 1 byte DTCSnapshotRecordNumber
    ReportDtcSnapshotIdentifier = 0x03,
    /// This function takes a 3 byte DTCMaskRecord and a 1 byte DTCSnapshotRecordNumber
    ReportDtcSnapshotRecordByDtcNumber = 0x04,

    /// This function takes a 1 byte DTCSnapshotRecordNumber
    ReportDtcSnapshotRecordByRecordNumber = 0x05,

    /// This function take a 3 byte DTCMaskRecord and a 1 byte DTCExtendedDataRecordNumber
    ReportDtcExtendedDataRecordByDtcNumber = 0x06,
    /// This function take a 3 byte DTCMaskRecord and a 1 byte DTCExtendedDataRecordNumber
    ReportMirrorMemoryDtcExtendedDataRecordByDtcNumber = 0x10,

    /// This function takes a 1 byte DTCSeverityMask and a 1 byte DTCStatusMask
    ReportNumberOfDtcBySeverityMaskRecord = 0x07,
    /// This function takes a 1 byte DTCSeverityMask and a 1 byte DTCStatusMask
    ReportDtcBySeverityMaskRecord = 0x08,

    /// This function takes a 3 byte DTCMaskRecord
    ReportSeverityInformationOfDtc = 0x09,

    /// This function take no additional arguments
    ReportSupportedDtc = 0x0A,
    /// This function take no additional arguments
    ReportFirstTestFailedDtc = 0x0B,
    /// This function take no additional arguments
    ReportFirstConfirmedDtc = 0x0C,
    /// This function take no additional arguments
    ReportMostRecentTestFailedDtc = 0x0D,
    /// This function take no additional arguments
    ReportMostRecentConfirmedDtc = 0x0E,
    /// This function take no additional arguments
    ReportDtcFaultDetectionCounter = 0x14,
    /// This function take no additional arguments
    ReportDtcWithPermanentStatus = 0x15,
}
