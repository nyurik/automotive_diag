# automotive_diag

[![GitHub](https://img.shields.io/badge/github-nyurik/automotive_diag-8da0cb?logo=github)](https://github.com/nyurik/automotive_diag)
[![crates.io version](https://img.shields.io/crates/v/automotive_diag.svg)](https://crates.io/crates/automotive_diag)
[![docs.rs docs](https://docs.rs/automotive_diag/badge.svg)](https://docs.rs/automotive_diag)
[![CI build](https://github.com/nyurik/automotive_diag/workflows/CI/badge.svg)](https://github.com/nyurik/automotive_diag/actions)

This crate provides low-level no_std structs and enums of the [Unified Diagnostic Services](https://en.wikipedia.org/wiki/Unified_Diagnostic_Services) specification for the road vehicles iso-14229-1 in Rust.

## Usage
All values are presented as Rust `enum`, and can be converted to/from their underlying numeric values using the `From<T>` and `TryFrom<u8>` traits.  Additionally, there is a `ByteWrapper<T>` enum to handle the non-standard `Extended(u8)` values in addition to the recognized `Standand(T)` ones.

```rust
use automotive_diag::ByteWrapper::{Extended, Standard};
use automotive_diag::uds::UdsCommand::{DiagnosticSessionControl, ECUReset};
use automotive_diag::uds::UdsCommandByte;

/// Handle a single command byte on the ECU side
fn handle_cmd_byte(cmd: u8) {
    match UdsCommandByte::from(cmd) {
        Standard(DiagnosticSessionControl) => {
            // handle_diag_session()
        },
        Standard(ECUReset) => {
            // handle_ecu_reset()
        },
        Extended(0x42) => {
            // handle_custom_cmd_42()
        },
        _ => {
            // handle all other commands
        }
    }
}
```

Additionally, support [KWP2000](https://en.wikipedia.org/wiki/Keyword_Protocol_2000) and [OBD-II](https://en.wikipedia.org/wiki/On-board_diagnostics) protocols.

## Credits
The code was forked from the amazing [rnd-ash/ecu_diagnostics](https://github.com/rnd-ash/ecu_diagnostics) project. The code was forked from the last MIT-versioned code before the MIT to GPL license migration.  Initially, this code was developed as a deprecated [auto_uds](https://crates.io/crates/auto_uds) crate.
