use embedded_graphics::{
    geometry::Point,
    iterator::raw::RawDataSlice,
    pixelcolor::raw::{LittleEndian, RawU1, RawU16, RawU24, RawU32, RawU4, RawU8},
    prelude::RawData,
};

use crate::{
    color_table::ColorTable,
    header::{Bpp, Header},
    raw_iter::RawPixels,
    ChannelMasks, ParseError, RowOrder,
};

/// Low-level access to BMP image data.
///
/// This struct can be used to access the image data in a BMP file at a lower level than with the
/// [`Bmp`](crate::Bmp) struct. It doesn't do automatic color conversion and doesn't apply the color
/// table, if it is present in the BMP file. For images with a color table the iterator returned by
/// [`pixels`](Self::pixels) will instead return the color indices, that can be looked up manually
/// using the [`ColorTable`] struct.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct RawBmp<'a> {
    /// Image header.
    header: Header,

    /// Color type.
    pub(crate) color_type: ColorType,

    /// Color table for color mapped images.
    color_table: Option<ColorTable<'a>>,

    /// Image data.
    image_data: &'a [u8],
}

impl<'a> RawBmp<'a> {
    /// Create a bitmap object from a byte slice.
    ///
    /// The created object keeps a shared reference to the input and does not dynamically allocate
    /// memory.
    pub fn from_slice(bytes: &'a [u8]) -> Result<Self, ParseError> {
        let (_remaining, (header, color_table)) = Header::parse(bytes)?;

        let color_type = ColorType::from_header(&header)?;

        let height = header
            .image_size
            .height
            .try_into()
            .map_err(|_| ParseError::InvalidImageDimensions)?;

        let data_length = header
            .bytes_per_row()
            .checked_mul(height)
            .ok_or(ParseError::InvalidImageDimensions)?;

        // The `get` is split into two calls to prevent an possible integer overflow.
        let image_data = &bytes
            .get(header.image_data_start..)
            .and_then(|bytes| bytes.get(..data_length))
            .ok_or(ParseError::UnexpectedEndOfFile)?;

        Ok(Self {
            header,
            color_type,
            color_table,
            image_data,
        })
    }

    /// Returns the color table associated with the image.
    pub const fn color_table(&self) -> Option<&ColorTable<'a>> {
        self.color_table.as_ref()
    }

    /// Returns a slice containing the raw image data.
    pub const fn image_data(&self) -> &'a [u8] {
        self.image_data
    }

    /// Returns a reference to the BMP header.
    pub const fn header(&self) -> &Header {
        &self.header
    }

    /// Returns an iterator over the raw pixels in the image.
    ///
    /// The iterator returns the raw pixel colors as [`u32`] values.  To automatically convert the
    /// raw values into [`embedded_graphics`] color types use [`Bmp::pixels`](crate::Bmp::pixels)
    /// instead.
    pub fn pixels(&self) -> RawPixels<'_> {
        RawPixels::new(self)
    }

    /// Returns the raw color of a pixel.
    ///
    /// Returns `None` if `p` is outside the image bounding box. Note that this function doesn't
    /// apply a color map, if the image contains one.
    pub fn pixel(&self, p: Point) -> Option<u32> {
        let width = self.header.image_size.width as i32;
        let height = self.header.image_size.height as i32;

        if p.x < 0 || p.x >= width || p.y < 0 || p.y >= height {
            return None;
        }

        // The specialized implementations of `Iterator::nth` for `Chunks` and
        // `RawDataSlice::IntoIter` are `O(1)`, which also makes this method `O(1)`.

        let mut row_chunks = self.image_data.chunks_exact(self.header.bytes_per_row());
        let row = match self.header.row_order {
            RowOrder::BottomUp => row_chunks.nth_back(p.y as usize),
            RowOrder::TopDown => row_chunks.nth(p.y as usize),
        }?;

        match self.header.bpp {
            Bpp::Bits1 => RawDataSlice::<RawU1, LittleEndian>::new(row)
                .into_iter()
                .nth(p.x as usize)
                .map(|raw| u32::from(raw.into_inner())),
            Bpp::Bits4 => RawDataSlice::<RawU4, LittleEndian>::new(row)
                .into_iter()
                .nth(p.x as usize)
                .map(|raw| u32::from(raw.into_inner())),
            Bpp::Bits8 => RawDataSlice::<RawU8, LittleEndian>::new(row)
                .into_iter()
                .nth(p.x as usize)
                .map(|raw| u32::from(raw.into_inner())),
            Bpp::Bits16 => RawDataSlice::<RawU16, LittleEndian>::new(row)
                .into_iter()
                .nth(p.x as usize)
                .map(|raw| u32::from(raw.into_inner())),
            Bpp::Bits24 => RawDataSlice::<RawU24, LittleEndian>::new(row)
                .into_iter()
                .nth(p.x as usize)
                .map(|raw| raw.into_inner()),
            Bpp::Bits32 => RawDataSlice::<RawU32, LittleEndian>::new(row)
                .into_iter()
                .nth(p.x as usize)
                .map(|raw| raw.into_inner()),
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum ColorType {
    Index1,
    Index4,
    Index8,
    Rgb555,
    Rgb565,
    Rgb888,
    Xrgb8888,
}

impl ColorType {
    pub(crate) fn from_header(header: &Header) -> Result<ColorType, ParseError> {
        Ok(match header.bpp {
            Bpp::Bits1 => ColorType::Index1,
            Bpp::Bits4 => ColorType::Index4,
            Bpp::Bits8 => ColorType::Index8,
            Bpp::Bits16 => {
                if let Some(masks) = header.channel_masks {
                    match masks {
                        ChannelMasks::RGB555 => ColorType::Rgb555,
                        ChannelMasks::RGB565 => ColorType::Rgb565,
                        _ => return Err(ParseError::UnsupportedChannelMasks),
                    }
                } else {
                    // According to the GDI docs the default 16 bpp color format is Rgb555 if no
                    // color masks are defined:
                    // https://docs.microsoft.com/en-us/windows/win32/api/wingdi/ns-wingdi-bitmapinfoheader
                    ColorType::Rgb555
                }
            }
            Bpp::Bits24 => ColorType::Rgb888,
            Bpp::Bits32 => {
                if let Some(masks) = header.channel_masks {
                    if masks == ChannelMasks::RGB888 {
                        ColorType::Xrgb8888
                    } else {
                        return Err(ParseError::UnsupportedChannelMasks);
                    }
                } else {
                    ColorType::Xrgb8888
                }
            }
        })
    }
}
