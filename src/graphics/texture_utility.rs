use std::error::Error;

use image::{DynamicImage, ImageBuffer, Rgba};

use super::{
    pixel_format::{ColorSpace, PixelFormat, PixelFormatInfo},
    pixel_format_decoder::PixelDecoder,
    pixel_format_encoder::PixelEncoder,
    swizzling::{swizzle_manager::Swizzler, Platform},
    texture::Image,
};

pub struct TextureEffects {
    pub rotate_90_left_count: u32,
    pub rotate_90_right_count: u32,
    pub flip_horizontal_count: u32,
    pub flip_vertical_count: u32,
    pub channel_filter: (bool, bool, bool, bool),
    pub swizzle: Option<Platform>,
    pub deswizzle: Option<Platform>,
    pub pixel_format: PixelFormat,
}

impl Default for TextureEffects {
    fn default() -> Self {
        Self {
            rotate_90_left_count: 0,
            rotate_90_right_count: 0,
            flip_horizontal_count: 0,
            flip_vertical_count: 0,
            channel_filter: (true, true, true, true),
            swizzle: None,
            deswizzle: None,
            pixel_format: PixelFormat::R8G8B8A8,
        }
    }
}

impl TextureEffects {
    /// Pipeline: Deswizzle -> Decompress -> Decode -> Process -> Compress? -> Swizzle (compressed or uncompressed) -> Decompress?
    pub fn get_transformed_rgba8_pixels(
        &self,
        image: &Image,
        display_compressed: bool,
    ) -> Result<(u32, u32, Vec<u8>), Box<dyn Error>> {
        let mut width = image.width;
        let mut height = image.height;
        let mut pixels = image.pixels.clone();

        if let Some(platform) = self.deswizzle {
            let swizzler = Swizzler::default();
            swizzler.deswizzle(
                &mut pixels,
                width,
                height,
                image.pixel_format_info.pixel_format,
                platform,
            )?;
        }

        let mut pixels =
            PixelDecoder::decode(&pixels, image.pixel_format_info.pixel_format, width, height)?;

        if self.flip_vertical_count % 2 == 1 {
            TextureProcessingUtility::flip_vertical_raw(&mut pixels, width, height);
        }

        if self.flip_horizontal_count % 2 == 1 {
            TextureProcessingUtility::flip_horizontal_raw(&mut pixels, width, height);
        }

        let total_rotation =
            (self.rotate_90_left_count as i32 % 4) - (self.rotate_90_right_count as i32 % 4);

        if total_rotation > 0 {
            for _ in 0..total_rotation {
                TextureProcessingUtility::rotate_90_left_raw(&mut pixels, width, height);
                std::mem::swap(&mut width, &mut height);
            }
        } else if total_rotation < 0 {
            for _ in 0..-total_rotation {
                TextureProcessingUtility::rotate_90_right_raw(&mut pixels, width, height);
                std::mem::swap(&mut width, &mut height);
            }
        }

        TextureProcessingUtility::filter_colors(&mut pixels, width, height, self.channel_filter);

        if display_compressed {
            pixels = PixelEncoder::encode(&pixels, self.pixel_format, width, height)?;
        }

        if let Some(platform) = self.swizzle {
            let swizzler = Swizzler::default();

            if display_compressed {
                swizzler.swizzle(&mut pixels, width, height, self.pixel_format, platform)?;
            } else {
                swizzler.swizzle(&mut pixels, width, height, PixelFormat::R8G8B8A8, platform)?;
            }
        }

        if display_compressed {
            pixels = PixelDecoder::decode(&pixels, self.pixel_format, width, height)?;
        }

        Ok((width, height, pixels))
    }

