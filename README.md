# 🛠️ WebARKit NFT Forge - Rust Edition

This project is a **pure Rust implementation** (with optional legacy C++ FFI fallback) of a high-performance **NFT Marker Creator** for WebARKit. It provides powerful tools to generate Natural Feature Tracking (NFT) markers that can be used in augmented reality applications.

✨ **Key Features:**
- **Universal Compatibility**: Generates NFT markers from images and exports them in a format compatible with a wide range of AR frameworks based on **WebARKitLib**, **ARtoolKit5**, and **ARToolKitX** (including *ARnft*, *JsartoolkitNFT*, *Jsartoolkit5*, and *AR.js*).
- **Core Reliability**: Built on top of the robust [`webarkitlib-rs`](https://github.com/webarkit/WebARKitLib-rs) crate for reliable core marker generation functionality.
- **Friendly GUI**: Includes an intuitive graphical interface built with `eframe` and `egui_extras` for user-friendly interaction.
- **Cross-Platform**: Designed to run seamlessly on **Windows**, **macOS**, and **Linux**.

🚀 The package now ships both as:

- 🖥️ A standalone **desktop application binary**
- 📦 A **reusable Rust library**

## 📋 Prerequisites

By default, the project compiles and runs in **pure Rust** and only requires:

- 🦀 **Rust**: Latest stable version.

If you explicitly want to use the legacy C++ backend (via the `ffi-backend` feature), you will also need:
- ⚙️ **C++ Toolchain**: A C++17 compatible compiler (MSVC on Windows, Clang/GCC on macOS/Linux).
- 🛠️ **CMake**: Required for building the FFI bindings.

## 💻 Installation

### 🔨 Build instructions

1. Clone the repository:
   ```bash
   git clone https://github.com/webarkit/webarkit-nft-forge-rs.git
   cd webarkit-nft-forge-rs
   ```

2. Build the project using Cargo. By default, this builds the **pure Rust** KPM feature extractor:
   ```bash
   cargo build --release
   ```

   *Legacy FFI Backend:* If you wish to build with the C++ FFI backend instead:
   ```bash
   cargo build --release --features ffi-backend
   ```

   *Optional features:* You can also enable SIMD optimizations and logging helpers:
   ```bash
   cargo build --release --features simd,log-helpers
   ```

3. Run the application:
   ```bash
   cargo run --release
   ```
   Or with the legacy FFI backend:
   ```bash
   cargo run --release --features ffi-backend
   ```

## 🕹️ Usage

### 🖼️ GUI Application

The application provides a user-friendly interface for generating NFT markers:

1. 🖼️ **Select Image**: Click "📁 Select Image" to choose a source JPG or PNG file.
2. 📁 **Output Directory**: Optionally select where to save the markers. Defaults to the current directory.
3. 🏷️ **Marker Name**: Provide a semantic name for your marker files.
4. 🎚️ **DPI Setting**: Use the slider to set the source image DPI (default 72, range 72-600).
5. 🚀 **Generate**: Click "🚀 Generate Marker". The process runs in the background, and you can monitor progress via the status bar.

The tool generates three files per marker:
- 📄 `.iset`: Image set metadata.
- 📉 `.fset`: Feature set data.
- 🧩 `.fset3`: KPM/FREAK feature data (required for NFT).

### 📚 Library

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

## ⚖️ License

This project is licensed under the [LGPL-3.0-or-later](LICENSE).