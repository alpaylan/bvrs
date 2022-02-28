use crate::rank_support::RankSupport;
use crate::BVSize;

struct SelectSupport<'bv> {
    r: &'bv mut RankSupport<'bv>,
}

impl<'bv> SelectSupport<'bv> {
    fn new(r: &'bv mut RankSupport<'bv>) -> SelectSupport<'bv> {
        SelectSupport { r }
    }
    pub fn compute_index(&mut self) {
        self.r.compute_index();
    }
}

impl<'bv> SelectSupport<'bv> {
    pub fn dummy_selectn(&self, i: usize) -> Option<u64> {
        let mut select = 0;
        let mut expected_ones = i;
        for ind in 0..self.r.bv.size.to_usize() {
            expected_ones -= self.r.bv.get(BVSize(ind)) as usize;
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
        let size = self.r.bv.size.to_usize();
        let max_rank = self.r.rank1(self.r.bv.size.to_u64() - 1);
        if i > max_rank {
            None
        } else {
            Some(self.binary_search_select1(0, size, i))
        }
    }
    pub fn binary_search_select1(&self, l: usize, r: usize, i: u64) -> u64 {
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
    pub fn overhead(self) -> usize {
        self.r.overhead()
    }
    pub fn save(&self, file_name: &str) -> std::io::Result<()> {
        self.r.save(file_name)
    }
    pub fn load(&mut self, file_name: &str) -> std::io::Result<()> {
        self.r.load(file_name)
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
            let mut r = RankSupport::new(&b);
            let mut s = SelectSupport::new(&mut r);
            s.compute_index();
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
