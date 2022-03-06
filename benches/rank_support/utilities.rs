use bvrs::{BitVec, RankSupport};
use rand;
use rand::distributions::{Distribution, Standard};
use rand::Rng;
use std::collections::HashSet;
use std::ops::Range;

pub struct RankSupportUtilities;

impl RankSupportUtilities {
    pub fn uniform_sample_range(range: Range<u64>, size: usize) -> Vec<u64> {
        let mut positions: Vec<u64> = Vec::with_capacity(size);
        let mut rng = rand::thread_rng();
        for _ in 0..size {
            positions.push(rng.gen_range(range.clone()));
        }
        positions
    }
    pub fn rank1_over_list(r: &RankSupport, positions: &Vec<u64>) {
        for p in positions {
            r.rank1(*p);
        }
    }
    pub fn dummy_rank_over_list(bv: &BitVec, positions: &Vec<u64>) {
        for p in positions {
            RankSupport::dummy_rankn(bv, *p as usize);
        }
    }
}
