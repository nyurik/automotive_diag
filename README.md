# auto_uds

[![GitHub](https://img.shields.io/badge/github-nyurik/auto_uds-8da0cb?logo=github)](https://github.com/nyurik/auto_uds)
[![crates.io version](https://img.shields.io/crates/v/auto_uds.svg)](https://crates.io/crates/auto_uds)
[![docs.rs docs](https://docs.rs/auto_uds/badge.svg)](https://docs.rs/auto_uds)
[![CI build](https://github.com/nyurik/auto_uds/workflows/CI/badge.svg)](https://github.com/nyurik/auto_uds/actions)

This crate provides low-level no_std structs and enums of the [Unified Diagnostic Services](https://en.wikipedia.org/wiki/Unified_Diagnostic_Services) specification for the road vehicles iso-14229-1 in Rust.

## Usage
All values are presented as Rust `enum`, and can be converted to/from their underlying numeric values using the `From<T>` and `TryFrom<u8>` traits.  Additionally, there is a `ByteWrapper<T>` enum to handle the non-standard `Extended(u8)` values in addition to the recognized `Standand(T)` ones.

```rust
use auto_uds::ByteWrapper::{Extended, Standard};
use auto_uds::UdsCommand::{DiagnosticSessionControl, ECUReset};
use auto_uds::UdsCommandByte;

/// Handle a single command byte on the ECU side
fn handle_cmd_byte(cmd: u8) {
    match UdsCommandByte::from(cmd) {
        Standard(DiagnosticSessionControl) => {
            // handle_diag_session()
        },
        Standard(ECUReset) => {
            // handle_ecu_reset()
        },
        Extended(0xCF) => {
            // handle_custom_cmd_CF()
        },
        _ => {
            // handle all other commands
        }
    }
}
```

## Credits
The code was forked from the amazing [rnd-ash/ecu_diagnostics](https://github.com/rnd-ash/ecu_diagnostics) project. The code was forked from the last MIT-versioned code before the MIT to GPL license migration.
