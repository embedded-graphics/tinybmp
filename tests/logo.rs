use embedded_graphics::{
    image::{GetPixel, Image, ImageRaw},
    iterator::raw::RawDataSlice,
    pixelcolor::{raw::LittleEndian, Bgr888, Rgb555, Rgb565, Rgb888},
    prelude::*,
};
use tinybmp::Bmp;

const WIDTH: usize = 240;
const HEIGHT: usize = 320;

// TODO: use e-g framebuffer when it's added
#[derive(Debug, PartialEq)]
struct Framebuffer<C> {
    pixels: [[C; 240]; 320],
}

impl<C: PixelColor + From<Rgb888> + std::fmt::Debug> Framebuffer<C> {
    pub fn new() -> Self {
        let color = C::from(Rgb888::BLACK);

        Self {
            pixels: [[color; WIDTH]; HEIGHT],
        }
    }

    pub fn from_image(image: impl ImageDrawable<Color = C>) -> Self {
        let mut framebuffer = Framebuffer::<C>::new();
        Image::new(&image, Point::zero())
            .draw(&mut framebuffer)
            .unwrap();
        framebuffer
    }

    pub fn pixels(&self) -> impl Iterator<Item = C> + '_ {
        self.pixels.iter().flatten().copied()
    }

    pub fn assert_eq(&self, expected: &Self) {
        let zipped = || self.pixels().zip(expected.pixels());

        let errors = zipped().filter(|(a, b)| a != b).count();
        let first_error = zipped()
            .enumerate()
            .find(|(_, (a, b))| a != b)
            .map(|(i, (a, b))| (Point::new((i % WIDTH) as i32, (i / WIDTH) as i32), a, b));

        //let first_error = self.pixels()

        if self != expected {
            let first_error = first_error.unwrap();
            panic!(
                "framebuffer differs from expected\n{} errors\nfirst error at ({}): {:?} (expected {:?})",
                errors,
                first_error.0,
                first_error.1,
                first_error.2,
            );
        }
    }
}

impl<C: PixelColor> DrawTarget for Framebuffer<C> {
    type Color = C;
    type Error = std::convert::Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = embedded_graphics::Pixel<Self::Color>>,
    {
        for Pixel(p, c) in pixels {
            self.pixels[p.y as usize][p.x as usize] = c;
        }

        Ok(())
    }
}

impl<C> OriginDimensions for Framebuffer<C> {
    fn size(&self) -> embedded_graphics::prelude::Size {
        Size::new(240, 320)
    }
}

fn draw_raw<C>(data: &[u8]) -> Framebuffer<C>
where
    C: PixelColor + From<C::Raw> + From<Rgb888> + std::fmt::Debug,
    for<'a> RawDataSlice<'a, C::Raw, LittleEndian>: IntoIterator<Item = C::Raw>,
{
    let raw = ImageRaw::<C, LittleEndian>::new(data, 240);

    Framebuffer::from_image(raw)
}

fn draw_bmp<C>(data: &[u8]) -> Framebuffer<C>
where
    C: PixelColor + From<Rgb555> + From<Rgb565> + From<Rgb888> + std::fmt::Debug,
{
    let bmp = Bmp::<C>::from_slice(data).unwrap();

    Framebuffer::from_image(bmp)
}

fn draw_bmp_pixel_getter<C>(data: &[u8]) -> Framebuffer<C>
where
    C: PixelColor + From<Rgb555> + From<Rgb565> + From<Rgb888> + std::fmt::Debug,
{
    let bmp = Bmp::<C>::from_slice(data).unwrap();

    let mut fb = Framebuffer::new();

    let bb = bmp.bounding_box();
    assert_eq!(bb.top_left, Point::zero());
    assert_eq!(bb.size, Size::new(240, 320));
    assert_eq!(bb.points().count(), 320 * 240);

    bmp.bounding_box()
        .points()
        .map(|p| Pixel(p, bmp.pixel(p).unwrap()))
        .draw(&mut fb)
        .unwrap();

    fb
}

#[test]
fn logo_indexed_1bpp() {
    let raw = draw_raw::<Bgr888>(include_bytes!("logo-indexed-1bpp.raw"));
    let bmp = draw_bmp::<Bgr888>(include_bytes!("logo-indexed-1bpp.bmp"));

    bmp.assert_eq(&raw);
}

