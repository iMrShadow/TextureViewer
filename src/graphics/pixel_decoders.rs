pub fn decode_rgb888_pixel(pixel: &[u8]) -> [u8; 4] {
    let r8 = pixel[0];
    let g8 = pixel[1];
    let b8 = pixel[2];
    let a8 = 0xFF;

    [r8, g8, b8, a8]
}

pub fn decode_bgr888_pixel(pixel: &[u8]) -> [u8; 4] {
    let r8 = pixel[2];
    let g8 = pixel[1];
    let b8 = pixel[0];
    let a8 = 0xFF;

    [r8, g8, b8, a8]
}
