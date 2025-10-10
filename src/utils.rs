use core::fmt::Debug;

/// A wrapper around a byte, which can be either an ISO-standardized value for a specific enum
/// or an implementation-specific/invalid `Extended` value wrapping original byte.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ByteWrapper<T> {
    Standard(T),
    Extended(u8),
}

impl<T: Debug> Debug for ByteWrapper<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            // For standard values, delegate to the Debug implementation of the inner type.
            Self::Standard(v) => Debug::fmt(v, f),
            Self::Extended(v) => write!(f, "Extended({v:#04X})"),
        }
    }
}

#[cfg(feature = "defmt")]
impl<T: defmt::Format> defmt::Format for ByteWrapper<T> {
    fn format(&self, fmt: defmt::Formatter) {
        match self {
            // For standard values, delegate to the Debug implementation of the inner type.
            Self::Standard(v) => defmt::Format::format(v, fmt),
            Self::Extended(v) => defmt::write!(fmt, "Extended({:#04X})", v),
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

/// For a byte enum, generate `TryFrom<u8> -> $enum_name`, `From<$enum_name> -> u8`, and `ByteWrapper` alias with conversions
macro_rules! enum_wrapper {
    ($ns:tt, $enum_name:tt, $enum_wrapper:tt) => {
        $crate::utils::enum_impls!($ns, $enum_name, u8);
        $crate::utils::enum_byte_wrapper!($ns, $enum_name, $enum_wrapper);
    };
    ($ns:tt, $enum_name:tt, $enum_wrapper:tt, display = $($arg:tt)*) => {
        $crate::utils::enum_wrapper!($ns, $enum_name, $enum_wrapper);
        $crate::utils::assert_display_hash!($enum_name, $($arg)*);
    };
}

/// Generate `TryFrom<$typ>` and `From<$enum_name>` implementations for a given enum with its type
macro_rules! enum_impls {
    ($ns:tt, $enum_name:tt, $typ:ty) => {
        impl TryFrom<$typ> for $crate::$ns::$enum_name {
            type Error = &'static str;
            fn try_from(value: $typ) -> Result<Self, Self::Error> {
                $crate::$ns::$enum_name::from_repr(value)
                    .ok_or("Failed to convert enum to numeric value!")
            }
        }

        impl From<$crate::$ns::$enum_name> for $typ {
            fn from(value: $crate::$ns::$enum_name) -> Self {
                value as Self
            }
        }

        #[cfg(test)]
        mod enum_impls_tests {
            #[test]
            fn test_try_from() {
                for value in <$typ>::MIN..=<$typ>::MAX {
                    if let Ok(v) = $crate::$ns::$enum_name::try_from(value) {
                        let enc: $typ = v.into();
                        assert_eq!(value, enc, "{value:#04X} → {v:?} → {enc:#04X}");
                        assert_eq!(
                            $crate::$ns::$enum_name::from_repr(value).unwrap(),
                            v,
                            "{value:#04X} → {v:?}"
                        );
                    }
                }
            }

            #[test]
            #[cfg(feature = "iter")]
            fn test_iter() {
                use ::strum::IntoEnumIterator as _;

                assert_ne!(0, $crate::$ns::$enum_name::iter().count());
                for value in $crate::$ns::$enum_name::iter() {
                    let enc: $typ = value.into();
                    assert_eq!(value, $crate::$ns::$enum_name::try_from(enc).unwrap());
                }
            }
        }
    };
}

/// Generate `ByteWrapper` alias and conversions for a given enum
macro_rules! enum_byte_wrapper {
    ($ns:tt, $enum_name:tt, $enum_wrapper:tt) => {
        #[doc = concat!("Store a single byte, either as a `Standard(", stringify!($enum_name), ")`, or as an `Extended(u8)`.")]
        pub type $enum_wrapper = $crate::ByteWrapper<$enum_name>;

        impl From<$crate::$ns::$enum_name> for $crate::$ns::$enum_wrapper {
            fn from(value: $crate::$ns::$enum_name) -> Self {
                Self::Standard(value)
            }
        }

        // Implementing it as part of the macro because from_repr is not part of a trait
        // https://github.com/Peternator7/strum/issues/251
        impl From<u8> for $crate::ByteWrapper<$crate::$ns::$enum_name> {
            fn from(value: u8) -> Self {
                match $crate::$ns::$enum_name::from_repr(value) {
                    Some(v) => $crate::ByteWrapper::Standard(v),
                    None => $crate::ByteWrapper::Extended(value),
                }
            }
        }

        #[cfg(test)]
        mod enum_byte_wrapper_tests {
            #[test]
            #[allow(non_snake_case)]
            fn test_from() {
                for value in 0x00_u8..=0xFF {
                    let v = $crate::$ns::$enum_wrapper::from(value);
                    let enc: u8 = v.into();
                    assert_eq!(value, enc, "Wrapped {value:#04X} → {v:?} → {enc:#04X}");
                }
            }
        }
    };
}

/// Generates a python wrapper test
macro_rules! python_test {
    ($ns:ident, $enum_name:ident, $val1:ident) => {
        $crate::utils::python_test!($ns, $enum_name, $val1, $val1 =>
            format!("assert a == enm.{}
                       assert a == b
                    ", stringify!($val1)));
    };

    ($ns:ident, $enum_name:ident, $val1:ident, $val2:ident) => {
        $crate::utils::python_test!($ns, $enum_name, $val1, $val2 =>
            format!("assert a == enm.{}
                       assert b == enm.{}
                       assert a != b
                    ", stringify!($val1), stringify!($val2)));
    };

    ($ns:ident, $enum_name:ident, $val1:ident, $val2:ident => $code:expr) => {
        #[cfg(all(test, feature = "pyo3"))]
        mod enum_python_tests {
            use pyo3::{Py, Python};

            #[test]
            fn test_py() {
                Python::initialize();
                Python::attach(|py| {
                    let a = Py::new(py, $crate::$ns::$enum_name::$val1).unwrap();
                    let b = Py::new(py, $crate::$ns::$enum_name::$val2).unwrap();
                    let enm = py.get_type::<$crate::$ns::$enum_name>();
                    pyo3::py_run!(py, a b enm, &$code)
                })
            }
        }
    };
}

/// Compute the combined value of all `Display` representations of all enum variants for hashing purposes.
#[cfg(all(test, feature = "std", feature = "iter", feature = "display"))]
pub(crate) fn calc_display_hash<T: strum::IntoEnumIterator + std::fmt::Display>() -> u64 {
    use std::hash::Hasher as _;

    let mut hasher = xxhash_rust::xxh3::Xxh3::new();
    for val in T::iter() {
        hasher.write(format!("{val}").as_bytes());
    }
    hasher.finish()
}

/// Assert that the display hash of an enum matches the expected value.
macro_rules! assert_display_hash {
    ($enm:ty, $($arg:tt)*) => {
        #[cfg(all(test, feature = "std", feature = "iter", feature = "display"))]
        mod display_hash_tests {
            use super::*;
            use insta::assert_debug_snapshot;
            use crate::utils::calc_display_hash;

            #[test]
            fn test_display_hash() {
                assert_debug_snapshot!(calc_display_hash::<$enm>(), $($arg)*);
            }
        }
    };
}

pub(crate) use {assert_display_hash, enum_byte_wrapper, enum_impls, enum_wrapper, python_test};
