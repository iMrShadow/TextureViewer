use std::fs;
use std::io::{self, Read};
use std::path::Path;

use crate::graphics::pixel_format::PixelFormatInfo;

use super::pixel_format_decoder::PixelDecoder;

#[derive(Default)]
pub struct Texture {
    pub metadata: TexMetadata,
    pub images: Vec<Image>,
}

pub struct Image {
    pub width: u32,
    pub height: u32,
    pub format: PixelFormatInfo,
    pub row_pitch: u32,
    pub slice_pitch: u32,
    pub pixels: Vec<u8>,
}

#[derive(Default)]
pub struct TexMetadata {
    pub width: u32,
    pub height: u32,
    pub depth: u32,
    pub array_size: u32,
    pub mip_levels: u32,
    pub format: PixelFormatInfo,
    pub alpha_mode: u32,
    pub dimensions: TexDimension,
    pub is_cubemap: bool,
    pub is_volumemap: bool,
}

#[derive(Default)]
pub enum TexDimension {
    Tex1D,
    #[default]
    Tex2D,
    Tex3D,
}

impl Texture {
    /// Creates a new, empty texture.
    pub fn new() -> Self {
        Self {
            metadata: TexMetadata {
                width: 0,
                height: 0,
                depth: 0,
                array_size: 0,
                mip_levels: 0,
                format: PixelFormatInfo::default(),
                alpha_mode: 0,
                dimensions: TexDimension::Tex2D,
                is_cubemap: false,
                is_volumemap: false,
            },
            images: Vec::new(),
        }
    }

    // Level (mip index), layer (array index), slice (z index)
    pub fn get_image(&self, level: u32, layer: u32, slice: u32) -> Result<&Image, String> {
        if level >= self.metadata.mip_levels || layer >= self.metadata.array_size {
            return Err(format!(
                "Image index out of bounds: {} >= {}",
                level, self.metadata.mip_levels
            ));
        }

        let max_slices = (1.max(self.metadata.depth >> level)).max(1); // Ensure at least 1G

        if self.metadata.is_cubemap {
            if layer >= self.metadata.array_size {
                return Err(format!("Image index out of bounds: {} >= 6", layer));
            }
        } else {
            if layer >= max_slices {
                return Err(format!(
                    "Image index out of bounds: {} >= {}",
                    layer, max_slices
                ));
            }
        }

        let mut index = 0;
        for mip in 0..level {
            // Calculate the total number of images in previous mip levels
            let depth = 1.max(self.metadata.depth >> mip);
            index += (depth * self.metadata.array_size) as usize;
        }

        // Add the offset for the current layer and slice
        index += (layer * max_slices + slice) as usize;

        // Return the requested image
        self.images
            .get(index)
            .ok_or_else(|| format!("Invalid image index: {}", index))
    }

    pub fn get_rgba8_data(&self, level: u32, layer: u32, slice: u32) -> (u32, u32, Vec<u8>) {
        match self.get_image(level, layer, slice) {
            Ok(image_data) => self.get_rgba8_data_from_image(image_data),
            Err(e) => {
                eprintln!("Error getting image data: {}", e);
                (0, 0, Vec::new())
            }
        }
    }

    pub fn get_rgba8_data_from_image(&self, image: &Image) -> (u32, u32, Vec<u8>) {
        match PixelDecoder::decode(
            &image.pixels,
            image.format.pixel_format,
            image.format.pixel_data_type,
            image.format.color_space,
            image.width,
            image.height,
        ) {
            Ok(decoded_data) => (image.width, image.height, decoded_data),
            Err(e) => {
                eprintln!("Error decoding image data: {}", e);
                (0, 0, Vec::new())
            }
        }
    }

    pub fn rotate(&mut self, degrees: u32) {
        for image in self.images.iter_mut() {
            let (width, height) = (image.width, image.height);
            let mut rotated_data = vec![0; (width * height * 4) as usize];

            for y in 0..height {
                for x in 0..width {
                    let src_index = (y * width + x) as usize * 4;
                    let dest_index = ((width - x - 1) * height + y) as usize * 4;

                    rotated_data[dest_index] = image.pixels[src_index];
                    rotated_data[dest_index + 1] = image.pixels[src_index + 1];
                    rotated_data[dest_index + 2] = image.pixels[src_index + 2];
                    rotated_data[dest_index + 3] = image.pixels[src_index + 3];
                }
            }

            image.pixels = rotated_data;
            std::mem::swap(&mut image.width, &mut image.height);
        }
    }

    pub fn flip_left(&mut self) {
        for image in self.images.iter_mut() {
            let (width, height) = (image.width, image.height);
            let mut flipped_data = vec![0; (width * height * 4) as usize];

            for y in 0..height {
                for x in 0..width {
                    let src_index = (y * width + x) as usize * 4;
                    let dest_index = (y * width + (width - x - 1)) as usize * 4;

                    flipped_data[dest_index] = image.pixels[src_index];
                    flipped_data[dest_index + 1] = image.pixels[src_index + 1];
                    flipped_data[dest_index + 2] = image.pixels[src_index + 2];
                    flipped_data[dest_index + 3] = image.pixels[src_index + 3];
                }
            }

            image.pixels = flipped_data;
        }
    }

    pub fn flip_up(&mut self) {
        for image in self.images.iter_mut() {
            let (width, height) = (image.width, image.height);
            let mut flipped_data = vec![0; (width * height * 4) as usize];

            for y in 0..height {
                for x in 0..width {
                    let src_index = (y * width + x) as usize * 4;
                    let dest_index = ((height - y - 1) * width + x) as usize * 4;

                    flipped_data[dest_index] = image.pixels[src_index];
                    flipped_data[dest_index + 1] = image.pixels[src_index + 1];
                    flipped_data[dest_index + 2] = image.pixels[src_index + 2];
                    flipped_data[dest_index + 3] = image.pixels[src_index + 3];
                }
            }

            image.pixels = flipped_data;
        }
    }

    pub fn flip_down(&mut self) {
        for image in self.images.iter_mut() {
            let (width, height) = (image.width, image.height);
            let mut flipped_data = vec![0; (width * height * 4) as usize];

            for y in 0..height {
                for x in 0..width {
                    let src_index = (y * width + x) as usize * 4;
                    let dest_index = (y * width + (height - x - 1)) as usize * 4;

                    flipped_data[dest_index] = image.pixels[src_index];
                    flipped_data[dest_index + 1] = image.pixels[src_index + 1];
                    flipped_data[dest_index + 2] = image.pixels[src_index + 2];
                    flipped_data[dest_index + 3] = image.pixels[src_index + 3];
                }
            }

            image.pixels = flipped_data;
        }
    }

    pub fn flip_right(&mut self) {
        for image in self.images.iter_mut() {
            let (width, height) = (image.width, image.height);
            let mut flipped_data = vec![0; (width * height * 4) as usize];

            for y in 0..height {
                for x in 0..width {
                    let src_index = (y * width + x) as usize * 4;
                    let dest_index = (y * width + (width - x - 1)) as usize * 4;

                    flipped_data[dest_index] = image.pixels[src_index];
                    flipped_data[dest_index + 1] = image.pixels[src_index + 1];
                    flipped_data[dest_index + 2] = image.pixels[src_index + 2];
                    flipped_data[dest_index + 3] = image.pixels[src_index + 3];
                }
            }

            image.pixels = flipped_data;
        }
    }
}
