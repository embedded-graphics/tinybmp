//! This example displays BMP images using the embedded-graphics simulator.
//!
//! Basic usage: `cargo run --example display -- BMP_FILE`
//!
//! More usage and arguments can be listed by running `cargo run --example display -- --help`

use clap::{ArgEnum, Parser};
use embedded_graphics::{
    image::Image,
    pixelcolor::{BinaryColor, Gray8, Rgb555, Rgb565, Rgb888},
    prelude::*,
};
use embedded_graphics_simulator::{
    OutputSettings, OutputSettingsBuilder, SimulatorDisplay, Window,
};
use std::{fs, num::NonZeroU32, path::PathBuf};
use tinybmp::Bmp;

#[derive(Debug, Clone, Copy, ArgEnum)]
#[clap(rename_all = "PascalCase")]
enum ColorType {
    Rgb555,
    Rgb565,
    Rgb888,
    Gray8,
    BinaryColor,
}

#[derive(Parser)]
struct Args {
    /// Pixel scale
    #[clap(long, default_value = "1")]
    scale: NonZeroU32,

    /// Display color type
    #[clap(arg_enum, long, default_value = "Rgb888")]
    color_type: ColorType,

    /// BMP file
    bmp_file: PathBuf,
}

fn display_bmp<C>(data: &[u8], settings: &OutputSettings)
where
    C: PixelColor + From<Rgb555> + From<Rgb565> + From<Rgb888> + Into<Rgb888>,
{
    let bmp = Bmp::<C>::from_slice(&data).unwrap();

    let mut display = SimulatorDisplay::<Rgb888>::new(bmp.size());

    Image::new(&bmp, Point::zero())
        .draw(&mut display.color_converted())
        .unwrap();

    let mut window = Window::new("BMP viewer", &settings);
    window.show_static(&display);
}

fn main() {
    let args = Args::parse();

    let settings = OutputSettingsBuilder::new()
        .scale(args.scale.into())
        .build();

    let data = fs::read(&args.bmp_file).unwrap();

    match args.color_type {
        ColorType::Rgb555 => display_bmp::<Rgb555>(&data, &settings),
        ColorType::Rgb565 => display_bmp::<Rgb565>(&data, &settings),
        ColorType::Rgb888 => display_bmp::<Rgb888>(&data, &settings),
        ColorType::Gray8 => display_bmp::<Gray8>(&data, &settings),
        ColorType::BinaryColor => display_bmp::<BinaryColor>(&data, &settings),
    }
}
