# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.26](https://github.com/oxibus/automotive_diag/compare/v0.1.25...v0.1.26) - 2025-12-12

### Other

All part of ([#67](https://github.com/oxibus/automotive_diag/pull/67)):
- include `doip` feature as part of the default
- minor cleanup of the justfile recipes
- bump pyo3 dep version
- bump MSRV to 1.74

## [0.1.25](https://github.com/oxibus/automotive_diag/compare/v0.1.24...v0.1.25) - 2025-10-10

### Other

- upgrade to pyo3 0.26 ([#63](https://github.com/oxibus/automotive_diag/pull/63))

## [0.1.24](https://github.com/oxibus/automotive_diag/compare/v0.1.23...v0.1.24) - 2025-10-01

### Other

- revert to RELEASE_PLZ_TOKEN ([#61](https://github.com/oxibus/automotive_diag/pull/61))
- use github tokens in release ([#59](https://github.com/oxibus/automotive_diag/pull/59))

## [0.1.23](https://github.com/nyurik/automotive_diag/compare/v0.1.22...v0.1.23) - 2025-09-19

### Other

- migrate this project to [OxiBUS](https://github.com/oxibus) GitHub organization
- minor CI cleanups and improvements

## [0.1.22](https://github.com/oxibus/automotive_diag/compare/v0.1.21...v0.1.22) - 2025-08-16

### Other

- Bump actions/checkout from 4 to 5 in the all-actions-version-updates group ([#53](https://github.com/oxibus/automotive_diag/pull/53))
- [pre-commit.ci] pre-commit autoupdate ([#52](https://github.com/oxibus/automotive_diag/pull/52))
- *(ci)* format Cargo.toml ([#51](https://github.com/oxibus/automotive_diag/pull/51))

## [0.1.21](https://github.com/oxibus/automotive_diag/compare/v0.1.20...v0.1.21) - 2025-06-12

### Fixed

- ensure 2-digit hex formatting ([#50](https://github.com/oxibus/automotive_diag/pull/50)), reported by @ColinFinck ([#49](https://github.com/oxibus/automotive_diag/issues/49))

### Other

- use release-plz token in dependabot ci

## [0.1.20](https://github.com/oxibus/automotive_diag/compare/v0.1.19...v0.1.20) - 2025-06-08

### Other

- remove default ci perms
- update deps on release

## [0.1.19](https://github.com/oxibus/automotive_diag/compare/v0.1.18...v0.1.19) - 2025-06-08

### Other

- improve release-plz CI process

## [0.1.18](https://github.com/oxibus/automotive_diag/compare/v0.1.17...v0.1.18) - 2025-06-06

- refactored CI, enabled release automation
