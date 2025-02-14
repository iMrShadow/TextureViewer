use std::error::Error;

use crate::graphics::pixel_format::PixelFormat;

use super::{
    ps4::PS4Swizzler, psvita::PSVitaSwizzler, xbox_360::Xbox360Swizzler, Platform, Swizzable,
};

/// Swizzler is a manager for swizzling and deswizzling images. It is also flexible and can be extended with custom swizzles.
pub struct Swizzler {
    swizzlers: Vec<Box<dyn Swizzable>>,
}

impl Default for Swizzler {
    /// Create a new codec manager with all the built-in codecs registered
    fn default() -> Self {
        let mut swizzlers = Swizzler::new();
        swizzlers.register_swizzler(PS4Swizzler);
        swizzlers.register_swizzler(PSVitaSwizzler);
        swizzlers.register_swizzler(Xbox360Swizzler);

        swizzlers
    }
}

impl Swizzler {
    pub fn new() -> Self {
        Swizzler {
            swizzlers: Vec::new(),
        }
    }

    /// Register a new swizzler
    pub fn register_swizzler<T: Swizzable + 'static>(&mut self, swizzler: T) {
        self.swizzlers.push(Box::new(swizzler));
    }

    /// Swizzle the image data
    pub fn swizzle(
        &self,
        data: &mut [u8],
        width: u32,
        height: u32,
        pixel_format: PixelFormat,
        platform: Platform,
    ) -> Result<(), Box<dyn Error>> {
        for swizzle in &self.swizzlers {
            if swizzle.get_platform() == platform {
                return swizzle.swizzle(data, width, height, pixel_format);
            }
        }

        return Err("No swizzling found for platform!".into());
    }

    /// Deswizzle the image data
    pub fn deswizzle(
        &self,
        data: &mut [u8],
        width: u32,
        height: u32,
        pixel_format: PixelFormat,
        platform: Platform,
    ) -> Result<(), Box<dyn Error>> {
        for swizzle in &self.swizzlers {
            if swizzle.get_platform() == platform {
                return swizzle.deswizzle(data, width, height, pixel_format);
            }
        }

        return Err("No deswizzling found for platform!".into());
    }
}
