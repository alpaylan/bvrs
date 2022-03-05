use bvrs::SparseArray;
use bvrs::*;
use criterion::{criterion_group, BenchmarkId, Criterion, PlotConfiguration};
use rand;
use rand::distributions::{Distribution, Standard};
use rand::Rng;
use std::collections::HashSet;

fn create_sparse_array_elements<Element>(size: usize, sparsity: f64) -> Vec<(usize, Element)>
where
    Element: Copy,
    Standard: Distribution<Element>,
{
    let actual_size = (size as f64 * sparsity) as usize;
    let mut positions_hashset: HashSet<usize> =
        std::collections::HashSet::with_capacity(actual_size);
    let mut rng = rand::thread_rng();
    for _ in 0..actual_size {
        positions_hashset.insert(rng.gen_range(0..size));
    }
    let mut positions: Vec<usize> = positions_hashset.into_iter().map(|x| x).collect();
    positions.sort();
    let mut elements = vec![];
    for i in positions {
        elements.push((i, rand::random()));
    }
    elements
}

pub fn create_sparse_array<T: Copy>(elements: &[(usize, T)]) -> SparseArray<'static, T> {
    let mut sparse_array: SparseArray<T> = SparseArray::new(elements.len());
    for (pos, elem) in elements {
        sparse_array.append(*elem, *pos);
    }
    sparse_array
}

pub fn benchmark(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("Generate Sparse Array");
    group.plot_config(PlotConfiguration::default());

    for size in (4..=16).map(|i| i * i * i * 8) {
        for sparsity in [0.01, 0.03, 0.05, 0.1] {
            let elements: Vec<(usize, u64)> = create_sparse_array_elements(size, sparsity);
            group.bench_with_input(
                BenchmarkId::new(format!("Sparsity={}", sparsity), &size),
                &size,
                |bencher, _| {
                    bencher.iter(|| create_sparse_array(criterion::black_box(&elements)));
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
