use bytenum::Bytenum;

/// [`crate::DtcSubFunction::ReadDtcInformation`] sub-function definitions
#[repr(u8)]
#[derive(Bytenum, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum DtcSubFunction {
    /// This function takes a 1 byte DTCStatusMask
    ReportNumberOfDTCByStatusMask = 0x01,
    /// This function takes a 1 byte DTCStatusMask
    ReportDTCByStatusMask = 0x02,
    /// This function takes a 1 byte DTCStatusMask
    ReportMirrorMemoryDTCByStatusMask = 0x0F,
    /// This function takes a 1 byte DTCStatusMask
    ReportNumberOfMirrorMemoryDTCByStatusMask = 0x11,
    /// This function takes a 1 byte DTCStatusMask
    ReportNumberOfEmissionsRelatedOBDDTCByStatusMask = 0x12,
    /// This function takes a 1 byte DTCStatusMask
    ReportEmissionsRelatedOBDDTCByStatusMask = 0x13,

    /// This function takes a 3 byte DTCMaskRecord and a 1 byte DTCSnapshotRecordNumber
    ReportDTCSnapshotIdentifier = 0x03,
    /// This function takes a 3 byte DTCMaskRecord and a 1 byte DTCSnapshotRecordNumber
    ReportDTCSnapshotRecordByDTCNumber = 0x04,

    /// This function takes a 1 byte DTCSnapshotRecordNumber
    ReportDTCSnapshotRecordByRecordNumber = 0x05,

    /// This function take a 3 byte DTCMaskRecord and a 1 byte DTCExtendedDataRecordNumber
    ReportDTCExtendedDataRecordByDTCNumber = 0x06,
    /// This function take a 3 byte DTCMaskRecord and a 1 byte DTCExtendedDataRecordNumber
    ReportMirrorMemoryDTCExtendedDataRecordByDTCNumber = 0x10,

    /// This function takes a 1 byte DTCSeverityMask and a 1 byte DTCStatusMask
    ReportNumberOfDTCBySeverityMaskRecord = 0x07,
    /// This function takes a 1 byte DTCSeverityMask and a 1 byte DTCStatusMask
    ReportDTCBySeverityMaskRecord = 0x08,

    /// This function takes a 3 byte DTCMaskRecord
    ReportSeverityInformationOfDTC = 0x09,

    /// This function take no additional arguments
    ReportSupportedDTC = 0x0A,
    /// This function take no additional arguments
    ReportFirstTestFailedDTC = 0x0B,
    /// This function take no additional arguments
    ReportFirstConfirmedDTC = 0x0C,
    /// This function take no additional arguments
    ReportMostRecentTestFailedDTC = 0x0D,
    /// This function take no additional arguments
    ReportMostRecentConfirmedDTC = 0x0E,
    /// This function take no additional arguments
    ReportDTCFaultDetectionCounter = 0x14,
    /// This function take no additional arguments
    ReportDTCWithPermanentStatus = 0x15,
}

impl From<DtcSubFunction> for u8 {
    fn from(value: DtcSubFunction) -> Self {
        value as u8
    }
}

#[cfg(test)]
mod tests {
    crate::test_encode_decode_enum!(DtcSubFunction);
}
