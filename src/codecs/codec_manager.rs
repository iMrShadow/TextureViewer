use std::{collections::HashMap, error::Error, path::Path};

use crate::graphics::{texture::Texture, texture_utility::TextureEffects};

use super::{
    bmp::BMPCodec, dds::DDSCodec, hdr::HDRCodec, jpeg::JPEGCodec, png::PNGCodec, tga::TGACodec,
    tiff::TIFFCodec, ImageCodec,
};

/// Codec manager is flexible. It can be used to load and save textures from different formats, and additional custom codecs can be added.
pub struct CodecManager {
    codecs: Vec<Box<dyn ImageCodec>>,
    extension_map: HashMap<String, usize>,
}

impl Default for CodecManager {
    /// Create a new codec manager with all the built-in codecs registered
    fn default() -> Self {
        let mut codec_manager = CodecManager::new();
        codec_manager.register_codec(PNGCodec);
        codec_manager.register_codec(JPEGCodec);
        codec_manager.register_codec(DDSCodec);
        codec_manager.register_codec(BMPCodec);
        codec_manager.register_codec(TGACodec);
        codec_manager.register_codec(TIFFCodec);
        codec_manager.register_codec(HDRCodec);
        codec_manager
    }
}

impl CodecManager {
    pub fn new() -> Self {
        CodecManager {
            codecs: Vec::new(),
            extension_map: HashMap::new(),
        }
    }

    /// Register a new codec
    pub fn register_codec<T: ImageCodec + 'static>(&mut self, codec: T) {
        // Store the codec and map its supported extensions
        let index = self.codecs.len();
        self.codecs.push(Box::new(codec));

        for ext in self.codecs[index].supported_extensions() {
            self.extension_map.insert(ext.to_lowercase(), index);
        }
    }

    /// Get the codec for a specific extension
    pub fn get_codec_for_extension(&self, ext: &str) -> Option<&dyn ImageCodec> {
        self.extension_map
            .get(&ext.to_lowercase())
            .and_then(|&index| self.codecs.get(index))
            .map(|codec| codec.as_ref())
    }

    /// Load a texture from a file
    pub fn load_from_file(&self, path: &Path) -> Result<Texture, Box<dyn Error>> {
        let ext = path
            .extension()
            .and_then(|e| e.to_str())
            .ok_or("No file extension")?;

        self.get_codec_for_extension(ext)
            .ok_or_else(|| format!("Unsupported format: {}", ext))?
            .load_from_file(path.to_path_buf())
    }

    /// Save a texture to a file
    pub fn save_to_file(
        &self,
        path: &Path,
        texture: &Texture,
        effects: &TextureEffects,
    ) -> Result<(), Box<dyn Error>> {
        let ext = path
            .extension()
            .and_then(|e| e.to_str())
            .ok_or("No file extension")?;

        let transformed_texture = texture.new_transformed_texture(&effects)?;

        self.get_codec_for_extension(ext)
            .ok_or_else(|| format!("Unsupported format: {}", ext))?
            .save_to_file(path.to_path_buf(), &transformed_texture)
    }

    /// Get a list of all registered extensions
    pub fn get_registered_extensions(&self) -> Vec<String> {
        self.extension_map.keys().cloned().collect()
    }
}
