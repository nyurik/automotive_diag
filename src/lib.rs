//! Module for UDS (Unified diagnostic services - ISO 14229-1)
//!
//! Theoretically, this module should be compliant with any ECU which implements
//! UDS (Typically any ECU produced after 2006 supports this)

#![cfg_attr(not(feature = "std"), no_std)]
#![deny(clippy::all, clippy::pedantic)]
#![allow(clippy::missing_errors_doc)]

mod commands;
mod communication_control;
mod communication_level;
mod errors;
mod read_dtc_information;
mod reset_types;
mod scaling_byte_ext;
mod scaling_byte_high;
mod security_access;
mod session_types;

pub use commands::*;
pub use communication_control::*;
pub use communication_level::*;
pub use errors::*;
pub use read_dtc_information::*;
pub use reset_types::*;
pub use scaling_byte_ext::*;
pub use scaling_byte_high::*;
pub use security_access::*;
pub use session_types::*;

#[cfg(test)]
mod tests {
    #[macro_export]
    macro_rules! test_encode_decode_enum {
        ($enum:tt) => {
            #[test]
            #[allow(non_snake_case)]
            fn $enum() {
                for value in 0_u8..=0xFF {
                    if let Ok(v) = crate::$enum::try_from(value) {
                        let enc: u8 = v.into();
                        assert_eq!(value, enc, "0x{value:x} → {v:?} → 0x{enc:x}");
                    }
                }
            }
        };
    }
}
