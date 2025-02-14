use std::error::Error;

use crate::graphics::pixel_format::PixelFormat;

use super::Swizzable;

// ReverseBox - https://github.com/bartlomiejduda/ReverseBox/tree/main/reversebox/image/swizzling
pub struct PS4Swizzler;

impl Swizzable for PS4Swizzler {
    fn swizzle(
        &self,
        pixels: &mut [u8],
        width: u32,
        height: u32,
        pixel_format: PixelFormat,
    ) -> Result<(), Box<dyn Error>> {
        swizzle_ps4(
            pixels,
            width,
            height,
            pixel_format.get_block_width(),
            pixel_format.get_block_height(),
            pixel_format.get_bytes_per_block(),
        );

        Ok(())
    }

    fn deswizzle(
        &self,
        pixels: &mut [u8],
        width: u32,
        height: u32,
        pixel_format: PixelFormat,
    ) -> Result<(), Box<dyn Error>> {
        unswizzle_ps4(
            pixels,
            width,
            height,
            pixel_format.get_block_width(),
            pixel_format.get_block_height(),
            pixel_format.get_bytes_per_block(),
        );

        Ok(())
    }

    fn get_platform(&self) -> super::Platform {
        super::Platform::PS4
    }
}

fn calculate_morton_index_ps4(t: u32, input_img_width: u32, input_img_height: u32) -> u32 {
    let (mut num1, mut num2, mut num3, mut num4) = (1, 1, 0, 0);
    let (mut img_width, mut img_height) = (input_img_width, input_img_height);
    let mut t = t;

    while img_width > 1 || img_height > 1 {
        if img_width > 1 {
            num3 += num2 * (t & 1);
            t >>= 1;
            num2 *= 2;
            img_width >>= 1;
        }
        if img_height > 1 {
            num4 += num1 * (t & 1);
            t >>= 1;
            num1 *= 2;
            img_height >>= 1;
        }
    }
    num4 * input_img_width + num3
}

fn unswizzle_ps4(
    image_data: &mut [u8],
    img_width: u32,
    img_height: u32,
    block_width: u32,
    block_height: u32,
    block_data_size: u32,
) {
    let mut temp_data = image_data.to_vec();
    let mut source_index = 0;
    let img_height = img_height / block_height;
    let img_width = img_width / block_width;

    for y in 0..((img_height + 7) / 8) {
        for x in 0..((img_width + 7) / 8) {
            for t in 0..64 {
                let morton_index = calculate_morton_index_ps4(t, 8, 8);
                let data_y = morton_index / 8;
                let data_x = morton_index % 8;
                if x * 8 + data_x < img_width && y * 8 + data_y < img_height {
                    let destination_index = (block_data_size
                        * ((y * 8 + data_y) * img_width + x * 8 + data_x))
                        as usize;
                    temp_data[destination_index..destination_index + block_data_size as usize]
                        .copy_from_slice(
                            &image_data[source_index..source_index + block_data_size as usize],
                        );
                    source_index += block_data_size as usize;
                }
            }
        }
    }

    image_data.copy_from_slice(&temp_data);
}

fn swizzle_ps4(
    image_data: &mut [u8],
    img_width: u32,
    img_height: u32,
    block_width: u32,
    block_height: u32,
    block_data_size: u32,
) {
    let mut temp_data = image_data.to_vec();
    let mut source_index = 0;
    let img_height = img_height / block_height;
    let img_width = img_width / block_width;

    for y in 0..((img_height + 7) / 8) {
        for x in 0..((img_width + 7) / 8) {
            for t in 0..64 {
                let morton_index = calculate_morton_index_ps4(t, 8, 8);
                let data_y = morton_index / 8;
                let data_x = morton_index % 8;
                if x * 8 + data_x < img_width && y * 8 + data_y < img_height {
                    let destination_index = (block_data_size
                        * ((y * 8 + data_y) * img_width + x * 8 + data_x))
                        as usize;
                    temp_data[source_index..source_index + block_data_size as usize]
                        .copy_from_slice(
                            &image_data
                                [destination_index..destination_index + block_data_size as usize],
                        );
                    source_index += block_data_size as usize;
                }
            }
        }
    }
    image_data.copy_from_slice(&temp_data);
}
