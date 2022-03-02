#![allow(unused_must_use)]

use crate::bit_vec::*;
use serde::{Deserialize, Serialize};
use serde_json;
use std::borrow::{Borrow, Cow};
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RankSupport<'bv> {
    pub(crate) bv: Cow<'bv, BitVec>,
    pub(crate) rs: Vec<BitVec>,
    pub(crate) rb: Vec<Vec<BitVec>>,
    pub(crate) rp: Vec<Vec<BitVec>>,
}

impl<'bv> RankSupport<'bv> /* Data Structure Construction */ {
    fn compute_rs(bv: &BitVec) -> Vec<BitVec> {
        let log2n = (bv.size as f64).log2();
        let super_block_size = BitVecSize((log2n * log2n / 2.0).ceil() as usize);
        // println!("SuperBlockSize: {:?}", super_block_size);
        let super_block_space = BitVecSize(((bv.size + 1) as f64).log2().ceil() as usize);
        let vec_size = (bv.size as f64 / (super_block_size.to_usize() as f64)).ceil() as usize;
        let mut super_blocks: Vec<u64> = Vec::with_capacity(vec_size);
        for _ in 0..vec_size {
            super_blocks.push(0);
        }
        // println!("Before {:?}", super_blocks);
        let mut count = 0;
        for i in 0..(vec_size - 1) {
            for j in 0..super_block_size.to_usize() {
                count += bv.get_u8(i * super_block_size.to_usize() + j) as u64;
            }
            super_blocks[i + 1] = count;
            // println!("At {}: {:?}", i, super_blocks);
        }
        // println!("SuperBlocks {:?}", super_blocks);
        super_blocks
            .iter()
            .map(|x| BitVec::from_u64(x.clone(), super_block_space.to_usize()))
            .collect()
    }
    fn compute_rb(bv: &BitVec, rs: &Vec<BitVec>) -> Vec<Vec<BitVec>> {
        let log2n = (bv.size as f64).log2();
        let super_block_size = BitVecSize((log2n * log2n / 2.0).ceil() as usize);
        let block_size = BitVecSize((log2n / 2.0).ceil() as usize);
        // println!("Blocksize: {:?}", block_size);
        let block_space = BitVecSize(((super_block_size.to_usize()) as f64).log2().ceil() as usize);
        let vec_size = rs.len();
        let sub_vec_size = log2n.ceil() as usize;
        let mut blocks: Vec<Vec<u64>> = Vec::with_capacity(vec_size);
        for i in 0..vec_size {
            blocks.push(Vec::with_capacity(sub_vec_size));
            for _ in 0..sub_vec_size {
                blocks[i].push(0);
            }
        }
        for i in 0..vec_size {
            let mut count = 0;
            for j in 0..(sub_vec_size - 1) {
                for k in 0..block_size.to_usize() {
                    count += bv
                        .get_u8(i * super_block_size.to_usize() + j * block_size.to_usize() + k)
                        as u64;
                }
                blocks[i][j + 1] = count;
            }
        }
        // println!("Blocks: {:?}", blocks);
        blocks
            .into_iter()
            .map(|v| {
                v.into_iter()
                    .map(|x| BitVec::from_u64(x.clone(), block_space.to_usize()))
                    .collect()
            })
            .collect()
    }
    fn compute_rp(bv: &BitVec) -> Vec<Vec<BitVec>> {
        let log2n = (bv.size as f64).log2();
        let super_block_size = (log2n * log2n / 2.0).ceil() as usize;
        let block_size = (log2n / 2.0).ceil() as usize;
        let block_space = (super_block_size as f64).log2().ceil() as usize;
        let lookup_table_size = 2_usize.pow(block_size as u32);
        let mut lookup_table: Vec<Vec<u64>> = Vec::with_capacity(lookup_table_size);
        for _ in 0..lookup_table_size {
            lookup_table.push(Vec::with_capacity(block_size));
        }
        for i in 0..lookup_table_size {
            for j in 0..block_size {
                // println!("Round (i: {}) (j: {})", i, j);
                let temp_bv = BitVec::from_u64(i as u64, block_size);
                let rank = RankSupport::dummy_rankn(&temp_bv, j);
                lookup_table[i].push(rank);
            }
        }
        // println!("Lookup: {:?}", lookup_table);
        lookup_table
            .into_iter()
            .map(|v| {
                v.into_iter()
                    .map(|x| BitVec::from_u64(x, block_space))
                    .collect()
            })
            .collect()
    }
    pub(in crate) fn compute_index(&mut self) {
        self.rs = RankSupport::compute_rs(self.bv.borrow());
        self.rb = RankSupport::compute_rb(self.bv.borrow(), &self.rs);
        self.rp = RankSupport::compute_rp(self.bv.borrow());
    }
    pub(in crate) fn new_with_index_computation(bit_vec: Cow<'bv, BitVec>) -> RankSupport {
        let bv = bit_vec;
        let rs = RankSupport::compute_rs(bv.borrow());
        let rb = RankSupport::compute_rb(bv.borrow(), &rs);
        let rp = RankSupport::compute_rp(bv.borrow());
        RankSupport { bv, rs, rb, rp }
    }
    pub fn set(&mut self, i: usize) {
        self.bv.to_mut().set(i);
    }
}

