# Changelog

[`tinybmp`](https://crates.io/crates/tinybmp) is a no_std, low memory footprint BMP loading library for embedded applications.

<!-- next-header -->

## [Unreleased] - ReleaseDate

### Fixed 

- [#19](https://github.com/embedded-graphics/tinybmp/pull/19) Add support for color mapped 1bpp images

## [0.3.1] - 2021-06-16

### Changed

- [#13](https://github.com/embedded-graphics/tinybmp/pull/13) Bump embedded-graphics minimum version from 0.7.0 to 0.7.1

## [0.3.0] - 2021-06-06

## [0.3.0-beta.2] - 2021-05-24

### Changed

- **(breaking)** [#9](https://github.com/embedded-graphics/tinybmp/pull/9) Added support for BMP images saved in top-down row order. A computed field `row_order` is added to the `Header` struct to capture whether the image data is ordered top-down, or the standard bottom-up.

## [0.3.0-beta.1] - 2021-05-24

## [0.3.0-alpha.1] - 2020-12-27

### Changed

- **(breaking)** [#3](https://github.com/embedded-graphics/tinybmp/pull/3) `tinybmp` now depends on `embedded-graphics-core` instead of `embedded-graphics`.

## [0.3.0-alpha.1 - `embedded-graphics` repository] - 2020-12-27

> Note: PR numbers from this point onwards are from the old `embedded-graphics/embedded-graphics` repository. New PR numbers above this note refer to PRs in the `embedded-graphics/tinybmp` repository.

### Added

- [#453](https://github.com/embedded-graphics/embedded-graphics/pull/453) `DynamicBmp` was added to load images with an unknown color format at compile time.

### Changed

- **(breaking)** [#420](https://github.com/embedded-graphics/embedded-graphics/pull/420) To support the new embedded-graphics 0.7 image API a color type parameter was added to `Bmp`.
- **(breaking)** [#444](https://github.com/embedded-graphics/embedded-graphics/pull/444) The `graphics` feature was removed and the `embedded-graphics` dependency is now non optional.
- **(breaking)** [#444](https://github.com/embedded-graphics/embedded-graphics/pull/444) `Bmp` no longer implements `IntoIterator`. Pixel iterators can now be created using the `pixels` methods.
- **(breaking)** [#444](https://github.com/embedded-graphics/embedded-graphics/pull/444) `Bmp::width` and `Bmp::height` were replaced by `Bmp::size` which requires `embedded_graphics::geometry::OriginDimensions` to be in scope (also included in the embedded-graphics `prelude`).
- **(breaking)** [#444](https://github.com/embedded-graphics/embedded-graphics/pull/444) `Bmp::from_slice` now checks if the image BPP matches the specified color type. To report possible errors it now returns the dedicated error type `ParseError` instead of `()`.
- **(breaking)** [#444](https://github.com/embedded-graphics/embedded-graphics/pull/444) `Bmp::bpp` was renamed to `Bmp::color_bpp` to be consistent with `tinytga` and the return type was changed to an enum.
- **(breaking)** [#453](https://github.com/embedded-graphics/embedded-graphics/pull/453) The methods to access the raw image data and header were moved to a new `RawBmp` type, which can be used on its own or can be accessed by using `Bmp::as_raw` or `DynamicBmp::as_raw`.

## [0.2.3] - 2020-05-26

### Added

- #352 Added support for decoding 1 bit pixel depth BMP images.

## [0.2.2] - 2020-03-20

## [0.2.1] - 2020-02-17

- [#244](https://github.com/embedded-graphics/embedded-graphics/pull/244) Added `.into_iter()` support to the `Bmp` struct to get an iterator over every pixel in the image.

### Changed

- **(breaking)** [#247](https://github.com/embedded-graphics/embedded-graphics/pull/247) "reverse" integration of tinybmp into [`embedded-graphics`](https://crates.io/crates/embedded-graphics). tinybmp now has a `graphics` feature that must be turned on to enable embedded-graphics support. The `bmp` feature from embedded-graphics is removed.

  **Before**

  `Cargo.toml`

  ```toml
  [dependencies]
  embedded-graphics = { version = "0.6.0-alpha.3", features = [ "bmp" ]}
  ```

  Your code

  ```rust
  use embedded_graphics::prelude::*;
  use embedded_graphics::image::ImageBmp;

  let image = ImageBmp::new(include_bytes!("../../../assets/patch.bmp")).unwrap();
  display.draw(&image);
  ```

  **After**

  `Cargo.toml`

  ```toml
  [dependencies]
  embedded-graphics = "0.6.0"
  tinybmp = { version = "*", features = [ "graphics" ]}
  ```

  Your code

  ```rust
  use embedded_graphics::{prelude::*, image::Image};
  use tinybmp::Bmp;

  let image = Bmp::new(include_bytes!("../../../assets/patch.bmp")).unwrap();
  let image = Image::new(&image);
  display.draw(&image);
  ```

## 0.1.1

### Fixed

- #218 Test README examples in CI and update them to work with latest crate versions.

### Changed

- #228 Upgraded to nom 5 internally. No user-facing changes.

## 0.1.0

### Added

- Release `tinybmp` crate to crates.io

<!-- next-url -->
[unreleased]: https://github.com/embedded-graphics/tinybmp/compare/v0.3.1...HEAD

[0.3.1]: https://github.com/embedded-graphics/tinybmp/compare/v0.3.0...v0.3.1
[0.3.0]: https://github.com/embedded-graphics/tinybmp/compare/v0.3.0-beta.2...v0.3.0
[0.3.0-beta.2]: https://github.com/embedded-graphics/tinybmp/compare/v0.3.0-beta.1...v0.3.0-beta.2
[0.3.0-beta.1]: https://github.com/embedded-graphics/tinybmp/compare/v0.3.0-alpha.1...v0.3.0-beta.1
[0.3.0-alpha.1]: https://github.com/embedded-graphics/tinybmp/compare/after-split...v0.3.0-alpha.1
[0.3.0-alpha.1 - `embedded-graphics` repository]: https://github.com/embedded-graphics/embedded-graphics/compare/tinybmp-v0.2.3...before-split
[0.2.3]: https://github.com/embedded-graphics/embedded-graphics/compare/tinybmp-v0.2.2...tinybmp-v0.2.3
[0.2.2]: https://github.com/embedded-graphics/embedded-graphics/compare/tinybmp-v0.2.0...tinybmp-v0.2.2
[0.2.1]: https://github.com/embedded-graphics/embedded-graphics/compare/tinybmp-v0.1.1...tinybmp-v0.2.1
