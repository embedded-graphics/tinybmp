use embedded_graphics::{pixelcolor::Rgb888, prelude::IntoStorage, Pixel};
use tinybmp::Bmp;

#[test]
fn colors_8bpp_indexed() {
    let bmp = Bmp::<'_, Rgb888>::from_slice(include_bytes!("./colors_8bpp_indexed.bmp"))
        .expect("Failed to parse");

    assert!(
        bmp.as_raw().color_table().is_some(),
        "there should be a color table for this image"
    );

    let pixels: Vec<u32> = bmp
        .pixels()
        .map(|Pixel(_pos, color)| color.into_storage())
        .collect();

    // 4px x 6px image. Check that iterator returns all pixels in it
    assert_eq!(pixels.len(), 4 * 6);

    let expected = Bmp::<'_, Rgb888>::from_slice(include_bytes!("./colors_8bpp_non_indexed.bmp"))
        .expect("Failed to parse non_indexed");

    let expected_pixels: Vec<u32> = expected
        .pixels()
        .map(|Pixel(_pos, color)| color.into_storage())
        .collect();

    assert_eq!(pixels, expected_pixels);
}
