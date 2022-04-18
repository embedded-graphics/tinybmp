use criterion::{criterion_group, criterion_main, Criterion};
use embedded_graphics::pixelcolor::Rgb888;
use tinybmp::Bmp;

const BMP_FILE: &[u8] = include_bytes!("../tests/colors_rgb888_24bit.bmp");

fn parser_benchmarks(c: &mut Criterion) {
    c.bench_function("parse BMP", |b| {
        b.iter(|| Bmp::<Rgb888>::from_slice(BMP_FILE).unwrap())
    });
}

criterion_group!(benches, parser_benchmarks);
criterion_main!(benches);
