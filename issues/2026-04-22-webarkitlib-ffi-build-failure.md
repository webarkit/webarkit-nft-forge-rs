## Summary
Cannot build `webarkitlib-rs` with `ffi-backend` from crates.io because C++ dependencies are missing and require Python bootstrapping.

## Environment
- **Product/Service**: webarkitlib-rs
- **Region/Version**: v0.3.3
- **Browser/OS**: Cross-platform (Cargo build)

## Reproduction Steps
1. Add `webarkitlib-rs = { version = "0.3.3", features = ["ffi-backend"] }` to a project's `Cargo.toml`.
2. Run `cargo build`.

## Expected Behavior
The crate should compile and link the C++ dependencies successfully via `build.rs`.

## Actual Behavior
Compilation fails in `cc-rs` because the C++ files are missing from the downloaded crate. The `build.rs` expects files at the relative path `../../benchmarks/c_benchmark/src/WebARKitLib`. 
**Note:** The user cannot build the `WebARKitLib` lib natively through Cargo because it needs to be bootstrapped with Python (via `python bootstrap.py`), which Cargo does not execute automatically when resolving dependencies.

## Error Details
```
error occurred in cc-rs: command did not execute successfully (status code exit code: 2): "cl.exe" ... "-c" "...\benchmarks\c_benchmark\src\WebARKitLib\lib\SRC\KPM\FreakMatcher\facade\visual_database_facade.cpp"
```

## Visual Evidence
N/A

## Impact
**High** - Breaks the `ffi-backend` feature for any consumer downloading the crate from `crates.io` or as a git dependency, preventing compilation without manual intervention.

## Additional Context
**Proposed Solution**: Modify `webarkitlib-rs/crates/core/build.rs` to automatically download and extract the required `WebARKitLib` C++ dependencies into Cargo's `OUT_DIR` using standard Rust networking crates (like `ureq` and `tar`/`flate2`). This eliminates the need for Python bootstrapping and ensures the crate builds seamlessly out of the box for users.
