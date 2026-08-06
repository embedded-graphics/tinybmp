use core::{iter, slice};

use embedded_graphics::{
    iterator::raw::RawDataSlice,
    pixelcolor::raw::{LittleEndian, RawU1, RawU16, RawU24, RawU32, RawU4, RawU8},
    prelude::*,
};

use crate::{
    header::{Bpp, CompressionMethod, RowOrder},
    raw_bmp::RawBmp,
};

/// Iterator over raw pixel colors.
#[allow(missing_debug_implementations)]
pub struct RawColors<'a, R>
where
    RawDataSlice<'a, R, LittleEndian>: IntoIterator<Item = R>,
{
    rows: slice::ChunksExact<'a, u8>,
    row_order: RowOrder,
    current_row: iter::Take<<RawDataSlice<'a, R, LittleEndian> as IntoIterator>::IntoIter>,
    width: usize,
}

impl<'a, R> RawColors<'a, R>
where
    RawDataSlice<'a, R, LittleEndian>: IntoIterator<Item = R>,
{
    /// Create a new raw color iterator.
    pub(crate) fn new(raw_bmp: &'a RawBmp<'a>) -> Self {
        let header = raw_bmp.header();

        let width = header.image_size.width as usize;

        Self {
            rows: raw_bmp
                .image_data()
                .chunks_exact(header.bytes_per_row_uncompressed()),
            row_order: raw_bmp.header().row_order,
            current_row: RawDataSlice::new(&[]).into_iter().take(0),
            width,
        }
    }
}

impl<'a, R> Iterator for RawColors<'a, R>
where
    RawDataSlice<'a, R, LittleEndian>: IntoIterator<Item = R>,
{
    type Item = R;

    fn next(&mut self) -> Option<Self::Item> {
        self.current_row.next().or_else(|| {
            let next_row = match self.row_order {
                RowOrder::TopDown => self.rows.next(),
                RowOrder::BottomUp => self.rows.next_back(),
            }?;

            self.current_row = RawDataSlice::new(next_row).into_iter().take(self.width);

            self.current_row.next()
        })
    }
}

