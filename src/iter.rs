use core::marker::PhantomData;

use embedded_graphics::{
    pixelcolor::{
        raw::{RawU16, RawU24},
        Rgb555, Rgb565, Rgb888,
    },
    prelude::*,
};

use crate::{raw_bmp::ColorType, raw_iter::RawPixels, Bmp, ColorTable, RawPixel};

/// Iterator over the pixels in a BMP image.
///
/// See the [`pixels`](Bmp::pixels) method documentation for more information.
#[allow(missing_debug_implementations)]
pub struct Pixels<'a, C>
where
    C: PixelColor + From<Rgb555> + From<Rgb565> + From<Rgb888>,
{
    raw_pixels: RawPixels<'a>,
    color_table: Option<&'a ColorTable<'a>>,
    image_color_type: ColorType,
    target_color_type: PhantomData<C>,
}

impl<'a, C> Pixels<'a, C>
where
    C: PixelColor + From<Rgb555> + From<Rgb565> + From<Rgb888>,
{
    pub(crate) fn new(bmp: &'a Bmp<'a, C>) -> Self {
        let raw_pixels = RawPixels::new(&bmp.raw_bmp);

        Self {
            raw_pixels,
            color_table: bmp.raw_bmp.color_table(),
            image_color_type: bmp.raw_bmp.color_type,
            target_color_type: PhantomData,
        }
    }
}

impl<C> Iterator for Pixels<'_, C>
where
    C: PixelColor + From<Rgb555> + From<Rgb565> + From<Rgb888>,
{
    type Item = Pixel<C>;

    fn next(&mut self) -> Option<Self::Item> {
        let RawPixel { position, color } = self.raw_pixels.next()?;

        let color = match self.image_color_type {
            ColorType::Index1 | ColorType::Index4 | ColorType::Index8 => {
                self.color_table?.get(color).unwrap_or_default().into()
            }
            ColorType::Rgb555 => Rgb555::from(RawU16::from_u32(color)).into(),
            ColorType::Rgb565 => Rgb565::from(RawU16::from_u32(color)).into(),
            ColorType::Rgb888 | ColorType::Xrgb8888 => Rgb888::from(RawU24::from_u32(color)).into(),
        };

        Some(Pixel(position, color))
    }
}
