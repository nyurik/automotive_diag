# Automotive diagnostics in Rust

[![GitHub repo](https://img.shields.io/badge/github-oxibus/automotive_diag-8da0cb?logo=github)](https://github.com/oxibus/automotive_diag)
[![crates.io version](https://img.shields.io/crates/v/automotive_diag)](https://crates.io/crates/automotive_diag)
[![crate usage](https://img.shields.io/crates/d/automotive_diag)](https://crates.io/crates/automotive_diag)
[![docs.rs status](https://img.shields.io/docsrs/automotive_diag)](https://docs.rs/automotive_diag)
[![crates.io license](https://img.shields.io/crates/l/automotive_diag)](https://github.com/oxibus/automotive_diag/blob/main/LICENSE-APACHE)
[![CI build status](https://github.com/oxibus/automotive_diag/actions/workflows/ci.yml/badge.svg)](https://github.com/oxibus/automotive_diag/actions)
[![Codecov](https://img.shields.io/codecov/c/github/oxibus/automotive_diag)](https://app.codecov.io/gh/oxibus/automotive_diag)

This crate provides low-level `no_std` structs and enums of
the [Unified Diagnostic Services](https://en.wikipedia.org/wiki/Unified_Diagnostic_Services) (ISO-14229-1),
[KWP2000](https://en.wikipedia.org/wiki/Keyword_Protocol_2000) (ISO-142330),
[OBD-II](https://en.wikipedia.org/wiki/On-board_diagnostics) (ISO-9141)
and [DoIP](https://automotivevehicletesting.com/vehicle-diagnostics/doip/) (ISO-13400) diagnostic
specifications for the road vehicles in Rust. Optionally supports `defmt`, `serde`, and `pyo3`. See features section in the `Cargo.toml` file.

Most features have been implemented and documented to handle the most common diagnostic commands and responses.
If you find a missing feature, please open an issue or better yet, a pull request.

The long-term goal is to provide all core functionality for message definitions and (de-)serialization, supporting both the `no_std` and `std` environments.

## Usage

All values are presented as Rust `enum`, and can be converted to/from their underlying numeric values using
the `T::from_repr(u8)` and `u8::from(value)`. Most enums also have a corresponding `...Byte` enums as
`ByteWrapper<T>` to
handle the non-standard `Extended(u8)` values in addition to the defined `Standand(T)` ones.

```rust
use automotive_diag::ByteWrapper::{Extended, Standard};
use automotive_diag::uds::UdsCommand::{DiagnosticSessionControl, ECUReset};
use automotive_diag::uds::UdsCommandByte;

/// Handle a single command byte on the ECU side
fn handle_cmd_byte(cmd: u8) {
    match UdsCommandByte::from(cmd) {
        Standard(DiagnosticSessionControl) => {
            // handle_diag_session()
        }
        Standard(ECUReset) => {
            // handle_ecu_reset()
        }
        Extended(0x42) => {
            // handle_custom_cmd_42()
        }
        _ => {
            // handle all other commands
        }
    }
}
```

## Development

* This project is easier to develop with [just](https://github.com/casey/just#readme), a modern alternative to `make`.
  Install it with `cargo install just`.
* To get a list of available commands, run `just`.
* To run tests, use `just test`.

## Credits

The code was forked from the amazing [rnd-ash/ecu_diagnostics](https://github.com/rnd-ash/ecu_diagnostics) project. The
code was forked from the last MIT-versioned code before the MIT to GPL license migration. Initially, this code was
developed as a deprecated [auto_uds](https://crates.io/crates/auto_uds) crate. The `DoIP` definitions were adapted from
the MIT-licensed [samp-reston/doip-definitions](https://github.com/samp-reston/doip-definitions) project.

## License

Licensed under either of

* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <https://www.apache.org/licenses/LICENSE-2.0>)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or <https://opensource.org/licenses/MIT>)
  at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the
Apache-2.0 license, shall be dual-licensed as above, without any
additional terms or conditions.
