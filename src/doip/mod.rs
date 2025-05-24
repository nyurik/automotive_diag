//! Module for [Diagnostics over IP (`DoIP`)](https://automotivevehicletesting.com/vehicle-diagnostics/doip/) - ISO-13400

mod payload_type;
mod protocol_version;

pub use payload_type::*;
pub use protocol_version::*;
