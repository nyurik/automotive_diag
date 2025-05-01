crate::utils::enum_impls!(doip, DoIpPayloadType, u16);
crate::utils::assert_display_hash!(DoIpPayloadType, @"5886519332798079216");

/// Defines the variants of payloads available to `DoIP`.
///
/// `DoIpPayloadType` values map to the `u16` representing the bytes it makes up
/// within the `DoIP` packet.
#[repr(u16)]
#[derive(strum::FromRepr, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "display", derive(displaydoc::Display))]
#[cfg_attr(feature = "iter", derive(strum::EnumIter))]
#[cfg_attr(feature = "pyo3", pyo3::pyclass(eq, eq_int))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
// TODO: should this be DoIp-prefixed?
pub enum DoIpPayloadType {
    /// Generic Negative Acknowledge
    GenericNack = 0x0000,

    /// Vehicle Identification Request
    VehicleIdentificationRequest = 0x0001,

    /// Vehicle Identification Request by EID
    VehicleIdentificationRequestEid = 0x0002,

    /// Vehicle Identification Request by VIN
    VehicleIdentificationRequestVin = 0x0003,

    /// Vehicle Announcement Message
    VehicleAnnouncementMessage = 0x0004,

    /// Routing Activation Request
    RoutingActivationRequest = 0x0005,

    /// Routing Activation Response
    RoutingActivationResponse = 0x0006,

    /// Alive Check Request
    AliveCheckRequest = 0x0007,

    /// Alive Check Response
    AliveCheckResponse = 0x0008,

    /// Entity Status Request
    EntityStatusRequest = 0x4001,

    /// Entity Status Response
    EntityStatusResponse = 0x4002,

    /// Power Information Request
    PowerInformationRequest = 0x4003,

    /// Power Information Response
    PowerInformationResponse = 0x4004,

    /// Diagnostic Message
    DiagnosticMessage = 0x8001,

    /// Diagnostic Message Acknowledgement
    DiagnosticMessageAck = 0x8002,

    /// Diagnostic Message Negative Acknowledgement
    DiagnosticMessageNack = 0x8003,
}
