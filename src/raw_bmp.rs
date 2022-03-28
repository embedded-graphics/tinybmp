use crate::{
    header::{Bpp, Header},
    pixels::Pixels,
    raw_pixels::RawPixels,
    ParseError, RawPixel,
};
use core::convert::TryInto;
use embedded_graphics::{prelude::*, primitives::Rectangle};

/// A BMP-format bitmap.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct RawBmp<'a> {
    /// Image header.
    header: Header,

    /// Color table for color mapped images.
    color_table: Option<&'a [u8]>,

    /// Image data.
    image_data: &'a [u8],
}

impl<'a> RawBmp<'a> {
    /// Create a bitmap object from a byte slice.
    ///
    /// The created object keeps a shared reference to the input and does not dynamically allocate
    /// memory.
    ///
    /// In contrast to the [`from_slice`] constructor no color type needs to be specified when
    /// calling this method. This will disable all functions that requires a specified color type,
    /// like the [`pixels`] method.
    ///
    /// [`from_slice`]: #method.from_slice
    /// [`pixels`]: #method.pixels
    pub fn from_slice(bytes: &'a [u8]) -> Result<Self, ParseError> {
        let (_remaining, (header, color_table)) =
            Header::parse(bytes).map_err(|_| ParseError::Header)?;

        let image_data = &bytes[header.image_data_start..];

        Ok(Self {
            header,
            color_table,
            image_data,
        })
    }

    /// Returns the size of this image in pixels.
    pub fn size(&self) -> Size {
        self.header.image_size
    }

    /// Returns the BPP (bits per pixel) for this image.
    pub fn color_bpp(&self) -> Bpp {
        self.header.bpp
    }

    /// Gets the raw color table data associated with the image.
    pub fn color_table(&self) -> Option<&'a [u8]> {
        self.color_table
    }

    /// Returns a slice containing the raw image data.
    pub fn image_data(&self) -> &'a [u8] {
        self.image_data
    }

    /// Returns a reference to the BMP header.
    pub fn header(&self) -> &Header {
        &self.header
    }

    /// Returns an iterator over the raw pixels in the image.
    ///
    /// The iterator returns the raw pixel colors as `u32` values. To automatically convert the raw
    /// values into the color specified by `C` use [`pixels`] instead.
    ///
    /// [`pixels`]: #method.pixels
    pub fn pixels<'b>(&'b self) -> RawPixels<'b, 'a> {
        RawPixels::new(self)
    }

    /// Returns the row length in bytes.
    ///
    /// Each row in a BMP file is a multiple of 4 bytes long.
    pub(crate) fn bytes_per_row(&self) -> usize {
        let bits_per_row =
            self.header.image_size.width as usize * usize::from(self.header.bpp.bits());

        (bits_per_row + 31) / 32 * (32 / 8)
    }

    pub(crate) fn draw<D>(&self, target: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget,
        D::Color: From<<D::Color as PixelColor>::Raw>,
    {
        match self.color_table {
            Some(color_table)
                if self.color_bpp() == Bpp::Bits1 || self.color_bpp() == Bpp::Bits8 =>
            {
                target.fill_contiguous(
                    &Rectangle::new(Point::zero(), self.size()),
                    self.pixels().map(|RawPixel { position: _, color }| {
                        let offset = color as usize * 4;

                        // MSRV: Experiment with slice::as_chunks when it's stabilised
                        let raw =
                            u32::from_le_bytes(color_table[offset..offset + 4].try_into().unwrap());

                        <<D as DrawTarget>::Color as PixelColor>::Raw::from_u32(raw).into()
                    }),
                )
            }
            _ => target.fill_contiguous(
                &Rectangle::new(Point::zero(), self.size()),
                Pixels::new(self.pixels()).map(|Pixel(_, color)| color),
            ),
        }
    }
}
