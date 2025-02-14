use std::{error::Error, fs, io::Cursor, path::PathBuf};

use image::ImageFormat;

use super::ImageCodec;
use crate::graphics::{crate_image_utility::ImageUtility, texture::Texture};

pub struct HDRCodec;

impl ImageCodec for HDRCodec {
    fn save_to_memory(&self, texture: &Texture) -> Result<Vec<u8>, Box<dyn Error>> {
        if texture.metadata.array_size != 1 || texture.metadata.depth != 1 {
            return Err("HDR only supports single image textures!"
                .to_string()
                .into());
        }

        let image = &texture.images[0];

        let dynamic_image =
            ImageUtility::load_dynamic_image(image, image.pixel_format_info.pixel_format)?;

        let mut buffer = Vec::new();
        dynamic_image.write_to(&mut Cursor::new(&mut buffer), ImageFormat::Hdr)?;
        Ok(buffer)
    }

    fn save_to_file(&self, filepath: PathBuf, texture: &Texture) -> Result<(), Box<dyn Error>> {
        let bytes = self.save_to_memory(texture)?;

        Ok(fs::write(filepath, bytes)?)
    }

    fn load_from_memory(&self, source: &[u8]) -> Result<Texture, Box<dyn Error>> {
        ImageUtility::get_texture_from_bytes(source)
    }

    fn load_from_file(&self, filepath: PathBuf) -> Result<Texture, Box<dyn Error>> {
        self.load_from_memory(&fs::read(&filepath)?)
    }

    fn supported_extensions(&self) -> Vec<&'static str> {
        vec!["hdr"]
    }
}
