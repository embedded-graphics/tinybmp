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
    pub fn len(&self) -> usize {
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
    pub fn get<C: PixelColor + From<C::Raw>>(&self, index: u32) -> Option<C> {
        self.get_raw::<C::Raw>(index).map(Into::into)
    }
}
