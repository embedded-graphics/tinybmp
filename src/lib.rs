//! A small BMP parser primarily for embedded, no-std environments but usable anywhere.
//!
//! This crate is primarily targeted at drawing BMP images to [`embedded_graphics`] [`DrawTarget`]s,
//! but can also be used to parse BMP files for other applications.
//!
//! # Examples
//!
//! ## Draw a BMP image to an embedded-graphics draw target
//!
//! The [`Bmp`] struct is used together with [`embedded_graphics`]' [`Image`] struct to display BMP
//! files on any draw target.
//!
//! ```
//! # fn main() -> Result<(), core::convert::Infallible> {
//! use embedded_graphics::{image::Image, prelude::*};
//! use tinybmp::Bmp;
//! # use embedded_graphics::mock_display::MockDisplay;
//! # use embedded_graphics::pixelcolor::Rgb565;
//! # let mut display: MockDisplay<Rgb565> = MockDisplay::default();
//!
//! // Include the BMP file data.
//! let bmp_data = include_bytes!("../tests/chessboard-8px-color-16bit.bmp");
//!
//! // Parse the BMP file.
//! let bmp = Bmp::from_slice(bmp_data).unwrap();
//!
//! // Draw the image with the top left corner at (10, 20) by wrapping it in
//! // an embedded-graphics `Image`.
//! Image::new(&bmp, Point::new(10, 20)).draw(&mut display)?;
//! # Ok::<(), core::convert::Infallible>(()) }
//! ```
//!
//! ## Using the pixel iterator
//!
//! To access the image data for other applications the [`Bmp::pixels`] method returns an iterator
//! over all pixels in the BMP file. The colors inside the BMP file will automatically converted to
//! one of the [color types] in [`embedded_graphics`].
//!
//! ```
//! # fn main() -> Result<(), core::convert::Infallible> {
//! use embedded_graphics::{pixelcolor::Rgb888, prelude::*};
//! use tinybmp::Bmp;
//!
//! // Include the BMP file data.
//! let bmp_data = include_bytes!("../tests/chessboard-8px-24bit.bmp");
//!
//! // Parse the BMP file.
//! // Note that it is necessary to explicitly specify the color type which the colors in the BMP
//! // file will be converted into.
//! let bmp = Bmp::<Rgb888>::from_slice(bmp_data).unwrap();
//!
//! for Pixel(position, color) in bmp.pixels() {
//!     println!("R: {}, G: {}, B: {} @ ({})", color.r(), color.g(), color.b(), position);
//! }
//! # Ok::<(), core::convert::Infallible>(()) }
//! ```
//!
//! ## Accessing individual pixels
//!
//! [`Bmp::pixel`] can be used to get the color of individual pixels. The returned color will be automatically
//! converted to one of the [color types] in [`embedded_graphics`].
//!
//! ```
//! # fn main() -> Result<(), core::convert::Infallible> {
//! use embedded_graphics::{pixelcolor::Rgb888, image::GetPixel, prelude::*};
//! use tinybmp::Bmp;
//!
//! // Include the BMP file data.
//! let bmp_data = include_bytes!("../tests/chessboard-8px-24bit.bmp");
//!
//! // Parse the BMP file.
//! // Note that it is necessary to explicitly specify the color type which the colors in the BMP
//! // file will be converted into.
//! let bmp = Bmp::<Rgb888>::from_slice(bmp_data).unwrap();
//!
//! let pixel = bmp.pixel(Point::new(3, 2));
//!
//! assert_eq!(pixel, Some(Rgb888::WHITE));
//! # Ok::<(), core::convert::Infallible>(()) }
//! ```
//!
//! ## Accessing the raw image data
//!
//! For most applications the higher level access provided by [`Bmp`] is sufficient. But in case
//! lower level access is necessary the [`RawBmp`] struct can be used to access BMP [header
//! information] and the [color table]. A [`RawBmp`] object can be created directly from image data
//! by using [`from_slice`] or by accessing the underlying raw object of a [`Bmp`] object with
//! [`Bmp::as_raw`].
//!
//! Similar to [`Bmp::pixel`], [`RawBmp::pixel`] can be used to get raw pixel color values as a
//! `u32`.
//!
//! ```
//! use embedded_graphics::prelude::*;
//! use tinybmp::{RawBmp, Bpp, Header, RawPixel, RowOrder};
//!
//! let bmp = RawBmp::from_slice(include_bytes!("../tests/chessboard-8px-24bit.bmp"))
//!     .expect("Failed to parse BMP image");
//!
//! // Read the BMP header
//! assert_eq!(
//!     bmp.header(),
//!     &Header {
//!         file_size: 314,
//!         image_data_start: 122,
//!         bpp: Bpp::Bits24,
//!         image_size: Size::new(8, 8),
//!         image_data_len: 192,
//!         channel_masks: None,
//!         row_order: RowOrder::BottomUp,
//!     }
//! );
//!
//! # // Check that raw image data slice is the correct length (according to parsed header)
//! # assert_eq!(bmp.image_data().len(), bmp.header().image_data_len as usize);
//! // Get an iterator over the pixel coordinates and values in this image and load into a vec
//! let pixels: Vec<RawPixel> = bmp.pixels().collect();
//!
//! // Loaded example image is 8x8px
//! assert_eq!(pixels.len(), 8 * 8);
//!
//! // Individual raw pixel values can also be read
//! let pixel = bmp.pixel(Point::new(3, 2));
//!
//! // The raw value for a white pixel in the source image
//! assert_eq!(pixel, Some(0xFFFFFFu32));
//! ```
//!
//! # Minimum supported Rust version
//!
//! The minimum supported Rust version for tinybmp is `1.61` or greater. Ensure you have the correct
//! version of Rust installed, preferably through <https://rustup.rs>.
//!
//! <!-- README-LINKS
//! [`Bmp`]: https://docs.rs/tinybmp/latest/tinybmp/struct.Bmp.html
//! [`Bmp::pixels`]: https://docs.rs/tinybmp/latest/tinybmp/struct.Bmp.html#method.pixels
//! [`Bmp::pixel`]: https://docs.rs/tinybmp/latest/tinybmp/struct.Bmp.html#method.pixel
//! [`Bmp::as_raw`]: https://docs.rs/tinybmp/latest/tinybmp/struct.Bmp.html#method.as_raw
//! [`RawBmp`]: https://docs.rs/tinybmp/latest/tinybmp/struct.RawBmp.html
//! [`RawBmp::pixel`]: https://docs.rs/tinybmp/latest/tinybmp/struct.RawBmp.html#method.pixel
//! [header information]: https://docs.rs/tinybmp/latest/tinybmp/struct.RawBmp.html#method.header
//! [color table]: https://docs.rs/tinybmp/latest/tinybmp/struct.RawBmp.html#method.color_table
//! [`from_slice`]: https://docs.rs/tinybmp/latest/tinybmp/struct.RawBmp.html#method.from_slice
//!
//! [`embedded_graphics`]: https://docs.rs/embedded_graphics
//! [color types]: https://docs.rs/embedded-graphics/latest/embedded_graphics/pixelcolor/index.html#structs
//! [`DrawTarget`]: https://docs.rs/embedded-graphics/latest/embedded_graphics/draw_target/trait.DrawTarget.html
//! [`Image`]: https://docs.rs/embedded-graphics/latest/embedded_graphics/image/struct.Image.html
//! README-LINKS -->
//!
//! [`DrawTarget`]: embedded_graphics::draw_target::DrawTarget
//! [`Image`]: embedded_graphics::image::Image
//! [color types]: embedded_graphics::pixelcolor#structs
//! [header information]: RawBmp::header
//! [color table]: RawBmp::color_table
//! [`from_slice`]: RawBmp::from_slice