impl<'bv> RankSupport<'bv> /* Public API */ {
    pub fn new(bit_vec: &BitVec) -> RankSupport {
        RankSupport::new_with_index_computation(Cow::Borrowed(bit_vec))
    }
    pub fn dummy_rankn(bv: &BitVec, i: usize) -> u64 {
        let mut rank = 0;
        for ind in 0..=i {
            rank += bv.get(ind) as u64;
        }
        rank
    }
    pub fn rank1(&self, i: u64) -> u64 {
        let log2n = (self.bv.size as f64).log2();
        let super_block_size = (log2n * log2n / 2.0).ceil() as usize;
        let block_size = (log2n / 2.0).ceil() as usize;
        // println!("SuperBlockSize: {}", super_block_size);
        // println!("BlockSize: {}", block_size);
        let super_block_index = (i as usize / super_block_size) as usize;
        let i = i as usize % super_block_size;
        let block_index = (i / block_size) as usize;
        let lookup_rank_index = i % block_size;
        let value_from_super_block = self.rs[super_block_index].to_u64();
        let value_from_block = self.rb[super_block_index][block_index].to_u64();
        let left = super_block_index * super_block_size + block_index * block_size;
        let right = left + block_size;
        let lookup_row_index = self.bv.extract(left, right).to_u64() as usize;
        let value_from_lookup = self.rp[lookup_row_index][lookup_rank_index].to_u64();
        // println!("Lookup:");
        // print!("(left: {})\t", left);
        // print!("(right: {})\t", right);
        // print!("(extracted: {})\t", self.bv.extract(left, right));
        // print!("(row: {})\t", lookup_row_index);
        // println!("(rank: {})", lookup_rank_index);

        value_from_super_block + value_from_block + value_from_lookup
    }
    pub fn overhead(&self) -> usize {
        std::mem::size_of_val(&*self.rs)
            + std::mem::size_of_val(&*self.rb)
            + std::mem::size_of_val(&*self.rp)
    }
    pub fn save(&self, file_name: &str) -> std::io::Result<()> {
        let serialized = serde_json::to_string(&self)?;
        let mut file = File::create(file_name)?;
        file.write_all(serialized.as_bytes())?;
        Ok(())
    }
    pub fn load(file_name: String) -> std::io::Result<RankSupport<'bv>> {
        let mut file = File::open(file_name)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let deserialized: RankSupport = serde_json::from_str(&contents)?;
        Ok(deserialized)
    }
}

