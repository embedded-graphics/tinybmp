//! Bitmap header
//!
//! Information gleaned from [wikipedia](https://en.wikipedia.org/wiki/BMP_file_format) and
//! [this website](http://paulbourke.net/dataformats/bmp/)

use embedded_graphics::prelude::*;

mod dib_header;

use crate::{
    color_table::ColorTable,
    header::dib_header::DibHeader,
    parser::{le_u16, le_u32, take, take_slice},
    ParseError,
};

/// Bits per pixel.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[non_exhaustive]
pub enum Bpp {
    /// 1 bit per pixel.
    Bits1,
    /// 8 bits per pixel.
    Bits8,
    /// 16 bits per pixel.
    Bits16,
    /// 24 bits per pixel.
    Bits24,
    /// 32 bits per pixel.
    Bits32,
}

/// Image row order
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[non_exhaustive]
pub enum RowOrder {
    /// Bottom-up (standard)
    BottomUp,
    /// Top-down
    TopDown,
}

impl Default for RowOrder {
    fn default() -> Self {
        Self::BottomUp
    }
}

impl Bpp {
    fn new(value: u16) -> Result<Self, ParseError> {
        Ok(match value {
            1 => Self::Bits1,
            8 => Self::Bits8,
            16 => Self::Bits16,
            24 => Self::Bits24,
            32 => Self::Bits32,
            _ => return Err(ParseError::UnsupportedBpp(value)),
        })
    }

    fn parse(input: &[u8]) -> Result<(&[u8], Self), ParseError> {
        le_u16(input).and_then(|(input, value)| Ok((input, Self::new(value)?)))
    }

    /// Returns the number of bits.
    pub fn bits(self) -> u16 {
        match self {
            Self::Bits1 => 1,
            Self::Bits8 => 8,
            Self::Bits16 => 16,
            Self::Bits24 => 24,
            Self::Bits32 => 32,
        }
    }
}

/// BMP header information
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Header {
    /// Total file size in bytes.
    pub file_size: u32,

    /// Byte offset from beginning of file at which pixel data begins.
    pub image_data_start: usize,

    /// Image size in pixels.
    pub image_size: Size,

    /// Number of bits per pixel.
    pub bpp: Bpp,

    /// Length in bytes of the image data.
    pub image_data_len: u32,

    /// Bit masks for the color channels.
    pub channel_masks: Option<ChannelMasks>,

    /// Row order of the image data within the file
    pub row_order: RowOrder,
}

impl Header {
    pub(crate) fn parse(
        input: &[u8],
    ) -> Result<(&[u8], (Header, Option<ColorTable<'_>>)), ParseError> {
        // File header
        let (input, magic) = take::<2>(input)?;
        if &magic != b"BM" {
            return Err(ParseError::InvalidFileSignature);
        }

        let (input, file_size) = le_u32(input)?;
        let (input, _reserved_1) = le_u16(input)?;
        let (input, _reserved_2) = le_u16(input)?;
        let (input, image_data_start) = le_u32(input)?;

        // DIB header
        let (input, dib_header) = DibHeader::parse(input)?;

        match dib_header.bpp {
            // Images with BPP <= 8 MUST include a color table
            Bpp::Bits1 | Bpp::Bits8 if dib_header.color_table_num_entries == 0 => {
                return Err(ParseError::MissingColorTable);
            }
            _ => (),
        }

        let (input, color_table) = if dib_header.color_table_num_entries > 0 {
            // Each color table entry is 4 bytes long
            let (input, table) =
                take_slice(input, dib_header.color_table_num_entries as usize * 4)?;
            (input, Some(ColorTable::new(table)))
        } else {
            (input, None)
        };

        Ok((
            input,
            (
                Header {
                    file_size,
                    image_data_start: image_data_start as usize,
                    image_size: dib_header.image_size,
                    image_data_len: dib_header.image_data_len,
                    bpp: dib_header.bpp,
                    channel_masks: dib_header.channel_masks,
                    row_order: dib_header.row_order,
                },
                color_table,
            ),
        ))
    }
}

/// Masks for the color channels.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct ChannelMasks {
    /// Red channel mask.
    pub red: u32,
    /// Green channel mask.
    pub green: u32,
    /// Blue channel mask.
    pub blue: u32,
    /// Alpha channel mask.
    pub alpha: u32,
}

impl ChannelMasks {
    /// Rgb555 color masks.
    pub const RGB555: Self = Self {
        red: 0b11111_00000_00000,
        green: 0b00000_11111_00000,
        blue: 0b00000_00000_11111,
        alpha: 0,
    };

    /// Rgb565 color masks.
    pub const RGB565: Self = Self {
        red: 0b11111_000000_00000,
        green: 0b00000_111111_00000,
        blue: 0b00000_000000_11111,
        alpha: 0,
    };

    /// Rgb888 color masks.
    pub const RGB888: Self = Self {
        red: 0xFF0000,
        green: 0x00FF00,
        blue: 0x0000FF,
        alpha: 0,
    };
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum CompressionMethod {
    Rgb,
    Bitfields,
}

impl CompressionMethod {
    fn new(value: u32) -> Result<Self, ParseError> {
        Ok(match value {
            0 => Self::Rgb,
            3 => Self::Bitfields,
            _ => return Err(ParseError::UnsupportedCompressionMethod(value)),
        })
    }

    fn parse(input: &[u8]) -> Result<(&[u8], Self), ParseError> {
        le_u32(input).and_then(|(input, value)| Ok((input, Self::new(value)?)))
    }
}
