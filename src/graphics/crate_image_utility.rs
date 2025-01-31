use super::pixel_format::{ColorSpace, PixelDataType, PixelFormat, PixelFormatInfo};

pub fn get_pixel_format_from_image(color_type: image::ColorType) -> Option<PixelFormat> {
    match color_type {
        image::ColorType::L8 => Some(PixelFormat::L8),
        image::ColorType::La8 => Some(PixelFormat::LA8),
        image::ColorType::Rgb8 => Some(PixelFormat::RGB8),
        image::ColorType::Rgba8 => Some(PixelFormat::RGBA8),
        image::ColorType::Rgb16 => Some(PixelFormat::RGB16),
        image::ColorType::Rgba16 => Some(PixelFormat::RGBA16),
        image::ColorType::L16 => Some(PixelFormat::L16),
        image::ColorType::La16 => Some(PixelFormat::LA16),
        image::ColorType::Rgb32F => Some(PixelFormat::RGB32),
        image::ColorType::Rgba32F => Some(PixelFormat::RGBA32),
        _ => None,
    }
}

pub fn get_pixel_format_datatype_from_image(color_type: image::ColorType) -> Option<PixelDataType> {
    match color_type {
        image::ColorType::L8 => Some(PixelDataType::UNorm),
        image::ColorType::La8 => Some(PixelDataType::UNorm),
        image::ColorType::Rgb8 => Some(PixelDataType::UNorm),
        image::ColorType::Rgba8 => Some(PixelDataType::UNorm),
        image::ColorType::Rgb16 => Some(PixelDataType::UNorm),
        image::ColorType::Rgba16 => Some(PixelDataType::UNorm),
        image::ColorType::L16 => Some(PixelDataType::UNorm),
        image::ColorType::La16 => Some(PixelDataType::UNorm),
        image::ColorType::Rgb32F => Some(PixelDataType::Float),
        image::ColorType::Rgba32F => Some(PixelDataType::Float),
        _ => None,
    }
}

pub fn get_pixel_format_info_from_image(image: &image::DynamicImage) -> PixelFormatInfo {
    let color_type = image.color();
    let pixel_format = get_pixel_format_from_image(color_type).unwrap();
    let pixel_data_type = get_pixel_format_datatype_from_image(color_type).unwrap();
    let color_space = match color_type {
        image::ColorType::Rgb8 | image::ColorType::Rgba8 => ColorSpace::SRGB,
        _ => ColorSpace::Linear,
    };
    let is_premultiplied = false;

    PixelFormatInfo {
        pixel_format,
        pixel_data_type,
        color_space,
        is_premultiplied,
    }
}
