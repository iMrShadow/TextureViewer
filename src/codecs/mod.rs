pub mod codec_manager;
pub mod dds;
pub mod jpeg;
pub mod png;

use std::path::PathBuf;

use crate::graphics::texture::Texture;

pub trait ImageCodec {
    fn save_to_memory(&self, texture: &Texture) -> Result<Vec<u8>, String>;
    fn save_to_file(&self, filepath: PathBuf, texture: &Texture) -> Result<Vec<u8>, String>;
    fn load_from_memory(&self, source: &[u8]) -> Result<Texture, String>;
    fn load_from_file(&self, filepath: PathBuf) -> Result<Texture, String>;
    fn supported_extensions(&self) -> Vec<&'static str>;
}
