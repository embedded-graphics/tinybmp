use criterion::{criterion_group, criterion_main, Criterion};
use embedded_graphics::{
    image::Image,
    pixelcolor::{Rgb555, Rgb565, Rgb888},
    prelude::*,
};
use tinybmp::Bmp;

// TODO: use e-g framebuffer when it's added
struct Framebuffer<C> {
    pixels: [[C; 240]; 320],
}

impl<C: PixelColor + From<Rgb888>> Framebuffer<C> {
    pub fn new() -> Self {
        let color = C::from(Rgb888::BLACK);

        Self {
            pixels: [[color; 240]; 320],
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

fn parser_benchmarks(c: &mut Criterion) {
    c.bench_function("draw RGB555", |b| {
        let mut fb = Framebuffer::<Rgb555>::new();
        b.iter(|| {
            let bmp =
                Bmp::<Rgb555>::from_slice(include_bytes!("../tests/logo-rgb555.bmp")).unwrap();
            Image::new(&bmp, Point::zero()).draw(&mut fb).unwrap();
        })
    });

    c.bench_function("draw RGB565", |b| {
        let mut fb = Framebuffer::<Rgb565>::new();
        b.iter(|| {
            let bmp =
                Bmp::<Rgb565>::from_slice(include_bytes!("../tests/logo-rgb565.bmp")).unwrap();
            Image::new(&bmp, Point::zero()).draw(&mut fb).unwrap();
        })
    });

    c.bench_function("draw RGB888 24BPP", |b| {
        let mut fb = Framebuffer::<Rgb888>::new();
        b.iter(|| {
            let bmp = Bmp::<Rgb888>::from_slice(include_bytes!("../tests/logo-rgb888-24bpp.bmp"))
                .unwrap();
            Image::new(&bmp, Point::zero()).draw(&mut fb).unwrap();
        })
    });

    c.bench_function("draw RGB888 32BPP", |b| {
        let mut fb = Framebuffer::<Rgb888>::new();
        b.iter(|| {
            let bmp = Bmp::<Rgb888>::from_slice(include_bytes!("../tests/logo-rgb888-32bpp.bmp"))
                .unwrap();
            Image::new(&bmp, Point::zero()).draw(&mut fb).unwrap();
        })
    });

    c.bench_function("draw indexed 1BPP", |b| {
        let mut fb = Framebuffer::<Rgb888>::new();
        b.iter(|| {
            let bmp = Bmp::<Rgb888>::from_slice(include_bytes!("../tests/logo-indexed-1bpp.bmp"))
                .unwrap();
            Image::new(&bmp, Point::zero()).draw(&mut fb).unwrap();
        })
    });

    c.bench_function("draw indexed 4BPP", |b| {
        let mut fb = Framebuffer::<Rgb888>::new();
        b.iter(|| {
            let bmp = Bmp::<Rgb888>::from_slice(include_bytes!("../tests/logo-indexed-4bpp.bmp"))
                .unwrap();
            Image::new(&bmp, Point::zero()).draw(&mut fb).unwrap();
        })
    });

    c.bench_function("draw indexed 8BPP", |b| {
        let mut fb = Framebuffer::<Rgb888>::new();
        b.iter(|| {
            let bmp = Bmp::<Rgb888>::from_slice(include_bytes!("../tests/logo-indexed-8bpp.bmp"))
                .unwrap();
            Image::new(&bmp, Point::zero()).draw(&mut fb).unwrap();
        })
    });

    c.bench_function("draw dynamic RGB565 to RGB888", |b| {
        let mut fb = Framebuffer::<Rgb888>::new();
        b.iter(|| {
            let bmp = Bmp::from_slice(include_bytes!("../tests/logo-rgb565.bmp")).unwrap();
            Image::new(&bmp, Point::zero()).draw(&mut fb).unwrap();
        })
    });

    c.bench_function("draw dynamic RGB888 to RGB888", |b| {
        let mut fb = Framebuffer::<Rgb888>::new();
        b.iter(|| {
            let bmp = Bmp::from_slice(include_bytes!("../tests/logo-rgb888-24bpp.bmp")).unwrap();
            Image::new(&bmp, Point::zero()).draw(&mut fb).unwrap();
        })
    });

    c.bench_function("draw dynamic RGB565 to RGB565", |b| {
        let mut fb = Framebuffer::<Rgb565>::new();
        b.iter(|| {
            let bmp = Bmp::from_slice(include_bytes!("../tests/logo-rgb565.bmp")).unwrap();
            Image::new(&bmp, Point::zero()).draw(&mut fb).unwrap();
        })
    });

    c.bench_function("draw dynamic RGB888 to RGB565", |b| {
        let mut fb = Framebuffer::<Rgb565>::new();
        b.iter(|| {
            let bmp = Bmp::from_slice(include_bytes!("../tests/logo-rgb888-24bpp.bmp")).unwrap();
            Image::new(&bmp, Point::zero()).draw(&mut fb).unwrap();
        })
    });
}

criterion_group!(benches, parser_benchmarks);
criterion_main!(benches);
