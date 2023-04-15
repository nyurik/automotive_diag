#![doc = include_str!("../README.md")]
#![cfg_attr(not(feature = "std"), no_std)]
#![deny(clippy::all, clippy::pedantic)]
#![allow(clippy::missing_errors_doc, clippy::module_name_repetitions)]

#[cfg(feature = "with-kwp2000")]
pub mod kwp2000;
#[cfg(feature = "with-obd2")]
pub mod obd2;
#[cfg(feature = "with-uds")]
pub mod uds;

mod utils;
pub use utils::ByteWrapper;

#[cfg(test)]
mod tests {

    #[test]
    fn spot_test() {
        use crate::uds::UdsCommand::ECUReset;
        use crate::uds::{UdsCommand, UdsCommandByte};
        use crate::ByteWrapper::{Extended, Standard};

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
