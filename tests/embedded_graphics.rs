use embedded_graphics::{
    image::Image,
    mock_display::{ColorMapping, MockDisplay},
    pixelcolor::{Gray8, Rgb555, Rgb565, Rgb888},
    prelude::*,
    primitives::Rectangle,
};
use tinybmp::{Bmp, RowOrder};

#[test]
fn negative_top_left() {
    let image: Bmp<Rgb565> =
        Bmp::from_slice(include_bytes!("./chessboard-4px-color-16bit.bmp")).unwrap();
    let image = Image::new(&image, Point::zero()).translate(Point::new(-1, -1));

    assert_eq!(
        image.bounding_box(),
        Rectangle::new(Point::new(-1, -1), Size::new(4, 4))
    );
}

#[test]
fn dimensions() {
    let image: Bmp<Rgb565> =
        Bmp::from_slice(include_bytes!("./chessboard-4px-color-16bit.bmp")).unwrap();
    let image = Image::new(&image, Point::zero()).translate(Point::new(100, 200));

    assert_eq!(
        image.bounding_box(),
        Rectangle::new(Point::new(100, 200), Size::new(4, 4))
    );
}

fn expected_image_color<C>() -> MockDisplay<C>
where
    C: PixelColor + ColorMapping,
{
    MockDisplay::from_pattern(&[
        "KRGY", //
        "BMCW", //
    ])
}

fn expected_image_gray() -> MockDisplay<Gray8> {
    MockDisplay::from_pattern(&["08F"])
}

fn draw_image<I: ImageDrawable>(image_drawable: I) -> MockDisplay<I::Color> {
    let image = Image::new(&image_drawable, Point::zero());

    let mut display = MockDisplay::new();
    image.draw(&mut display).unwrap();

    display
}

fn test_color_pattern<C>(data: &[u8])
where
    C: PixelColor + From<Rgb555> + From<Rgb565> + From<Rgb888> + ColorMapping,
{
    let bmp = Bmp::<C>::from_slice(data).unwrap();
    draw_image(bmp).assert_eq(&expected_image_color());

    let bmp = Bmp::<Rgb565>::from_slice(data).unwrap();
    draw_image(bmp).assert_eq(&expected_image_color::<Rgb565>());

    let bmp = Bmp::<Rgb888>::from_slice(data).unwrap();
    draw_image(bmp).assert_eq(&expected_image_color::<Rgb888>());
}

#[test]
fn colors_rgb555() {
    test_color_pattern::<Rgb555>(include_bytes!("./colors_rgb555.bmp"));
}

#[test]
fn colors_rgb565() {
    test_color_pattern::<Rgb565>(include_bytes!("./colors_rgb565.bmp"));
}

#[test]
fn colors_rgb888_24bit() {
    test_color_pattern::<Rgb888>(include_bytes!("./colors_rgb888_24bit.bmp"));
}

#[test]
fn colors_rgb888_32bit() {
    test_color_pattern::<Rgb888>(include_bytes!("./colors_rgb888_32bit.bmp"));
}

#[test]
fn colors_grey8() {
    let bmp: Bmp<Gray8> = Bmp::from_slice(include_bytes!("./colors_grey8.bmp")).unwrap();
    draw_image(bmp).assert_eq(&expected_image_gray());

    let bmp = Bmp::<Rgb565>::from_slice(include_bytes!("./colors_grey8.bmp")).unwrap();
    let display = draw_image(bmp);
    display.assert_eq(&expected_image_gray().map(|c| c.into()));

    let bmp = Bmp::<Rgb888>::from_slice(include_bytes!("./colors_grey8.bmp")).unwrap();
    let display = draw_image(bmp);
    display.assert_eq(&expected_image_gray().map(|c| c.into()));
}

/// Test for issue #136
#[test]
fn issue_136_row_size_is_multiple_of_4_bytes() {
    let image: Bmp<Rgb565> = Bmp::from_slice(include_bytes!("./issue_136.bmp")).unwrap();
    let image = Image::new(&image, Point::zero());

    let mut display = MockDisplay::new();
    image.draw(&mut display).unwrap();

    display.assert_pattern(&[
        "WWWWKWWWW",
        "WKKKKWKKK",
        "WWWWKWKWW",
        "WKKKKWKKW",
        "WWWWKWWWW",
    ]);
}

/// Test for issue #8
#[test]
fn issue_8_height_is_negative() {
    let image_bottom_up: Bmp<Rgb888> =
        Bmp::from_slice(include_bytes!("./issue_8-image_bottom_up.bmp")).unwrap();
    let image_top_down: Bmp<Rgb888> =
        Bmp::from_slice(include_bytes!("./issue_8-image_top_down.bmp")).unwrap();

    assert_eq!(
        image_bottom_up.as_raw().header().row_order,
        RowOrder::BottomUp
    );
    assert_eq!(
        image_top_down.as_raw().header().row_order,
        RowOrder::TopDown
    );

    let image_bottom_up = Image::new(&image_bottom_up, Point::zero());
    let image_top_down = Image::new(&image_top_down, Point::zero());

    let mut bottom_up_display = MockDisplay::new();
    let mut top_down_display = MockDisplay::new();

    image_bottom_up.draw(&mut bottom_up_display).unwrap();
    image_top_down.draw(&mut top_down_display).unwrap();

    bottom_up_display.assert_pattern(&["WK", "KK"]);
    top_down_display.assert_eq(&bottom_up_display);
}
