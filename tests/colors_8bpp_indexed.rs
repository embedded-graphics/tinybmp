use tinybmp::RawBmp;

#[test]
fn colors_8bpp_indexed() {
    let bmp =
        RawBmp::from_slice(include_bytes!("./colors_8bpp_indexed.bmp")).expect("Failed to parse");

    assert!(bmp.color_table().is_some());

    let pixels: Vec<u32> = bmp.pixels().map(|pixel| pixel.color).collect();

    // 4px x 6px image. Check that iterator returns all pixels in it
    assert_eq!(pixels.len(), 4 * 6);

    let expected = RawBmp::from_slice(include_bytes!("./colors_8bpp_non_indexed.bmp"))
        .expect("Failed to parse non_indexed");

    let expected_pixels: Vec<u32> = expected.pixels().map(|pixel| pixel.color).collect();

    assert_eq!(pixels, expected_pixels);
}
