#!/usr/bin/env just --justfile

@_default:
    just --list --unsorted

# Clean all build artifacts
clean:
    cargo clean
    rm -f Cargo.lock

update:
    cargo +nightly -Z unstable-options update --breaking
    cargo update

# Run cargo clippy
clippy:
    cargo clippy --workspace --bins --tests --lib --benches --examples -- -D warnings
    cargo clippy --no-default-features --features with-uds -- -D warnings

# Test code formatting
test-fmt:
    cargo fmt --all -- --check

# Run cargo fmt
fmt:
    cargo +nightly fmt -- --config imports_granularity=Module,group_imports=StdExternalCrate

# Build and open code documentation
docs:
    cargo doc --no-deps --open

# Quick compile
check:
    RUSTFLAGS='-D warnings' cargo check
    RUSTFLAGS='-D warnings' cargo check --no-default-features --features with-uds


# Run all tests
test:
    RUSTFLAGS='-D warnings' cargo test
    RUSTFLAGS='-D warnings' cargo test --no-default-features --features with-kwp2000
    RUSTFLAGS='-D warnings' cargo test --no-default-features --features with-obd2
    RUSTFLAGS='-D warnings' cargo test --no-default-features --features with-uds
    RUSTFLAGS='-D warnings' cargo test --features serde

# Test documentation
test-doc:
    cargo test --doc
    RUSTDOCFLAGS="-D warnings" cargo doc --no-deps

rust-info:
    rustc --version
    cargo --version

# Run all tests as expected by CI
ci-test: rust-info test-fmt clippy check test test-doc

# Run minimal subset of tests to ensure compatibility with MSRV
ci-test-msrv: rust-info check test
