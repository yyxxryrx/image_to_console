#[cfg(feature = "sixel")]
mod tests {
    use image::RgbImage;
    use image_to_console_core::indexed_image::IndexedImage;

    #[test]
    fn test_indexed_image_creation() {
        // Create a simple 2x2 RGB image
        let mut img = RgbImage::new(2, 2);
        img.put_pixel(0, 0, image::Rgb([255, 0, 0])); // Red
        img.put_pixel(1, 0, image::Rgb([0, 255, 0])); // Green
        img.put_pixel(0, 1, image::Rgb([0, 0, 255])); // Blue
        img.put_pixel(1, 1, image::Rgb([255, 255, 255])); // White

        let indexed_img =
            IndexedImage::from_image(&img, 256, false, quantette::QuantizeMethod::wu(), quantette::ColorSpace::Srgb).unwrap();
        assert_eq!(indexed_img.width, 2);
        assert_eq!(indexed_img.height, 2);
        assert!(!indexed_img.palette.is_empty());
        assert_eq!(indexed_img.index_data.len(), 4);
    }

    #[test]
    fn test_indexed_image_pixel_access() {
        let mut img = RgbImage::new(2, 2);
        img.put_pixel(0, 0, image::Rgb([255, 0, 0])); // Red
        img.put_pixel(1, 0, image::Rgb([0, 255, 0])); // Green
        img.put_pixel(0, 1, image::Rgb([0, 0, 255])); // Blue
        img.put_pixel(1, 1, image::Rgb([255, 255, 255])); // White

        let indexed_img =
            IndexedImage::from_image(&img, 256, false, quantette::QuantizeMethod::wu(), quantette::ColorSpace::Srgb).unwrap();
        // Check we can access pixels
        let pixel00 = indexed_img.get_pixel(0, 0);
        let pixel10 = indexed_img.get_pixel(1, 0);
        let pixel01 = indexed_img.get_pixel(0, 1);
        let pixel11 = indexed_img.get_pixel(1, 1);

        // All pixels should have valid indices
        assert!(pixel00 < indexed_img.palette.len() as u8);
        assert!(pixel10 < indexed_img.palette.len() as u8);
        assert!(pixel01 < indexed_img.palette.len() as u8);
        assert!(pixel11 < indexed_img.palette.len() as u8);
    }
}
