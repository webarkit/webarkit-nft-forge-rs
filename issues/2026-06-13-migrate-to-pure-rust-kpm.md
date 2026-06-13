# Issue: Migrate to Pure Rust KPM (webarkitlib-rs v0.7.0)

## Summary
Migrate `webarkit-nft-forge-rs` to `webarkitlib-rs` v0.7.0 to utilize the newly available pure-Rust KPM (Keypoint Matching) implementation.

## Context
Previously, generating `.fset3` files required the C++ FFI-based FreakMatcher backend (`ffi-backend` feature). This created a heavy dependency chain involving C++ compilers, Python bootstrapping scripts, and manual configuration, which frequently caused build failures on standard environments (especially on Windows).

With `webarkitlib-rs` v0.7.0, KPM is fully implemented in Rust. We should update the project to:
1. Use `webarkitlib-rs = "0.7.0"`.
2. Remove the FFI-only conditional compilation around `.fset3` generation in `src/generate.rs`.
3. Switch default marker generation to use the pure Rust matcher.
4. Clean up any FFI-backend configurations if no longer needed.

## Expected Behavior
- The project should build seamlessly out-of-the-box using standard Cargo commands without requiring external C++ tools or Python.
- GUI marker generation should generate all three files: `.iset`, `.fset`, and `.fset3` using the native Rust backend.

## Proposed Steps
1. Create a GitHub issue.
2. Branch off to `feat/pure-rust-kpm`.
3. Modify dependencies in `Cargo.toml`.
4. Update `src/generate.rs` to instantiate and use the Rust-native FreakMatcher.
5. Verify build, run checks, and test marker generation manually.
6. Push changes and submit a PR.
