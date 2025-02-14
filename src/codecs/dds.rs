use std::{error::Error, fs, path::PathBuf};

use super::ImageCodec;
use crate::graphics::{
    crate_directxtex_utility::DirectXTexUtility,
    texture::{Image, TexMetadata, Texture},
};
use directxtex::{ScratchImage, CP_FLAGS_NONE, DDS_FLAGS_NONE, DXGI_FORMAT};

pub struct DDSCodec;

impl ImageCodec for DDSCodec {
    fn save_to_memory(&self, texture: &Texture) -> Result<Vec<u8>, Box<dyn Error>> {
        let new_tex_metadata = directxtex::TexMetadata {
            width: texture.metadata.width as usize,
            height: texture.metadata.height as usize,
            depth: texture.metadata.depth as usize,
            array_size: texture.metadata.array_size as usize,
            mip_levels: texture.metadata.mip_levels as usize,
            misc_flags: 0,
            misc_flags2: 0,
            format: DirectXTexUtility::get_dxgi_format_from_pixel_format(
                texture.metadata.pixel_format_info.pixel_format,
            ),
            dimension: DirectXTexUtility::get_texture_dimension(texture.metadata.dimensions),
        };

        let mut new_images = Vec::new();

        for img in &texture.images {
            let pitch = new_tex_metadata.format.compute_pitch(
                img.width as usize,
                img.height as usize,
                CP_FLAGS_NONE,
            )?;

            let new_image = directxtex::Image {
                width: img.width as usize,
                height: img.height as usize,
                row_pitch: pitch.row,
                slice_pitch: pitch.slice,
                pixels: img.pixels.as_ptr() as *mut u8,
                format: new_tex_metadata.format,
            };

            new_images.push(new_image);
        }

        let blob = directxtex::save_dds(&new_images, &new_tex_metadata, DDS_FLAGS_NONE)?;
        let buffer = blob.buffer();

        Ok(buffer.to_vec())
    }

    fn save_to_file(&self, filepath: PathBuf, texture: &Texture) -> Result<(), Box<dyn Error>> {
        let bytes = self.save_to_memory(texture)?;

        Ok(fs::write(filepath, bytes)?)
    }

    fn load_from_memory(&self, source: &[u8]) -> Result<Texture, Box<dyn Error>> {
        let (scratch, meta) = {
            let mut meta = Default::default();

            let mut scratch = ScratchImage::load_dds(
                &source,
                DDS_FLAGS_NONE,
                Some(&mut meta),
                Default::default(),
            )?;

            if scratch.metadata().format.is_planar() {
                scratch = scratch.decompress(DXGI_FORMAT::DXGI_FORMAT_R8G8B8A8_UNORM)?;
            }

            let meta = meta;

            (scratch, meta)
        };

        let images = scratch.images();
        let pixel_data = scratch.pixels();
        let mut index = 0;

        let mut new_images = Vec::with_capacity(images.len());

        let pixel_format_info =
            DirectXTexUtility::get_pixel_format_info_from_image(&scratch.metadata());

        for img in images {
            let width = img.width as u32;
            let height = img.height as u32;
            let row_pitch = img.row_pitch as u32;
            let slice_pitch = img.slice_pitch as u32;

            let pixels = Vec::from(&pixel_data[index..index + slice_pitch as usize]);

            let new_image = Image {
                width,
                height,
                pixel_format_info,
                row_pitch,
                slice_pitch,
                pixels,
            };

            new_images.push(new_image);

            index += slice_pitch as usize;
        }

        let new_tex_metadata = TexMetadata {
            width: meta.width as u32,
            height: meta.height as u32,
            depth: meta.depth as u32,
            array_size: meta.array_size as u32,
            mip_levels: meta.mip_levels as u32,
            pixel_format_info,
            alpha_mode: 0,
            dimensions: DirectXTexUtility::get_texture_dimension_from_directxtex(meta.dimension),
            is_cubemap: meta.is_cubemap(),
            is_volumemap: meta.is_volumemap(),
        };

        Ok(Texture {
            metadata: new_tex_metadata,
            images: new_images,
        })
    }

    fn load_from_file(&self, filepath: PathBuf) -> Result<Texture, Box<dyn Error>> {
        self.load_from_memory(&fs::read(&filepath)?)
    }

    fn supported_extensions(&self) -> Vec<&'static str> {
        vec!["dds"]
    }
}
