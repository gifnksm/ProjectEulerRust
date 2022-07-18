use criterion::{criterion_group, criterion_main, Criterion};
use prime::PrimeSet;

fn bench(c: &mut Criterion) {
    c.bench_function("get_5000th", |b| {
        b.iter(|| {
            let n = criterion::black_box(5000);
            PrimeSet::new().nth(n)
        })
    });
    c.bench_function("get_below_5000th", |b| {
        b.iter(|| {
            let ps = PrimeSet::new();
            ps.iter().take(5000).for_each(|p| {
                let _ = criterion::black_box(p);
            });
        })
    });
}

criterion_group!(benches, bench);
criterion_main!(benches);
