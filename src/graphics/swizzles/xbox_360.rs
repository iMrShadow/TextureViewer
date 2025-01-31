fn swap_byte_order(image_data: &[u8]) -> Vec<u8> {
    if image_data.len() % 2 != 0 {
        panic!("Data size must be a multiple of 2 bytes!");
    }
    
    let mut swapped_data = Vec::with_capacity(image_data.len());
    for chunk in image_data.chunks(2) {
        swapped_data.extend(chunk.iter().rev());
    }
    
    swapped_data
}

fn xg_address_2d_tiled_x(block_offset: usize, width_in_blocks: usize, texel_byte_pitch: usize) -> usize {
    let aligned_width = (width_in_blocks + 31) & !31;
    let log_bpp = (texel_byte_pitch >> 2) + ((texel_byte_pitch >> 1) >> (texel_byte_pitch >> 2));
    let offset_byte = block_offset << log_bpp;
    let offset_tile = (((offset_byte & !0xFFF) >> 3) + ((offset_byte & 0x700) >> 2) + (offset_byte & 0x3F));
    let offset_macro = offset_tile >> (7 + log_bpp);

    let macro_x = (offset_macro % (aligned_width >> 5)) << 2;
    let tile = (((offset_tile >> (5 + log_bpp)) & 2) + (offset_byte >> 6)) & 3;
    let macro_val = (macro_x + tile) << 3;
    let micro = ((((offset_tile >> 1) & !0xF) + (offset_tile & 0xF)) & ((texel_byte_pitch << 3) - 1)) >> log_bpp;

    macro_val + micro
}

fn xg_address_2d_tiled_y(block_offset: usize, width_in_blocks: usize, texel_byte_pitch: usize) -> usize {
    let aligned_width = (width_in_blocks + 31) & !31;
    let log_bpp = (texel_byte_pitch >> 2) + ((texel_byte_pitch >> 1) >> (texel_byte_pitch >> 2));
    let offset_byte = block_offset << log_bpp;
    let offset_tile = (((offset_byte & !0xFFF) >> 3) + ((offset_byte & 0x700) >> 2) + (offset_byte & 0x3F));
    let offset_macro = offset_tile >> (7 + log_bpp);

    let macro_y = (offset_macro / (aligned_width >> 5)) << 2;
    let tile = ((offset_tile >> (6 + log_bpp)) & 1) + ((offset_byte & 0x800) >> 10);
    let macro_val = (macro_y + tile) << 3;
    let micro = (((offset_tile & ((texel_byte_pitch << 6) - 1 & !0x1F)) + ((offset_tile & 0xF) << 1)) >> (3 + log_bpp)) & !1;

    macro_val + micro + ((offset_tile & 0x10) >> 4)
}

fn convert_x360_image_data(image_data: &[u8], image_width: usize, image_height: usize, block_pixel_size: usize, texel_byte_pitch: usize, swizzle_flag: bool) -> Vec<u8> {
    let mut converted_data = vec![0; image_data.len()];
    let width_in_blocks = image_width / block_pixel_size;
    let height_in_blocks = image_height / block_pixel_size;

    for j in 0..height_in_blocks {
        for i in 0..width_in_blocks {
            let block_offset = j * width_in_blocks + i;
            let x = xg_address_2d_tiled_x(block_offset, width_in_blocks, texel_byte_pitch);
            let y = xg_address_2d_tiled_y(block_offset, width_in_blocks, texel_byte_pitch);
            let src_byte_offset = (j * width_in_blocks * texel_byte_pitch) + (i * texel_byte_pitch);
            let dest_byte_offset = (y * width_in_blocks * texel_byte_pitch) + (x * texel_byte_pitch);
            
            if dest_byte_offset + texel_byte_pitch > converted_data.len() {
                continue;
            }
            
            if !swizzle_flag {
                converted_data[dest_byte_offset..dest_byte_offset + texel_byte_pitch]
                    .copy_from_slice(&image_data[src_byte_offset..src_byte_offset + texel_byte_pitch]);
            } else {
                converted_data[src_byte_offset..src_byte_offset + texel_byte_pitch]
                    .copy_from_slice(&image_data[dest_byte_offset..dest_byte_offset + texel_byte_pitch]);
            }
        }
    }
    
    converted_data
}

fn unswizzle_x360(image_data: &[u8], img_width: usize, img_height: usize, block_pixel_size: usize, texel_byte_pitch: usize) -> Vec<u8> {
    let swapped_data = swap_byte_order(image_data);
    convert_x360_image_data(&swapped_data, img_width, img_height, block_pixel_size, texel_byte_pitch, false)
}

fn swizzle_x360(image_data: &[u8], img_width: usize, img_height: usize, block_pixel_size: usize, texel_byte_pitch: usize) -> Vec<u8> {
    let swapped_data = swap_byte_order(image_data);
    convert_x360_image_data(&swapped_data, img_width, img_height, block_pixel_size, texel_byte_pitch, true)
}
