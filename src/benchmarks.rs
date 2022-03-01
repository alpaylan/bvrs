#[cfg(test)]
mod rank1_benchmarks {
    use crate::bit_vec::BitVec;
    use crate::rank_support::RankSupport;
    use easybench::bench_env;

    #[test]
    fn overhead() {
        let mut overheads = vec![];
        for i in 1..=84 {
            let size = i * i * 8;
            let b = BitVec::new_with_random(size);
            let mut r = RankSupport::new(std::borrow::Cow::Borrowed(&b));
            r.compute_index();
            let overhead = r.overhead();
            overheads.push((size, overhead, size as f64 / overhead as f64));
        }
        println!("Overheads: {:?}", overheads);
    }

    fn run_with_rank_support(r: &RankSupport, step_size: usize) {
        for j in (0..r.bv.size).step_by(step_size) {
            let smart_res = r.rank1(j as u64);
        }
    }
    #[test]
    fn time_rank1() {
        for i in 1..=84 {
            let size = i * i * 8;
            let b = BitVec::new_with_random(size);
            let mut r = RankSupport::new(std::borrow::Cow::Borrowed(&b));
            r.compute_index();
            println!(
                "Bench({}) = {}",
                size,
                bench_env((r, i * i), |(r, size)| {
                    run_with_rank_support(&r, size.clone())
                })
            );
        }
    }

    fn run_with_dummy_rank(b: &BitVec, step_size: usize) {
        for j in (0..b.size).step_by(step_size) {
            let smart_res = RankSupport::dummy_rankn(b, j as usize);
        }
    }

    #[test]
    fn time_dummy_rank() {
        for i in 1..=84 {
            let size = i * i * 8;
            let b = BitVec::new_with_random(size);
            println!(
                "Bench({}) = {}",
                size,
                bench_env((b, i * i), |(b, size)| {
                    run_with_dummy_rank(b, size.clone());
                })
            );
        }
    }
}
