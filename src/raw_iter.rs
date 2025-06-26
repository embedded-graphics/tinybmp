use core::{iter, slice};

use embedded_graphics::{
    iterator::raw::RawDataSlice,
    pixelcolor::raw::{LittleEndianMsb0, RawU1, RawU16, RawU24, RawU32, RawU4, RawU8},
    prelude::*,
    primitives::{rectangle, Rectangle},
};

use crate::{
    header::{Bpp, CompressionMethod, RowOrder},
    raw_bmp::RawBmp,
};

/// Iterator over raw pixel colors.
#[allow(missing_debug_implementations)]
pub struct RawColors<'a, R>
where
    RawDataSlice<'a, R, LittleEndianMsb0>: IntoIterator<Item = R>,
{
    rows: slice::ChunksExact<'a, u8>,
    row_order: RowOrder,
    current_row: iter::Take<<RawDataSlice<'a, R, LittleEndianMsb0> as IntoIterator>::IntoIter>,
    width: usize,
}

impl<'a, R> RawColors<'a, R>
where
    RawDataSlice<'a, R, LittleEndianMsb0>: IntoIterator<Item = R>,
{
    pub(crate) fn new(raw_bmp: &'a RawBmp<'a>) -> Self {
        let header = raw_bmp.header();

        let width = header.image_size.width as usize;

        Self {
            rows: raw_bmp.image_data().chunks_exact(header.bytes_per_row()),
            row_order: raw_bmp.header().row_order,
            current_row: RawDataSlice::new(&[]).into_iter().take(0),
            width,
        }
    }
}

impl<'a, R> Iterator for RawColors<'a, R>
where
    RawDataSlice<'a, R, LittleEndianMsb0>: IntoIterator<Item = R>,
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

