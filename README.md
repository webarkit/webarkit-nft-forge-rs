# WebARKit nft forge - Rust edition
This project is a Rust implementation of a high-performance NFT Marker Creator for WebARKit. It provides tools to generate NFT markers that can be used in augmented reality applications built with WebARKit. It can generate NFT markers from images and export them in a format compatible with a wide range of AR frameworks, based on WebARKitLib, ARtoolKit5, ARToolKitX including ARnft, JsartoolkitNFT, Jsartoolkit5, and AR.js, allowing developers to easily integrate AR experiences into their web applications. It is in active development and it depends on the webarkitlib-rs crate for core NFT marker generation functionality. The project also includes a GUI built with eframe and egui_extras for user-friendly interaction. The application is designed to be cross-platform, running on Windows, macOS, and Linux.

The package now ships both as:

- a desktop application binary
- a reusable Rust library

## Installation

### Build instructions
1. Clone the repository:
   ```bash
   git clone <repository-url>
   cd webarkit-nft-forge-rs
   ```
2. Build the project using Cargo:
   ```bash
   cargo build --release
   ``` 
3. Run the application:
   ```bash
   ./target/release/webarkit-nft-forge-rs
    ```

## Usage

### Application

1. Launch the application.  
2. Use the GUI to select an image file that you want to convert into an NFT marker.
3. Click the "Generate" button to create the NFT marker.
4. The generated NFT marker will be created by the application, and you can save it to your desired location.

### Library

Use the crate from Rust code by calling the exported API:

```rust
use webarkit_nft_forge_rs::generate_nft_marker;

fn main() -> Result<(), Box<dyn std::error::Error>> {
   let image_data = std::fs::read("marker.jpg")?;
   let marker = generate_nft_marker(&image_data)?;
   println!("Generated {} bytes", marker.len());
   Ok(())
}
```

## Publishing notes

This package is structured to publish as both a library crate and a binary crate. Before publishing to crates.io, add a valid `license` or `license-file` entry to `Cargo.toml`.