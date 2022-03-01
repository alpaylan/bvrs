use crate::rank_support::RankSupport;
use crate::select_support::SelectSupport;
use crate::{BVSize, BitVec};
use std::borrow::Borrow;
use std::ops::Deref;

struct SparseArray<'bv, T> {
    s: Box<SelectSupport<'bv>>,
    v: Vec<T>,
}
impl<'bv, T: Copy> SparseArray<'bv, T> {
    fn new(size: usize) -> Box<SparseArray<'bv, T>> {
        let bv = Box::new(BitVec::new(BVSize(size)));
        let r = Box::new(RankSupport::new(bv.as_ref()));
        let mut s = Box::new(SelectSupport::new(r.as_ref()));
        Box::new(SparseArray { s: s, v: vec![] })
    }
}

impl<'bv, T: Copy + Deref + Deref<Target = T>> SparseArray<'bv, T> {
    fn append(&mut self, elem: T, pos: usize) {
        self.v.push(elem);
        self.s.r.bv.set(pos, true);
    }

    fn get_at_rank(&self, u: usize, elem: &mut T) -> bool {
        if let Some(elem_) = self.v.get(u) {
            *elem = *elem_;
            true
        } else {
            false
        }
    }
    fn get_at_index(&mut self, u: usize, elem: &mut T) -> bool {
        let element_exists = self.s.r.bv.get(BVSize(u));
        if element_exists {
            let rank = self.s.r.rank1(u as u64);
            if let Some(elem_) = self.v.get(rank as usize) {
                *elem = *(elem_);
            }
        }

        element_exists
    }

    fn num_elem_at(&self, u: u64) -> u64 {
        self.s.r.rank1(u)
    }

    fn size(&self) -> usize {
        self.s.r.bv.size.to_usize()
    }

    fn num_elem(&self) -> usize {
        self.v.len()
    }

    fn save(&self, fname: &String) -> std::io::Result<()> {
        self.s.save(fname)
    }

    // fn load(&mut self, fname: &String) -> std::io::Result<()> {
    //     self.s.load(fname)
    // }
}

#[cfg(test)]
mod sparse_array_tests {
    use crate::bit_vec::BitVec;
    use crate::rank_support::RankSupport;
    use crate::select_support::SelectSupport;
    use crate::sparse_array::SparseArray;
    use std::ops::DerefMut;
    #[test]
    fn small_tests() {
        for i in 1..=128 {
            let size = i * 8;
            let b = BitVec::new_with_random(size);
            let r = RankSupport::new_with_index_computation(&b);
            let s = SelectSupport::new(&r);
            for j in 1..b.size.to_usize() {
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
