use core::convert::TryInto;

use embedded_graphics::{
    pixelcolor::{raw::RawU24, Rgb888},
    prelude::*,
};

/// Color table.
///
/// This struct provides access to the color table in a BMP file. Use
/// [`RawBmp::color_table`](crate::RawBmp::color_table) to get a reference to the color table of an
/// image.
///
/// Accessing the color table directly isn't necessary if images are drawn to an
/// [`embedded_graphics`] [`DrawTarget`](embedded_graphics::draw_target::DrawTarget). The conversion
/// of color indices to actual colors will be handled by [`Bmp`].
///
/// [`Bmp`]: crate::Bmp
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ColorTable<'a> {
    data: &'a [u8],
}

impl<'a> ColorTable<'a> {
    pub(crate) const fn new(data: &'a [u8]) -> Self {
        Self { data }
    }

    /// Returns the number of entries.
    pub const fn len(&self) -> usize {
        self.data.len() / 4
    }

    /// Returns a color table entry.
    ///
    /// `None` is returned if `index` is out of bounds.
    pub fn get(&self, index: u32) -> Option<Rgb888> {
        // MSRV: Experiment with slice::as_chunks when it's stabilized

        let offset = index as usize * 4;
        let bytes = self.data.get(offset..offset + 4)?;

        let raw = u32::from_le_bytes(bytes.try_into().unwrap());

        Some(RawU24::from_u32(raw).into())
    }
}
