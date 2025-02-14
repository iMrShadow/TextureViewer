use std::fmt;

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
    R8G8,
    L8,
    L8A8,
    B4G4R4A4,
    R8G8B8,
    B8G8R8,
    R8G8B8A8,
    B8G8R8A8,
    B8G8R8X8,
    B5G6R5,
    B5G5R5A1,
    B5G5R5X1,
    R16,
    L16, // Legacy 
    R16G16,
    L16A16, 
    R16G16B16,
    R16G16B16A16,
    R32,
    R32G32,
    R32G32B32,
    R32G32B32A32,
    R10G10B10A2,
    R11G11B10,
    R9G9B9E5,
    D16,
    BC1, // DXT1
    BC2, // DXT2 (premultiplied) / DXT3
    BC3, // DXT4 (premultiplied) / DXT5
    BC4, // ATI1
    BC5, // ATI2
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

impl fmt::Display for PixelFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            PixelFormat::Unknown => "Unknown",
            PixelFormat::R1 => "R1",
            PixelFormat::A8 => "A8",
            PixelFormat::R8 => "R8",
            PixelFormat::R8G8 => "R8G8",
            PixelFormat::L8 => "L8",
            PixelFormat::L8A8 => "L8A8",
            PixelFormat::B4G4R4A4 => "B4G4R4A4",
            PixelFormat::R8G8B8 => "R8G8B8",
            PixelFormat::B8G8R8 => "B8G8R8",
            PixelFormat::R8G8B8A8 => "R8G8B8A8",
            PixelFormat::B8G8R8A8 => "B8G8R8A8",
            PixelFormat::B8G8R8X8 => "B8G8R8X8",
            PixelFormat::B5G6R5 => "B5G6R5",
            PixelFormat::B5G5R5A1 => "B5G5R5A1",
            PixelFormat::B5G5R5X1 => "B5G5R5X1",
            PixelFormat::R16 => "R16",
            PixelFormat::L16 => "L16",
            PixelFormat::R16G16 => "R16G16",
            PixelFormat::L16A16 => "L16A16",
            PixelFormat::R16G16B16 => "R16G16B16",
            PixelFormat::R16G16B16A16 => "R16G16B16A16",
            PixelFormat::R32 => "R32",
            PixelFormat::R32G32 => "R32G32",
            PixelFormat::R32G32B32 => "R32G32B32",
            PixelFormat::R32G32B32A32 => "R32G32B32A32",
            PixelFormat::R10G10B10A2 => "R10G10B10A2",
            PixelFormat::R11G11B10 => "R11G11B10",
            PixelFormat::R9G9B9E5 => "R9G9B9E5 Shared Exp",
            PixelFormat::D16 => "D16",
            PixelFormat::BC1 => "BC1",
            PixelFormat::BC2 => "BC2",
            PixelFormat::BC3 => "BC3",
            PixelFormat::BC4 => "BC4",
            PixelFormat::BC5 => "BC5",
            PixelFormat::BC6H => "BC6H",
            PixelFormat::BC7 => "BC7",
        };

        write!(f, "{}", name)
    }
}

impl Default for PixelFormatInfo {
    fn default() -> Self {
        Self {
            pixel_format: PixelFormat::R8G8B8A8,
            pixel_data_type: PixelDataType::UNorm,
            color_space: ColorSpace::Linear,
            is_premultiplied: false,
        }
    }
}

impl PixelFormat {
    pub fn get_bits_per_pixel(self) -> u32 {
        match self {
            PixelFormat::R32G32B32A32 => 128,
            PixelFormat::R32G32B32 => 96,

            PixelFormat::R32G32 | PixelFormat::R16G16B16A16 => 64,

            PixelFormat::R16G16B16 => 48,
            PixelFormat::R32
            | PixelFormat::R16G16
            | PixelFormat::L16A16
            | PixelFormat::R8G8B8A8
            | PixelFormat::B8G8R8A8
            | PixelFormat::B8G8R8X8
            | PixelFormat::R9G9B9E5
            | PixelFormat::R10G10B10A2
            | PixelFormat::R11G11B10 => 32,

            PixelFormat::R8G8B8 | PixelFormat::B8G8R8 => 24,

            PixelFormat::R16
            | PixelFormat::L16
            | PixelFormat::R8G8
            | PixelFormat::L8A8
            | PixelFormat::B5G6R5
            | PixelFormat::B4G4R4A4 => 16,

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

    pub fn get_slice_pitch(self, width: u32, height: u32) -> u32 {
        if self.is_compressed() {
            let bh = std::cmp::max(1, (height + 3) / 4);
            return bh * self.get_row_pitch(width);
        }
        self.get_row_pitch(width) * height
    }

    pub fn get_row_pitch(self, width: u32) -> u32 {
        if self.is_compressed() {
            let bw = std::cmp::max(1, (width + 3) / 4);
            return bw * self.get_bytes_per_block();
        }

        (self.get_bits_per_pixel() * width + 7) / 8
    }

    pub fn get_bytes_per_block(self) -> u32 {
        match self {
            PixelFormat::BC1 | PixelFormat::BC4 => 8,
            PixelFormat::BC2 | PixelFormat::BC3 | PixelFormat::BC5 => 16,
            PixelFormat::BC6H => 16,
            PixelFormat::BC7 => 16,
            _ => self.get_bits_per_pixel() / 8,
        }
    }

    pub fn get_block_width(self) -> u32 {
        match self {
            PixelFormat::BC1
            | PixelFormat::BC2
            | PixelFormat::BC3
            | PixelFormat::BC4
            | PixelFormat::BC5
            | PixelFormat::BC6H
            | PixelFormat::BC7 => 4,
            _ => 1,
        }
    }

    pub fn get_block_height(self) -> u32 {
        match self {
            PixelFormat::BC1
            | PixelFormat::BC2
            | PixelFormat::BC3
            | PixelFormat::BC4
            | PixelFormat::BC5
            | PixelFormat::BC6H
            | PixelFormat::BC7 => 4,
            _ => 1,
        }
    }

    pub fn is_compressed(self) -> bool {
        match self {
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
}
