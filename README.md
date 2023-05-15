# TinyBMP

[![Build Status](https://circleci.com/gh/embedded-graphics/tinybmp/tree/master.svg?style=shield)](https://circleci.com/gh/embedded-graphics/tinybmp/tree/master)
[![Crates.io](https://img.shields.io/crates/v/tinybmp.svg)](https://crates.io/crates/tinybmp)
[![Docs.rs](https://docs.rs/tinybmp/badge.svg)](https://docs.rs/tinybmp)
[![embedded-graphics on Matrix](https://img.shields.io/matrix/rust-embedded-graphics:matrix.org)](https://matrix.to/#/#rust-embedded-graphics:matrix.org)

## [Documentation](https://docs.rs/tinybmp)

A small BMP parser primarily for embedded, no-std environments but usable anywhere.

This crate is primarily targeted at drawing BMP images to [`embedded_graphics`] [`DrawTarget`]s,
but can also be used to parse BMP files for other applications.

## Examples

### Draw a BMP image to an embedded-graphics draw target

The [`Bmp`] struct is used together with [`embedded_graphics`]' [`Image`] struct to display BMP
files on any draw target.

```rust
use embedded_graphics::{image::Image, prelude::*};
use tinybmp::Bmp;

// Include the BMP file data.
let bmp_data = include_bytes!("../tests/chessboard-8px-color-16bit.bmp");

// Parse the BMP file.
let bmp = Bmp::from_slice(bmp_data).unwrap();

// Draw the image with the top left corner at (10, 20) by wrapping it in
// an embedded-graphics `Image`.
Image::new(&bmp, Point::new(10, 20)).draw(&mut display)?;
```

### Using the pixel iterator

To access the image data for other applications the [`Bmp::pixels`] method returns an iterator
over all pixels in the BMP file. The colors inside the BMP file will automatically converted to
one of the [color types] in [`embedded_graphics`].

```rust
use embedded_graphics::{pixelcolor::Rgb888, prelude::*};
use tinybmp::Bmp;

// Include the BMP file data.
let bmp_data = include_bytes!("../tests/chessboard-8px-24bit.bmp");

// Parse the BMP file.
// Note that it is necessary to explicitly specify the color type which the colors in the BMP
// file will be converted into.
let bmp = Bmp::<Rgb888>::from_slice(bmp_data).unwrap();

for Pixel(position, color) in bmp.pixels() {
    println!("R: {}, G: {}, B: {} @ ({})", color.r(), color.g(), color.b(), position);
}
```

### Accessing individual pixels

[`Bmp::pixel`] can be used to get the color of individual pixels. The returned color will be automatically
converted to one of the [color types] in [`embedded_graphics`].

```rust
use embedded_graphics::{pixelcolor::Rgb888, image::GetPixel, prelude::*};
use tinybmp::Bmp;

// Include the BMP file data.
let bmp_data = include_bytes!("../tests/chessboard-8px-24bit.bmp");

// Parse the BMP file.
// Note that it is necessary to explicitly specify the color type which the colors in the BMP
// file will be converted into.
let bmp = Bmp::<Rgb888>::from_slice(bmp_data).unwrap();

let pixel = bmp.pixel(Point::new(3, 2));

assert_eq!(pixel, Some(Rgb888::WHITE));
```

### Accessing the raw image data

For most applications the higher level access provided by [`Bmp`] is sufficient. But in case
lower level access is necessary the [`RawBmp`] struct can be used to access BMP [header
information] and the [color table]. A [`RawBmp`] object can be created directly from image data
by using [`from_slice`] or by accessing the underlying raw object of a [`Bmp`] object with
[`Bmp::as_raw`].

Similar to [`Bmp::pixel`], [`RawBmp::pixel`] can be used to get raw pixel color values as a
`u32`.

```rust
use embedded_graphics::prelude::*;
use tinybmp::{RawBmp, Bpp, Header, RawPixel, RowOrder};

let bmp = RawBmp::from_slice(include_bytes!("../tests/chessboard-8px-24bit.bmp"))
    .expect("Failed to parse BMP image");

// Read the BMP header
assert_eq!(
    bmp.header(),
    &Header {
        file_size: 314,
        image_data_start: 122,
        bpp: Bpp::Bits24,
        image_size: Size::new(8, 8),
        image_data_len: 192,
        channel_masks: None,
        row_order: RowOrder::BottomUp,
    }
);

// Get an iterator over the pixel coordinates and values in this image and load into a vec
let pixels: Vec<RawPixel> = bmp.pixels().collect();

// Loaded example image is 8x8px
assert_eq!(pixels.len(), 8 * 8);

// Individual raw pixel values can also be read
let pixel = bmp.pixel(Point::new(3, 2));

// The raw value for a white pixel in the source image
assert_eq!(pixel, Some(0xFFFFFFu32));
```

## Minimum supported Rust version

The minimum supported Rust version for tinybmp is `1.61` or greater. Ensure you have the correct
version of Rust installed, preferably through <https://rustup.rs>.

[`Bmp`]: https://docs.rs/tinybmp/latest/tinybmp/struct.Bmp.html
[`Bmp::pixels`]: https://docs.rs/tinybmp/latest/tinybmp/struct.Bmp.html#method.pixels
[`Bmp::pixel`]: https://docs.rs/tinybmp/latest/tinybmp/struct.Bmp.html#method.pixel
[`Bmp::as_raw`]: https://docs.rs/tinybmp/latest/tinybmp/struct.Bmp.html#method.as_raw
[`RawBmp`]: https://docs.rs/tinybmp/latest/tinybmp/struct.RawBmp.html
[`RawBmp::pixel`]: https://docs.rs/tinybmp/latest/tinybmp/struct.RawBmp.html#method.pixel
[header information]: https://docs.rs/tinybmp/latest/tinybmp/struct.RawBmp.html#method.header
[color table]: https://docs.rs/tinybmp/latest/tinybmp/struct.RawBmp.html#method.color_table
[`from_slice`]: https://docs.rs/tinybmp/latest/tinybmp/struct.RawBmp.html#method.from_slice

[`embedded_graphics`]: https://docs.rs/embedded_graphics
[color types]: https://docs.rs/embedded-graphics/latest/embedded_graphics/pixelcolor/index.html#structs
[`DrawTarget`]: https://docs.rs/embedded-graphics/latest/embedded_graphics/draw_target/trait.DrawTarget.html
[`Image`]: https://docs.rs/embedded-graphics/latest/embedded_graphics/image/struct.Image.html

[color types]: embedded_graphics::pixelcolor#structs

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the
work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
