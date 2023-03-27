use core::fmt::Debug;

/// A wrapper around a byte, which can be either an ISO-standardized value for a specific enum,
/// or an implementation-specific/invalid `NonStandard` value wrapping original byte.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ByteWrapper<T> {
    Standard(T),
    NonStandard(u8),
}

impl<T: Debug> Debug for ByteWrapper<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            // For standard values, just delegate to the Debug implementation of the inner type.
            Self::Standard(v) => Debug::fmt(v, f),
            Self::NonStandard(v) => write!(f, "NonStandard({v:#02X})"),
        }
    }
}

impl<T: Into<u8>> From<ByteWrapper<T>> for u8 {
    fn from(value: ByteWrapper<T>) -> Self {
        match value {
            ByteWrapper::Standard(v) => v.into(),
            ByteWrapper::NonStandard(v) => v,
        }
    }
}

impl<T: TryFrom<u8>> From<u8> for ByteWrapper<T> {
    fn from(value: u8) -> Self {
        match T::try_from(value) {
            Ok(v) => ByteWrapper::Standard(v),
            Err(_) => ByteWrapper::NonStandard(value),
        }
    }
}

#[macro_export]
macro_rules! enum_wrapper {
    ($enum_name:tt, $enum_wrapper:tt) => {
        /// Stores a single byte, either as a `Standard($enum_name)`, or as an `NonStandard(u8)`.
        pub type $enum_wrapper = ByteWrapper<$enum_name>;

        #[cfg(test)]
        mod tests {
            #[test]
            #[allow(non_snake_case)]
            fn test_try_from() {
                for value in 0x00_u8..=0xFF {
                    if let Ok(v) = $crate::$enum_name::try_from(value) {
                        let enc: u8 = v.into();
                        assert_eq!(value, enc, "{value:#02X} → {v:?} → {enc:#02X}");
                    }
                }
            }
            #[test]
            #[allow(non_snake_case)]
            fn test_from() {
                for value in 0x00_u8..=0xFF {
                    let v = $crate::$enum_wrapper::from(value);
                    let enc: u8 = v.into();
                    assert_eq!(
                        value, enc,
                        "With wrapper: {value:#02X} → {v:?} → {enc:#02X}"
                    );
                }
            }
        }
    };
}
