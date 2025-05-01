crate::utils::enum_wrapper!(
    doip,
    DoIpProtocolVersion,
    DoIpProtocolVersionByte,
    display = @"12259587059646726960");

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
// TODO: should this be DoIp-prefixed?
pub enum DoIpProtocolVersion {
    // TODO: needed? This seems like a placeholder that shouldn't be used in Rust
    // /// Reserved Version
    // ReservedVer = 0x00,
    /// `DoIP` Payload Version: ISO-13400 2010 Version
    Iso13400_2010 = 0x01,

    /// `DoIP` Payload Version: ISO-13400 2012 Version
    Iso13400_2012 = 0x02,

    /// `DoIP` Payload Version: ISO-13400 2019 Version
    Iso13400_2019 = 0x03,

    /// `DoIP` Payload Version: ISO-13400 `2019_AMD1` Version
    Iso13400_2019Amd1 = 0x04,

    /// `DoIP` Payload Version: Default Version
    DefaultValue = 0xFF,
}

#[cfg(all(test, feature = "serde"))]
mod tests {
    use super::*;

    #[derive(serde::Serialize, serde::Deserialize)]
    struct TestStruct {
        command: DoIpProtocolVersion,
        command_byte: DoIpProtocolVersionByte,
    }

    #[test]
    fn test_serde() {
        let test = TestStruct {
            command: DoIpProtocolVersion::Iso13400_2019,
            command_byte: DoIpProtocolVersionByte::from(DoIpProtocolVersion::Iso13400_2019),
        };

        let json = serde_json::to_string(&test).unwrap();
        assert_eq!(
            json,
            r#"{"command":"Iso13400_2019","command_byte":{"Standard":"Iso13400_2019"}}"#
        );

        let deserialized: TestStruct = serde_json::from_str(&json).unwrap();

        assert_eq!(test.command, deserialized.command);
        assert_eq!(test.command_byte, deserialized.command_byte);
    }
}