#[test]
fn logo_indexed_1bpp_pixel_getter() {
    let raw = draw_raw::<Bgr888>(include_bytes!("logo-indexed-1bpp.raw"));
    let bmp = draw_bmp_pixel_getter::<Bgr888>(include_bytes!("logo-indexed-1bpp.bmp"));

    bmp.assert_eq(&raw);
}

#[test]
fn logo_indexed_4bpp() {
    let raw = draw_raw::<Bgr888>(include_bytes!("logo-indexed-4bpp.raw"));
    let bmp = draw_bmp::<Bgr888>(include_bytes!("logo-indexed-4bpp.bmp"));

    bmp.assert_eq(&raw);
}

#[test]
fn logo_indexed_4bpp_pixel_getter() {
    let raw = draw_raw::<Bgr888>(include_bytes!("logo-indexed-4bpp.raw"));
    let bmp = draw_bmp_pixel_getter::<Bgr888>(include_bytes!("logo-indexed-4bpp.bmp"));

    bmp.assert_eq(&raw);
}

#[test]
fn logo_indexed_8bpp() {
    let raw = draw_raw::<Bgr888>(include_bytes!("logo-indexed-8bpp.raw"));
    let bmp = draw_bmp::<Bgr888>(include_bytes!("logo-indexed-8bpp.bmp"));

    bmp.assert_eq(&raw);
}

#[test]
fn logo_indexed_8bpp_pixel_getter() {
    let raw = draw_raw::<Bgr888>(include_bytes!("logo-indexed-8bpp.raw"));
    let bmp = draw_bmp_pixel_getter::<Bgr888>(include_bytes!("logo-indexed-8bpp.bmp"));

    bmp.assert_eq(&raw);
}

#[test]
fn logo_rgb555() {
    let raw = draw_raw::<Rgb555>(include_bytes!("logo-rgb555.raw"));
    let bmp = draw_bmp::<Rgb555>(include_bytes!("logo-rgb555.bmp"));

    bmp.assert_eq(&raw);
}

#[test]
fn logo_rgb555_pixel_getter() {
    let raw = draw_raw::<Rgb555>(include_bytes!("logo-rgb555.raw"));
    let bmp = draw_bmp_pixel_getter::<Rgb555>(include_bytes!("logo-rgb555.bmp"));

    bmp.assert_eq(&raw);
}

#[test]
fn logo_rgb565() {
    let raw = draw_raw::<Rgb565>(include_bytes!("logo-rgb565.raw"));
    let bmp = draw_bmp::<Rgb565>(include_bytes!("logo-rgb565.bmp"));

    bmp.assert_eq(&raw);
}

#[test]
fn logo_rgb565_pixel_getter() {
    let raw = draw_raw::<Rgb565>(include_bytes!("logo-rgb565.raw"));
    let bmp = draw_bmp_pixel_getter::<Rgb565>(include_bytes!("logo-rgb565.bmp"));

    bmp.assert_eq(&raw);
}

#[test]
fn logo_rgb888_24bpp() {
    let raw = draw_raw::<Bgr888>(include_bytes!("logo-rgb888.raw"));
    let bmp = draw_bmp::<Bgr888>(include_bytes!("logo-rgb888-24bpp.bmp"));

    bmp.assert_eq(&raw);
}

#[test]
fn logo_rgb888_24bpp_pixel_getter() {
    let raw = draw_raw::<Bgr888>(include_bytes!("logo-rgb888.raw"));
    let bmp = draw_bmp_pixel_getter::<Bgr888>(include_bytes!("logo-rgb888-24bpp.bmp"));

    bmp.assert_eq(&raw);
}

#[test]
fn logo_rgb888_32bpp() {
    let raw = draw_raw::<Bgr888>(include_bytes!("logo-rgb888.raw"));
    let bmp = draw_bmp::<Bgr888>(include_bytes!("logo-rgb888-32bpp.bmp"));

    bmp.assert_eq(&raw);
}

#[test]
fn logo_rgb888_32bpp_pixel_getter() {
    let raw = draw_raw::<Bgr888>(include_bytes!("logo-rgb888.raw"));
    let bmp = draw_bmp_pixel_getter::<Bgr888>(include_bytes!("logo-rgb888-32bpp.bmp"));

    bmp.assert_eq(&raw);
}
