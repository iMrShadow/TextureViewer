use image::Pixel;

#[derive(Debug, Clone, Copy)]
pub struct PixelFormatInfo {
    pub pixel_format: PixelFormat,
    pub pixel_data_type: PixelDataType,
    pub color_space: ColorSpace,
    pub is_premultiplied: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PixelFormat {
    Unknown,
    R1,
    A8,
    R8,
    RG8,
    L8,
    LA8,
    RGBA4,
    BGRA4,
    RGB8,
    BGR8,
    RGBA8,
    BGRA8,
    BGRX8,
    R5G6B5,
    B5G6R5,
    B5G5R5A1,
    B5G5R5X1,
    R16,
    L16,
    RG16,
    LA16,
    RGB16,
    RGBA16,
    R32,
    RG32,
    RGB32,
    RGBA32,
    R10G10B10A2,
    R11G11B10,
    R9G9B9E5SharedExp,
    D16,
    BC1, // DXT1
    BC2, // DXT2 / DXT3
    BC3, // DXT4 / DXT5
    BC4,
    BC5,
    BC6H,
    BC7,
}

#[derive(Debug, Clone, Copy)]
pub enum PixelDataType {
    SNorm,
    UNorm,
    SInt,
    UInt,
    Float,
}

#[derive(Debug, Clone, Copy)]
pub enum ColorSpace {
    Linear,
    SRGB,
}

impl Default for PixelFormatInfo {
    fn default() -> Self {
        Self {
            pixel_format: PixelFormat::Unknown,
            pixel_data_type: PixelDataType::UNorm,
            color_space: ColorSpace::Linear,
            is_premultiplied: false,
        }
    }
}

pub fn get_slice_pitch(width: u32, height: u32, format: &PixelFormat) -> u32 {
    if is_compressed_format(format) {
        let nbh = std::cmp::max(1, (height + 3) / 4);
        return nbh * get_row_pitch(width, format);
    }
    get_row_pitch(width, format) * height
}

pub fn get_row_pitch(width: u32, format: &PixelFormat) -> u32 {
    if is_compressed_format(format) {
        let nbw = std::cmp::max(1, (width + 3) / 4);
        return nbw * get_bytes_per_block(format);
    }
    (get_bits_per_pixel(format) * width + 7) / 8
}

pub fn get_bits_per_pixel(format: &PixelFormat) -> u32 {
    match format {
        PixelFormat::RGBA32 => 128,
        PixelFormat::RGB32 => 96,

        PixelFormat::RG32 | PixelFormat::RGBA16 => 64,

        PixelFormat::RGB16 => 48,
        PixelFormat::R32
        | PixelFormat::RG16
        | PixelFormat::LA16
        | PixelFormat::RGBA8
        | PixelFormat::BGRA8
        | PixelFormat::BGRX8
        | PixelFormat::R9G9B9E5SharedExp
        | PixelFormat::R10G10B10A2
        | PixelFormat::R11G11B10 => 32,

        PixelFormat::R16
        | PixelFormat::L16
        | PixelFormat::RG8
        | PixelFormat::LA8
        | PixelFormat::R5G6B5
        | PixelFormat::B5G6R5
        | PixelFormat::BGRA4
        | PixelFormat::RGBA4 => 16,

        PixelFormat::R8
        | PixelFormat::L8
        | PixelFormat::A8
        | PixelFormat::BC2
        | PixelFormat::BC3
        | PixelFormat::BC5
        | PixelFormat::BC6H
        | PixelFormat::BC7 => 8,

        PixelFormat::BC1 | PixelFormat::BC4 => 4,

        PixelFormat::R1 => 1,
        _ => 0,
    }
}

pub fn get_bytes_per_block(format: &PixelFormat) -> u32 {
    match format {
        PixelFormat::BC1 | PixelFormat::BC4 => 8,
        PixelFormat::BC2 | PixelFormat::BC3 | PixelFormat::BC5 => 16,
        PixelFormat::BC6H => 16,
        PixelFormat::BC7 => 16,
        _ => 0,
    }
}

pub fn is_compressed_format(format: &PixelFormat) -> bool {
    match format {
        PixelFormat::BC1
        | PixelFormat::BC2
        | PixelFormat::BC3
        | PixelFormat::BC4
        | PixelFormat::BC5
        | PixelFormat::BC6H
        | PixelFormat::BC7 => true,
        _ => false,
    }
}
