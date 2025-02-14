use std::error::Error;

use bytemuck;
use image::{DynamicImage, ImageBuffer};

use super::{
    pixel_format::{ColorSpace, PixelDataType, PixelFormat, PixelFormatInfo},
    texture::{Image, TexDimension, TexMetadata, Texture},
};

pub struct ImageUtility;

impl ImageUtility {
    pub fn get_pixel_format_from_image(color_type: image::ColorType) -> Option<PixelFormat> {
        match color_type {
            image::ColorType::L8 => Some(PixelFormat::L8),
            image::ColorType::La8 => Some(PixelFormat::L8A8),
            image::ColorType::Rgb8 => Some(PixelFormat::R8G8B8),
            image::ColorType::Rgba8 => Some(PixelFormat::R8G8B8A8),
            image::ColorType::Rgb16 => Some(PixelFormat::R16G16B16),
            image::ColorType::Rgba16 => Some(PixelFormat::R16G16B16A16),
            image::ColorType::L16 => Some(PixelFormat::L16),
            image::ColorType::La16 => Some(PixelFormat::L16A16),
            image::ColorType::Rgb32F => Some(PixelFormat::R32G32B32),
            image::ColorType::Rgba32F => Some(PixelFormat::R32G32B32A32),
            _ => None,
        }
    }

    pub fn get_pixel_format_datatype_from_image(
        color_type: image::ColorType,
    ) -> Option<PixelDataType> {
        match color_type {
            image::ColorType::L8 => Some(PixelDataType::UNorm),
            image::ColorType::La8 => Some(PixelDataType::UNorm),
            image::ColorType::Rgb8 => Some(PixelDataType::UNorm),
            image::ColorType::Rgba8 => Some(PixelDataType::UNorm),
            image::ColorType::Rgb16 => Some(PixelDataType::UNorm),
            image::ColorType::Rgba16 => Some(PixelDataType::UNorm),
            image::ColorType::L16 => Some(PixelDataType::UNorm),
            image::ColorType::La16 => Some(PixelDataType::UNorm),
            image::ColorType::Rgb32F => Some(PixelDataType::Float),
            image::ColorType::Rgba32F => Some(PixelDataType::Float),
            _ => None,
        }
    }

    pub fn get_pixel_format_info_from_image(image: &image::DynamicImage) -> PixelFormatInfo {
        let color_type = image.color();
        let pixel_format = Self::get_pixel_format_from_image(color_type).unwrap();
        let pixel_data_type = Self::get_pixel_format_datatype_from_image(color_type).unwrap();
        let color_space = match color_type {
            image::ColorType::Rgb8 | image::ColorType::Rgba8 => ColorSpace::SRGB,
            _ => ColorSpace::Linear,
        };
        let is_premultiplied = false;

        PixelFormatInfo {
            pixel_format,
            pixel_data_type,
            color_space,
            is_premultiplied,
        }
    }

    /// Converts raw pixel data into a `DynamicImage`, preserving format.
    pub fn load_dynamic_image(
        image: &Image,
        pixel_format: PixelFormat,
    ) -> Result<DynamicImage, Box<dyn Error>> {
        let width = image.width;
        let height = image.height;
        let pixels = &image.pixels;

        match pixel_format {
            // RGBA8
            PixelFormat::R8G8B8A8 => {
                let img_buffer = ImageBuffer::from_raw(width, height, pixels.clone())
                    .ok_or("Failed to create RGBA8 image buffer")?;
                Ok(DynamicImage::ImageRgba8(img_buffer))
            }

            // RGB8
            PixelFormat::R8G8B8 => {
                let img_buffer = ImageBuffer::from_raw(width, height, pixels.clone())
                    .ok_or("Failed to create RGB8 image buffer")?;
                Ok(DynamicImage::ImageRgb8(img_buffer))
            }

            // Grayscale (L8)
            PixelFormat::L8 => {
                let img_buffer = ImageBuffer::from_raw(width, height, pixels.clone())
                    .ok_or("Failed to create L8 image buffer")?;
                Ok(DynamicImage::ImageLuma8(img_buffer))
            }

            // Grayscale + Alpha (LA8)
            PixelFormat::L8A8 => {
                let img_buffer = ImageBuffer::from_raw(width, height, pixels.clone())
                    .ok_or("Failed to create LA8 image buffer")?;
                Ok(DynamicImage::ImageLumaA8(img_buffer))
            }

            // RGBA16
            PixelFormat::R16G16B16A16 => {
                let pixels_u16 = bytemuck::cast_slice::<u8, u16>(pixels);
                let img_buffer = ImageBuffer::from_raw(width, height, pixels_u16.to_vec())
                    .ok_or("Failed to create RGBA16 image buffer")?;
                Ok(DynamicImage::ImageRgba16(img_buffer))
            }

            // RGB32F
            PixelFormat::R32G32B32 => {
                let pixels_f32 = bytemuck::cast_slice::<u8, f32>(pixels);
                let img_buffer = ImageBuffer::from_raw(width, height, pixels_f32.to_vec())
                    .ok_or("Failed to create RGB32F image buffer")?;
                Ok(DynamicImage::ImageRgb32F(img_buffer))
            }

            // RGBA32F
            PixelFormat::R32G32B32A32 => {
                let pixels_f32 = bytemuck::cast_slice::<u8, f32>(pixels);
                let img_buffer = ImageBuffer::from_raw(width, height, pixels_f32.to_vec())
                    .ok_or("Failed to create RGBA32F image buffer")?;
                Ok(DynamicImage::ImageRgba32F(img_buffer))
            }

            _ => Err("Unsupported pixel format!".to_string().into()),
        }
    }

    pub fn get_texture_from_bytes(source: &[u8]) -> Result<Texture, Box<dyn Error>> {
        let img = image::load_from_memory(source).unwrap();

        let pixel_format_info = Self::get_pixel_format_info_from_image(&img);

        let width = img.width();
        let height = img.height();
        let row_pitch = pixel_format_info.pixel_format.get_row_pitch(width);
        let slice_pitch = pixel_format_info
            .pixel_format
            .get_slice_pitch(width, height);
        let pixels = img.into_bytes();

        let metadata = TexMetadata {
            width,
            height,
            depth: 1,
            array_size: 1,
            mip_levels: 1,
            pixel_format_info,
            alpha_mode: 0,
            dimensions: TexDimension::Tex2D,
            is_cubemap: false,
            is_volumemap: false,
        };

        let images = vec![Image {
            width,
            height,
            pixel_format_info,
            row_pitch,
            slice_pitch,
            pixels,
        }];

        Ok(Texture { metadata, images })
    }
}
