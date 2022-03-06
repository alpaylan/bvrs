#[cfg(test)]
mod benchmark_overhead {
    use bvrs::{BitVec, RankSupport};
    #[test]
    pub fn benchmark() {
        let mut overheads = vec![];
        for i in 1..=84 {
            let size = i * i * 8;
            let b = BitVec::new_with_random(size);
            let r = RankSupport::new(&b);
            let overhead = r.overhead();
            overheads.push((size, overhead));
        }
        println!("Overheads: {:?}", overheads);
    }
}
