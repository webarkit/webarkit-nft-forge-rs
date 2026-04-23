# WebARKit nft forge - Rust edition
This project is a Rust implementation of a high-performance NFT Marker Creator for WebARKit. It provides tools to generate NFT markers that can be used in augmented reality applications built with WebARKit. It can generate NFT markers from images and export them in a format compatible with a wide range of AR frameworks, based on WebARKitLib, ARtoolKit5, ARToolKitX including ARnft, JsartoolkitNFT, Jsartoolkit5, and AR.js, allowing developers to easily integrate AR experiences into their web applications. It is in active development and it depends on the webarkitlib-rs crate for core NFT marker generation functionality. The project also includes a GUI built with eframe and egui_extras for user-friendly interaction. The application is designed to be cross-platform, running on Windows, macOS, and Linux.

The package now ships both as:

- a desktop application binary
- a reusable Rust library

## Prerequisites

This project uses a C++ back-end (via `webarkitlib-rs`) for feature extraction. To build it, you need:

- **Rust**: Latest stable version.
- **C++ Toolchain**: A C++17 compatible compiler (MSVC on Windows, Clang/GCC on macOS/Linux).
- **CMake**: Required for building the FFI bindings.

## Installation

### Build instructions

1. Clone the repository:
   ```bash
   git clone https://github.com/webarkit/webarkit-nft-forge-rs.git
   cd webarkit-nft-forge-rs
   ```

2. Build the project using Cargo. You can use the `ffi-backend` feature for C++ marker generation:
   ```bash
   cargo build --release --features ffi-backend
   ```
   *Optional features:* You can also enable SIMD optimizations and logging helpers by passing additional features:
   ```bash
   cargo build --release --features ffi-backend,simd,log-helpers
   ```

3. Run the application:
   ```bash
   cargo run --release --features ffi-backend
   ```
   Or with all features:
   ```bash
   cargo run --release --features ffi-backend,simd,log-helpers
   ```

## Usage

### GUI Application

The application provides a user-friendly interface for generating NFT markers:

1. **Select Image**: Click "📁 Select Image" to choose a source JPG or PNG file.
2. **Output Directory**: Optionally select where to save the markers. Defaults to the current directory.
3. **Marker Name**: Provide a semantic name for your marker files.
4. **DPI Setting**: Use the slider to set the source image DPI (default 72, range 72-600).
5. **Generate**: Click "🚀 Generate Marker". The process runs in the background, and you can monitor progress via the status bar.

The tool generates three files per marker:
- `.iset`: Image set metadata.
- `.fset`: Feature set data.
- `.fset3`: KPM/FREAK feature data (required for NFT).

### Library

You can also use the core functionality as a library in your own Rust projects:

```rust
use std::sync::Arc;
use std::sync::atomic::AtomicU32;
use webarkit_nft_forge_rs::generate_nft_marker;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let image_data = std::fs::read("input.jpg")?;
    let output_dir = std::path::Path::new("./output");
    let progress = Arc::new(AtomicU32::new(0));

    generate_nft_marker(
        &image_data,
        1920, 1080, 3, // width, height, channels
        output_dir,
        "my_marker",
        72.0, // DPI
        Some(progress)
    )?;
    
    Ok(())
}
```

## License

This project is licensed under the [LGPL-3.0-or-later](LICENSE).