use embedded_graphics::prelude::*;
use tinybmp::{Bpp, Header, RawBmp, RowOrder};

const DATA: &[u8] = include_bytes!("./chessboard-8px-24bit.bmp");

#[test]
fn chessboard_8px_24bit() {
    let bmp = RawBmp::from_slice(DATA).expect("Failed to parse");

    assert_eq!(
        bmp.header(),
        &Header {
            file_size: 314,
            image_data_start: 122,
            bpp: Bpp::Bits24,
            image_size: Size::new(8, 8),
            image_data_len: 192,
            channel_masks: None,
            row_order: RowOrder::BottomUp,
        }
    );

    assert!(
        bmp.color_table().is_none(),
        "there should be no color table for this image"
    );

    assert_eq!(bmp.image_data().len(), 314 - 122);
}

#[test]
fn chessboard_8px_24bit_iter() {
    let bmp = RawBmp::from_slice(DATA).expect("Failed to parse");

    let pixels: Vec<u32> = bmp.pixels().map(|pixel| pixel.color).collect();

    assert_eq!(pixels.len(), 8 * 8);

    // 24BPP black/white chessboard
    let expected = vec![
        0xffffff, 0xffffff, 0x000000, 0x000000, 0xffffff, 0xffffff, 0x000000, 0x000000, //
        0xffffff, 0xffffff, 0x000000, 0x000000, 0xffffff, 0xffffff, 0x000000, 0x000000, //
        0x000000, 0x000000, 0xffffff, 0xffffff, 0x000000, 0x000000, 0xffffff, 0xffffff, //
        0x000000, 0x000000, 0xffffff, 0xffffff, 0x000000, 0x000000, 0xffffff, 0xffffff, //
        0xffffff, 0xffffff, 0x000000, 0x000000, 0xffffff, 0xffffff, 0x000000, 0x000000, //
        0xffffff, 0xffffff, 0x000000, 0x000000, 0xffffff, 0xffffff, 0x000000, 0x000000, //
        0x000000, 0x000000, 0xffffff, 0xffffff, 0x000000, 0x000000, 0xffffff, 0xffffff, //
        0x000000, 0x000000, 0xffffff, 0xffffff, 0x000000, 0x000000, 0xffffff, 0xffffff, //
    ];

    assert_eq!(pixels, expected);
}
