#[cfg(test)]
mod tests {

    use std::path::{Path, PathBuf};

    use TextureViewer::{
        codecs::{
            codec_manager::CodecManager, dds::DDSCodec, jpeg::JPEGCodec, png::PNGCodec, ImageCodec,
        },
        graphics::pixel_format::PixelFormat,
    };

    #[test]
    fn test_load_png() {
        let png_data = include_bytes!("test_images/sample.png");
        let codec = PNGCodec;

        let texture = codec
            .load_from_memory(png_data)
            .expect("Failed to load PNG!");

        assert_eq!(texture.metadata.width, 256);
        assert_eq!(texture.metadata.height, 256);
        assert_eq!(
            texture.metadata.pixel_format_info.pixel_format,
            PixelFormat::R8G8B8A8
        );

        let texture = codec
            .load_from_file(PathBuf::from("tests/test_images/sample.png"))
            .expect("Failed to load PNG!");

        assert_eq!(texture.metadata.width, 256);
        assert_eq!(texture.metadata.height, 256);
        assert_eq!(
            texture.metadata.pixel_format_info.pixel_format,
            PixelFormat::R8G8B8A8
        );
    }

    #[test]
    fn test_load_jpeg() {
        let jpeg_data = include_bytes!("test_images/sample.jpg");
        let codec = JPEGCodec;
        let texture = codec
            .load_from_memory(jpeg_data)
            .expect("Failed to load JPEG!");

        assert_eq!(texture.metadata.width, 256);
        assert_eq!(texture.metadata.height, 256);
        assert_eq!(
            texture.metadata.pixel_format_info.pixel_format,
            PixelFormat::R8G8B8
        );

        let texture = codec
            .load_from_file(PathBuf::from("tests/test_images/sample.jpg"))
            .expect("Failed to load JPG!");

        assert_eq!(texture.metadata.width, 256);
        assert_eq!(texture.metadata.height, 256);
        assert_eq!(
            texture.metadata.pixel_format_info.pixel_format,
            PixelFormat::R8G8B8
        );
    }

    #[test]
    fn test_load_dds() {
        let dds_data = include_bytes!("test_images/sample.dds");
        let codec = DDSCodec;
        let texture = codec
            .load_from_memory(dds_data)
            .expect("Failed to load DDS!");

        assert_eq!(texture.metadata.width, 2048);
        assert_eq!(texture.metadata.height, 2048);
        assert_eq!(
            texture.metadata.pixel_format_info.pixel_format,
            PixelFormat::BC1
        );
        assert_eq!(texture.metadata.depth, 1);
        assert_eq!(texture.metadata.mip_levels, 12);
        assert_eq!(texture.metadata.array_size, 6);
        assert_eq!(texture.metadata.is_cubemap, true);
        assert_eq!(texture.images.len(), 72);

        let texture = codec
            .load_from_file(PathBuf::from("tests/test_images/sample.dds"))
            .expect("Failed to load DDS");

        assert_eq!(texture.metadata.width, 2048);
        assert_eq!(texture.metadata.height, 2048);
        assert_eq!(
            texture.metadata.pixel_format_info.pixel_format,
            PixelFormat::BC1
        );
        assert_eq!(texture.metadata.depth, 1);
        assert_eq!(texture.metadata.mip_levels, 12);
        assert_eq!(texture.metadata.array_size, 6);
        assert_eq!(texture.metadata.is_cubemap, true);
        assert_eq!(texture.images.len(), 72);
    }

    #[test]
    fn test_load_image_from_file() {
        let mut codec_manager = CodecManager::new();

        codec_manager.register_codec(PNGCodec);
        codec_manager.register_codec(JPEGCodec);

        let texture = codec_manager.load_from_file(Path::new("tests/test_images/sample.png"));

        if let Ok(texture) = texture {
            assert_eq!(texture.metadata.width, 256);
            assert_eq!(texture.metadata.height, 256);
            assert_eq!(
                texture.metadata.pixel_format_info.pixel_format,
                PixelFormat::R8G8B8A8
            );
        } else {
            panic!("Failed to load image from file");
        }
    }
}