    /// Pipeline: Deswizzle -> Decompress -> Decode -> Process -> Compress -> Swizzle
    pub fn get_transformed_pixels(&self, image: &Image) -> Result<Image, Box<dyn Error>> {
        let mut width = image.width;
        let mut height = image.height;
        let image_pixel_format = image.pixel_format_info.pixel_format;
        let mut pixels = image.pixels.clone();

        if let Some(platform) = self.deswizzle {
            let swizzler = Swizzler::default();
            swizzler.deswizzle(&mut pixels, width, height, image_pixel_format, platform)?;
        }

        let mut pixels = PixelDecoder::decode(&pixels, image_pixel_format, width, height)?;

        if self.flip_vertical_count % 2 == 1 {
            TextureProcessingUtility::flip_vertical_raw(&mut pixels, width, height);
        }

        if self.flip_horizontal_count % 2 == 1 {
            TextureProcessingUtility::flip_horizontal_raw(&mut pixels, width, height);
        }

        let total_rotation =
            (self.rotate_90_left_count as i32 % 4) - (self.rotate_90_right_count as i32 % 4);

        if total_rotation > 0 {
            for _ in 0..total_rotation {
                TextureProcessingUtility::rotate_90_left_raw(&mut pixels, width, height);
                std::mem::swap(&mut width, &mut height);
            }
        } else if total_rotation < 0 {
            for _ in 0..-total_rotation {
                TextureProcessingUtility::rotate_90_right_raw(&mut pixels, width, height);
                std::mem::swap(&mut width, &mut height);
            }
        }

        TextureProcessingUtility::filter_colors(&mut pixels, width, height, self.channel_filter);

        let mut pixels = PixelEncoder::encode(&mut pixels, self.pixel_format, width, height)?;

        if let Some(platform) = self.swizzle {
            let swizzler = Swizzler::default();
            swizzler.swizzle(&mut pixels, width, height, self.pixel_format, platform)?;
        }

        Ok(Image {
            width,
            height,
            pixel_format_info: {
                PixelFormatInfo {
                    pixel_format: self.pixel_format,
                    color_space: ColorSpace::Linear,
                    pixel_data_type: super::pixel_format::PixelDataType::UNorm,
                    is_premultiplied: false,
                }
            },
            row_pitch: self.pixel_format.get_row_pitch(width),
            slice_pitch: self.pixel_format.get_slice_pitch(width, height),
            pixels,
        })
    }
}

/// Basic processing utilities
struct TextureProcessingUtility;

impl TextureProcessingUtility {
    pub fn flip_vertical_raw(pixels: &mut [u8], width: u32, height: u32) {
        for y in 0..(height / 2) {
            for x in 0..width {
                let top_index = (y * width + x) as usize * 4;
                let bottom_index = ((height - y - 1) * width + x) as usize * 4;

                for i in 0..4 {
                    pixels.swap(top_index + i, bottom_index + i);
                }
            }
        }
    }

    pub fn flip_horizontal_raw(pixels: &mut [u8], width: u32, height: u32) {
        for y in 0..height {
            for x in 0..(width / 2) {
                let left_index = (y * width + x) as usize * 4;
                let right_index = (y * width + (width - x - 1)) as usize * 4;

                for i in 0..4 {
                    pixels.swap(left_index + i, right_index + i);
                }
            }
        }
    }

    pub fn rotate_90_left_raw(pixels: &mut [u8], width: u32, height: u32) {
        let img = DynamicImage::ImageRgba8(
            ImageBuffer::<Rgba<u8>, _>::from_raw(width, height, pixels.to_vec())
                .expect("Failed to create image buffer!"),
        );

        let rotated_img = img.rotate270();

        let rotated_pixels = rotated_img.to_rgba8();
        pixels.copy_from_slice(&rotated_pixels);
    }

    pub fn rotate_90_right_raw(pixels: &mut [u8], width: u32, height: u32) {
        let img = DynamicImage::ImageRgba8(
            ImageBuffer::<Rgba<u8>, _>::from_raw(width, height, pixels.to_vec())
                .expect("Failed to create image buffer!"),
        );

        let rotated_img = img.rotate90();

        let rotated_pixels = rotated_img.to_rgba8();
        pixels.copy_from_slice(&rotated_pixels);
    }

    pub fn filter_colors(
        pixels: &mut [u8],
        width: u32,
        height: u32,
        (r, g, b, a): (bool, bool, bool, bool),
    ) {
        for y in 0..height {
            for x in 0..width {
                let index = (y * width + x) as usize * 4;
                pixels[index] = pixels[index] * (r as u8);
                pixels[index + 1] = pixels[index + 1] * (g as u8);
                pixels[index + 2] = pixels[index + 2] * (b as u8);
                pixels[index + 3] = if a { pixels[index + 3] } else { 255 };
            }
        }
    }
}
