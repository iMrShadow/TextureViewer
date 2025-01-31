use block_compression::{decode, BC6HSettings, BC7Settings};

use super::{
    crate_directxtex_utility::decompress_bc_to_rgba,
    pixel_decoders::{
        decode_bgr565_pixel, decode_bgra5551_pixel, decode_bgra8888_pixel, decode_l16_pixel,
        decode_rgb888_pixel,
    },
    pixel_format::{ColorSpace, PixelDataType, PixelFormat},
};

use super::pixel_decoders::{decode_l8_pixel, decode_la88_pixel};

pub struct PixelDecoder;

impl PixelDecoder {
    pub fn decode(
        data: &[u8],
        format: PixelFormat,
        data_type: PixelDataType,
        color_space: ColorSpace,
        width: u32,
        height: u32,
    ) -> Result<Vec<u8>, String> {
        match format {
            PixelFormat::R1 => {
                // Convert R1 to RGBA8
                let mut output = Vec::with_capacity(data.len() * 8 * 4); // Each 8-bit input produces 4 bytes

                //  pixel_decoders::decode_rgb161616_pixel(data, &mut output);

                Ok(output)
            }

            PixelFormat::L8 | PixelFormat::R8 => {
                // Convert L8/R8 to RGBA8
                let mut output = Vec::with_capacity(data.len() * 4);
                for &pixel in data {
                    output.extend_from_slice(&decode_l8_pixel(pixel));
                }

                Ok(output)
            }

            PixelFormat::LA8 | PixelFormat::RG8 => {
                let mut output = Vec::with_capacity((data.len() / 2) * 4);
                for chunk in data.chunks_exact(2) {
                    let pixel = u16::from_le_bytes([chunk[0], chunk[1]]);
                    output.extend_from_slice(&decode_la88_pixel(pixel));
                }
                Ok(output)
            }

            PixelFormat::L16 | PixelFormat::R16 => {
                let mut output = Vec::with_capacity((data.len() / 2) * 4);
                for chunk in data.chunks_exact(2) {
                    let pixel = u16::from_le_bytes([chunk[0], chunk[1]]);
                    output.extend_from_slice(&decode_l16_pixel(pixel));
                }
                Ok(output)
            }

            PixelFormat::B5G5R5X1 | PixelFormat::B5G5R5A1 => {
                let mut output = Vec::with_capacity((data.len() / 2) * 4);
                for chunk in data.chunks_exact(2) {
                    let pixel = u16::from_le_bytes([chunk[0], chunk[1]]);
                    output.extend_from_slice(&decode_bgra5551_pixel(pixel));
                }
                Ok(output)
            }

            PixelFormat::RGBA8 => {
                // Direct passthrough for RGBA8_UNORM
                Ok(data.to_vec())
            }

            PixelFormat::B5G6R5 => {
                let mut output = Vec::with_capacity((data.len() / 2) * 4);
                for chunk in data.chunks_exact(2) {
                    let pixel = u16::from_le_bytes([chunk[0], chunk[1]]);
                    output.extend_from_slice(&decode_bgr565_pixel(pixel));
                }

                Ok(output)
            }

            PixelFormat::BGRA8 => {
                let mut output = Vec::with_capacity(data.len());
                for chunk in data.chunks_exact(4) {
                    output.extend_from_slice(&decode_bgra8888_pixel(u32::from_le_bytes([
                        chunk[0], chunk[1], chunk[2], chunk[3],
                    ])));
                }
                Ok(output)
            }

            PixelFormat::BC1
            | PixelFormat::BC2
            | PixelFormat::BC3
            | PixelFormat::BC4
            | PixelFormat::BC5
            | PixelFormat::BC6H
            | PixelFormat::BC7 => {
                decompress_bc_to_rgba(format, width as usize, height as usize, data.to_vec())
            }

            PixelFormat::R8 => {
                // Convert R8 to RGBA8
                let mut output = Vec::with_capacity(data.len() * 4);

                Ok(output)
            }

            PixelFormat::LA8 => {
                // Convert LA8 to RGBA8
                let mut output = Vec::with_capacity(data.len() * 2);

                Ok(output)
            }

            PixelFormat::RGB8 => {
                // Convert RGB8 to RGBA8
                let mut output = Vec::with_capacity((data.len() / 3) * 4);

                for chunk in data.chunks_exact(3) {
                    output.extend_from_slice(&decode_rgb888_pixel(&[chunk[0], chunk[1], chunk[2]]));
                }

                Ok(output)
            }

            _ => Err(format!("Unsupported format: {:?}", format)),
        }
    }
}

// fn decompress_bc_blocks_as_rgba8(
//     bc_variant: block_compression::CompressionVariant,
//     width: u32,
//     height: u32,
//     data: &[u8],
// ) -> Result<Vec<u8>, String> {
//     let decompressed_size = (width * height * 4) as usize;

//     let mut output = vec![0u8; decompressed_size];
//     block_compression::decode::decompress_blocks_as_rgba8(
//         bc_variant,
//         width,
//         height,
//         data,
//         &mut output,
//     );

//     Ok(output)
// }
