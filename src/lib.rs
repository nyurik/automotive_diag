#![cfg_attr(feature = "default", doc = include_str!("../README.md"))]
#![cfg_attr(not(feature = "std"), no_std)]

// It makes no sense to use this crate without at least one of the core features enabled
#[cfg(not(any(feature = "with-kwp2000", feature = "with-obd2", feature = "with-uds")))]
compile_error!(
    "At least one of the features `with-kwp2000`, `with-obd2`, or `with-uds` must be enabled!"
);

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
    #[cfg(feature = "with-uds")]
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
