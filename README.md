# auto_uds

[![GitHub](https://img.shields.io/badge/github-nyurik/auto_uds-8da0cb?logo=github)](https://github.com/nyurik/auto_uds)
[![crates.io version](https://img.shields.io/crates/v/auto_uds.svg)](https://crates.io/crates/auto_uds)
[![docs.rs docs](https://docs.rs/auto_uds/badge.svg)](https://docs.rs/auto_uds)
[![CI build](https://github.com/nyurik/auto_uds/workflows/CI/badge.svg)](https://github.com/nyurik/auto_uds/actions)

This crate provides low-level no_std structs and enums of the [Unified Diagnostic Services](https://en.wikipedia.org/wiki/Unified_Diagnostic_Services) specification for the road vehicles iso-14229-1 in Rust.

## Usage
All values are presented as Rust `enum`, and can be converted to/from their underlying numeric values using the `From` and `TryFrom` traits.  Additionally, there is a `ByteWrapper<T>` enum that distinguishes between the `Standand(T)` and `Extended(u8)` values.

```rust
use auto_uds::{UdsError, UdsErrorByte};
use auto_uds::UdsError::*;

fn main() {
    assert_eq!(UdsError::try_from(0x10), Ok(GeneralReject));
    assert_eq!(UdsErrorByte::from(0x10), UdsErrorByte::Standard(GeneralReject));

    assert!(UdsError::try_from(0xA0).is_err());
    assert_eq!(UdsErrorByte::from(0xA0), UdsErrorByte::Extended(0xA0));
}
```

## Credits
The code was forked from the amazing [rnd-ash/ecu_diagnostics](https://github.com/rnd-ash/ecu_diagnostics) project. The code was forked from the last MIT-versioned code before the MIT to GPL license migration.
