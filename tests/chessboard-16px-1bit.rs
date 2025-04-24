use embedded_graphics::{
    pixelcolor::{BinaryColor, Rgb888},
    prelude::*,
};
use tinybmp::{Bmp, Bpp, CompressionMethod, Header, RawBmp, RowOrder};

#[test]
fn chessboard_16px_1bit() {
    let bmp =
        RawBmp::from_slice(include_bytes!("./chessboard-16px-1bit.bmp")).expect("Failed to parse");

    assert_eq!(
        bmp.header(),
        &Header {
            file_size: 94,
            image_data_start: 62,
            bpp: Bpp::Bits1,
            image_size: Size::new(16, 16),
            image_data_len: 32,
            channel_masks: None,
            row_order: RowOrder::BottomUp,
            compression_method: CompressionMethod::Rgb,
        }
    );

    let color_table = bmp.color_table().unwrap();
    assert_eq!(color_table.len(), 2);
    assert_eq!(color_table.get(1), Some(Rgb888::BLACK));
    assert_eq!(color_table.get(0), Some(Rgb888::WHITE));
    assert_eq!(color_table.get(2), None);

    assert_eq!(bmp.image_data().len(), 94 - 62);
}

#[test]
fn chessboard_16px_1bit_iter_raw() {
    let bmp =
        RawBmp::from_slice(include_bytes!("./chessboard-16px-1bit.bmp")).expect("Failed to parse");

    let pixels: Vec<u32> = bmp.pixels().map(|pixel| pixel.color).collect();

    // 16px x 16px image. Check that iterator returns all pixels in it
    assert_eq!(pixels.len(), 16 * 16);

    let w = 1u32;
    let b = 0u32;

    let expected = vec![
        w, w, b, b, w, w, b, b, w, w, b, b, w, w, b, b, //
        w, w, b, b, w, w, b, b, w, w, b, b, w, w, b, b, //
        b, b, w, w, b, b, w, w, b, b, w, w, b, b, w, w, //
        b, b, w, w, b, b, w, w, b, b, w, w, b, b, w, w, //
        w, w, b, b, w, w, b, b, w, w, b, b, w, w, b, b, //
        w, w, b, b, w, w, b, b, w, w, b, b, w, w, b, b, //
        b, b, w, w, b, b, w, w, b, b, w, w, b, b, w, w, //
        b, b, w, w, b, b, w, w, b, b, w, w, b, b, w, w, //
        w, w, b, b, w, w, b, b, w, w, b, b, w, w, b, b, //
        w, w, b, b, w, w, b, b, w, w, b, b, w, w, b, b, //
        b, b, w, w, b, b, w, w, b, b, w, w, b, b, w, w, //
        b, b, w, w, b, b, w, w, b, b, w, w, b, b, w, w, //
        w, w, b, b, w, w, b, b, w, w, b, b, w, w, b, b, //
        w, w, b, b, w, w, b, b, w, w, b, b, w, w, b, b, //
        b, b, w, w, b, b, w, w, b, b, w, w, b, b, w, w, //
        b, b, w, w, b, b, w, w, b, b, w, w, b, b, w, w, //
    ];

    assert_eq!(pixels, expected);
}

#[test]
fn chessboard_16px_1bit_iter() {
    let bmp = Bmp::<'_, Rgb888>::from_slice(include_bytes!("./chessboard-16px-1bit.bmp"))
        .expect("Failed to parse");

    let pixels: Vec<u32> = bmp
        .pixels()
        .map(|Pixel(_pos, color)| color.into_storage().into())
        .collect();

    // 16px x 16px image. Check that iterator returns all pixels in it
    assert_eq!(pixels.len(), 16 * 16);

    // Imagemagick inverts using a color mapping table which maps a 0 to [255, 255, 255, 0], hence
    // this instead of a simple `1` value.
    let w = Rgb888::WHITE.into_storage();
    let b = Rgb888::BLACK.into_storage();

    let expected = vec![
        w, w, b, b, w, w, b, b, w, w, b, b, w, w, b, b, //
        w, w, b, b, w, w, b, b, w, w, b, b, w, w, b, b, //
        b, b, w, w, b, b, w, w, b, b, w, w, b, b, w, w, //
        b, b, w, w, b, b, w, w, b, b, w, w, b, b, w, w, //
        w, w, b, b, w, w, b, b, w, w, b, b, w, w, b, b, //
        w, w, b, b, w, w, b, b, w, w, b, b, w, w, b, b, //
        b, b, w, w, b, b, w, w, b, b, w, w, b, b, w, w, //
        b, b, w, w, b, b, w, w, b, b, w, w, b, b, w, w, //
        w, w, b, b, w, w, b, b, w, w, b, b, w, w, b, b, //
        w, w, b, b, w, w, b, b, w, w, b, b, w, w, b, b, //
        b, b, w, w, b, b, w, w, b, b, w, w, b, b, w, w, //
        b, b, w, w, b, b, w, w, b, b, w, w, b, b, w, w, //
        w, w, b, b, w, w, b, b, w, w, b, b, w, w, b, b, //
        w, w, b, b, w, w, b, b, w, w, b, b, w, w, b, b, //
        b, b, w, w, b, b, w, w, b, b, w, w, b, b, w, w, //
        b, b, w, w, b, b, w, w, b, b, w, w, b, b, w, w, //
    ];

    assert_eq!(pixels, expected);
}