#![no_std]
#![deny(missing_docs)]
#![deny(missing_debug_implementations)]
#![deny(missing_copy_implementations)]
#![deny(trivial_casts)]
#![deny(trivial_numeric_casts)]
#![deny(unsafe_code)]
#![deny(unstable_features)]
#![deny(unused_import_braces)]
#![deny(unused_qualifications)]
#![deny(rustdoc::broken_intra_doc_links)]
#![deny(rustdoc::private_intra_doc_links)]

use core::marker::PhantomData;

use embedded_graphics::{
    image::GetPixel,
    pixelcolor::{
        raw::{RawU1, RawU16, RawU24, RawU32, RawU4, RawU8},
        Rgb555, Rgb565, Rgb888,
    },
    prelude::*,
    primitives::Rectangle,
};

mod color_table;
mod header;
mod iter;
mod parser;
mod raw_bmp;
mod raw_iter;

use raw_bmp::ColorType;
use raw_iter::RawColors;

pub use color_table::ColorTable;
pub use header::{Bpp, ChannelMasks, Header, RowOrder};
pub use iter::Pixels;
pub use raw_bmp::RawBmp;
pub use raw_iter::{RawPixel, RawPixels};

/// A BMP-format bitmap.
///
/// See the [crate-level documentation](crate) for more information.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Bmp<'a, C> {
    raw_bmp: RawBmp<'a>,
    color_type: PhantomData<C>,
}

