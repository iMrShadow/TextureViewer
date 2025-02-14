use std::{error::Error, fs, io::Cursor, path::PathBuf};

use directxtex::{ScratchImage, DXGI_FORMAT, TEX_FILTER_DEFAULT};
use image::ImageFormat;

use super::ImageCodec;
use crate::graphics::{
    crate_directxtex_utility::DirectXTexUtility,
    crate_image_utility::ImageUtility,
    texture::{Image, TexMetadata, Texture},
};

pub struct TGACodec;

impl ImageCodec for TGACodec {
    fn save_to_memory(&self, texture: &Texture) -> Result<Vec<u8>, Box<dyn Error>> {
        if texture.metadata.array_size != 1 || texture.metadata.depth != 1 {
            return Err("TGA only supports single image textures!"
                .to_string()
                .into());
        }

        let image = &texture.images[0];

        let dynamic_image =
            ImageUtility::load_dynamic_image(image, image.pixel_format_info.pixel_format)?;

        let mut buffer = Vec::new();
        dynamic_image.write_to(&mut Cursor::new(&mut buffer), ImageFormat::Tga)?;
        Ok(buffer)
    }

    fn save_to_file(&self, filepath: PathBuf, texture: &Texture) -> Result<(), Box<dyn Error>> {
        let bytes = self.save_to_memory(texture)?;

        Ok(fs::write(filepath, bytes)?)
    }

    fn load_from_memory(&self, source: &[u8]) -> Result<Texture, Box<dyn Error>> {
        let (scratch, meta) = {
            let mut meta = Default::default();
            let mut scratch =
                ScratchImage::load_tga(&source, Default::default(), Some(&mut meta)).unwrap();

            if DXGI_FORMAT::is_compressed(scratch.metadata().format) {
                let hr = scratch.decompress(DXGI_FORMAT::DXGI_FORMAT_R8G8B8A8_UNORM);

                if hr.is_err() {
                    return Err(format!("Failed to decompress texture: {:?}", hr).into());
                }

                scratch = hr.unwrap();
            }

            if DXGI_FORMAT::DXGI_FORMAT_R8G8B8A8_UNORM != scratch.metadata().format {
                let hr = scratch.convert(
                    DXGI_FORMAT::DXGI_FORMAT_R8G8B8A8_UNORM,
                    TEX_FILTER_DEFAULT,
                    0.5,
                );

                if hr.is_err() {
                    return Err(format!("Failed to convert texture: {:?}", hr).into());
                }

                scratch = hr.unwrap();
            }

            (scratch, meta)
        };

        let images = scratch.images();
        let pixel_data = scratch.pixels();
        let mut index = 0;

        let mut new_images = Vec::with_capacity(images.len());

        let pixel_format_info = DirectXTexUtility::get_pixel_format_info_from_image(&meta);

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

        let new_metadata = TexMetadata {
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
            metadata: new_metadata,
            images: new_images,
        })
    }

    fn load_from_file(&self, filepath: PathBuf) -> Result<Texture, Box<dyn Error>> {
        self.load_from_memory(&fs::read(&filepath)?)
    }

    fn supported_extensions(&self) -> Vec<&'static str> {
        vec!["tga"]
    }
}
