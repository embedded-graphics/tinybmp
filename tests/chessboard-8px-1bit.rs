use embedded_graphics::{pixelcolor::Rgb888, prelude::*};
use tinybmp::{Bpp, Header, RawBmp, RowOrder};

#[test]
fn chessboard_8px_1bit() {
    let bmp =
        RawBmp::from_slice(include_bytes!("./chessboard-8px-1bit.bmp")).expect("Failed to parse");

    assert_eq!(
        bmp.header(),
        &Header {
            file_size: 94,
            image_data_start: 62,
            bpp: Bpp::Bits1,
            image_size: Size::new(8, 8),
            image_data_len: 32,
            channel_masks: None,
            row_order: RowOrder::BottomUp,
            color_table: Some(&[0, 0, 0, 0, 255, 255, 255, 255]),
        }
    );

    assert_eq!(bmp.image_data().len(), 94 - 62);
}

#[test]
fn chessboard_8px_1bit_iter() {
    let bmp =
        RawBmp::from_slice(include_bytes!("./chessboard-8px-1bit.bmp")).expect("Failed to parse");

    let pixels: Vec<u32> = bmp.pixels().map(|pixel| pixel.color).collect();

    // 8px x 8px image. Check that iterator returns all pixels in it
    assert_eq!(pixels.len(), 8 * 8);

    // Imagemagick inverts using a color mapping table which maps a 0 to [255, 255, 255, 0], hence
    // this instead of a simple `1` value.
    let w: u32 = Rgb888::WHITE.into_storage();

    let expected = vec![
        w, w, 0, 0, w, w, 0, 0, //
        w, w, 0, 0, w, w, 0, 0, //
        0, 0, w, w, 0, 0, w, w, //
        0, 0, w, w, 0, 0, w, w, //
        w, w, 0, 0, w, w, 0, 0, //
        w, w, 0, 0, w, w, 0, 0, //
        0, 0, w, w, 0, 0, w, w, //
        0, 0, w, w, 0, 0, w, w, //
    ];

    assert_eq!(pixels, expected);
}

#[test]
fn chessboard_8px_1bit_iter_inverted() {
    // Inverted image created with Imagemagick command:
    // convert chessboard-8px-1bit.bmp -negate -type bilevel chessboard-8px-1bit-inverted.bmp
    let bmp = RawBmp::from_slice(include_bytes!("./chessboard-8px-1bit-inverted.bmp"))
        .expect("Failed to parse");

    let pixels: Vec<u32> = bmp.pixels().map(|pixel| pixel.color).collect();

    // 8px x 8px image. Check that iterator returns all pixels in it
    assert_eq!(pixels.len(), 8 * 8);

    // Imagemagick inverts using a color mapping table which maps a 0 to [255, 255, 255, 0], hence
    // this instead of a simple `1` value.
    let w = Rgb888::WHITE.into_storage();
    let b = 0u32;

    let expected = vec![
        b, b, w, w, b, b, w, w, //
        b, b, w, w, b, b, w, w, //
        w, w, b, b, w, w, b, b, //
        w, w, b, b, w, w, b, b, //
        b, b, w, w, b, b, w, w, //
        b, b, w, w, b, b, w, w, //
        w, w, b, b, w, w, b, b, //
        w, w, b, b, w, w, b, b, //
    ];

    assert_eq!(pixels, expected);
}
