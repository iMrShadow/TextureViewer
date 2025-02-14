use std::error::Error;

use crate::graphics::pixel_format::PixelFormatInfo;

use super::texture_utility::TextureEffects;

#[derive(Default)]
pub struct Texture {
    pub metadata: TexMetadata,
    pub images: Vec<Image>,
}

#[derive(Default, Clone)]
pub struct Image {
    pub width: u32,
    pub height: u32,
    pub pixel_format_info: PixelFormatInfo,
    pub row_pitch: u32,
    pub slice_pitch: u32,
    pub pixels: Vec<u8>,
}

#[derive(Default, Clone, Copy)]
pub struct TexMetadata {
    pub width: u32,
    pub height: u32,
    pub depth: u32,
    pub array_size: u32,
    pub mip_levels: u32,
    pub pixel_format_info: PixelFormatInfo,
    pub alpha_mode: u32,
    pub dimensions: TexDimension,
    pub is_cubemap: bool,
    pub is_volumemap: bool,
}

#[derive(Default, Clone, Copy)]
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
                pixel_format_info: PixelFormatInfo::default(),
                alpha_mode: 0,
                dimensions: TexDimension::Tex2D,
                is_cubemap: false,
                is_volumemap: false,
            },
            images: Vec::new(),
        }
    }

    // Level (mip index), layer (array index), slice (z index)
    // DirectXTex - GetImage
    pub fn get_image(&self, mip: u32, item: u32, slice: u32) -> Result<&Image, Box<dyn Error>> {
        if mip >= self.metadata.mip_levels {
            return Err(format!(
                "Mip index out of bounds: {} >= {}",
                mip, self.metadata.mip_levels
            )
            .into());
        }

        match self.metadata.dimensions {
            TexDimension::Tex1D | TexDimension::Tex2D => {
                if slice > 0 {
                    return Err(format!("Slice index out of bounds: {} >= 0", slice).into());
                }

                if item >= self.metadata.array_size {
                    return Err(format!(
                        "Item index out of bounds: {} >= {}",
                        item, self.metadata.array_size
                    )
                    .into());
                }

                self.images
                    .get((item * self.metadata.mip_levels + mip) as usize)
                    .ok_or_else(|| {
                        format!(
                            "Image not found for mip: {}, item: {}, slice: {}",
                            mip, item, slice
                        )
                        .into()
                    })
            }

            TexDimension::Tex3D => {
                if item > 0 {
                    return Err(format!("Item index out of bounds: {} >= 0", item).into());
                }

                let mut index = 0;
                let mut depth = self.metadata.depth;

                for _ in 0..mip {
                    index += depth;
                    if depth > 1 {
                        depth >>= 1;
                    }
                }

                if slice >= depth {
                    return Err(format!("Slice index out of bounds: {} >= {}", slice, depth).into());
                }

                index += depth;

                self.images.get((index) as usize).ok_or_else(|| {
                    format!(
                        "Image not found for mip: {}, item: {}, slice: {}",
                        mip, item, slice
                    )
                    .into()
                })
            }
        }
    }

    pub fn get_rgba8_data(&self, level: u32, layer: u32, slice: u32) -> (u32, u32, Vec<u8>) {
        match self.get_image(level, layer, slice) {
            Ok(image_data) => (
                image_data.width,
                image_data.height,
                image_data.pixels.clone(),
            ),
            Err(e) => {
                eprintln!("Error getting image data: {}", e);
                (0, 0, Vec::new())
            }
        }
    }

    pub fn is_compressed(&self) -> bool {
        self.metadata.pixel_format_info.pixel_format.is_compressed()
    }

    /// Apply effects for saving the texture
    /// Pipeline: Deswizzle -> Decompress -> Decode -> Process -> Compress -> Swizzle
    pub fn new_transformed_texture(
        &self,
        effects: &TextureEffects,
    ) -> Result<Texture, Box<dyn Error>> {
        let mut metadata = self.metadata.clone();

        metadata.pixel_format_info = PixelFormatInfo {
            pixel_format: effects.pixel_format,
            pixel_data_type: super::pixel_format::PixelDataType::UNorm,
            color_space: super::pixel_format::ColorSpace::Linear,
            is_premultiplied: false,
        };

        let total_rotation =
            (effects.rotate_90_left_count as i32 % 4) - (effects.rotate_90_right_count as i32 % 4);

        if total_rotation != 0 {
            std::mem::swap(&mut metadata.width, &mut metadata.height);
        }

        let mut new_images = Vec::new();

        for image in &self.images {
            new_images.push(effects.get_transformed_pixels(image)?);
        }

        Ok(Texture {
            metadata,
            images: new_images,
        })
    }
}
