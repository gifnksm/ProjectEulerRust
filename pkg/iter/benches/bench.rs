use criterion::{criterion_group, criterion_main, Criterion};
use iter::BitCombination;

fn bench(c: &mut Criterion) {
    c.bench_function("comb last", |b| {
        b.iter(|| {
            let cnt = criterion::black_box(5);
            let size = criterion::black_box(10);
            BitCombination::new(cnt, size).last()
        })
    });
    c.bench_function("comb all", |b| {
        b.iter(|| {
            let cnt = criterion::black_box(5);
            let size = criterion::black_box(10);
            BitCombination::new(cnt, size).for_each(|bits| {
                let _ = criterion::black_box(bits);
            })
        })
    });
}

criterion_group!(benches, bench);
criterion_main!(benches);
