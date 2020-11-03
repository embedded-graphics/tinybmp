# TinyBMP

[![Build Status](https://circleci.com/gh/embedded-graphics/embedded-graphics/tree/master.svg?style=shield)](https://circleci.com/gh/embedded-graphics/embedded-graphics/tree/master)
[![Crates.io](https://img.shields.io/crates/v/tinybmp.svg)](https://crates.io/crates/tinybmp)
[![Docs.rs](https://docs.rs/tinybmp/badge.svg)](https://docs.rs/tinybmp)
[![embedded-graphics on Matrix](https://img.shields.io/matrix/rust-embedded-graphics:matrix.org)](https://matrix.to/#/#rust-embedded-graphics:matrix.org)

## [Documentation](https://docs.rs/tinybmp)

A small BMP parser designed for embedded, no-std environments but usable anywhere. Beyond
parsing the image header, no other allocations are made.

To use `tinybmp` without `embedded-graphics` the raw data for individual pixels in an image
can be accessed using the `raw_pixels` and `raw_image_data` methods provided by the `Bmp`
struct.

## Examples

### Draw a BMP image to an `embedded-graphics` draw target

This example loads a 16BPP image and draws it to an `embedded-graphics` compatible display.

```rust
use embedded_graphics::{image::Image, prelude::*};
use tinybmp::Bmp;

// Load 16BPP 8x8px image
let bmp: Bmp<Rgb565> = Bmp::from_slice(include_bytes!("../tests/chessboard-8px-color-16bit.bmp")).unwrap();

let image = Image::new(&bmp, Point::zero());

image.draw(&mut display)?;
```

### Accessing the raw image data

This example demonstrates how the image header and raw image data can be accessed to use
`tinybmp` without `embedded-graphics`.

```rust
use tinybmp::{Bmp, Bpp, Header, RawPixel};

let bmp = Bmp::from_slice_raw(include_bytes!("../tests/chessboard-8px-24bit.bmp"))
    .expect("Failed to parse BMP image");

// Read the BMP header
assert_eq!(
    bmp.header,
    Header {
        file_size: 314,
        image_data_start: 122,
        bpp: Bpp::Bits24,
        image_width: 8,
        image_height: 8,
        image_data_len: 192
    }
);

// Check that raw image data slice is the correct length (according to parsed header)
assert_eq!(bmp.raw_image_data().len(), bmp.header.image_data_len as usize);

// Get an iterator over the pixel coordinates and values in this image and load into a vec
let pixels: Vec<RawPixel> = bmp.raw_pixels().collect();

// Loaded example image is 8x8px
assert_eq!(pixels.len(), 8 * 8);
```

[`embedded-graphics`]: https://crates.io/crates/embedded-graphics

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the
work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
