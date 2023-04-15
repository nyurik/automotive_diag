#![doc = include_str!("../README.md")]
#![cfg_attr(not(feature = "std"), no_std)]
#![deny(clippy::all, clippy::pedantic)]
#![allow(clippy::missing_errors_doc)]

mod comm_control;
mod comm_level;
mod commands;
mod errors;
mod read_dtc_information;
mod reset_types;
mod scaling_byte;
mod scaling_byte_ext;
mod security_access;
mod session_types;
mod utils;

pub use comm_control::*;
pub use comm_level::*;
pub use commands::*;
pub use errors::*;
pub use read_dtc_information::*;
pub use reset_types::*;
pub use scaling_byte::*;
pub use scaling_byte_ext::*;
pub use security_access::*;
pub use session_types::*;
pub use utils::ByteWrapper;

#[cfg(test)]
mod tests {

    #[test]
    fn spot_test() {
        use crate::ByteWrapper::{Extended, Standard};
        use crate::UdsCommand::ECUReset;
        use crate::{UdsCommand, UdsCommandByte};

        assert_eq!(UdsCommandByte::from(0x11), Standard(ECUReset));
        assert_eq!(UdsCommand::try_from(0x11), Ok(ECUReset));
        assert_eq!(ECUReset as u8, 0x11);
        assert_eq!(u8::from(ECUReset), 0x11);
        assert_eq!(u8::from(Standard(ECUReset)), 0x11);
        assert_eq!(UdsCommandByte::from(ECUReset), Standard(ECUReset));

        assert!(UdsCommand::try_from(0x42).is_err());
        assert_eq!(UdsCommandByte::from(0x42), Extended(0x42));
    }
}
