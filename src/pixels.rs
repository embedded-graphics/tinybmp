use crate::{raw_pixels::RawPixels, Bpp, RawPixel};
use core::{convert::TryInto, marker::PhantomData};
use embedded_graphics::prelude::*;

/// Iterator over the pixels in a BMP image.
///
/// See the [`pixels`] method documentation for more information.
///
/// [`pixels`]: struct.Bmp.html#method.pixels
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Pixels<'a, 'b, C> {
    raw: RawPixels<'a, 'b>,
    color_type: PhantomData<C>,
}

impl<'a, 'b, C> Pixels<'a, 'b, C> {
    pub(crate) fn new(raw: RawPixels<'a, 'b>) -> Self {
        Self {
            raw,
            color_type: PhantomData,
        }
    }
}

impl<C> Iterator for Pixels<'_, '_, C>
where
    C: PixelColor + From<<C as PixelColor>::Raw>,
{
    type Item = Pixel<C>;

    fn next(&mut self) -> Option<Self::Item> {
        self.raw.next().map(|RawPixel { position, color }| {
            let color = match self.raw.raw_bmp.color_bpp() {
                // 1 and 8 BPP images may use a color table if one is provided
                Bpp::Bits1 | Bpp::Bits8 => {
                    let table = self.raw.raw_bmp.color_table();

                    // Each color table entry is 4 bytes long
                    let offset = color as usize * 4;

                    // MSRV: Experiment with slice::as_chunks when it's stabilised
                    u32::from_le_bytes(table[offset..offset + 4].try_into().unwrap())
                }
                // Color table should be ignored for any other bit depth
                _ => color,
            };

            Pixel(position, C::Raw::from_u32(color).into())
        })
    }
}
