use embedded_graphics::pixelcolor::Rgb888;
use tinybmp::{Bmp, ParseError};

#[test]
fn zero_width() {
    assert_eq!(
        Bmp::<Rgb888>::from_slice(include_bytes!("error-width-0.bmp")),
        Err(ParseError::InvalidImageDimensions)
    );
}

#[test]
fn negative_width() {
    assert_eq!(
        Bmp::<Rgb888>::from_slice(include_bytes!("error-width-negative.bmp")),
        Err(ParseError::InvalidImageDimensions)
    );
}

#[test]
fn zero_height() {
    assert_eq!(
        Bmp::<Rgb888>::from_slice(include_bytes!("error-height-0.bmp")),
        Err(ParseError::InvalidImageDimensions)
    );
}
