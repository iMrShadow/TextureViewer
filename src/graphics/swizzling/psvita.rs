use std::error::Error;

use crate::graphics::pixel_format::PixelFormat;

use super::Swizzable;

// ReverseBox - https://github.com/bartlomiejduda/ReverseBox/tree/main/reversebox/image/swizzling
pub struct PSVitaSwizzler;

impl Swizzable for PSVitaSwizzler {
    fn swizzle(
        &self,
        pixels: &mut [u8],
        width: u32,
        height: u32,
        pixel_format: PixelFormat,
    ) -> Result<(), Box<dyn Error>> {
        if !is_supported_format(pixel_format) {
            return Err("Unsupported swizzling format!".into());
        }
        vita_swizzle(
            pixels,
            width as usize,
            height as usize,
            pixel_format.get_bytes_per_block() as usize,
            pixel_format.get_bits_per_pixel() as usize,
            0,
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
        if !is_supported_format(pixel_format) {
            println!("{}", pixel_format.to_string());
            return Err("Unsupported swizzling format!".into());
        }

        vita_deswizzle(
            pixels,
            width as usize,
            height as usize,
            pixel_format.get_bytes_per_block() as usize,
            pixel_format.get_bits_per_pixel() as usize,
        );
        Ok(())
    }

    fn get_platform(&self) -> super::Platform {
        super::Platform::PSVita
    }
}

fn is_supported_format(pixel_format: PixelFormat) -> bool {
    match pixel_format {
        PixelFormat::R8G8B8A8
        | PixelFormat::B8G8R8A8
        | PixelFormat::R16G16B16A16
        | PixelFormat::R32G32B32A32 => true,
        _ => false,
    }
}

fn vita_deswizzle(
    image_data: &mut [u8],
    width: usize,
    height: usize,
    bytes_per_pixel: usize,
    format_bpp: usize,
) {
    if bytes_per_pixel >= image_data.len() {
        return;
    }
    let buffer_size = (format_bpp * width * height) / 8;
    let mut temp_data = vec![0; buffer_size.max(bytes_per_pixel)];

    let max_u = (width as f64).log2() as usize;
    let max_v = (height as f64).log2() as usize;

    for j in 0..(width * height) {
        if j * bytes_per_pixel >= image_data.len() {
            break;
        }
        let mut u = 0;
        let mut v = 0;
        let mut orig_coord = j;
        for k in 0..max_u.max(max_v) {
            if k < max_v {
                v |= (orig_coord & 1) << k;
                orig_coord >>= 1;
            }
            if k < max_u {
                u |= (orig_coord & 1) << k;
                orig_coord >>= 1;
            }
        }
        if u < width && v < height {
            let src_idx = j * bytes_per_pixel;
            let dst_idx = (v * width + u) * bytes_per_pixel;
            temp_data[dst_idx..dst_idx + bytes_per_pixel]
                .copy_from_slice(&image_data[src_idx..src_idx + bytes_per_pixel]);
        }
    }
    image_data.copy_from_slice(&temp_data);
}

fn vita_swizzle(
    image_data: &mut [u8],
    width: usize,
    height: usize,
    bytes_per_pixel: usize,
    format_bpp: usize,
    min_buffer_size: usize,
) {
    if bytes_per_pixel >= image_data.len() {
        return;
    }
    let buffer_size = (format_bpp * width * height) / 8;
    let mut temp_data = vec![0; buffer_size.max(min_buffer_size)];

    let max_u = (width as f64).log2() as usize;
    let max_v = (height as f64).log2() as usize;

    for j in 0..(width * height) {
        if j * bytes_per_pixel >= image_data.len() {
            break;
        }
        let mut u = 0;
        let mut v = 0;
        let mut orig_coord = j;
        for k in 0..max_u.max(max_v) {
            if k < max_v {
                v |= (orig_coord & 1) << k;
                orig_coord >>= 1;
            }
            if k < max_u {
                u |= (orig_coord & 1) << k;
                orig_coord >>= 1;
            }
        }
        if u < width && v < height {
            let src_idx = (v * width + u) * bytes_per_pixel;
            let dst_idx = j * bytes_per_pixel;
            temp_data[dst_idx..dst_idx + bytes_per_pixel]
                .copy_from_slice(&image_data[src_idx..src_idx + bytes_per_pixel]);
        }
    }
    image_data.copy_from_slice(&temp_data);
}
