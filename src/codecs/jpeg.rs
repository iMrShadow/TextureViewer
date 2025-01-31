use std::path::PathBuf;

use image::{load_from_memory, GenericImageView, ImageFormat, ImageReader};
use serde::de::value::Error;

use super::ImageCodec;
use crate::graphics::{
    pixel_format::{get_row_pitch, get_slice_pitch},
    crate_image_utility::{self, get_pixel_format_info_from_image},
    texture::{Image, TexDimension, TexMetadata, Texture},
};

pub struct JPEGCodec;

impl ImageCodec for JPEGCodec {
    fn save_to_memory(&self, texture: &Texture) -> Result<Vec<u8>, String> {
        // Implement JPEG encoding logic here
        Ok(vec![]) // Placeholder
    }

    fn save_to_file(&self, filepath: PathBuf, texture: &Texture) -> Result<Vec<u8>, String> {
        // Implement JPEG encoding logic here
        Ok(vec![]) // Placeholder
    }

    fn load_from_memory(&self, source: &[u8]) -> Result<Texture,String> {
        // Implement JPEG decoding logic here
        let img = image::load_from_memory(source).unwrap();

        let format = get_pixel_format_info_from_image(&img);

        let width = img.width();
        let height = img.height();
        let row_pitch = get_row_pitch(width, &format.pixel_format);
        let slice_pitch = get_slice_pitch(width, height, &format.pixel_format);
        let images = img.into_bytes();

        let metadata = TexMetadata {
            width,
            height,
            depth: 1,
            array_size: 1,
            mip_levels: 1,
            format,
            alpha_mode: 0,
            dimensions: TexDimension::Tex2D,
            is_cubemap: false,
            is_volumemap: false,
        };

        let images = vec![Image {
            width,
            height,
            format,
            row_pitch,
            slice_pitch,
            pixels: images,
        }];

        Ok(Texture { metadata, images })
    }

    fn load_from_file(&self, filepath: PathBuf) -> Result<Texture, String> {
        let data = std::fs::read(filepath).map_err(|_| "Failed to read file")?;
        self.load_from_memory(&data)
    }

    fn supported_extensions(&self) -> Vec<&'static str> {
        vec!["jpg", "jpeg"]
    }
}
