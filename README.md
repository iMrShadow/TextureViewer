# Texture Viewer

A simple texture viewer in Rust using egui.
- Opening various image formats
- Simple editing like rotation, flipping and toggling color channels
- Conversion/compression to different pixel formats
- Ability to preview after compression
- Swizzling/deswizzling for console platforms
- Preview 2D, cubemap and 2D array textures with their mips
- Save into different formats

![Example Screenshot](/assets/screenshot_1.png "Example")

### Features:

#### Texture/Image Formats Support
- DDS (via DirectXTex. decoding and encoding: BC1, BC2, BC3, BC4, BC5 and BC6H. BC7 can be only decoded)
- TGA (decoding via DirectXTex, encoding via image-rs)
- PNG, JPEG, BMP, TIFF, HDR (via image-rs)

#### Platform Support
- Windows
