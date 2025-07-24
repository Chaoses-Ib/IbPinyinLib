use std::hint::black_box;

use criterion::{criterion_group, criterion_main, Criterion};
use ib_matcher::{
    matcher::{IbMatcher, PinyinMatchConfig},
    pinyin::PinyinNotation,
};

pub fn criterion_benchmark(c: &mut Criterion) {
    let pinyin = PinyinMatchConfig::notations(PinyinNotation::DiletterXiaohe);

    c.bench_function("traversal_10", |b| {
        b.iter(|| {
            IbMatcher::builder(black_box("rstwhenenterfolder"))
                .analyze(true)
                .pinyin(pinyin.shallow_clone())
                .build()
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