/// Iterator over the raw colors in the image.
///
/// See [`RawBmp::colors`](RawBmp::colors) for more information.
pub enum DynamicRawColors<'a> {
    /// 1 bit per pixel
    Bpp1(RawColors<'a, RawU1>),
    /// 4 bits per pixel
    Bpp4(RawColors<'a, RawU4>),
    /// 8 bits per pixel
    Bpp8(RawColors<'a, RawU8>),
    /// 16 bits per pixel
    Bpp16(RawColors<'a, RawU16>),
    /// 24 bits per pixel
    Bpp24(RawColors<'a, RawU24>),
    /// 32 bits per pixel
    Bpp32(RawColors<'a, RawU32>),
    /// RLE encoded with 4 bits per pixel
    Bpp4Rle(RleColors<RawU4, Rle4Runs<'a>>),
    /// RLE encoded with 8 bits per pixel
    Bpp8Rle(RleColors<RawU8, Rle8Runs<'a>>),
}

impl core::fmt::Debug for DynamicRawColors<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            DynamicRawColors::Bpp1(_) => f.debug_tuple("DynamicRawColors::Bpp1").finish(),
            DynamicRawColors::Bpp4(_) => f.debug_tuple("DynamicRawColors::Bpp4").finish(),
            DynamicRawColors::Bpp8(_) => f.debug_tuple("DynamicRawColors::Bpp8").finish(),
            DynamicRawColors::Bpp16(_) => f.debug_tuple("DynamicRawColors::Bpp16").finish(),
            DynamicRawColors::Bpp24(_) => f.debug_tuple("DynamicRawColors::Bpp24").finish(),
            DynamicRawColors::Bpp32(_) => f.debug_tuple("DynamicRawColors::Bpp32").finish(),
            DynamicRawColors::Bpp4Rle(_) => f.debug_tuple("DynamicRawColors::Bpp4Rle").finish(),
            DynamicRawColors::Bpp8Rle(_) => f.debug_tuple("DynamicRawColors::Bpp8Rle").finish(),
        }
    }
}

impl DynamicRawColors<'_> {
    /// Get the row order of the bitmap.
    pub fn row_order(&self) -> RowOrder {
        match self {
            DynamicRawColors::Bpp1(colors) => colors.row_order,
            DynamicRawColors::Bpp4(colors) => colors.row_order,
            DynamicRawColors::Bpp8(colors) => colors.row_order,
            DynamicRawColors::Bpp16(colors) => colors.row_order,
            DynamicRawColors::Bpp24(colors) => colors.row_order,
            DynamicRawColors::Bpp32(colors) => colors.row_order,
            DynamicRawColors::Bpp4Rle(_) => RowOrder::BottomUp,
            DynamicRawColors::Bpp8Rle(_) => RowOrder::BottomUp,
        }
    }
}

impl Iterator for DynamicRawColors<'_> {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            DynamicRawColors::Bpp1(colors) => colors.next().map(|r| u32::from(r.into_inner())),
            DynamicRawColors::Bpp4(colors) => colors.next().map(|r| u32::from(r.into_inner())),
            DynamicRawColors::Bpp8(colors) => colors.next().map(|r| u32::from(r.into_inner())),
            DynamicRawColors::Bpp16(colors) => colors.next().map(|r| u32::from(r.into_inner())),
            DynamicRawColors::Bpp24(colors) => colors.next().map(|r| r.into_inner()),
            DynamicRawColors::Bpp32(colors) => colors.next().map(|r| r.into_inner()),
            DynamicRawColors::Bpp4Rle(colors) => colors.next().map(|r| u32::from(r.into_inner())),
            DynamicRawColors::Bpp8Rle(colors) => colors.next().map(|r| u32::from(r.into_inner())),
        }
    }
}

/// A run of pixels in an RLE compressed bitmap.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RleRun<R> {
    /// A run of pixels of a single color.
    Solid {
        /// The color of every pixel in the run.
        color: R,
        /// The number of pixels in the run.
        count: usize,
    },
    /// A run of pixels alternating between two colors, starting with `colors[0]`.
    Alternating {
        /// The two alternating colors.
        colors: [R; 2],
        /// The number of pixels in the run.
        count: usize,
    },
}

impl<R> RleRun<R> {
    /// Returns the number of pixels in the run.
    pub const fn count(&self) -> usize {
        match self {
            RleRun::Solid { count, .. } => *count,
            RleRun::Alternating { count, .. } => *count,
        }
    }
}

/// Common interface of the RLE run iterators.
///
/// The iterators return `None` at the end of each row and are restarted for the
/// next row with [`start_row`](RleRuns::start_row). Use
/// [`is_complete`](RleRuns::is_complete) to distinguish the end of the bitmap
/// from the end of a row.
pub trait RleRuns: Iterator {
    /// Indicates that a new line is starting. Required for correct RLE decoding.
    fn start_row(&mut self);

    /// Returns whether the end of the bitmap was reached.
    fn is_complete(&self) -> bool;
}

/// Iterator over individual BMP RLE encoded pixels.
///
/// Transforms the runs produced by an [`RleRuns`] iterator back into individual
/// pixel colors.
#[derive(Debug)]
pub struct RleColors<C, I> {
    runs: I,
    colors: [C; 2],
    count: usize,
    phase: usize,
}

impl<C: Copy + Default, I: RleRuns<Item = RleRun<C>>> RleColors<C, I> {
    /// Create a new RLE pixel iterator.
    pub(crate) fn new(runs: I) -> Self {
        Self {
            runs,
            colors: [C::default(); 2],
            count: 0,
            phase: 0,
        }
    }

