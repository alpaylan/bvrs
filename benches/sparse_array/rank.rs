use super::utilities::SparseArrayUtilities;
use bvrs::SparseArray;
use criterion::{criterion_group, BenchmarkId, Criterion, PlotConfiguration};

pub fn benchmark(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("Get Rank Sparse Array");
    group.plot_config(PlotConfiguration::default());

    for size in (4..=16).map(|i| i * i * i * 8) {
        for sparsity in [0.01, 0.03, 0.05, 0.1] {
            let elements: Vec<(usize, u64)> =
                SparseArrayUtilities::create_sparse_array_elements(size, sparsity);
            let sa = SparseArrayUtilities::create_sparse_array(&elements);
            let positions = SparseArrayUtilities::uniform_sample_range((0..size), 1000);
            group.bench_with_input(
                BenchmarkId::new(format!("Sparsity={}", sparsity), &size),
                &size,
                |bencher, _| {
                    bencher.iter(|| {
                        SparseArrayUtilities::get_rank_sparse_array(
                            &sa,
                            criterion::black_box(&positions),
                        );
                    });
                },
            );
        }
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