#[cfg(test)]
mod rank1_tests {
    use crate::bit_vec::BitVec;
    use crate::rank_support::RankSupport;
    #[test]
    fn small_tests() {
        for i in 1..=128 {
            let size = i * 8;
            let b = BitVec::new_with_random(size);
            let mut r = RankSupport::new(&b);
            r.compute_index();
            for j in 0..b.size {
                let dummy_res = RankSupport::dummy_rankn(&b, j);
                let smart_res = r.rank1(j as u64);
                assert_eq!(
                    dummy_res, smart_res,
                    "<============= Case [size: {}, point: {}] Starts ==============>\n \
                    BV = {}\n\
                    Rank = {}\n\
                    Dummy Rank = {}\n\
                    <============= Case [size: {}, point: {}] Ends ==============>\n",
                    size, j, b, smart_res, dummy_res, size, j
                );
            }
        }
    }
    #[test]
    fn small_tests_two() {
        for i in 1..=128 {
            let size = i * 8;
            let b = BitVec::new_with_random(size);
            let r = RankSupport::new(&b);
            for j in 0..b.size {
                let dummy_res = RankSupport::dummy_rankn(&b, j);
                let smart_res = r.rank1(j as u64);
                assert_eq!(
                    dummy_res, smart_res,
                    "<============= Case [size: {}, point: {}] Starts ==============>\n \
                    BV = {}\n\
                    Rank = {}\n\
                    Dummy Rank = {}\n\
                    <============= Case [size: {}, point: {}] Ends ==============>\n",
                    size, j, b, smart_res, dummy_res, size, j
                );
            }
        }
    }
    #[test]
    fn medium_tests() {
        for i in 128..=160 {
            let size = i * 128;
            let b = BitVec::new_with_random(size);
            let mut r = RankSupport::new(&b);
            r.compute_index();
            for j in (0..b.size).step_by(80) {
                // println!("Test {} {}", i, j);
                let dummy_res = RankSupport::dummy_rankn(&b, j);
                let smart_res = r.rank1(j as u64);
                assert_eq!(
                    dummy_res, smart_res,
                    "<============= Case [size: {}, point: {}] Starts ==============>\n \
                    BV = {}\n\
                    Rank = {}\n\
                    Dummy Rank = {}\n\
                    <============= Case [size: {}, point: {}] Ends ==============>\n",
                    size, j, b, smart_res, dummy_res, size, j
                );
            }
        }
    }
    #[test]
    fn large_tests() {
        for i in 256..=280 {
            let size = i * 128;
            let b = BitVec::new_with_random(size);
            let mut r = RankSupport::new(&b);
            r.compute_index();
            for j in (0..b.size).step_by(250) {
                // println!("Test {} {}", i, j);
                let dummy_res = RankSupport::dummy_rankn(&b, j);
                let smart_res = r.rank1(j as u64);
                assert_eq!(
                    dummy_res, smart_res,
                    "<============= Case [size: {}, point: {}] Starts ==============>\n \
                    BV = {}\n\
                    Rank = {}\n\
                    Dummy Rank = {}\n\
                    <============= Case [size: {}, point: {}] Ends ==============>\n",
                    size, j, b, smart_res, dummy_res, size, j
                );
            }
        }
    }
    #[test]
    fn very_large_tests() {
        {
            let size = 40960;
            let b = BitVec::new_with_random(size);
            let mut r = RankSupport::new(&b);
            r.compute_index();
            for j in (0..b.size).step_by(250) {
                let dummy_res = RankSupport::dummy_rankn(&b, j);
                let smart_res = r.rank1(j as u64);
                assert_eq!(
                    dummy_res, smart_res,
                    "<============= Case [size: {}, point: {}] Starts ==============>\n \
                    BV = {}\n\
                    Rank = {}\n\
                    Dummy Rank = {}\n\
                    <============= Case [size: {}, point: {}] Ends ==============>\n",
                    size, j, b, smart_res, dummy_res, size, j
                );
            }
        }
        {
            let size = 51200;
            let b = BitVec::new_with_random(size);
            let mut r = RankSupport::new(&b);
            r.compute_index();
            for j in (0..b.size).step_by(250) {
                let dummy_res = RankSupport::dummy_rankn(&b, j);
                let smart_res = r.rank1(j as u64);
                assert_eq!(
                    dummy_res, smart_res,
                    "<============= Case [size: {}, point: {}] Starts ==============>\n \
                    BV = {}\n\
                    Rank = {}\n\
                    Dummy Rank = {}\n\
                    <============= Case [size: {}, point: {}] Ends ==============>\n",
                    size, j, b, smart_res, dummy_res, size, j
                );
            }
        }
        {
            let size = 61440;
            let b = BitVec::new_with_random(size);
            let mut r = RankSupport::new(&b);
            r.compute_index();
            for j in (0..b.size).step_by(250) {
                let dummy_res = RankSupport::dummy_rankn(&b, j);
                let smart_res = r.rank1(j as u64);
                assert_eq!(
                    dummy_res, smart_res,
                    "<============= Case [size: {}, point: {}] Starts ==============>\n \
                    BV = {}\n\
                    Rank = {}\n\
                    Dummy Rank = {}\n\
                    <============= Case [size: {}, point: {}] Ends ==============>\n",
                    size, j, b, smart_res, dummy_res, size, j
                );
            }
        }
    }
}

#[cfg(test)]
mod save_load_tests {
    use crate::bit_vec::BitVec;
    use crate::rank_support::RankSupport;
    #[test]
    fn simple_test() {
        {
            let size = 128;
            let b = BitVec::new_with_random(size);
            let mut r = RankSupport::new(&b);
            r.compute_index();
            r.save("example.txt");
            let r2 = RankSupport::load("example.txt".to_owned()).unwrap();
            for j in 0..b.size {
                let dummy_res = RankSupport::dummy_rankn(&b, j);
                let smart_res = r2.rank1(j as u64);
                assert_eq!(
                    dummy_res, smart_res,
                    "<============= Case [size: {}, point: {}] Starts ==============>\n \
                    BV = {}\n\
                    Rank = {}\n\
                    Dummy Rank = {}\n\
                    <============= Case [size: {}, point: {}] Ends ==============>\n",
                    size, j, b, smart_res, dummy_res, size, j
                );
            }
        }
    }
    #[test]
    fn fail_test_file_not_found() {
        {
            let size = 128;
            let b = BitVec::new_with_random(size);
            let mut r = RankSupport::new(&b);
            r.compute_index();
            r.save("example.txt");
            let res = RankSupport::load("example2.txt".to_owned());
            assert!(res.is_err());
        }
    }
    #[test]
    fn new_with_load_test() {
        {
            let size = 128;
            let b = BitVec::new_with_random(size);
            let b = b.clone();
            let r = RankSupport::new(&b);
            r.save("example1.txt");
            let r2 = RankSupport::load("example1.txt".to_owned()).unwrap();
            for j in 0..b.size {
                let dummy_res = RankSupport::dummy_rankn(&b, j);
                let smart_res = r2.rank1(j as u64);
                assert_eq!(
                    dummy_res, smart_res,
                    "<============= Case [size: {}, point: {}] Starts ==============>\n \
                    BV = {}\n\
                    Rank = {}\n\
                    Dummy Rank = {}\n\
                    <============= Case [size: {}, point: {}] Ends ==============>\n",
                    size, j, b, smart_res, dummy_res, size, j
                );
            }
        }
    }
}