    /// Gets a mutable reference to the underlying runs iterator.
    pub fn as_runs(&mut self) -> &mut I {
        &mut self.runs
    }

    /// Consumes `self`, returning the underlying runs iterator.
    pub fn to_runs(self) -> I {
        self.runs
    }
}

impl<C: Copy + Default, I: RleRuns<Item = RleRun<C>>> Iterator for RleColors<C, I> {
    type Item = C;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.count > 0 {
                self.count -= 1;
                let color = self.colors[self.phase];
                self.phase ^= 1;
                return Some(color);
            }

            match self.runs.next() {
                Some(RleRun::Solid { color, count }) => {
                    self.colors = [color; 2];
                    self.count = count;
                    self.phase = 0;
                }
                Some(RleRun::Alternating { colors, count }) => {
                    self.colors = colors;
                    self.count = count;
                    self.phase = 0;
                }
                None if self.runs.is_complete() => return None,
                // End of a row: continue with the next one.
                None => self.runs.start_row(),
            }
        }
    }
}

pub struct PixelPoints {
    /// The location of the next pixel.
    next_pixel: Point,
    /// The number of pixels in a row.
    width: u32,
    /// Delta for row movement.
    delta_y: i32,
}

impl PixelPoints {
    pub(crate) fn new(image_size: Size, row_order: RowOrder) -> Self {
        let next_pixel = match row_order {
            RowOrder::TopDown => Point::new(0, 0),
            RowOrder::BottomUp => Point::new(0, (image_size.height - 1) as i32),
        };
        let delta_y = match row_order {
            RowOrder::TopDown => 1,
            RowOrder::BottomUp => -1,
        };
        Self {
            next_pixel,
            width: image_size.width,
            delta_y,
        }
    }
}

impl Iterator for PixelPoints {
    type Item = Point;
    fn next(&mut self) -> Option<Self::Item> {
        let old_position = self.next_pixel;
        self.next_pixel.x += 1;
        if self.next_pixel.x == self.width as i32 {
            self.next_pixel.x = 0;
            self.next_pixel.y += self.delta_y;
        }
        Some(old_position)
    }
}

/// Iterator over BMP RLE8 encoded runs.
#[derive(Debug)]
pub struct Rle8Runs<'a> {
    data: &'a [u8],
    absolute: Option<Absolute8>,
    at_row_end: bool,
    complete: bool,
}

/// Absolute mode literal pixels that still need to be emitted.
#[derive(Debug, Clone, Copy)]
struct Absolute8 {
    remaining: u8,
    has_padding: bool,
}

impl<'a> Rle8Runs<'a> {
    /// Create a new RLE run iterator.
    pub(crate) fn new(raw_bmp: &RawBmp<'a>) -> Rle8Runs<'a> {
        Rle8Runs {
            data: raw_bmp.image_data(),
            absolute: None,
            at_row_end: false,
            complete: false,
        }
    }
}

impl Iterator for Rle8Runs<'_> {
    type Item = RleRun<RawU8>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.complete || self.at_row_end {
                return None;
            }

            if let Some(absolute) = self.absolute {
                let Some(&value) = self.data.first() else {
                    self.complete = true;
                    return None;
                };

                let skip = if absolute.remaining == 0 && absolute.has_padding {
                    // remove the padding byte too
                    2
                } else {
                    1
                };
                let Some(rest) = self.data.get(skip..) else {
                    self.complete = true;
                    return None;
                };
                self.data = rest;

                self.absolute = if absolute.remaining == 0 {
                    None
                } else {
                    Some(Absolute8 {
                        remaining: absolute.remaining - 1,
                        ..absolute
                    })
                };

                return Some(RleRun::Solid {
                    color: RawU8::from(value),
                    count: 1,
                });
            }

            let Some(&[length, param]) = self.data.first_chunk() else {
                self.complete = true;
                return None;
            };
            self.data = &self.data[2..];