impl<'a, C> Bmp<'a, C>
where
    C: PixelColor + From<Rgb555> + From<Rgb565> + From<Rgb888>,
{
    /// Creates a bitmap object from a byte slice.
    ///
    /// The created object keeps a shared reference to the input and does not dynamically allocate
    /// memory.
    pub fn from_slice(bytes: &'a [u8]) -> Result<Self, ParseError> {
        let raw_bmp = RawBmp::from_slice(bytes)?;

        Ok(Self {
            raw_bmp,
            color_type: PhantomData,
        })
    }

    /// Returns an iterator over the pixels in this image.
    ///
    /// The iterator always starts at the top left corner of the image, regardless of the row order
    /// of the BMP file. The coordinate of the first pixel is `(0, 0)`.
    pub fn pixels(&self) -> Pixels<'_, C> {
        Pixels::new(self)
    }

    /// Returns a reference to the raw BMP image.
    ///
    /// The [`RawBmp`] instance can be used to access lower level information about the BMP file.
    pub const fn as_raw(&self) -> &RawBmp<'a> {
        &self.raw_bmp
    }
}

impl<C> ImageDrawable for Bmp<'_, C>
where
    C: PixelColor + From<Rgb555> + From<Rgb565> + From<Rgb888>,
{
    type Color = C;

    fn draw<D>(&self, target: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = C>,
    {
        let area = self.bounding_box();

        match self.raw_bmp.color_type {
            ColorType::Index1 => {
                if let Some(color_table) = self.raw_bmp.color_table() {
                    let fallback_color = C::from(Rgb888::BLACK);
                    let color_table: [C; 2] = [
                        color_table.get(0).map(Into::into).unwrap_or(fallback_color),
                        color_table.get(1).map(Into::into).unwrap_or(fallback_color),
                    ];

                    let colors = RawColors::<RawU1>::new(&self.raw_bmp).map(|index| {
                        color_table
                            .get(usize::from(index.into_inner()))
                            .copied()
                            .unwrap_or(fallback_color)
                    });
                    target.fill_contiguous(&area, colors)
                } else {
                    Ok(())
                }
            }
            ColorType::Index4 => {
                if let Some(color_table) = self.raw_bmp.color_table() {
                    let fallback_color = C::from(Rgb888::BLACK);

                    let colors = RawColors::<RawU4>::new(&self.raw_bmp).map(|index| {
                        color_table
                            .get(u32::from(index.into_inner()))
                            .map(Into::into)
                            .unwrap_or(fallback_color)
                    });

                    target.fill_contiguous(&area, colors)
                } else {
                    Ok(())
                }
            }
            ColorType::Index8 => {
                if let Some(color_table) = self.raw_bmp.color_table() {
                    let fallback_color = C::from(Rgb888::BLACK);

                    let colors = RawColors::<RawU8>::new(&self.raw_bmp).map(|index| {
                        color_table
                            .get(u32::from(index.into_inner()))
                            .map(Into::into)
                            .unwrap_or(fallback_color)
                    });

                    target.fill_contiguous(&area, colors)
                } else {
                    Ok(())
                }
            }
            ColorType::Rgb555 => target.fill_contiguous(
                &area,
                RawColors::<RawU16>::new(&self.raw_bmp).map(|raw| Rgb555::from(raw).into()),
            ),
            ColorType::Rgb565 => target.fill_contiguous(
                &area,
                RawColors::<RawU16>::new(&self.raw_bmp).map(|raw| Rgb565::from(raw).into()),
            ),
            ColorType::Rgb888 => target.fill_contiguous(
                &area,
                RawColors::<RawU24>::new(&self.raw_bmp).map(|raw| Rgb888::from(raw).into()),
            ),
            ColorType::Xrgb8888 => target.fill_contiguous(
                &area,
                RawColors::<RawU32>::new(&self.raw_bmp)
                    .map(|raw| Rgb888::from(RawU24::new(raw.into_inner())).into()),
            ),
        }
    }

    fn draw_sub_image<D>(&self, target: &mut D, area: &Rectangle) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        self.draw(&mut target.translated(-area.top_left).clipped(area))
    }
}

impl<C> OriginDimensions for Bmp<'_, C>
where
    C: PixelColor,
{
    fn size(&self) -> Size {
        self.raw_bmp.header().image_size
    }
}

