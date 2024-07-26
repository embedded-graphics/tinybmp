//! Device Independent Bitmap (DIB) header.

use embedded_graphics::prelude::*;

use crate::{
    header::CompressionMethod,
    parser::{le_i32, le_u16, le_u32, take_slice},
    try_const, Bpp, ChannelMasks, ParseError, RowOrder,
};

const DIB_INFO_HEADER_SIZE: u32 = 40;
const DIB_V3_HEADER_SIZE: u32 = 56;
const DIB_V4_HEADER_SIZE: u32 = 108;
const DIB_V5_HEADER_SIZE: u32 = 124;

/// Device Independent Bitmap (DIB) header.
#[derive(Debug)]
pub struct DibHeader {
    pub image_size: Size,
    pub bpp: Bpp,
    pub compression: CompressionMethod,
    pub image_data_len: u32,
    pub channel_masks: Option<ChannelMasks>,
    pub header_type: HeaderType,
    pub row_order: RowOrder,
    pub color_table_num_entries: u32,
}

impl DibHeader {
    pub const fn parse(input: &[u8]) -> Result<(&[u8], Self), ParseError> {
        let (input, dib_header_length) = try_const!(le_u32(input));

        // The header size in the BMP includes its own u32, so we strip it out by subtracting 4
        // bytes to get the right final offset to the end of the header.
        let Some(data_length) = dib_header_length.checked_sub(4) else {
            return Err(ParseError::UnsupportedHeaderLength(dib_header_length));
        };
        let (input, dib_header_data) = try_const!(take_slice(input, data_length as usize));

        // Add 4 back on so the constants remain the correct size relative to the BMP
        // documentation/specs.
        let header_type = match dib_header_length {
            DIB_V3_HEADER_SIZE => HeaderType::V3,
            DIB_V4_HEADER_SIZE => HeaderType::V4,
            DIB_V5_HEADER_SIZE => HeaderType::V5,
            DIB_INFO_HEADER_SIZE => HeaderType::Info,
            _ => return Err(ParseError::UnsupportedHeaderLength(dib_header_length)),
        };

        // Fields common to all DIB variants
        let (dib_header_data, image_width) = try_const!(le_i32(dib_header_data));
        let (dib_header_data, image_height) = try_const!(le_i32(dib_header_data));
        let (dib_header_data, _color_planes) = try_const!(le_u16(dib_header_data));
        let (dib_header_data, bpp) = try_const!(Bpp::parse(dib_header_data));

        // Extra fields defined by DIB variants
        // Variants are described in
        // <https://www.liquisearch.com/bmp_file_format/file_structure/dib_header_bitmap_information_header>
        // and <https://docs.microsoft.com/en-us/windows/win32/gdi/bitmap-header-types>
        let (dib_header_data, compression_method) =
            try_const!(CompressionMethod::parse(dib_header_data));
        let (dib_header_data, image_data_len) = try_const!(le_u32(dib_header_data));
        let (dib_header_data, _pels_per_meter_x) = try_const!(le_u32(dib_header_data));
        let (dib_header_data, _pels_per_meter_y) = try_const!(le_u32(dib_header_data));
        let (dib_header_data, colors_used) = try_const!(le_u32(dib_header_data));
        let (dib_header_data, _colors_important) = try_const!(le_u32(dib_header_data));

        let (_dib_header_data, channel_masks) = if header_type.is_at_least(HeaderType::V3)
            && matches!(compression_method, CompressionMethod::Bitfields)
        {
            let (dib_header_data, mask_red) = try_const!(le_u32(dib_header_data));
            let (dib_header_data, mask_green) = try_const!(le_u32(dib_header_data));
            let (dib_header_data, mask_blue) = try_const!(le_u32(dib_header_data));
            let (dib_header_data, mask_alpha) = try_const!(le_u32(dib_header_data));

            (
                dib_header_data,
                Some(ChannelMasks {
                    red: mask_red,
                    green: mask_green,
                    blue: mask_blue,
                    alpha: mask_alpha,
                }),
            )
        } else {
            (dib_header_data, None)
        };

        let color_table_num_entries = if colors_used == 0 && bpp.bits() < 16 {
            1 << bpp.bits()
        } else {
            colors_used
        };

        if image_width <= 0 || image_height == 0 {
            return Err(ParseError::InvalidImageDimensions);
        }

        let row_order = if image_height < 0 {
            RowOrder::TopDown
        } else {
            RowOrder::BottomUp
        };

        Ok((
            input,
            Self {
                header_type,
                image_size: Size::new(image_width.unsigned_abs(), image_height.unsigned_abs()),
                image_data_len,
                bpp,
                channel_masks,
                compression: compression_method,
                row_order,
                color_table_num_entries,
            },
        ))
    }
}

// Note: Do not change the order of the enum variants!
#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord, Hash)]
pub enum HeaderType {
    Info,
    V3,
    V4,
    V5,
}

impl HeaderType {
    const fn is_at_least(self, header_type: HeaderType) -> bool {
        self as u8 >= header_type as u8
    }
}