            if length > 0 {
                // An encoded run
                return Some(RleRun::Solid {
                    color: RawU8::from(param),
                    count: usize::from(length),
                });
            }

            // The first byte of the pair can be set to zero to indicate an
            // escape character that denotes the end of a line, the end of a
            // bitmap, or a delta, depending on the value of the second byte.
            match param {
                // End of line - move to next row
                0 => self.at_row_end = true,
                // End of bitmap
                1 => self.complete = true,
                // Delta encoding is unsupported.
                2 => self.complete = true,
                _ => {
                    // Absolute mode
                    self.absolute = Some(Absolute8 {
                        remaining: param - 1,
                        // Odd lengths in RLE8 require 1 byte padding
                        has_padding: (param % 2) != 0,
                    });
                }
            }
        }
    }
}

impl RleRuns for Rle8Runs<'_> {
    fn start_row(&mut self) {
        self.at_row_end = false;
    }

    fn is_complete(&self) -> bool {
        self.complete
    }
}

/// Iterator over BMP RLE4 encoded runs.
#[derive(Debug)]
pub struct Rle4Runs<'a> {
    data: &'a [u8],
    absolute: Option<Absolute4>,
    at_row_end: bool,
    complete: bool,
}

/// Absolute mode literal nibbles that still need to be emitted.
#[derive(Debug, Clone, Copy)]
struct Absolute4 {
    remaining: u8,
    is_odd: bool,
    has_padding: bool,
}

impl<'a> Rle4Runs<'a> {
    /// Create a new RLE run iterator.
    pub(crate) fn new(raw_bmp: &RawBmp<'a>) -> Rle4Runs<'a> {
        Rle4Runs {
            data: raw_bmp.image_data(),
            absolute: None,
            at_row_end: false,
            complete: false,
        }
    }
}

impl Iterator for Rle4Runs<'_> {
    type Item = RleRun<RawU4>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.complete || self.at_row_end {
                return None;
            }

            if let Some(absolute) = self.absolute {
                // Here `remaining` is a count of nibbles, not bytes

                // [00, 04, 45, 67] => 4 5 6 7
                //                     ^ ^ ^ ^
                //                     | | | +-- remaining is 0, is_odd is 0, we want [1]right
                //                     | | +-- remaining is 1, is_odd is 0, we want [1]left
                //                     | +-- remaining is 2, is_odd is 0, we want [0]right
                //                     +-- remaining is 3, is_odd is 0, we want [0]left
                // [00, 03, 45, 60] => 4 5 6
                //                     ^ ^ ^
                //                     | | +-- remaining is 0, is_odd is 1, we want [1]left
                //                     | +-- remaining is 1, is_odd is 1, we want [0]right
                //                     +-- remaining is 2, is_odd is 1, we want [0]left

                let remaining = absolute.remaining;
                let remaining_is_odd = (remaining % 2) != 0;
                let want_left = remaining_is_odd != absolute.is_odd;

                let Some(&value) = self.data.first() else {
                    self.complete = true;
                    return None;
                };
                let nibble_value = if want_left { value >> 4 } else { value & 0x0F };

                let mut skip = 0;
                if !want_left || remaining == 0 {
                    skip += 1;
                }
                if remaining == 0 && absolute.has_padding {
                    // remove the padding byte too
                    skip += 1;
                }
                if skip > 0 {
                    let Some(rest) = self.data.get(skip..) else {
                        self.complete = true;
                        return None;
                    };
                    self.data = rest;
                }

                self.absolute = if remaining == 0 {
                    None
                } else {
                    Some(Absolute4 {
                        remaining: remaining - 1,
                        ..absolute
                    })
                };

                return Some(RleRun::Solid {
                    color: RawU4::from(nibble_value),
                    count: 1,
                });
            }

            let Some(&[length, param]) = self.data.first_chunk() else {
                self.complete = true;
                return None;
            };
            self.data = &self.data[2..];

