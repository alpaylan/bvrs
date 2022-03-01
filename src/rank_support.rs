#![allow(unused_must_use)]

use crate::bit_vec::*;
use serde::{Deserialize, Serialize};
use serde_json;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct RankSupportCopy {
    pub(crate) rs: Vec<BitVec>,
    pub(crate) rb: Vec<Vec<BitVec>>,
    pub(crate) rp: Vec<Vec<BitVec>>,
}
impl RankSupportCopy {
    fn new(r: &RankSupport) -> RankSupportCopy {
        RankSupportCopy {
            rs: r.rs.clone(),
            rb: r.rb.clone(),
            rp: r.rp.clone(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct RankSupport<'bv> {
    pub(crate) bv: &'bv BitVec,
    pub(crate) rs: Vec<BitVec>,
    pub(crate) rb: Vec<Vec<BitVec>>,
    pub(crate) rp: Vec<Vec<BitVec>>,
}

impl<'bv> RankSupport<'bv> /* Data Structure Construction */ {
    fn compute_rs(bv: &BitVec) -> Vec<BitVec> {
        let log2n = (bv.size.to_usize() as f64).log2();
        let super_block_size = BVSize((log2n * log2n / 2.0).ceil() as usize);
        // println!("SuperBlockSize: {:?}", super_block_size);
        let super_block_space = BVSize(((bv.size.to_usize() + 1) as f64).log2().ceil() as usize);
        let vec_size =
            (bv.size.to_usize() as f64 / (super_block_size.to_usize() as f64)).ceil() as usize;
        let mut super_blocks: Vec<u64> = Vec::with_capacity(vec_size);
        for _ in 0..vec_size {
            super_blocks.push(0);
        }
        // println!("Before {:?}", super_blocks);
        let mut count = 0;
        for i in 0..(vec_size - 1) {
            for j in 0..super_block_size.to_usize() {
                count += bv.get_u8(BVSize(i * super_block_size.to_usize() + j)) as u64;
            }
            super_blocks[i + 1] = count;
            // println!("At {}: {:?}", i, super_blocks);
        }
        // println!("SuperBlocks {:?}", super_blocks);
        super_blocks
            .iter()
            .map(|x| BitVec::from_u64(x.clone(), super_block_space))
            .collect()
    }
    fn compute_rb(bv: &BitVec, rs: &Vec<BitVec>) -> Vec<Vec<BitVec>> {
        let log2n = (bv.size.to_usize() as f64).log2();
        let super_block_size = BVSize((log2n * log2n / 2.0).ceil() as usize);
        let block_size = BVSize((log2n / 2.0).ceil() as usize);
        // println!("Blocksize: {:?}", block_size);
        let block_space = BVSize(((super_block_size.to_usize()) as f64).log2().ceil() as usize);
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
                    count += bv.get_u8(BVSize(
                        i * super_block_size.to_usize() + j * block_size.to_usize() + k,
                    )) as u64;
                }
                blocks[i][j + 1] = count;
            }
        }
        // println!("Blocks: {:?}", blocks);
        blocks
            .into_iter()
            .map(|v| {
                v.into_iter()
                    .map(|x| BitVec::from_u64(x.clone(), block_space))
                    .collect()
            })
            .collect()
    }
    fn compute_rp(bv: &BitVec) -> Vec<Vec<BitVec>> {
        let log2n = (bv.size.to_usize() as f64).log2();
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
                let temp_bv = BitVec::from_u64(i as u64, BVSize(block_size));
                let rank = RankSupport::dummy_rankn(&temp_bv, j);
                lookup_table[i].push(rank);
            }
        }
        // println!("Lookup: {:?}", lookup_table);
        lookup_table
            .into_iter()
            .map(|v| {
                v.into_iter()
                    .map(|x| BitVec::from_u64(x, BVSize(block_space)))
                    .collect()
            })
            .collect()
    }
}
impl<'bv> RankSupport<'bv> /* Public API */ {
    pub fn new(bit_vec: &BitVec) -> RankSupport {
        let bv = bit_vec;
        let rs = vec![];
        let rb = vec![];
        let rp = vec![];
        RankSupport { bv, rs, rb, rp }
    }
    pub fn new_with_index_computation(bit_vec: &BitVec) -> RankSupport {
        let bv = bit_vec;
        let rs = RankSupport::compute_rs(bit_vec);
        let rb = RankSupport::compute_rb(bit_vec, &rs);
        let rp = RankSupport::compute_rp(bit_vec);
        RankSupport { bv, rs, rb, rp }
    }
    pub fn new_with_load(
        bit_vec: &'bv BitVec,
        file_name: String,
    ) -> Result<RankSupport<'bv>, Box<dyn Error>> {
        let bv = bit_vec;
        let mut file = File::open(file_name)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let deserialized_rc: RankSupportCopy = serde_json::from_str(&contents)?;
        let rb = deserialized_rc.rb;
        let rs = deserialized_rc.rs;
        let rp = deserialized_rc.rp;
        Ok(RankSupport { bv, rs, rb, rp })
    }
    pub fn compute_index(&mut self) {
        self.rs = RankSupport::compute_rs(self.bv);
        self.rb = RankSupport::compute_rb(self.bv, &self.rs);
        self.rp = RankSupport::compute_rp(self.bv);
    }
    pub fn dummy_rank1(&self, i: u64) -> u64 {
        todo!()
    }
    pub fn dummy_rankn(bv: &BitVec, i: usize) -> u64 {
        let mut rank = 0;
        for ind in 0..=i {
            rank += bv.get(BVSize(ind)) as u64;
        }
        rank
    }
    pub fn rank1(&self, i: u64) -> u64 {
        let log2n = (self.bv.size.to_usize() as f64).log2();
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
        let rc = RankSupportCopy::new(self);
        let serialized_rc = serde_json::to_string(&rc)?;
        let mut file = File::create(file_name)?;
        file.write_all(serialized_rc.as_bytes())?;
        Ok(())
    }
    pub fn load(&mut self, file_name: &str) -> std::io::Result<()> {
        let mut file = File::open(file_name)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let deserialized_rc: RankSupportCopy = serde_json::from_str(&contents)?;
        self.rb = deserialized_rc.rb;
        self.rs = deserialized_rc.rs;
        self.rp = deserialized_rc.rp;
        Ok(())
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
            for j in 0..b.size.to_usize() {
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
            let r = RankSupport::new_with_index_computation(&b);
            for j in 0..b.size.to_usize() {
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
            for j in (0..b.size.to_usize()).step_by(80) {
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
            for j in (0..b.size.to_usize()).step_by(250) {
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
            for j in (0..b.size.to_usize()).step_by(250) {
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
            for j in (0..b.size.to_usize()).step_by(250) {
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
            for j in (0..b.size.to_usize()).step_by(250) {
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
            let mut r2 = RankSupport::new(&b);
            r2.load("example.txt");
            for j in 0..b.size.to_usize() {
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
            let mut r2 = RankSupport::new(&b);
            let res = r2.load("example2.txt");
            assert!(res.is_err());
        }
    }
    #[test]
    fn new_with_load_test() {
        {
            let size = 128;
            let b = BitVec::new_with_random(size);
            let b = b.clone();
            let r = RankSupport::new_with_index_computation(&b);
            r.save("example.txt");
            let r2 = RankSupport::new_with_load(&b, "example.txt".to_owned()).unwrap();
            println!("{:?}", r2);
            for j in 0..b.size.to_usize() {
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
