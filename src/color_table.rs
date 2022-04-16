use core::convert::TryInto;
use embedded_graphics::prelude::{PixelColor, RawData};

/// Color table.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ColorTable<'a> {
    data: &'a [u8],
}

impl<'a> ColorTable<'a> {
    pub(crate) fn new(data: &'a [u8]) -> Self {
        Self { data }
    }

    /// Returns the number of entries.
    // Only used in tests, hence the allow
    #[allow(unused)]
    fn len(&self) -> usize {
        return self.data.len() / 4;
    }

    /// Returns the raw value of a color table entry.
    ///
    /// `None` is returned if `index` is out of bounds.
    pub fn get_raw<R: RawData>(&self, index: u32) -> Option<R> {
        // MSRV: Experiment with slice::as_chunks when it's stabilized

        let offset = index as usize * 4;
        let bytes = self.data.get(offset..offset + 4)?;

        let raw = u32::from_le_bytes(bytes.try_into().unwrap());

        Some(R::from_u32(raw))
    }

    /// Returns a color table entry.
    ///
    /// `None` is returned if `index` is out of bounds.
    pub fn get<C: PixelColor + From<<C as PixelColor>::Raw>>(&self, index: u32) -> Option<C> {
        self.get_raw::<C::Raw>(index).map(Into::into)
    }
}

// TODO: When the color table is made public, the tests below should be moved into the corresponding
// files under `tests/`.
#[cfg(test)]
mod tests {
    use crate::{Bmp, RawBmp};
    use embedded_graphics::pixelcolor::{raw::RawU32, Rgb888};

    #[test]
    fn chessboard_8px_1bit() {
        let bmp = RawBmp::from_slice(include_bytes!("../tests/chessboard-8px-1bit.bmp"))
            .expect("Failed to parse");

        let color_table = bmp.color_table().unwrap();
        assert_eq!(color_table.len(), 2);
        assert_eq!(color_table.get_raw(0), Some(RawU32::new(0x00000000)));
        assert_eq!(color_table.get_raw(1), Some(RawU32::new(0xFFFFFFFF)));
        assert_eq!(color_table.get_raw(2), Option::<RawU32>::None);

        assert_eq!(bmp.image_data().len(), 94 - 62);
    }

    #[test]
    fn chessboard_8px_16bit() {
        let bmp = RawBmp::from_slice(include_bytes!("../tests/chessboard-8px-color-16bit.bmp"))
            .expect("Failed to parse");

        assert!(
            bmp.color_table().is_none(),
            "there should be no color table for this image"
        );
    }

    #[test]
    fn chessboard_8px_24bit() {
        let bmp = RawBmp::from_slice(include_bytes!("../tests/chessboard-8px-24bit.bmp"))
            .expect("Failed to parse");

        assert!(
            bmp.color_table().is_none(),
            "there should be no color table for this image"
        );
    }

    #[test]
    fn colors_8bpp_indexed() {
        let bmp = Bmp::<'_, Rgb888>::from_slice(include_bytes!("../tests/colors_8bpp_indexed.bmp"))
            .expect("Failed to parse");

        assert!(
            bmp.as_raw().color_table().is_some(),
            "there should be a color table for this image"
        );
    }
}
