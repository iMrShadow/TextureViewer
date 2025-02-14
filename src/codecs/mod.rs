pub mod bmp;
pub mod codec_manager;
pub mod dds;
pub mod hdr;
pub mod jpeg;
pub mod png;
pub mod tga;
pub mod tiff;

use std::{error::Error, path::PathBuf};

use crate::graphics::texture::Texture;

/// Trait for image codecs
/// Implement this trait to add more image codecs and add them in codec_manager.rs
pub trait ImageCodec {
    fn save_to_memory(&self, texture: &Texture) -> Result<Vec<u8>, Box<dyn Error>>;
    fn save_to_file(&self, filepath: PathBuf, texture: &Texture) -> Result<(), Box<dyn Error>>;
    fn load_from_memory(&self, source: &[u8]) -> Result<Texture, Box<dyn Error>>;
    fn load_from_file(&self, filepath: PathBuf) -> Result<Texture, Box<dyn Error>>;
    fn supported_extensions(&self) -> Vec<&'static str>;
}
