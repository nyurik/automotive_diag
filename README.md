# automotive_diag

[![GitHub](https://img.shields.io/badge/github-nyurik/automotive_diag-8da0cb?logo=github)](https://github.com/nyurik/automotive_diag)
[![crates.io version](https://img.shields.io/crates/v/automotive_diag.svg)](https://crates.io/crates/automotive_diag)
[![docs.rs docs](https://docs.rs/automotive_diag/badge.svg)](https://docs.rs/automotive_diag)
[![crates.io version](https://img.shields.io/crates/l/automotive_diag.svg)](https://github.com/nyurik/automotive_diag/blob/main/LICENSE-APACHE)
[![CI build](https://github.com/nyurik/automotive_diag/actions/workflows/ci.yml/badge.svg)](https://github.com/nyurik/automotive_diag/actions)

This crate provides low-level no_std structs and enums of the [Unified Diagnostic Services](https://en.wikipedia.org/wiki/Unified_Diagnostic_Services) (ISO-14229-1), [KWP2000](https://en.wikipedia.org/wiki/Keyword_Protocol_2000) (ISO-142330) and [OBD-II](https://en.wikipedia.org/wiki/On-board_diagnostics) (ISO-9141) specifications for the road vehicles in Rust.

## Usage
All values are presented as Rust `enum`, and can be converted to/from their underlying numeric values using the `From<T>` and `TryFrom<u8>` traits.  Most enums also have a corresponding `...Byte` enums as `ByteWrapper<T>` to handle the non-standard `Extended(u8)` values in addition to the defined `Standand(T)` ones.

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

## Development
* This project is easier to develop with [just](https://github.com/casey/just#readme), a modern alternative to `make`. Install it with `cargo install just`.
* To get a list of available commands, run `just`.
* To run tests, use `just test`.
* On `git push`, it will run a few validations, including `cargo fmt`, `cargo clippy`, and `cargo test`.  Use `git push --no-verify` to skip these checks.

## Credits
The code was forked from the amazing [rnd-ash/ecu_diagnostics](https://github.com/rnd-ash/ecu_diagnostics) project. The code was forked from the last MIT-versioned code before the MIT to GPL license migration.  Initially, this code was developed as a deprecated [auto_uds](https://crates.io/crates/auto_uds) crate.

## License

Licensed under either of

* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)
  at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the
Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
