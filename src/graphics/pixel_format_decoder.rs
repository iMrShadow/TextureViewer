use super::{
    crate_directxtex_utility::DirectXTexUtility,
    pixel_decoders::{decode_bgr888_pixel, decode_rgb888_pixel},
    pixel_format::PixelFormat,
};

pub struct PixelDecoder;

impl PixelDecoder {
    pub fn decode(
        pixels: &[u8],
        format: PixelFormat,
        width: u32,
        height: u32,
    ) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        match format {
            PixelFormat::R8G8B8A8 => {
                // No conversion needed
                Ok(pixels.to_vec())
            }

            PixelFormat::R8G8B8 => {
                // Convert RGB8 to RGBA8
                let mut output = Vec::with_capacity((pixels.len() / 3) * 4);

                for chunk in pixels.chunks_exact(3) {
                    output.extend_from_slice(&decode_rgb888_pixel(&[chunk[0], chunk[1], chunk[2]]));
                }

                Ok(output)
            }

            PixelFormat::B8G8R8 => {
                // Convert BGR8 to RGBA8
                let mut output = Vec::with_capacity((pixels.len() / 3) * 4);

                for chunk in pixels.chunks_exact(3) {
                    output.extend_from_slice(&decode_bgr888_pixel(&[chunk[0], chunk[1], chunk[2]]));
                }

                Ok(output)
            }

            PixelFormat::R1
            | PixelFormat::L8
            | PixelFormat::R8
            | PixelFormat::L16
            | PixelFormat::R16
            | PixelFormat::R8G8
            | PixelFormat::L8A8
            | PixelFormat::R16G16
            | PixelFormat::R9G9B9E5
            | PixelFormat::B4G4R4A4
            | PixelFormat::B5G6R5
            | PixelFormat::R11G11B10
            | PixelFormat::R10G10B10A2
            | PixelFormat::B5G5R5A1
            | PixelFormat::B5G5R5X1
            | PixelFormat::B8G8R8X8
            | PixelFormat::B8G8R8A8
            | PixelFormat::R16G16B16A16
            | PixelFormat::R32G32B32
            | PixelFormat::R32G32B32A32 => {
                // Convert from other formats using DirectXTex
                let output = DirectXTexUtility::convert_dxgi_to_rgba(
                    format,
                    width as usize,
                    height as usize,
                    pixels.to_vec(),
                )?;
                Ok(output)
            }

            PixelFormat::BC1
            | PixelFormat::BC2
            | PixelFormat::BC3
            | PixelFormat::BC4
            | PixelFormat::BC5
            | PixelFormat::BC6H
            | PixelFormat::BC7 => {
                // Convert from BC formats using DirectXTex
                let output = DirectXTexUtility::decompress_bc_to_rgba(
                    format,
                    width as usize,
                    height as usize,
                    pixels.to_vec(),
                )?;
                Ok(output)
            }

            _ => Err("Unsupported pixel format".into()),
        }
    }
}
