use std::hint::black_box;

use criterion::{Criterion, criterion_group, criterion_main};

static WORDS: &str = include_str!("../src/data/words.in.txt");

pub fn criterion_benchmark(c: &mut Criterion) {
    // daachorse_charwise
    {
        use daachorse::*;
        let ac = CharwiseDoubleArrayAhoCorasickBuilder::new()
            // .start_kind(StartKind::Anchored)
            .match_kind(MatchKind::LeftmostLongest)
            .build::<_, _, u32>(black_box(WORDS.split('\n')))
            .unwrap();
        c.bench_function("hit_daachorse_charwise", |b| {
            b.iter(|| ac.leftmost_find_iter(black_box("この場合")).next())
        });
    }
    {
        use daachorse::*;
        let ac = CharwiseDoubleArrayAhoCorasickBuilder::new()
            // .start_kind(StartKind::Anchored)
            .match_kind(MatchKind::LeftmostLongest)
            .build::<_, _, u32>(black_box(WORDS.split('\n')))
            .unwrap();
        c.bench_function("miss_daachorse_charwise", |b| {
            b.iter(|| ac.leftmost_find_iter(black_box("このすば")).next())
        });
    }

    // daachorse
    {
        use daachorse::*;
        let ac = DoubleArrayAhoCorasickBuilder::new()
            // .start_kind(StartKind::Anchored)
            .match_kind(MatchKind::LeftmostLongest)
            .build::<_, _, u32>(black_box(WORDS.split('\n')))
            .unwrap();
        c.bench_function("hit_daachorse", |b| {
            b.iter(|| ac.leftmost_find_iter(black_box("この場合")).next())
        });
    }
    {
        use daachorse::*;
        let ac = DoubleArrayAhoCorasickBuilder::new()
            // .start_kind(StartKind::Anchored)
            .match_kind(MatchKind::LeftmostLongest)
            .build::<_, _, u32>(black_box(WORDS.split('\n')))
            .unwrap();
        c.bench_function("miss_daachorse", |b| {
            b.iter(|| ac.leftmost_find_iter(black_box("このすば")).next())
        });
    }

    // daachorse_charwise middle
    {
        use daachorse::*;
        let ac = CharwiseDoubleArrayAhoCorasickBuilder::new()
            // .start_kind(StartKind::Anchored)
            .match_kind(MatchKind::LeftmostLongest)
            .build::<_, _, u32>(black_box(WORDS.split('\n')))
            .unwrap();
        c.bench_function("hit_middle_daachorse_charwise", |b| {
            b.iter(|| {
                ac.leftmost_find_iter(black_box("。。。。。。。。。。。。。この場合"))
                    .next()
            })
        });
    }
    {
        use daachorse::*;
        let ac = CharwiseDoubleArrayAhoCorasickBuilder::new()
            // .start_kind(StartKind::Anchored)
            .match_kind(MatchKind::LeftmostLongest)
            .build::<_, _, u32>(black_box(WORDS.split('\n')))
            .unwrap();
        c.bench_function("miss_middle_daachorse_charwise", |b| {
            b.iter(|| {
                ac.leftmost_find_iter(black_box("。。。。。。。。。。。。。このすば"))
                    .next()
            })
        });
    }

    // ac
    {
        use aho_corasick::*;
        let ac = AhoCorasick::builder()
            .start_kind(StartKind::Anchored)
            .match_kind(MatchKind::LeftmostLongest)
            .build(black_box(WORDS.split('\n')))
            .unwrap();
        c.bench_function("miss_ac", |b| {
            b.iter(|| ac.find(Input::new(black_box("このすば")).anchored(Anchored::Yes)))
        });
    }
    {
        use aho_corasick::*;
        let ac = AhoCorasick::builder()
            .start_kind(StartKind::Anchored)
            .match_kind(MatchKind::LeftmostLongest)
            .build(black_box(WORDS.split('\n')))
            .unwrap();
        c.bench_function("hit_ac", |b| {
            b.iter(|| ac.find(Input::new(black_box("この場合")).anchored(Anchored::Yes)))
        });
    }

    // Deserialize
    {
        use daachorse::*;
        let ac = CharwiseDoubleArrayAhoCorasickBuilder::new()
            // .start_kind(StartKind::Anchored)
            .match_kind(MatchKind::LeftmostLongest)
            .build::<_, _, u32>(black_box(WORDS.split('\n')))
            .unwrap();
        let bytes = ac.serialize();
        // 9.64 MiB
        dbg!(bytes.len());
        c.bench_function("deserialize_daachorse_charwise", |b| {
            b.iter(|| unsafe { CharwiseDoubleArrayAhoCorasick::<u32>::deserialize_unchecked(&bytes) }.0)
        });
    }
    {
        use daachorse::*;
        let ac = DoubleArrayAhoCorasickBuilder::new()
            // .start_kind(StartKind::Anchored)
            .match_kind(MatchKind::LeftmostLongest)
            .build::<_, _, u32>(black_box(WORDS.split('\n')))
            .unwrap();
        let bytes = ac.serialize();
        // 14.83 MiB
        dbg!(bytes.len());
        c.bench_function("deserialize_daachorse_charwise", |b| {
            b.iter(|| unsafe { DoubleArrayAhoCorasick::<u32>::deserialize_unchecked(&bytes) }.0)
        });
    }

    // Build
    {
        use daachorse::*;
        c.bench_function("build_daachorse", |b| {
            b.iter(|| {
                DoubleArrayAhoCorasickBuilder::new()
                    // .start_kind(StartKind::Anchored)
                    .match_kind(MatchKind::LeftmostLongest)
                    .build::<_, _, u32>(black_box(WORDS.split('\n')))
                    .unwrap()
            })
        });
    }
    {
        use daachorse::*;
        c.bench_function("build_daachorse_charwise", |b| {
            b.iter(|| {
                CharwiseDoubleArrayAhoCorasickBuilder::new()
                    // .start_kind(StartKind::Anchored)
                    .match_kind(MatchKind::LeftmostLongest)
                    .build::<_, _, u32>(black_box(WORDS.split('\n')))
                    .unwrap()
            })
        });
    }
    {
        use aho_corasick::*;
        c.bench_function("build_ac", |b| {
            b.iter(|| {
                AhoCorasick::builder()
                    .start_kind(StartKind::Anchored)
                    .match_kind(MatchKind::LeftmostLongest)
                    .build(black_box(WORDS.split('\n')))
                    .unwrap()
            })
        });
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