enum DynamicRawColors<'a> {
    Bpp1(RawColors<'a, RawU1>),
    Bpp4(RawColors<'a, RawU4>),
    Bpp8(RawColors<'a, RawU8>),
    Bpp16(RawColors<'a, RawU16>),
    Bpp24(RawColors<'a, RawU24>),
    Bpp32(RawColors<'a, RawU32>),
    Bpp4Rle(Rle4Pixels<'a>),
    Bpp8Rle(Rle8Pixels<'a>),
}

impl<'a> DynamicRawColors<'a> {
    pub fn new(raw_bmp: &'a RawBmp<'a>) -> Self {
        let header = raw_bmp.header();
        match header.compression_method {
            CompressionMethod::Rle4 => DynamicRawColors::Bpp4Rle(Rle4Pixels::new(raw_bmp)),
            CompressionMethod::Rle8 => DynamicRawColors::Bpp8Rle(Rle8Pixels::new(raw_bmp)),
            CompressionMethod::Rgb | CompressionMethod::Bitfields => match header.bpp {
                Bpp::Bits1 => DynamicRawColors::Bpp1(RawColors::new(raw_bmp)),
                Bpp::Bits4 => DynamicRawColors::Bpp4(RawColors::new(raw_bmp)),
                Bpp::Bits8 => DynamicRawColors::Bpp8(RawColors::new(raw_bmp)),
                Bpp::Bits16 => DynamicRawColors::Bpp16(RawColors::new(raw_bmp)),
                Bpp::Bits24 => DynamicRawColors::Bpp24(RawColors::new(raw_bmp)),
                Bpp::Bits32 => DynamicRawColors::Bpp32(RawColors::new(raw_bmp)),
            },
        }
    }
}

/// The state for our RLE* decoder
#[derive(Debug)]
enum RleState {
    /// Need to read two bytes
    Starting,
    /// Producing a sequence of identical values
    Running {
        remaining: u8,
        value: u8,
        is_odd: bool,
    },
    /// Producing a sequence of unique values
    Absolute {
        remaining: u8,
        is_odd: bool,
        has_padding: bool,
    },
    /// Ran out of pixels
    EndOfBitmap,
}

/// Iterator over individual BMP RLE8 encoded pixels.
///
/// Each pixel is returned as a `u32` regardless of the bit depth of the source image.
#[derive(Debug)]
pub struct Rle8Pixels<'a> {
    /// Our source data
    data: &'a [u8],
    /// Our state
    rle_state: RleState,
    /// The width of a line in pixels
    width: u32,
    /// The location of the next pixel
    next_pixel: Point,
}

impl<'a> Rle8Pixels<'a> {
    /// Create a new RLE pixel iterator.
    pub(crate) fn new(raw_bmp: &RawBmp<'a>) -> Rle8Pixels<'a> {
        let header = raw_bmp.header();
        Rle8Pixels {
            data: raw_bmp.image_data(),
            rle_state: RleState::Starting,
            width: header.image_size.width,
            // RLE encoded bitmaps are upside down
            next_pixel: Point::new(0, (header.image_size.height - 1) as i32),
        }
    }

    /// Bump the cursor to the next position in the bitmap.
    ///
    /// Note that RLE bitmaps are upside down.
    fn move_position(&mut self) -> Point {
        let old_position = self.next_pixel;
        self.next_pixel.x += 1;
        if self.next_pixel.x == self.width as i32 {
            self.next_pixel.x = 0;
            self.next_pixel.y = self.next_pixel.y.saturating_sub(1);
        }
        old_position
    }
}

impl<'a> Iterator for Rle8Pixels<'a> {
    type Item = RawPixel;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.rle_state {
                RleState::EndOfBitmap => {
                    return None;
                }
                RleState::Absolute {
                    remaining,
                    is_odd,
                    has_padding,
                } => {
                    if remaining == 0 {
                        self.rle_state = RleState::Starting;
                    } else {
                        self.rle_state = RleState::Absolute {
                            remaining: remaining.saturating_sub(1),
                            is_odd,
                            has_padding,
                        };
                    }
                    let value = *self.data.first()?;
                    if remaining == 0 && has_padding {
                        self.data = self.data.get(2..)?;
                    } else {
                        self.data = self.data.get(1..)?;
                    }
                    let this_pixel = self.move_position();
                    return Some(RawPixel::new(this_pixel, u32::from(value)));
                }
                RleState::Running {
                    remaining,
                    value,
                    is_odd,
                } => {
                    if remaining == 0 {
                        self.rle_state = RleState::Starting;
                    } else {
                        self.rle_state = RleState::Running {
                            remaining: remaining.saturating_sub(1),
                            value,
                            is_odd,
                        };
                    }
                    let this_pixel = self.move_position();
                    return Some(RawPixel::new(this_pixel, u32::from(value)));
                }
                RleState::Starting => {
                    let length = *self.data.get(0)?;
                    let param = *self.data.get(1)?;
                    self.data = &self.data.get(2..)?;
                    match length {
                        0 => {
                            // The first byte of the pair can be set to zero to
                            // indicate an escape character that denotes the end of
                            // a line, the end of a bitmap, or a delta, depending on
                            // the value of the second byte. The interpretation of
                            // the escape depends on the value of the second byte of
                            // the pair, which can be one of the following values.
                            match param {
                                0 => {
                                    // End of line
                                    if self.next_pixel.x != 0 {
                                        return None;
                                    }
                                }
                                1 => {
                                    // End of bitmap
                                    self.rle_state = RleState::EndOfBitmap;
                                }
                                2 => {
                                    // Delta encoding is unsupported.
                                    return None;
                                }
                                _ => {
                                    // Absolute mode
                                    self.rle_state = RleState::Absolute {
                                        remaining: param.saturating_sub(1),
                                        is_odd: (param % 2) != 0,
                                        // Odd lengths in RLE8 require 1 byte padding
                                        has_padding: (param % 2) != 0,
                                    };
                                }
                            }
                        }
                        _ => {
                            // An encoded run
                            self.rle_state = RleState::Running {
                                remaining: length.saturating_sub(1),
                                value: param,
                                is_odd: (length % 2) != 0,
                            };
                        }
                    }
                }
            }
        }
    }
}

/// Iterator over individual BMP RLE4 encoded pixels.
///
/// Each pixel is returned as a `u32` regardless of the bit depth of the source image.
#[derive(Debug)]
pub struct Rle4Pixels<'a> {
    /// Our source data
    data: &'a [u8],
    /// Our state
    rle_state: RleState,
    /// The width of a line in pixels
    width: u32,
    /// The location of the next pixel
    next_pixel: Point,
}

impl<'a> Rle4Pixels<'a> {
    /// Create a new RLE pixel iterator.
    pub(crate) fn new(raw_bmp: &RawBmp<'a>) -> Rle4Pixels<'a> {
        let header = raw_bmp.header();
        Rle4Pixels {
            data: raw_bmp.image_data(),
            rle_state: RleState::Starting,
            width: header.image_size.width,
            // RLE encoded bitmaps are upside down
            next_pixel: Point::new(0, (header.image_size.height - 1) as i32),
        }
    }

    /// Bump the cursor to the next position in the bitmap.
    ///
    /// Note that RLE bitmaps are upside down.
    fn move_position(&mut self) -> Point {
        let old_position = self.next_pixel;
        self.next_pixel.x += 1;
        if self.next_pixel.x == self.width as i32 {
            self.next_pixel.x = 0;
            self.next_pixel.y = self.next_pixel.y.saturating_sub(1);
        }
        old_position
    }
}

impl<'a> Iterator for Rle4Pixels<'a> {
    type Item = RawPixel;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.rle_state {
                RleState::EndOfBitmap => {
                    return None;
                }
                RleState::Absolute {
                    remaining,
                    is_odd,
                    has_padding,
                } => {
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

                    let remaining_is_odd = (remaining % 2) != 0;
                    let want_left = remaining_is_odd != is_odd;

                    if remaining == 0 {
                        self.rle_state = RleState::Starting;
                    } else {
                        self.rle_state = RleState::Absolute {
                            remaining: remaining.saturating_sub(1),
                            is_odd,
                            has_padding,
                        };
                    }

                    let value = *self.data.first()?;
                    let nibble_value = if want_left { value >> 4 } else { value & 0x0F };
                    if !want_left || remaining == 0 {
                        self.data = self.data.get(1..)?;
                    }
                    if remaining == 0 && has_padding {
                        // remove the padding byte too
                        self.data = self.data.get(1..)?;
                    }
                    let this_pixel = self.move_position();
                    return Some(RawPixel::new(this_pixel, u32::from(nibble_value)));
                }
                RleState::Running {
                    remaining,
                    value,
                    is_odd,
                } => {
                    // [03, 04] => 0 4 0
                    //             ^ ^ ^
                    //             | | +-- remaining is 0, is_odd is 1, we want left
                    //             | +-- remaining is 1, is_odd is 1, we want right
                    //             +-- remaining is 2, is_odd is 1, we want left
                    // [04, 04] => 0 4 0 4
                    //             ^ ^ ^ ^
                    //             | | | +-- remaining is 0, is_odd is 0, we want right
                    //             | | +-- remaining is 1, is_odd is 0, we want left
                    //             | +-- remaining is 2, is_odd is 0, we want right
                    //             +-- remaining is 3, is_odd is 0, we want left

                    let remaining_is_odd = (remaining % 2) != 0;
                    let want_left = remaining_is_odd != is_odd;

                    let nibble_value = if want_left { value >> 4 } else { value & 0x0F };

                    if remaining == 0 {
                        self.rle_state = RleState::Starting;
                    } else {
                        self.rle_state = RleState::Running {
                            remaining: remaining.saturating_sub(1),
                            value,
                            is_odd,
                        };
                    }

                    let this_pixel = self.move_position();
                    return Some(RawPixel::new(this_pixel, u32::from(nibble_value)));
                }
                RleState::Starting => {
                    let length = *self.data.get(0)?;
                    let param = *self.data.get(1)?;
                    self.data = &self.data.get(2..)?;
                    match length {
                        0 => {
                            // The first byte of the pair can be set to zero to
                            // indicate an escape character that denotes the end of
                            // a line, the end of a bitmap, or a delta, depending on
                            // the value of the second byte. The interpretation of
                            // the escape depends on the value of the second byte of
                            // the pair, which can be one of the following values.
                            match param {
                                0 => {
                                    // End of line
                                    if self.next_pixel.x != 0 {
                                        return None;
                                    }
                                }
                                1 => {
                                    // End of bitmap
                                    self.rle_state = RleState::EndOfBitmap;
                                }
                                2 => {
                                    // Delta encoding is unsupported.
                                    return None;
                                }
                                _ => {
                                    // Absolute mode
                                    self.rle_state = RleState::Absolute {
                                        remaining: param.saturating_sub(1),
                                        is_odd: (param % 2) != 0,
                                        // padding if the number of *bytes* is odd
                                        has_padding: ((param >> 1) % 2) != 0,
                                    };
                                }
                            }
                        }
                        _ => {
                            // An encoded run
                            self.rle_state = RleState::Running {
                                remaining: length.saturating_sub(1),
                                value: param,
                                is_odd: (length % 2) != 0,
                            };
                        }
                    }
                }
            }
        }
    }
}

/// Iterator over individual BMP pixels.
///
/// Each pixel is returned as a `u32` regardless of the bit depth of the source image.
#[allow(missing_debug_implementations)]
pub struct RawPixels<'a> {
    colors: DynamicRawColors<'a>,
    points: rectangle::Points,
}

impl<'a> RawPixels<'a> {
    pub(crate) fn new(raw_bmp: &'a RawBmp<'a>) -> Self {
        Self {
            colors: DynamicRawColors::new(raw_bmp),
            points: Rectangle::new(Point::zero(), raw_bmp.header().image_size).points(),
        }
    }
}

impl Iterator for RawPixels<'_> {
    type Item = RawPixel;

    fn next(&mut self) -> Option<Self::Item> {
        let color = match &mut self.colors {
            DynamicRawColors::Bpp1(colors) => colors.next().map(|r| u32::from(r.into_inner())),
            DynamicRawColors::Bpp4(colors) => colors.next().map(|r| u32::from(r.into_inner())),
            DynamicRawColors::Bpp8(colors) => colors.next().map(|r| u32::from(r.into_inner())),
            DynamicRawColors::Bpp16(colors) => colors.next().map(|r| u32::from(r.into_inner())),
            DynamicRawColors::Bpp24(colors) => colors.next().map(|r| r.into_inner()),
            DynamicRawColors::Bpp32(colors) => colors.next().map(|r| r.into_inner()),
            DynamicRawColors::Bpp4Rle(state) => {
                return state.next();
            }
            DynamicRawColors::Bpp8Rle(state) => {
                return state.next();
            }
        }?;

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

impl RawPixel {
    /// Creates a new raw pixel.
    pub const fn new(position: Point, color: u32) -> Self {
        Self { position, color }
    }
}
