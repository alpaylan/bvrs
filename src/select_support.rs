#![allow(dead_code)]
use crate::rank_support::RankSupport;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::fs::File;
use std::io::{Read, Write};

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct SelectSupport<'bv> {
    pub(crate) r: Cow<'bv, RankSupport<'bv>>,
}

impl<'bv> SelectSupport<'bv> {
    fn binary_search_select1(&self, l: usize, r: usize, i: u64) -> u64 {
        if l == r && i == 1 {
            return l as u64;
        }
        let m = ((l + r) / 2) as u64;
        let rank_0 = self.r.rank1(m);
        let rank_1 = self.r.rank1(m + 1);
        if rank_0 == (i - 1) && rank_1 == i {
            m + 1
        } else if rank_1 < i {
            self.binary_search_select1(m as usize, r, i)
        } else {
            self.binary_search_select1(l, m as usize, i)
        }
    }
    pub(in crate) fn compute_index(&mut self) {
        self.r.to_mut().compute_index();
    }
    pub(in crate) fn set(&mut self, i: usize) {
        self.r.to_mut().set(i);
    }
    pub(in crate) fn rank1(&self, u: u64) -> u64 {
        self.r.rank1(u)
    }
    pub(in crate) fn get_size(&self) -> usize {
        self.r.bv.size
    }
}

impl<'bv> SelectSupport<'bv> /* Public API */ {
    pub fn new(r: Cow<'bv, RankSupport<'bv>>) -> SelectSupport<'bv> {
        SelectSupport { r }
    }
    pub fn dummy_selectn(&self, i: usize) -> Option<u64> {
        let mut select = 0;
        let mut expected_ones = i;
        for ind in 0..self.r.bv.size {
            expected_ones -= self.r.bv.get(ind) as usize;
            if expected_ones == 0 {
                select = ind as u64;
                break;
            }
        }
        if expected_ones > 0 {
            None
        } else {
            Some(select)
        }
    }
    pub fn select1(&self, i: u64) -> Option<u64> {
        let size = self.r.bv.size;
        let max_rank = self.r.rank1(self.r.bv.size as u64 - 1);
        if i > max_rank {
            None
        } else {
            Some(self.binary_search_select1(0, size, i))
        }
    }
    pub fn overhead(self) -> usize {
        self.r.overhead()
    }
    pub fn save(&self, file_name: &str) -> std::io::Result<()> {
        let serialized = serde_json::to_string(&self)?;
        let mut file = File::create(file_name)?;
        file.write_all(serialized.as_bytes())?;
        Ok(())
    }
    pub fn load(file_name: String) -> std::io::Result<SelectSupport<'bv>> {
        let mut file = File::open(file_name)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let deserialized: SelectSupport = serde_json::from_str(&contents)?;
        Ok(deserialized)
    }
}

#[cfg(test)]
mod select1_tests {
    use crate::bit_vec::BitVec;
    use crate::rank_support::RankSupport;
    use crate::select_support::SelectSupport;
    #[test]
    fn small_tests() {
        for i in 1..=128 {
            let size = i * 8;
            let b = BitVec::new_with_random(size);
            let r = RankSupport::new_with_index_computation(std::borrow::Cow::Borrowed(&b));
            let s = SelectSupport::new(std::borrow::Cow::Borrowed(&r));
            for j in 1..b.size {
                let dummy_res = SelectSupport::dummy_selectn(&s, j);
                let smart_res = s.select1(j as u64);
                assert_eq!(
                    dummy_res, smart_res,
                    "<============= Case [size: {}, point: {}] Starts ==============>\n \
                    BV = {}\n\
                    Select = {:?}\n\
                    Dummy Select = {:?}\n\
                    <============= Case [size: {}, point: {}] Ends ==============>\n",
                    size, j, b, smart_res, dummy_res, size, j
                );
            }
        }
    }
}