            if length > 0 {
                // An encoded run of two alternating nibbles
                let left = RawU4::from(param >> 4);
                let right = RawU4::from(param & 0x0F);
                let count = usize::from(length);

                return Some(if left == right {
                    RleRun::Solid { color: left, count }
                } else {
                    RleRun::Alternating {
                        colors: [left, right],
                        count,
                    }
                });
            }

            // The first byte of the pair can be set to zero to indicate an
            // escape character that denotes the end of a line, the end of a
            // bitmap, or a delta, depending on the value of the second byte.
            match param {
                // End of line - move to next row
                0 => self.at_row_end = true,
                // End of bitmap
                1 => self.complete = true,
                // Delta encoding is unsupported.
                2 => self.complete = true,
                num_pixels => {
                    let num_bytes = num_pixels.div_ceil(2);
                    // Absolute mode
                    self.absolute = Some(Absolute4 {
                        remaining: num_pixels - 1,
                        is_odd: (num_pixels % 2) != 0,
                        // padding if the number of *bytes* is odd
                        has_padding: num_bytes & 1 != 0,
                    });
                }
            }
        }
    }
}

impl RleRuns for Rle4Runs<'_> {
    fn start_row(&mut self) {
        self.at_row_end = false;
    }

    fn is_complete(&self) -> bool {
        self.complete
    }
}

/// Iterator over individual BMP pixels.
///
/// Each pixel is returned as a `u32` regardless of the bit depth of the source image.
#[allow(missing_debug_implementations)]
pub struct RawPixels<'a> {
    pub(crate) colors: DynamicRawColors<'a>,
    pub(crate) points: PixelPoints,
}

impl<'a> RawPixels<'a> {
    pub(crate) fn new(raw_bmp: &'a RawBmp<'a>) -> Self {
        let header = raw_bmp.header();
        match header.compression_method {
            CompressionMethod::Rle4 => {
                let colors = RleColors::new(Rle4Runs::new(raw_bmp));
                let points = PixelPoints::new(header.image_size, RowOrder::BottomUp);
                Self {
                    colors: DynamicRawColors::Bpp4Rle(colors),
                    points,
                }
            }
            CompressionMethod::Rle8 => {
                let colors = RleColors::new(Rle8Runs::new(raw_bmp));
                let points = PixelPoints::new(header.image_size, RowOrder::BottomUp);
                Self {
                    colors: DynamicRawColors::Bpp8Rle(colors),
                    points,
                }
            }
            CompressionMethod::Rgb | CompressionMethod::Bitfields => {
                let points = PixelPoints::new(header.image_size, header.row_order);
                let colors = match header.bpp {
                    Bpp::Bits1 => DynamicRawColors::Bpp1(RawColors::new(raw_bmp)),
                    Bpp::Bits4 => DynamicRawColors::Bpp4(RawColors::new(raw_bmp)),
                    Bpp::Bits8 => DynamicRawColors::Bpp8(RawColors::new(raw_bmp)),
                    Bpp::Bits16 => DynamicRawColors::Bpp16(RawColors::new(raw_bmp)),
                    Bpp::Bits24 => DynamicRawColors::Bpp24(RawColors::new(raw_bmp)),
                    Bpp::Bits32 => DynamicRawColors::Bpp32(RawColors::new(raw_bmp)),
                };
                Self { colors, points }
            }
        }
    }
}

impl Iterator for RawPixels<'_> {
    type Item = RawPixel;

    fn next(&mut self) -> Option<Self::Item> {
        let color = self.colors.next()?;
        let position = self.points.next()?;
        Some(RawPixel { position, color })
    }
}

/// Pixel with raw pixel color stored as a `u32`.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Default)]
pub struct RawPixel {
    /// The position relative to the top left corner of the image.
    pub position: Point,

    /// The raw pixel color.
    pub color: u32,
}
