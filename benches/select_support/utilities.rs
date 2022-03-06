use bvrs::{BitVec, RankSupport, SelectSupport};
use rand;
use rand::distributions::{Distribution, Standard};
use rand::Rng;
use std::ops::Range;

pub struct SelectSupportUtilities;

impl SelectSupportUtilities {
    pub fn uniform_sample_range(range: Range<u64>, size: usize) -> Vec<u64> {
        let mut positions: Vec<u64> = Vec::with_capacity(size);
        let mut rng = rand::thread_rng();
        for _ in 0..size {
            positions.push(rng.gen_range(range.clone()));
        }
        positions
    }
    pub fn select1_over_list(s: &SelectSupport, positions: &Vec<u64>) {
        for p in positions {
            s.select1(*p);
        }
    }
}
