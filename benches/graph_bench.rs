use criterion::{black_box, criterion_group, criterion_main, Criterion};
use fenotype::networks::gnm_random_graph;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("gnm_random_graph 1000", |b| {
        b.iter(|| gnm_random_graph(black_box(1_000), black_box(100), black_box(false)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
