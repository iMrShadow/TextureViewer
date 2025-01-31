pub fn decode_r1_pixel(pixel: u8) -> [u8; 4] {
    let r8 = pixel & 0xFF;
    let g8 = pixel & 0xFF;
    let b8 = pixel & 0xFF;
    let a8 = 0xFF;
    return [r8, g8, b8, a8];
}

pub fn decode_l8_pixel(pixel: u8) -> [u8; 4] {
    let r8 = pixel;
    let g8 = pixel;
    let b8 = pixel;
    let a8 = 0xFF;
    return [r8, g8, b8, a8];
}

pub fn decode_la88_pixel(pixel: u16) -> [u8; 4] {
    let l = pixel & 0xFF;
    let a = (pixel >> 8) & 0xFF;

    let r8 = l as u8;
    let g8 = l as u8;
    let b8 = l as u8;
    let a8 = a as u8;

    return [r8, g8, b8, a8];
}

pub fn decode_l16_pixel(pixel: u16) -> [u8; 4] {
    let l = pixel & 0xFFFF;
    let l = l >> 8;

    let r8 = l as u8;
    let g8 = l as u8;
    let b8 = l as u8;
    let a8 = 0xFF as u8;

    return [r8, g8, b8, a8];
}

pub fn decode_bgrx5551_pixel(pixel: u16) -> [u8; 4] {
    let b = (pixel & 0x1F) as u8; // Extract 5-bit Blue
    let g = ((pixel >> 5) & 0x1F) as u8; // Extract 5-bit Green
    let r = ((pixel >> 10) & 0x1F) as u8; // Extract 5-bit Red

    // Convert 5-bit channels to 8-bit by scaling and bit-replicating
    let r8 = (r << 3) | (r >> 2); // Red: Scale up from 5 bits to 8 bits
    let g8 = (g << 3) | (g >> 2); // Green: Scale up from 5 bits to 8 bits
    let b8 = (b << 3) | (b >> 2); // Blue: Scale up from 5 bits to 8 bits
    let a8 = 0xFF;

    [r8, g8, b8, a8] // Return RGBA as an array
}

pub fn decode_bgra5551_pixel(pixel: u16) -> [u8; 4] {
    let b = (pixel & 0x1F) as u8; // Extract 5-bit Blue
    let g = ((pixel >> 5) & 0x1F) as u8; // Extract 5-bit Green
    let r = ((pixel >> 10) & 0x1F) as u8; // Extract 5-bit Red
    let a = ((pixel >> 15) & 0x1) as u8; // Extract 1-bit Alpha

    // Convert 5-bit channels to 8-bit by scaling and bit-replicating
    let r8 = (r << 3) | (r >> 2); // Red: Scale up from 5 bits to 8 bits
    let g8 = (g << 3) | (g >> 2); // Green: Scale up from 5 bits to 8 bits
    let b8 = (b << 3) | (b >> 2); // Blue: Scale up from 5 bits to 8 bits
    let a8 = if a == 0 { 0x00 } else { 0xFF }; // Alpha: Convert 1-bit to full 8-bit (0 or 255)

    [r8, g8, b8, a8] // Return RGBA as an array
}

pub fn decode_bgr565_pixel(pixel: u16) -> [u8; 4] {
    // Convert 5-bit channels to 8-bit by scaling and bit-replicating
    let r8 = (((pixel >> 11) & 0x1F) * 0xFF) as u8; // 0x1F
    let g8 = (((pixel >> 5) & 0x3F) * 0xFF) as u8; // 0x3F
    let b8 = (((pixel >> 0) & 0x1F) * 0xFF) as u8; // 0x1F
    let a8 = 0xFF;

    [r8, g8, b8, a8] // Return RGBA as an array
}
// TODO! It's not u32
pub fn decode_rgb888_pixel(pixel: &[u8]) -> [u8; 4] {
    // Convert 5-bit channels to 8-bit by scaling and bit-replicating
    let r8 = pixel[0]; // 0x1F
    let g8 = pixel[1]; // 0x3F
    let b8 = pixel[2]; // 0x1F
    let a8 = 0xFF;

    [r8, g8, b8, a8] // Return RGBA as an array
}

pub fn decode_bgr888_pixel(pixel: u32) -> [u8; 4] {
    // Convert 5-bit channels to 8-bit by scaling and bit-replicating
    let r8 = ((pixel >> 16) & 0xff) as u8; // 0x1F
    let g8 = ((pixel >> 8) & 0xff) as u8; // 0x3F
    let b8 = ((pixel >> 0) & 0xff) as u8; // 0x1F
    let a8 = 0xFF;

    [r8, g8, b8, a8] // Return RGBA as an array
}

