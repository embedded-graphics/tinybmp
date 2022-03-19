use tinybmp::RawBmp;

#[test]
fn colors_8bpp_indexed() {
    let bmp =
        RawBmp::from_slice(include_bytes!("./colors_8bpp_indexed.bmp")).expect("Failed to parse");

    let pixels: Vec<u32> = bmp.pixels().map(|pixel| pixel.color).collect();

    // 4px x 6px image. Check that iterator returns all pixels in it
    assert_eq!(pixels.len(), 4 * 6);

    let expected = vec![
        0u32, 16711680, 65280, 16776960, //
        255, 16711935, 65535, 16777215, //
        0, 10682112, 41727, 65373, //
        16711842, 16735488, 6095103, 16777215, //
        0, 9830457, 3773952, 9854208, //
        14742, 6094998, 38493, 9868950,
    ];

    assert_eq!(pixels, expected);
}
