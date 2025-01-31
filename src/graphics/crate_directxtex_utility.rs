use directxtex::{
    Image, CP_FLAGS, DXGI_FORMAT, DXGI_FORMAT_R8G8B8A8_UNORM,
};

use super::{
    pixel_format::{ColorSpace, PixelDataType, PixelFormat, PixelFormatInfo},
    texture::TexDimension,
};

pub fn get_pixel_format_from_image(dxgi_format: directxtex::DXGI_FORMAT) -> PixelFormat {
    match dxgi_format {
        directxtex::DXGI_FORMAT_R1_UNORM => PixelFormat::R1,
        directxtex::DXGI_FORMAT_A8_UNORM => PixelFormat::A8,
        directxtex::DXGI_FORMAT_R8_UNORM => PixelFormat::R8,
        directxtex::DXGI_FORMAT_R8G8_UNORM => PixelFormat::RG8,
        directxtex::DXGI_FORMAT_R8G8B8A8_UNORM => PixelFormat::RGBA8,
        directxtex::DXGI_FORMAT_R8G8B8A8_UNORM_SRGB => PixelFormat::RGBA8,
        directxtex::DXGI_FORMAT_B8G8R8A8_UNORM => PixelFormat::BGRA8,
        directxtex::DXGI_FORMAT_B8G8R8A8_UNORM_SRGB => PixelFormat::BGRA8,
        directxtex::DXGI_FORMAT_B8G8R8X8_UNORM => PixelFormat::BGRX8,
        directxtex::DXGI_FORMAT_B8G8R8X8_UNORM_SRGB => PixelFormat::BGRX8,
        directxtex::DXGI_FORMAT_B5G6R5_UNORM => PixelFormat::B5G6R5,
        directxtex::DXGI_FORMAT_B5G5R5A1_UNORM => PixelFormat::B5G5R5A1,
        directxtex::DXGI_FORMAT_R16_UNORM => PixelFormat::R16,
        directxtex::DXGI_FORMAT_R16G16_UNORM => PixelFormat::RG16,
        directxtex::DXGI_FORMAT_R16G16B16A16_UNORM => PixelFormat::RGBA16,
        directxtex::DXGI_FORMAT_R32_FLOAT => PixelFormat::R32,
        directxtex::DXGI_FORMAT_R32G32_FLOAT => PixelFormat::RG32,
        directxtex::DXGI_FORMAT_R32G32B32_FLOAT => PixelFormat::RGB32,
        directxtex::DXGI_FORMAT_R32G32B32A32_FLOAT => PixelFormat::RGBA32,
        directxtex::DXGI_FORMAT_R10G10B10A2_UNORM => PixelFormat::R10G10B10A2,
        directxtex::DXGI_FORMAT_R11G11B10_FLOAT => PixelFormat::R11G11B10,
        directxtex::DXGI_FORMAT_R9G9B9E5_SHAREDEXP => PixelFormat::R9G9B9E5SharedExp,
        directxtex::DXGI_FORMAT_D16_UNORM => PixelFormat::D16,
        directxtex::DXGI_FORMAT_BC1_UNORM => PixelFormat::BC1,
        directxtex::DXGI_FORMAT_BC1_UNORM_SRGB => PixelFormat::BC1,
        directxtex::DXGI_FORMAT_BC2_UNORM => PixelFormat::BC2,
        directxtex::DXGI_FORMAT_BC2_UNORM_SRGB => PixelFormat::BC2,
        directxtex::DXGI_FORMAT_BC3_UNORM => PixelFormat::BC3,
        directxtex::DXGI_FORMAT_BC3_UNORM_SRGB => PixelFormat::BC3,
        directxtex::DXGI_FORMAT_BC4_UNORM => PixelFormat::BC4,
        directxtex::DXGI_FORMAT_BC5_UNORM => PixelFormat::BC5,
        directxtex::DXGI_FORMAT_BC6H_UF16 => PixelFormat::BC6H,
        directxtex::DXGI_FORMAT_BC7_UNORM => PixelFormat::BC7,
        directxtex::DXGI_FORMAT_BC7_UNORM_SRGB => PixelFormat::BC7,
        _ => PixelFormat::Unknown,
    }
}

