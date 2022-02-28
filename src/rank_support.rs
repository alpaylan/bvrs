use crate::bit_vec::*;
use std::path::Component::Prefix;

pub struct RankSupport<'bv> {
    bv: &'bv BitVec,
    pub(crate) rs: Vec<BitVec>,
    pub(crate) rb: Vec<Vec<BitVec>>,
    pub(crate) rp: Vec<Vec<BitVec>>,
}

impl<'bv> RankSupport<'bv> /* Data Structure Construction */ {
    fn compute_rs(bv: &BitVec) -> Vec<BitVec> {
        let log2n = (bv.size.to_usize() as f64).log2();
        let super_block_size = BVSize((log2n * log2n / 2.0).ceil() as usize);
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
        println!("Final {:?}", super_blocks);
        super_blocks
            .iter()
            .map(|x| BitVec::from_u64(x.clone(), super_block_space))
            .collect()
    }
    fn compute_rb(bv: &BitVec, rs: &Vec<BitVec>) -> Vec<Vec<BitVec>> {
        let log2n = (bv.size.to_usize() as f64).log2();
        let super_block_size = BVSize((log2n * log2n / 2.0).ceil() as usize);
        let block_size = BVSize((log2n / 2.0).ceil() as usize);
        println!("Blocksize: {:?}", block_size);
        let block_space = BVSize(((super_block_size.to_usize()) as f64).log2().ceil() as usize);
        let vec_size = rs.len();
        let sub_vec_size = log2n.ceil() as usize;
        let mut blocks: Vec<Vec<u64>> = Vec::with_capacity(vec_size);
        for i in 0..vec_size {
            blocks.push(Vec::with_capacity(sub_vec_size));
            for j in 0..sub_vec_size {
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
        println!("Blocks: {:?} - BlockSpace : {:?}", blocks, block_space);
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
                let temp_bv = BitVec::from_u64(i as u64, BVSize(block_space));
                let rank = RankSupport::dummy_rankn(&temp_bv, j);
                lookup_table[i].push(rank);
            }
        }
        println!("Lookup: {:?}", lookup_table);
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
        let rs = RankSupport::compute_rs(&bv);
        let rb = RankSupport::compute_rb(&bv, &rs);
        let rp = RankSupport::compute_rp(&bv);
        RankSupport { bv, rs, rb, rp }
    }
    pub fn dummy_rank1(&self, i: u64) -> u64 {
        unimplemented!()
    }
    pub fn dummy_rankn(bv: &BitVec, i: usize) -> u64 {
        let mut rank = 0;
        for ind in 0..=i {
            rank += bv.get(BVSize(i - ind)) as u64;
        }
        rank
    }
    pub fn rank1(&self, i: u64) -> u64 {
        let super_block_size =
            (self.bv.size.to_usize() as f64 / self.rs.len() as f64).ceil() as usize;
        let block_size = super_block_size / self.rb[0].len();
        // let block_size = (log2n / 2.0).ceil() as usize;
        let super_block_index = (i as usize / super_block_size) as usize;
        let i = i as usize % super_block_size;
        let block_index = (i / block_size) as usize;
        // println!("Let's Debug");
        // print!("SuperBlockSize : {} || \t", super_block_size);
        // print!("SuperBlockIndex : {} || \t", super_block_index);
        // print!("BlockSize : {} || \t", block_size);
        // println!("BlockIndex : {}", block_index);
        let lookup_rank_index = i % block_size;
        let value_from_super_block = self.rs[super_block_index].to_u64();
        let value_from_block = self.rb[super_block_index][block_index].to_u64();
        let left = super_block_index * super_block_size + block_index * block_size;
        let right = left + block_size;
        let lookup_row_index = self.bv.extract(left, right).to_u64() as usize;
        println!(
            "Extracted({}, {}) {} - Index - {}",
            left,
            right,
            self.bv.extract(left, right),
            lookup_row_index
        );
        let value_from_lookup = self.rp[lookup_row_index][lookup_rank_index].to_u64();
        value_from_super_block + value_from_block + value_from_lookup
    }
    pub fn overhead(self) -> usize {
        std::mem::size_of_val(&*self.rs)
            + std::mem::size_of_val(&*self.rb)
            + std::mem::size_of_val(&*self.rp)
    }
    pub fn save(file_name: &str) {
        unimplemented!()
    }
    pub fn load(file_name: &str) {
        unimplemented!()
    }
}