pub fn decode_rgba8888_pixel(pixel: u32) -> [u8; 4] {
    // Convert 5-bit channels to 8-bit by scaling and bit-replicating
    let r8 = ((pixel >> 0) & 0xff) as u8; // 0x1F
    let g8 = ((pixel >> 8) & 0xff) as u8; // 0x3F
    let b8 = ((pixel >> 16) & 0xff) as u8; // 0x1F
    let a8 = ((pixel >> 24) & 0xff) as u8;

    [r8, g8, b8, a8] // Return RGBA as an array
}

pub fn decode_bgra8888_pixel(pixel: u32) -> [u8; 4] {
    // Convert 5-bit channels to 8-bit by scaling and bit-replicating
    let r8 = ((pixel >> 16) & 0xff) as u8; // 0x1F
    let g8 = ((pixel >> 8) & 0xff) as u8; // 0x3F
    let b8 = ((pixel >> 0) & 0xff) as u8; // 0x1F
    let a8 = ((pixel >> 24) & 0xff) as u8;

    [r8, g8, b8, a8] // Return RGBA as an array
}

pub fn decode_rgbx8888_pixel(pixel: u32) -> [u8; 4] {
    // Convert 5-bit channels to 8-bit by scaling and bit-replicating
    let r8 = ((pixel >> 0) & 0xff) as u8; // 0x1F
    let g8 = ((pixel >> 8) & 0xff) as u8; // 0x3F
    let b8 = ((pixel >> 16) & 0xff) as u8; // 0x1F
    let a8 = 0xFF;

    [r8, g8, b8, a8] // Return RGBA as an array
}

pub fn decode_bgrx8888_pixel(pixel: u32) -> [u8; 4] {
    // Convert 5-bit channels to 8-bit by scaling and bit-replicating
    let r8 = ((pixel >> 16) & 0xff) as u8; // 0x1F
    let g8 = ((pixel >> 8) & 0xff) as u8; // 0x3F
    let b8 = ((pixel >> 0) & 0xff) as u8; // 0x1F
    let a8 = 0xFF;

    [r8, g8, b8, a8] // Return RGBA as an array
}

pub fn decode_rgba4444_pixel(pixel: u16) -> [u8; 4] {
    // Convert 5-bit channels to 8-bit by scaling and bit-replicating
    let r8 = (((pixel >> 0) & 0xff) * 0x10) as u8; // 0x1F
    let g8 = (((pixel >> 4) & 0xff) * 0x10) as u8; // 0x3F
    let b8 = (((pixel >> 8) & 0xff) * 0x10) as u8; // 0x1F
    let a8 = (((pixel >> 12) & 0xff) * 0x10) as u8;

    [r8, g8, b8, a8] // Return RGBA as an array
}

pub fn decode_bgra4444_pixel(pixel: u16) -> [u8; 4] {
    // Convert 5-bit channels to 8-bit by scaling and bit-replicating
    let r8 = (((pixel >> 8) & 0xff) * 0x10) as u8; // 0x1F
    let g8 = (((pixel >> 4) & 0xff) * 0x10) as u8; // 0x3F
    let b8 = (((pixel >> 0) & 0xff) * 0x10) as u8; // 0x1F
    let a8 = (((pixel >> 12) & 0xff) * 0x10) as u8;

    [r8, g8, b8, a8] // Return RGBA as an array
}
//TODO: PIXEL???
pub fn decode_rgb161616_pixel(pixel: u64) -> [u8; 4] {
    // Convert 5-bit channels to 8-bit by scaling and bit-replicating
    let r8 = ((pixel >> 0) & 0xff) as u8; // 0x1F
    let g8 = ((pixel >> 16) & 0xff) as u8; // 0x3F
    let b8 = ((pixel >> 32) & 0xff) as u8; // 0x1F
    let a8 = ((pixel >> 48) & 0xff) as u8;

    [r8, g8, b8, a8] // Return RGBA as an array
}

pub fn decode_rgba16161616_pixel(pixel: u64) -> [u8; 4] {
    // Convert 5-bit channels to 8-bit by scaling and bit-replicating
    let r8 = ((pixel >> 0) & 0xff) as u8; // 0x1F
    let g8 = ((pixel >> 16) & 0xff) as u8; // 0x3F
    let b8 = ((pixel >> 32) & 0xff) as u8; // 0x1F
    let a8 = ((pixel >> 48) & 0xff) as u8;

    [r8, g8, b8, a8] // Return RGBA as an array
}

pub fn decode_rgba32323232_pixel(pixel: u128) -> [u8; 4] {
    // Convert 5-bit channels to 8-bit by scaling and bit-replicating
    let r8 = ((pixel >> 0) & 0xff) as u8; // 0x1F
    let g8 = ((pixel >> 16) & 0xff) as u8; // 0x3F
    let b8 = ((pixel >> 32) & 0xff) as u8; // 0x1F
    let a8 = ((pixel >> 48) & 0xff) as u8;

    [r8, g8, b8, a8] // Return RGBA as an array
}
