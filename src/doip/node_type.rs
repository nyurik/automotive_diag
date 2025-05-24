use crate::utils::{enum_wrapper, python_test};

enum_wrapper!(doip, NodeType, NodeTypeByte, display = @"13625208806899800470");
python_test!(doip, NodeType, DoIpGateway, DoIpNode);

/// Used in `EntityStatusResponse`, `NodeType` provides the possibilities of the
/// `node_type` field.
///
/// Used to understand the result of a `DoIP` packet.
#[repr(u8)]
#[derive(strum::FromRepr, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "display", derive(displaydoc::Display))]
#[cfg_attr(feature = "iter", derive(strum::EnumIter))]
#[cfg_attr(feature = "pyo3", pyo3::pyclass(eq, eq_int))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum NodeType {
    /// `DoIP` Gateway
    DoIpGateway = 0x00,

    /// `DoIP` Node
    DoIpNode = 0x01,
}
