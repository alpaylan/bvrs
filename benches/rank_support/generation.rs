#![allow(dead_code)]
use bvrs::{BitVec, RankSupport};
use criterion::{criterion_group, BenchmarkId, Criterion, PlotConfiguration};

pub fn benchmark(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("Generate Rank Data Structure");
    group.plot_config(PlotConfiguration::default());

    for size in (1..=16).map(|i| i * i * i * 8) {
        let bv = BitVec::new_with_random(size);
        group.bench_with_input(
            BenchmarkId::new(format!("Size:"), &size),
            &size,
            |bencher, _| {
                bencher.iter(|| RankSupport::new(&bv));
            },
        );
    }

    group.finish();
}

criterion_group! {
    name =
        benches;

    config =
        Criterion::default()
            .sample_size(30)
            .confidence_level(0.95)
            .with_plots();

    targets =
        benchmark,
}
