use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ib_pinyin::{matcher::PinyinMatcher, pinyin::PinyinNotation};

pub fn criterion_benchmark(c: &mut Criterion) {
    let matcher = PinyinMatcher::builder("pysseve")
        .pinyin_notations(PinyinNotation::Ascii | PinyinNotation::AsciiFirstLetter)
        .build();

    assert!(matcher.is_match("pyssEverything"));
    c.bench_function("is_match_ascii", |b| {
        b.iter(|| matcher.is_match(black_box("pyssEverything")))
    });

    assert!(matcher.find("pyssEverything").is_some());
    c.bench_function("find_ascii", |b| {
        b.iter(|| matcher.find(black_box("pyssEverything")))
    });

    c.bench_function("build", |b| {
        b.iter(|| {
            PinyinMatcher::builder("pysseve")
                .pinyin_notations(PinyinNotation::Ascii | PinyinNotation::AsciiFirstLetter)
                .build()
        })
    });

    assert!(matcher.is_match("拼音搜索Everything"));
    c.bench_function("is_match", |b| {
        b.iter(|| matcher.is_match(black_box("拼音搜索Everything")))
    });

    assert!(matcher.find("拼音搜索Everything").is_some());
    c.bench_function("find", |b| {
        b.iter(|| matcher.find(black_box("拼音搜索Everything")))
    });

    assert!(matcher.find("あなたは誰拼音搜索Everything").is_some());
    c.bench_function("find_5", |b| {
        b.iter(|| matcher.find(black_box("あなたは誰拼音搜索Everything")))
    });

    assert!(matcher.find("あなたは誰拼音搜索Evvrything").is_none());
    c.bench_function("find_5_miss", |b| {
        b.iter(|| matcher.find(black_box("あなたは誰拼音搜索Evvrything")))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
