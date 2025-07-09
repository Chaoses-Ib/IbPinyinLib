use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ib_matcher::{matcher::PinyinMatcher, pinyin::PinyinNotation};

pub fn criterion_benchmark(c: &mut Criterion) {
    {
        assert!("pysseverything".find("pysseve").is_some());
        c.bench_function("find_ascii_std", |b| {
            b.iter(|| black_box("pysseverything").find("pysseve"))
        });
    }

    {
        let ac = aho_corasick::AhoCorasick::builder()
            .ascii_case_insensitive(true)
            .build(&["pysseve"])
            .unwrap();
        assert!(ac.find("pyssEverything").is_some());
        c.bench_function("find_ascii_ac", |b| {
            b.iter(|| ac.find(black_box("pyssEverything")))
        });
    }

    {
        let regex = regex::RegexBuilder::new("pysseve")
            .unicode(false)
            .case_insensitive(true)
            .build()
            .unwrap();
        assert!(regex.find("pyssEverything").is_some());
        c.bench_function("find_ascii_regex", |b| {
            b.iter(|| regex.find(black_box("pyssEverything")))
        });

        let regex = regex::bytes::RegexBuilder::new("pysseve")
            .unicode(false)
            .case_insensitive(true)
            .build()
            .unwrap();
        assert!(regex.find(b"pyssEverything").is_some());
        c.bench_function("find_ascii_regex_bytes", |b| {
            b.iter(|| regex.find(black_box(b"pyssEverything")))
        });

        let regex = regex::bytes::RegexBuilder::new("\\x70\\x79\\x73\\x73\\x65\\x76\\x65")
            .unicode(false)
            .case_insensitive(true)
            .build()
            .unwrap();
        assert!(regex.find(b"pyssEverything").is_some());
        c.bench_function("find_ascii_regex_bytes_x", |b| {
            b.iter(|| regex.find(black_box(b"pyssEverything")))
        });
    }

    {
        let matcher = PinyinMatcher::builder("pysseve")
            .pinyin_notations(PinyinNotation::Ascii | PinyinNotation::AsciiFirstLetter)
            .build();
        assert!(matcher.find("pyssEverything").is_some());
        c.bench_function("find_ascii", |b| {
            b.iter(|| matcher.find(black_box("pyssEverything")))
        });
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
