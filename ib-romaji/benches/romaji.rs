use std::hint::black_box;

use criterion::{Criterion, criterion_group, criterion_main};
use ib_romaji::HepburnRomanizer;

pub fn criterion_benchmark(c: &mut Criterion) {
    let data = HepburnRomanizer::new();

    assert_eq!(data.romanize("日"), None);
    c.bench_function("hepburn_miss", |b| {
        b.iter(|| data.romanize(black_box("日")))
    });
    assert_eq!(data.romanize("は"), Some((3, "ha")));
    c.bench_function("hepburn", |b| b.iter(|| data.romanize(black_box("は"))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