impl<C> GetPixel for Bmp<'_, C>
where
    C: PixelColor + From<Rgb555> + From<Rgb565> + From<Rgb888>,
{
    type Color = C;

    fn pixel(&self, p: Point) -> Option<Self::Color> {
        match self.raw_bmp.color_type {
            ColorType::Index1 => self
                .raw_bmp
                .color_table()
                .and_then(|color_table| color_table.get(self.raw_bmp.pixel(p)?))
                .map(Into::into),
            ColorType::Index4 => self
                .raw_bmp
                .color_table()
                .and_then(|color_table| color_table.get(self.raw_bmp.pixel(p)?))
                .map(Into::into),
            ColorType::Index8 => self
                .raw_bmp
                .color_table()
                .and_then(|color_table| color_table.get(self.raw_bmp.pixel(p)?))
                .map(Into::into),
            ColorType::Rgb555 => self
                .raw_bmp
                .pixel(p)
                .map(|raw| Rgb555::from(RawU16::from_u32(raw)).into()),
            ColorType::Rgb565 => self
                .raw_bmp
                .pixel(p)
                .map(|raw| Rgb565::from(RawU16::from_u32(raw)).into()),
            ColorType::Rgb888 => self
                .raw_bmp
                .pixel(p)
                .map(|raw| Rgb888::from(RawU24::from_u32(raw)).into()),
            ColorType::Xrgb8888 => self
                .raw_bmp
                .pixel(p)
                .map(|raw| Rgb888::from(RawU24::from_u32(raw)).into()),
        }
    }
}

/// Parse error.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum ParseError {
    /// The image uses an unsupported bit depth.
    UnsupportedBpp(u16),

    /// Unexpected end of file.
    UnexpectedEndOfFile,

    /// Invalid file signatures.
    ///
    /// BMP files must start with `BM`.
    InvalidFileSignature([u8; 2]),

    /// Unsupported compression method.
    UnsupportedCompressionMethod(u32),

    /// Unsupported header length.
    UnsupportedHeaderLength(u32),

    /// Unsupported channel masks.
    UnsupportedChannelMasks,

    /// Invalid image dimensions.
    InvalidImageDimensions,
}

#[cfg(test)]
mod tests {
    use super::*;

    const BMP_DATA: &[u8] = include_bytes!("../tests/chessboard-8px-1bit.bmp");

    fn bmp_data() -> [u8; 94] {
        BMP_DATA.try_into().unwrap()
    }

    #[test]
    fn error_unsupported_bpp() {
        // Replace BPP value with an invalid value of 42.
        let mut data = bmp_data();
        data[0x1C..0x1C + 2].copy_from_slice(&42u16.to_le_bytes());

        assert_eq!(
            Bmp::<Rgb888>::from_slice(&data),
            Err(ParseError::UnsupportedBpp(42))
        );
    }

    #[test]
    fn error_empty_file() {
        assert_eq!(
            Bmp::<Rgb888>::from_slice(&[]),
            Err(ParseError::UnexpectedEndOfFile)
        );
    }

    #[test]
    fn error_truncated_header() {
        let data = &BMP_DATA[0..10];

        assert_eq!(
            Bmp::<Rgb888>::from_slice(data),
            Err(ParseError::UnexpectedEndOfFile)
        );
    }

    #[test]
    fn error_truncated_image_data() {
        let (_, data) = BMP_DATA.split_last().unwrap();

        assert_eq!(
            Bmp::<Rgb888>::from_slice(data),
            Err(ParseError::UnexpectedEndOfFile)
        );
    }

    #[test]
    fn error_invalid_signature() {
        // Replace signature with "EG".
        let mut data = bmp_data();
        data[0..2].copy_from_slice(b"EG");

        assert_eq!(
            Bmp::<Rgb888>::from_slice(&data),
            Err(ParseError::InvalidFileSignature([b'E', b'G']))
        );
    }

    #[test]
    fn error_compression_method() {
        // Replace compression method with BI_JPEG (4).
        let mut data = bmp_data();
        data[0x1E..0x1E + 4].copy_from_slice(&4u32.to_le_bytes());

        assert_eq!(
            Bmp::<Rgb888>::from_slice(&data),
            Err(ParseError::UnsupportedCompressionMethod(4))
        );
    }

    #[test]
    fn error_header_length() {
        // Replace header length with invalid length of 16.
        let mut data = bmp_data();
        data[0x0E..0x0E + 4].copy_from_slice(&16u32.to_le_bytes());

        assert_eq!(
            Bmp::<Rgb888>::from_slice(&data),
            Err(ParseError::UnsupportedHeaderLength(16))
        );
    }
}