pub fn get_dxgi_format_from_pixel_format(pixel_format: PixelFormat) -> DXGI_FORMAT {
    match pixel_format {
        PixelFormat::R1 => DXGI_FORMAT::DXGI_FORMAT_R1_UNORM,
        PixelFormat::A8 => DXGI_FORMAT::DXGI_FORMAT_A8_UNORM,
        PixelFormat::R8 => DXGI_FORMAT::DXGI_FORMAT_R8_UNORM,
        PixelFormat::RG8 => DXGI_FORMAT::DXGI_FORMAT_R8G8_UNORM,
        PixelFormat::RGBA8 => DXGI_FORMAT::DXGI_FORMAT_R8G8B8A8_UNORM,
        PixelFormat::BGRA8 => DXGI_FORMAT::DXGI_FORMAT_B8G8R8A8_UNORM,
        PixelFormat::BGRX8 => DXGI_FORMAT::DXGI_FORMAT_B8G8R8X8_UNORM,

        PixelFormat::B5G6R5 => DXGI_FORMAT::DXGI_FORMAT_B5G6R5_UNORM,
        PixelFormat::B5G5R5A1 => DXGI_FORMAT::DXGI_FORMAT_B5G5R5A1_UNORM,

        PixelFormat::R16 => DXGI_FORMAT::DXGI_FORMAT_R16_UNORM,
        PixelFormat::RG16 => DXGI_FORMAT::DXGI_FORMAT_R16G16_UNORM,
        PixelFormat::RGBA16 => DXGI_FORMAT::DXGI_FORMAT_R16G16B16A16_UNORM,

        PixelFormat::R32 => DXGI_FORMAT::DXGI_FORMAT_R32_FLOAT,
        PixelFormat::RG32 => DXGI_FORMAT::DXGI_FORMAT_R32G32_FLOAT,
        PixelFormat::RGB32 => DXGI_FORMAT::DXGI_FORMAT_R32G32B32_FLOAT,
        PixelFormat::RGBA32 => DXGI_FORMAT::DXGI_FORMAT_R32G32B32A32_FLOAT,

        PixelFormat::R10G10B10A2 => DXGI_FORMAT::DXGI_FORMAT_R10G10B10A2_UNORM,
        PixelFormat::R11G11B10 => DXGI_FORMAT::DXGI_FORMAT_R11G11B10_FLOAT,
        PixelFormat::R9G9B9E5SharedExp => DXGI_FORMAT::DXGI_FORMAT_R9G9B9E5_SHAREDEXP,

        PixelFormat::D16 => DXGI_FORMAT::DXGI_FORMAT_D16_UNORM,

        PixelFormat::BC1 => DXGI_FORMAT::DXGI_FORMAT_BC1_UNORM,
        PixelFormat::BC2 => DXGI_FORMAT::DXGI_FORMAT_BC2_UNORM,
        PixelFormat::BC3 => DXGI_FORMAT::DXGI_FORMAT_BC3_UNORM,
        PixelFormat::BC4 => DXGI_FORMAT::DXGI_FORMAT_BC4_UNORM,
        PixelFormat::BC5 => DXGI_FORMAT::DXGI_FORMAT_BC5_UNORM,
        PixelFormat::BC6H => DXGI_FORMAT::DXGI_FORMAT_BC6H_UF16,
        PixelFormat::BC7 => DXGI_FORMAT::DXGI_FORMAT_BC7_UNORM,

        _ => DXGI_FORMAT::DXGI_FORMAT_UNKNOWN,
    }
}

pub fn get_pixel_format_datatype_from_image(
    dxgi_format: directxtex::DXGI_FORMAT,
) -> Option<PixelDataType> {
    match dxgi_format.format_data_type() {
        directxtex::FORMAT_TYPE_UNORM => Some(PixelDataType::UNorm),
        directxtex::FORMAT_TYPE_SNORM => Some(PixelDataType::SNorm),
        directxtex::FORMAT_TYPE_UINT => Some(PixelDataType::UInt),
        directxtex::FORMAT_TYPE_SINT => Some(PixelDataType::SInt),
        directxtex::FORMAT_TYPE_FLOAT => Some(PixelDataType::Float),
        _ => None,
    }
}

pub fn get_pixel_format_info_from_image(metadata: &directxtex::TexMetadata) -> PixelFormatInfo {
    let dxgi_format = metadata.format;
    let pixel_format = get_pixel_format_from_image(dxgi_format);
    let pixel_data_type = get_pixel_format_datatype_from_image(dxgi_format).unwrap();
    let color_space = if dxgi_format.is_srgb() {
        ColorSpace::SRGB
    } else {
        ColorSpace::Linear
    };

    let is_premultiplied = metadata.is_pm_alpha();

    PixelFormatInfo {
        pixel_format,
        pixel_data_type,
        color_space,
        is_premultiplied,
    }
}

pub fn get_texture_dimension(dimension: directxtex::TEX_DIMENSION) -> TexDimension {
    match dimension {
        directxtex::TEX_DIMENSION_TEXTURE1D => TexDimension::Tex1D,
        directxtex::TEX_DIMENSION_TEXTURE2D => TexDimension::Tex2D,
        directxtex::TEX_DIMENSION_TEXTURE3D => TexDimension::Tex3D,
        _ => TexDimension::Tex2D,
    }
}

pub fn decompress_bc_to_rgba(
    pixel_format: PixelFormat,
    width: usize,
    height: usize,
    bc1_data: Vec<u8>,
) -> Result<Vec<u8>, String> {
    // Create an Image instance from the BC1 data

    let format = get_dxgi_format_from_pixel_format(pixel_format);

    let pitch = format.compute_pitch(width, height, CP_FLAGS::CP_FLAGS_NONE);

    let row_pitch = match pitch {
        Ok(p) => p.row,
        Err(_) => return Err("Failed to compute row pitch".to_string()),
    };
    let slice_pitch = match pitch {
        Ok(p) => p.slice,
        Err(_) => return Err("Failed to compute slice pitch".to_string()),
    };

    let src_image = Image {
        width,
        height,
        format,
        pixels: bc1_data.as_ptr() as *mut u8,
        row_pitch,
        slice_pitch,
    };

    let new_scratch_image = src_image.decompress(DXGI_FORMAT_R8G8B8A8_UNORM).unwrap();
    let slice_pitch = new_scratch_image.image(0, 0,0).unwrap().slice_pitch;
    let pixels = Vec::from(&new_scratch_image.pixels()[0..slice_pitch]);

    Ok(pixels)
}
