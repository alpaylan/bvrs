use crate::rank_support::RankSupport;
use crate::select_support::SelectSupport;
use crate::BitVec;
use std::borrow::{BorrowMut, Cow};
use std::ops::{Deref, DerefMut};

struct SparseArray<'bv, T> {
    // bv: BitVec,
    // r: RankSupport<'bv>,
    s: SelectSupport<'bv>,
    v: Vec<T>,
}
impl<'bv, T: Copy> SparseArray<'bv, T> {
    fn new(size: usize) -> SparseArray<'bv, T> {
        let bv = Cow::Owned(BitVec::new(size));
        let r = Cow::Owned(RankSupport::new(bv));
        let s = SelectSupport::new(r);
        let v = vec![];
        SparseArray { s, v }
    }
}

impl<'bv, T: Copy + Deref<Target = T>> SparseArray<'bv, T> {
    fn append(&mut self, elem: T, pos: usize) {
        self.v.push(elem);
        self.borrow_mut()
            .s
            .borrow_mut()
            .r
            .borrow_mut()
            .bv
            .borrow_mut();
        self.borrow_mut().s.r.bv.set(pos, true);
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
        let element_exists = self.s.r.bv.get(u);
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
        self.s.r.bv.size
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
