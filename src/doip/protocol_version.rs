use crate::utils::{enum_wrapper, python_test};

enum_wrapper!(
    doip,
    ProtocolVersion,
    ProtocolVersionByte,
    display = @"15161726968649792658");
python_test!(doip, ProtocolVersion, V2010, V2012);

/// Available version of the `DoIP` protocol as per ISO-13400.
///
/// Maps to `u8` values for available `DoIP` protocols which are supported by this
/// crate and ISO-13400.
#[repr(u8)]
#[derive(strum::FromRepr, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "display", derive(displaydoc::Display))]
#[cfg_attr(feature = "iter", derive(strum::EnumIter))]
#[cfg_attr(feature = "pyo3", pyo3::pyclass(eq, eq_int))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ProtocolVersion {
    // TODO: needed? This seems like a placeholder that shouldn't be used in Rust
    // /// Reserved Version
    // ReservedVer = 0x00,
    /// `DoIP` Payload Version: ISO-13400 2010 Version
    V2010 = 0x01,

    /// `DoIP` Payload Version: ISO-13400 2012 Version
    V2012 = 0x02,

    /// `DoIP` Payload Version: ISO-13400 2019 Version
    V2019 = 0x03,

    /// `DoIP` Payload Version: ISO-13400 `2019_AMD1` Version
    V2019amd1 = 0x04,
    // TODO: needed? This seems like a placeholder that shouldn't be used in Rust
    // /// `DoIP` Payload Version: Default Version
    // DefaultValue = 0xFF,
}

#[cfg(all(test, feature = "serde"))]
mod tests {
    use super::*;

    #[derive(serde::Serialize, serde::Deserialize)]
    struct TestStruct {
        command: ProtocolVersion,
        command_byte: ProtocolVersionByte,
    }

    #[test]
    fn test_serde() {
        let test = TestStruct {
            command: ProtocolVersion::V2019,
            command_byte: ProtocolVersionByte::from(ProtocolVersion::V2019),
        };

        let json = serde_json::to_string(&test).unwrap();
        assert_eq!(
            json,
            r#"{"command":"V2019","command_byte":{"Standard":"V2019"}}"#
        );

        let deserialized: TestStruct = serde_json::from_str(&json).unwrap();

        assert_eq!(test.command, deserialized.command);
        assert_eq!(test.command_byte, deserialized.command_byte);
    }
}
