use std::{fs, path::PathBuf};

use directxtex::ScratchImage;
use image::{load_from_memory, ImageReader};

use super::ImageCodec;
use crate::graphics::{
    crate_directxtex_utility::get_pixel_format_info_from_image,
    crate_directxtex_utility::get_texture_dimension,
    pixel_format::{get_row_pitch, get_slice_pitch},
    texture::{Image, TexMetadata, Texture},
};

pub struct DDSCodec;

impl ImageCodec for DDSCodec {
    fn save_to_memory(&self, texture: &Texture) -> Result<Vec<u8>, String> {
        // Implement PNG encoding logic here
        Ok(vec![]) // Placeholder
    }

    fn save_to_file(&self, filepath: PathBuf, texture: &Texture) -> Result<Vec<u8>, String> {
        // Implement PNG encoding logic here
        Ok(vec![]) // Placeholder
    }

    fn load_from_memory(&self, source: &[u8]) -> Result<Texture, String> {
        let (scratch, meta, dds) = {
            let mut meta = Default::default();
            let mut dds = Default::default();
            let scratch = ScratchImage::load_dds(
                &source,
                Default::default(),
                Some(&mut meta),
                Some(&mut dds),
            )
            .unwrap();
            (scratch, meta, dds)
        };

        let images = scratch.images();
        let pixel_data = scratch.pixels();
        let mut index = 0;

        let mut new_images = Vec::with_capacity(images.len());

        let pixel_format_info = get_pixel_format_info_from_image(&meta);

        for img in images {
            let width = img.width as u32;
            let height = img.height as u32;
            let row_pitch = img.row_pitch as u32;
            let slice_pitch = img.slice_pitch as u32;

            let pixels = Vec::from(&pixel_data[index..index + slice_pitch as usize]);

            let new_image = Image {
                width,
                height,
                format: pixel_format_info,
                row_pitch,
                slice_pitch,
                pixels,
            };

            new_images.push(new_image);

            index += slice_pitch as usize;
        }

        let new_metadata = TexMetadata {
            width: meta.width as u32,
            height: meta.height as u32,
            depth: meta.depth as u32,
            array_size: meta.array_size as u32,
            mip_levels: meta.mip_levels as u32,
            format: pixel_format_info,
            alpha_mode: 0,
            dimensions: get_texture_dimension(meta.dimension),
            is_cubemap: meta.is_cubemap(),
            is_volumemap: meta.is_volumemap(),
        };

        Ok(Texture {
            metadata: new_metadata,
            images: new_images,
        })
    }

    fn load_from_file(&self, filepath: PathBuf) -> Result<Texture, String> {
        match fs::read(&filepath) {
            Ok(data) => self.load_from_memory(&data),
            Err(e) => Err(format!("Failed to read file {}: {}", filepath.display(), e)),
        }
    }

    fn supported_extensions(&self) -> Vec<&'static str> {
        vec!["dds"]
    }
}
