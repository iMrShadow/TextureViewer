use std::{collections::HashMap, path::Path};

use crate::graphics::texture::Texture;

use super::{dds::DDSCodec, jpeg::JPEGCodec, png::PNGCodec, ImageCodec};

pub struct CodecManager {
    codecs: Vec<Box<dyn ImageCodec>>,
    extension_map: HashMap<String, usize>,
}

impl CodecManager {
    pub fn new() -> Self {
        CodecManager {
            codecs: Vec::new(),
            extension_map: HashMap::new(),
        }
    }

    pub fn new_codec_manager() -> Self {
        let mut codec_manager = CodecManager::new();
        codec_manager.register_codec(PNGCodec);
        codec_manager.register_codec(JPEGCodec);
        codec_manager.register_codec(DDSCodec);
        codec_manager
    }

    pub fn register_codec<T: ImageCodec + 'static>(&mut self, codec: T) {
        // Store the codec and map its supported extensions
        let index = self.codecs.len();
        self.codecs.push(Box::new(codec));

        for ext in self.codecs[index].supported_extensions() {
            self.extension_map.insert(ext.to_lowercase(), index);
        }
    }

    pub fn get_codec_for_extension(&self, ext: &str) -> Option<&dyn ImageCodec> {
        self.extension_map
            .get(&ext.to_lowercase())
            .and_then(|&index| self.codecs.get(index))
            .map(|codec| codec.as_ref())
    }

    pub fn load_from_file(&self, path: &Path) -> Result<Texture, String> {
        let ext = path
            .extension()
            .and_then(|e| e.to_str())
            .ok_or("No file extension")?;

        self.get_codec_for_extension(ext)
            .ok_or_else(|| format!("Unsupported format: {}", ext))?
            .load_from_file(path.to_path_buf())
    }

    pub fn save_to_file(&self, path: &Path, texture: &Texture) -> Result<Vec<u8>, String> {
        let ext = path
            .extension()
            .and_then(|e| e.to_str())
            .ok_or("No file extension")?;

        self.get_codec_for_extension(ext)
            .ok_or_else(|| format!("Unsupported format: {}", ext))?
            .save_to_file(path.to_path_buf(), texture)
    }

    pub fn get_registered_extensions(&self) -> Vec<String> {
        self.extension_map.keys().cloned().collect()
    }
}
