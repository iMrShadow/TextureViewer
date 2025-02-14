pub fn encode_rgb888_pixel(pixel: &[u8; 4]) -> [u8; 3] {
    let r8 = pixel[0];
    let g8 = pixel[1];
    let b8 = pixel[2];

    [r8, g8, b8]
}

pub fn encode_bgr888_pixel(pixel: &[u8; 4]) -> [u8; 3] {
    let r8 = pixel[2];
    let g8 = pixel[1];
    let b8 = pixel[0];

    [r8, g8, b8]
}
