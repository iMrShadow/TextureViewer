#[cfg(test)]
mod tests {

    use std::path::{Path, PathBuf};

    use TextureViewer::{
        codecs::{codec_manager::CodecManager, jpeg::JPEGCodec, png::PNGCodec, ImageCodec},
        graphics::pixel_format::PixelFormat,
    };

    #[test]
    fn test_load_png() {
        let png_data = include_bytes!("test_images/sample.png");
        let codec = PNGCodec;

        let texture = codec
            .load_from_memory(png_data)
            .expect("Failed to load PNG");

        assert_eq!(texture.metadata.width, 256);
        assert_eq!(texture.metadata.height, 256);
        assert_eq!(texture.metadata.format.pixel_format, PixelFormat::RGBA8);

        let texture = codec
            .load_from_file(PathBuf::from("tests/test_images/sample.png"))
            .expect("Failed to load PNG");

        assert_eq!(texture.metadata.width, 256);
        assert_eq!(texture.metadata.height, 256);
        assert_eq!(texture.metadata.format.pixel_format, PixelFormat::RGBA8);
    }

    #[test]
    fn test_load_jpeg() {
        let jpeg_data = include_bytes!("test_images/sample.jpg");
        let codec = JPEGCodec;
        let texture = codec
            .load_from_memory(jpeg_data)
            .expect("Failed to load JPEG");

        assert_eq!(texture.metadata.width, 256);
        assert_eq!(texture.metadata.height, 256);
        assert_eq!(texture.metadata.format.pixel_format, PixelFormat::RGB8);

        let texture = codec
            .load_from_file(PathBuf::from("tests/test_images/sample.jpg"))
            .expect("Failed to load PNG");

        assert_eq!(texture.metadata.width, 256);
        assert_eq!(texture.metadata.height, 256);
        assert_eq!(texture.metadata.format.pixel_format, PixelFormat::RGB8);
    }

    #[test]
    fn test_load_image_from_file() {
        let mut codec_manager = CodecManager::new();

        // Register codecs
        codec_manager.register_codec(PNGCodec);
        codec_manager.register_codec(JPEGCodec);

        // Load a texture
        let texture = codec_manager.load_from_file(Path::new("tests/test_images/sample.png"));

        if let Ok(texture) = texture {
            assert_eq!(texture.metadata.width, 256);
            assert_eq!(texture.metadata.height, 256);
            assert_eq!(texture.metadata.format.pixel_format, PixelFormat::RGBA8);
        } else {
            panic!("Failed to load image from file");
        }

        // // Save a texture
        // codec_manager
        //     .save_to_file(Path::new("output.png"), &texture)
        //     .expect("Failed to save texture");
    }
    // #[test]
    // fn test_load_from_file() {
    //     let codec = ImageCodec;
    //     let texture = codec
    //         .load_from_file(PathBuf::from("test_images/sample.png"))
    //         .expect("Failed to load image from file");

    //     assert_eq!(texture.metadata.width, 256);
    //     assert_eq!(texture.metadata.height, 256);
    //     assert_eq!(texture.metadata.format.pixel_format, PixelFormat::RGBA8);
    // }
}
