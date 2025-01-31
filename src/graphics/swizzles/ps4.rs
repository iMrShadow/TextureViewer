fn calculate_morton_index_ps4(mut t: usize, input_img_width: usize, input_img_height: usize) -> usize {
    let (mut num1, mut num2, mut num3, mut num4) = (1, 1, 0, 0);
    let (mut img_width, mut img_height) = (input_img_width, input_img_height);
    
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

fn unswizzle_ps4(image_data: &[u8], img_width: usize, img_height: usize, block_width: usize, block_height: usize, block_data_size: usize) -> Vec<u8> {
    let mut unswizzled_data = vec![0; image_data.len()];
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
                    let destination_index = block_data_size * ((y * 8 + data_y) * img_width + x * 8 + data_x);
                    unswizzled_data[destination_index..destination_index + block_data_size]
                        .copy_from_slice(&image_data[source_index..source_index + block_data_size]);
                    source_index += block_data_size;
                }
            }
        }
    }
    
    unswizzled_data
}

fn swizzle_ps4(image_data: &[u8], img_width: usize, img_height: usize, block_width: usize, block_height: usize, block_data_size: usize) -> Vec<u8> {
    let mut swizzled_data = vec![0; image_data.len()];
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
                    let destination_index = block_data_size * ((y * 8 + data_y) * img_width + x * 8 + data_x);
                    swizzled_data[source_index..source_index + block_data_size]
                        .copy_from_slice(&image_data[destination_index..destination_index + block_data_size]);
                    source_index += block_data_size;
                }
            }
        }
    }
    
    swizzled_data
}
