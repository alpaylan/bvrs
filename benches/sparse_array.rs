use bvrs::SparseArray;
use bvrs::*;
use criterion::{criterion_group, BenchmarkId, Criterion, PlotConfiguration};
use rand;
use rand::Rng;
use std::collections::HashSet;

pub trait SparseArrayElement<T> {
    fn gen_elem() -> T;
}

impl SparseArrayElement<u64> for u64 {
    fn gen_elem() -> u64 {
        let mut rng = rand::thread_rng();
        rng.gen::<u64>()
    }
}
pub fn create_sparse_array_elements<T: SparseArrayElement<T>>(
    size: usize,
    sparsity: f64,
) -> Vec<(usize, T)> {
    let actual_size = (size as f64 * sparsity) as usize;
    let mut positions_hashset: HashSet<usize> =
        std::collections::HashSet::with_capacity(actual_size);
    let mut rng = rand::thread_rng();
    for _ in 0..actual_size {
        positions_hashset.insert(rng.gen_range(0..size));
    }
    let mut positions: Vec<usize> = positions_hashset.into_iter().map(|x| x).collect();
    positions.sort();
    let mut elements: Vec<(usize, T)> = vec![];
    for i in positions {
        elements.push((i, T::gen_elem()));
    }
    elements
}

pub fn create_sparse_array<T: SparseArrayElement<T>>(
    elements: Vec<(usize, T)>,
) -> SparseArray<'static, T> {
    let mut sparse_array: SparseArray<T> = SparseArray::new(elements.len());
    for (pos, elem) in elements {
        sparse_array.append(elem, pos);
    }
    sparse_array
}

pub fn benchmark(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("Rank");
    group.plot_config(PlotConfiguration::default());

    for size in (1..=10).map(|i| i * i * i * 8).collect::<Vec<usize>>() {
        for sparsity in (1..=5)
            .map(|i| (i * i * i) as f64 * 0.01)
            .collect::<Vec<f64>>()
        {
            let elements: Vec<(usize, u64)> = create_sparse_array_elements(size, sparsity);
            group.bench_function(
                BenchmarkId::new("size", format!("({}, {})", size, sparsity)),
                |bencher| {
                    bencher.iter(|| create_sparse_array(criterion::black_box(elements.clone())));
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
