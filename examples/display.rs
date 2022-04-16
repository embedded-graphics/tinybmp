//! This example displays BMP images using the embedded-graphics simulator.
//!
//! Basic usage: `cargo run --example display -- COLOR_TYPE BMP_IMAGE`
//!
//! More usage and arguments can be listed by running `cargo run --example display -- --help`

use clap::{ArgEnum, Parser};
use embedded_graphics::{
    image::Image,
    pixelcolor::{BinaryColor, Gray8, Rgb555, Rgb565, Rgb888},
    prelude::*,
};
use embedded_graphics_simulator::{OutputSettingsBuilder, SimulatorDisplay, Window};
use std::{fs, num::NonZeroU32, path::PathBuf};
use tinybmp::{Bmp, DynamicBmp};

#[derive(Debug, Clone, Copy, ArgEnum)]
enum ColorType {
    Rgb555,
    Rgb565,
    Rgb888,
    Gray8,
    BinaryColor,
}

#[derive(Parser)]
struct Args {
    #[clap(long, default_value = "1")]
    scale: NonZeroU32,
    #[clap(long)]
    dynamic: bool,
    #[clap(arg_enum)]
    color_type: ColorType,
    bmp_file: PathBuf,
}

fn draw_bmp<C>(data: &[u8]) -> SimulatorDisplay<Rgb888>
where
    C: PixelColor + From<C::Raw> + Into<Rgb888>,
{
    let bmp = Bmp::<C>::from_slice(&data).unwrap();

    let mut display = SimulatorDisplay::<Rgb888>::new(bmp.size());

    Image::new(&bmp, Point::zero())
        .draw(&mut display.color_converted())
        .unwrap();

    display
}

fn draw_dynamic_bmp<C>(data: &[u8]) -> SimulatorDisplay<Rgb888>
where
    C: PixelColor
        + From<C::Raw>
        + Into<Rgb888>
        + From<Rgb565>
        + From<Rgb555>
        + From<Rgb888>
        + From<Gray8>,
{
    let bmp = DynamicBmp::<C>::from_slice(&data).unwrap();

    let mut display = SimulatorDisplay::<Rgb888>::new(bmp.size());

    Image::new(&bmp, Point::zero())
        .draw(&mut display.color_converted())
        .unwrap();

    display
}

fn main() {
    let args = Args::parse();

    let data = fs::read(&args.bmp_file).unwrap();

    let display = if args.dynamic {
        match args.color_type {
            ColorType::Rgb555 => draw_dynamic_bmp::<Rgb555>(&data),
            ColorType::Rgb565 => draw_dynamic_bmp::<Rgb565>(&data),
            ColorType::Rgb888 => draw_dynamic_bmp::<Rgb888>(&data),
            ColorType::Gray8 => draw_dynamic_bmp::<Gray8>(&data),
            ColorType::BinaryColor => draw_dynamic_bmp::<BinaryColor>(&data),
        }
    } else {
        match args.color_type {
            ColorType::Rgb555 => draw_bmp::<Rgb555>(&data),
            ColorType::Rgb565 => draw_bmp::<Rgb565>(&data),
            ColorType::Rgb888 => draw_bmp::<Rgb888>(&data),
            ColorType::Gray8 => draw_bmp::<Gray8>(&data),
            ColorType::BinaryColor => draw_bmp::<BinaryColor>(&data),
        }
    };

    let settings = OutputSettingsBuilder::new()
        .scale(args.scale.into())
        .build();

    let mut window = Window::new("BMP viewer", &settings);
    window.show_static(&display);
}
