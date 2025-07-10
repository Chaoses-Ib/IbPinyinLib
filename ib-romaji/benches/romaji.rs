use std::hint::black_box;

use criterion::{Criterion, criterion_group, criterion_main};
use ib_romaji::HepburnRomanizer;

pub fn criterion_benchmark(c: &mut Criterion) {
    let data = HepburnRomanizer::builder().kana(true).build();

    assert_eq!(data.romanize_kana("日"), None);
    c.bench_function("kana_miss", |b| {
        b.iter(|| data.romanize_kana(black_box("日")))
    });
    assert_eq!(data.romanize_kana("は"), Some((3, "ha")));
    c.bench_function("kana", |b| b.iter(|| data.romanize_kana(black_box("は"))));

    c.bench_function("build_word", |b| {
        b.iter(|| HepburnRomanizer::builder().word(true).build())
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
