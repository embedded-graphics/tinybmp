//! Device Independent Bitmap (DIB) header.

use crate::{header::CompressionMethod, Bpp, ChannelMasks, RowOrder};
use embedded_graphics::geometry::Size;
use nom::{
    combinator::map,
    error::{ErrorKind, ParseError},
    multi::length_data,
    number::complete::{le_i32, le_u16, le_u32},
    IResult,
};

const DIB_INFO_HEADER_SIZE: usize = 40;
const DIB_V3_HEADER_SIZE: usize = 56;
const DIB_V4_HEADER_SIZE: usize = 108;
const DIB_V5_HEADER_SIZE: usize = 124;

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
    /// Entry length of color table (NOT length in bytes)
    pub color_table_num_entries: u32,
}

impl DibHeader {
    pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        // The header size in the BMP includes its own u32, so we strip it out by subtracting 4
        // bytes to get the right final offset to the end of the header.
        let (input, dib_header_data) = length_data(map(le_u32, |len| len - 4))(input)?;

        // Add 4 back on so the constants remain the correct size relative to the BMP
        // documentation/specs.
        let header_type = match dib_header_data.len() + 4 {
            DIB_V3_HEADER_SIZE => HeaderType::V3,
            DIB_V4_HEADER_SIZE => HeaderType::V4,
            DIB_V5_HEADER_SIZE => HeaderType::V5,
            DIB_INFO_HEADER_SIZE => HeaderType::Info,
            _ => {
                return Err(nom::Err::Failure(nom::error::Error::from_error_kind(
                    dib_header_data,
                    ErrorKind::LengthValue,
                )))
            }
        };

        // Fields common to all DIB variants
        let (dib_header_data, image_width) = le_u32(dib_header_data)?;
        let (dib_header_data, image_height) = le_i32(dib_header_data)?;
        let (dib_header_data, _color_planes) = le_u16(dib_header_data)?;
        let (dib_header_data, bpp) = Bpp::parse(dib_header_data)?;

        // Extra fields defined by DIB variants
        // Variants are described in
        // <https://www.liquisearch.com/bmp_file_format/file_structure/dib_header_bitmap_information_header>
        // and <https://docs.microsoft.com/en-us/windows/win32/gdi/bitmap-header-types>
        let (dib_header_data, compression_method) = CompressionMethod::parse(dib_header_data)?;
        let (dib_header_data, image_data_len) = le_u32(dib_header_data)?;
        let (dib_header_data, _pels_per_meter_x) = le_u32(dib_header_data)?;
        let (dib_header_data, _pels_per_meter_y) = le_u32(dib_header_data)?;
        let (dib_header_data, colors_used) = le_u32(dib_header_data)?;
        let (dib_header_data, _colors_important) = le_u32(dib_header_data)?;

        let (_dib_header_data, channel_masks) = if header_type.is_at_least(HeaderType::V3)
            && compression_method == CompressionMethod::Bitfields
        {
            let (dib_header_data, mask_red) = le_u32(dib_header_data)?;
            let (dib_header_data, mask_green) = le_u32(dib_header_data)?;
            let (dib_header_data, mask_blue) = le_u32(dib_header_data)?;
            let (dib_header_data, mask_alpha) = le_u32(dib_header_data)?;

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

        // Number of colors in the color table. If the specific count is zero, the entire color
        // space should be used.
        let color_table_size: u32 = if colors_used == 0 {
            bpp.bits().pow(2).into()
        } else {
            colors_used
        };

        let color_table_size = match bpp {
            Bpp::Bits1 | Bpp::Bits8 => {
                if color_table_size > 0 {
                    Some(color_table_size)
                } else {
                    None
                }
            }
            _ => {
                // Color table is not used at BPP > 8 even if present
                None
            }
        };

        let row_order = if image_height < 0 {
            RowOrder::TopDown
        } else {
            RowOrder::BottomUp
        };

        Ok((
            input,
            Self {
                header_type,
                image_size: Size::new(image_width, image_height.abs() as u32),
                image_data_len,
                bpp,
                channel_masks,
                compression: compression_method,
                row_order,
                color_table_size,
            },
        ))
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum HeaderType {
    Info,
    V3,
    V4,
    V5,
}

impl HeaderType {
    fn is_at_least(self, header_type: HeaderType) -> bool {
        self as u8 >= header_type as u8
    }
}
