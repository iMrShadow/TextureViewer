use super::pixel_format::{ColorSpace, PixelDataType, PixelFormat};

pub struct PixelEncoder;

impl PixelEncoder {
   pub fn encode(&self, pixels: &[f32], format: PixelFormat, data_type: PixelDataType, color_space: ColorSpace) -> Result<Vec<u8>, String> {
        match format {
            PixelFormat::RGBA8 => {
                // Example: Encode normalized floats into RGBA8_UNORM
                if pixels.len() % 4 != 0 {
                    return Err("Pixel buffer must be divisible by 4 for RGBA8_UNORM".into());
                }
                let encoded: Vec<u8> = pixels
                    .iter()
                    .map(|&v| (v.clamp(0.0, 1.0) * 255.0).round() as u8)
                    .collect();
                Ok(encoded)
            }
            // Add cases for other formats...
            _ => Err(format!("Unsupported format: {:?}", format)),
        }
    }
}