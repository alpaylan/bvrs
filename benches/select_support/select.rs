use crate::select_support::utilities::SelectSupportUtilities;
use bvrs::{BitVec, RankSupport, SelectSupport};
use criterion::{criterion_group, BenchmarkId, Criterion, PlotConfiguration};
use std::borrow::Cow;

pub fn benchmark(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("Get Select1");
    group.plot_config(PlotConfiguration::default());

    for size in (1..=16).map(|i| i * i * i * 8 as u64) {
        let bv = BitVec::new_with_random(size as usize);
        let r = RankSupport::new(&bv);
        let s = SelectSupport::new(Cow::Borrowed(&r));
        let positions = SelectSupportUtilities::uniform_sample_range((0..size), 1000);

        group.bench_with_input(
            BenchmarkId::new(format!("Size:"), &size),
            &size,
            |bencher, _| {
                bencher.iter(|| {
                    SelectSupportUtilities::select1_over_list(&s, criterion::black_box(&positions));
                });
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
