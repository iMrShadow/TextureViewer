fn bsr(x: usize) -> usize {
    if x == 0 {
        panic!("bsr is undefined for 0");
    }
    usize::BITS as usize - x.leading_zeros() as usize - 1
}

fn enclosing_power_of_2(x: usize) -> usize {
    1 << bsr(x + (x - 1))
}

fn align(value: usize, alignment: usize) -> usize {
    (value + (alignment - 1)) & !(alignment - 1)
}

fn get_morton_index_psvita_dreamcast(x: usize, y: usize, width: usize, height: usize) -> usize {
    let log_w = bsr(width);
    let log_h = bsr(height);
    let d = log_w.min(log_h);
    let mut index = 0;
    
    for i in 0..d {
        index |= ((x & (1 << i)) << (i + 1)) | ((y & (1 << i)) << i);
    }
    
    if width < height {
        index |= (y & !(width - 1)) << d;
    } else {
        index |= (x & !(height - 1)) << d;
    }
    
    index
}

fn unswizzle_psvita_dreamcast(pixel_data: &[u8], img_width: usize, img_height: usize, bpp: usize) -> Vec<u8> {
    let mut converted_data = vec![0; pixel_data.len()];
    let width_pow2 = enclosing_power_of_2(img_width);
    let height_pow2 = enclosing_power_of_2(img_height);
    
    let mx = get_morton_index_psvita_dreamcast(width_pow2 - 1, 0, width_pow2, height_pow2);
    let my = get_morton_index_psvita_dreamcast(0, height_pow2 - 1, width_pow2, height_pow2);
    let pixel_size = bpp / 8;
    
    let mut oy = 0;
    let mut dest_offset = 0;
    for y in 0..img_height {
        let mut ox = 0;
        for x in 0..img_width {
            let src_offset = (ox + oy) * pixel_size;
            converted_data[dest_offset..dest_offset + pixel_size].copy_from_slice(&pixel_data[src_offset..src_offset + pixel_size]);
            dest_offset += pixel_size;
            ox = (ox - mx) & mx;
        }
        oy = (oy - my) & my;
    }
    
    converted_data
}
