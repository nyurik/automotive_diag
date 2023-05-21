use core::fmt::Debug;

/// A wrapper around a byte, which can be either an ISO-standardized value for a specific enum,
/// or an implementation-specific/invalid `Extended` value wrapping original byte.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ByteWrapper<T> {
    Standard(T),
    Extended(u8),
}

impl<T: Debug> Debug for ByteWrapper<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            // For standard values, just delegate to the Debug implementation of the inner type.
            Self::Standard(v) => Debug::fmt(v, f),
            Self::Extended(v) => write!(f, "Extended({v:#02X})"),
        }
    }
}

impl<T: Into<u8>> From<ByteWrapper<T>> for u8 {
    fn from(value: ByteWrapper<T>) -> Self {
        match value {
            ByteWrapper::Standard(v) => v.into(),
            ByteWrapper::Extended(v) => v,
        }
    }
}

impl<T: TryFrom<u8>> From<u8> for ByteWrapper<T> {
    fn from(value: u8) -> Self {
        match T::try_from(value) {
            Ok(v) => ByteWrapper::Standard(v),
            Err(_) => ByteWrapper::Extended(value),
        }
    }
}

#[macro_export]
macro_rules! enum_wrapper {
    ($ns:tt, $enum_name:tt, $enum_wrapper:tt) => {
        #[doc = concat!("Store a single byte, either as a `Standard(", stringify!($enum_name), ")`, or as an `Extended(u8)`.")]
        pub type $enum_wrapper = $crate::ByteWrapper<$enum_name>;

        impl From<$crate::$ns::$enum_name> for $crate::$ns::$enum_wrapper {
            fn from(value: $crate::$ns::$enum_name) -> Self {
                Self::Standard(value)
            }
        }

        #[cfg(test)]
        mod enum_wrapper_tests {
            #[test]
            #[allow(non_snake_case)]
            fn test_try_from() {
                for value in 0x00_u8..=0xFF {
                    if let Ok(v) = $crate::$ns::$enum_name::try_from(value) {
                        let enc: u8 = v.into();
                        assert_eq!(value, enc, "{value:#02X} → {v:?} → {enc:#02X}");
                    }
                }
            }

            #[test]
            #[allow(non_snake_case)]
            fn test_from() {
                for value in 0x00_u8..=0xFF {
                    let v = $crate::$ns::$enum_wrapper::from(value);
                    let enc: u8 = v.into();
                    assert_eq!(value, enc, "Wrapped {value:#02X} → {v:?} → {enc:#02X}");
                }
            }
        }
    };
}
