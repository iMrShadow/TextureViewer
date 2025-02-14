use std::{error::Error, fmt};

use super::pixel_format::PixelFormat;

pub mod ps4;
pub mod psvita;
pub mod swizzle_manager;
pub mod xbox_360;

/// Trait for swizzling and deswizzling images
/// Implement this trait to add more swizzlers for other platforms
pub trait Swizzable {
    fn swizzle(
        &self,
        pixels: &mut [u8],
        width: u32,
        height: u32,
        pixel_format: PixelFormat,
    ) -> Result<(), Box<dyn Error>>;
    fn deswizzle(
        &self,
        pixels: &mut [u8],
        width: u32,
        height: u32,
        pixel_format: PixelFormat,
    ) -> Result<(), Box<dyn Error>>;
    fn get_platform(&self) -> Platform;
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Platform {
    PS4,
    PSVita,
    Xbox360,
    PS3,
    Switch,
    WiiU,
    Wii,
    Xbox,
    GameCube,
    PS2,
}

impl fmt::Display for Platform {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Platform::PS4 => "PS4",
            Platform::PSVita => "PS Vita",
            Platform::Xbox360 => "Xbox 360",
            Platform::PS3 => "PS3",
            Platform::Switch => "Nintendo Switch",
            Platform::WiiU => "Wii U",
            Platform::Wii => "Wii",
            Platform::Xbox => "Xbox 1",
            Platform::GameCube => "GameCube",
            Platform::PS2 => "PS2",
        };

        write!(f, "{}", name)
    }
}
