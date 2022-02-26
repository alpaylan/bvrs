use crate::bit_vec::*;

pub struct RankSupport<'bv> {
    bv: &'bv BitVec,
    pub(crate) rs: Vec<BitVec>,
    pub(crate) rr: Vec<Vec<BitVec>>,
    rp: BitVec,
}

impl<'bv> RankSupport<'bv> {
    pub fn new(bit_vec: &BitVec) -> RankSupport {
        let bv = bit_vec;
        let rs = RankSupport::compute_rs(&bv);
        let rr = RankSupport::compute_rr(&bv, &rs);
        let rp = RankSupport::compute_rp(&bv);
        RankSupport { bv, rs, rr, rp }
    }
    fn compute_rs(bv: &BitVec) -> Vec<BitVec> {
        let log2n = (bv.size.to_usize() as f64).log2();
        let super_block_size = BVSize((log2n * log2n / 2.0).ceil() as usize);
        let super_block_space =
            BVSize(((super_block_size.to_usize() + 1) as f64).log2().ceil() as usize);
        let vec_size =
            (bv.size.to_usize() as f64 / (super_block_size.to_usize() as f64)).ceil() as usize;
        let mut super_blocks: Vec<u64> = Vec::with_capacity(vec_size);
        println!(
            "bvsize: {:?} || superblocksize : {:?} || vecsize: {}",
            bv.size, super_block_size, vec_size
        );
        for _ in 0..vec_size {
            super_blocks.push(0);
        }
        // println!("Before {:?}", super_blocks);
        for i in 0..(vec_size - 1) {
            let mut count = 0;
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
    fn compute_rr(bv: &BitVec, rs: &Vec<BitVec>) -> Vec<Vec<BitVec>> {
        let log2n = (bv.size.to_usize() as f64).log2();
        let super_block_size = BVSize((log2n * log2n / 2.0).ceil() as usize);
        let block_size = BVSize((log2n / 2.0).ceil() as usize);
        let block_space = BVSize(((block_size.to_usize() + 1) as f64).log2().ceil() as usize);
        let vec_size = rs.len();
        let sub_vec_size = log2n as usize;
        let mut blocks: Vec<Vec<u64>> = Vec::with_capacity(vec_size);
        for i in 0..vec_size {
            blocks.push(Vec::with_capacity(sub_vec_size));
            for j in 0..sub_vec_size {
                blocks[i].push(0);
            }
        }
        for i in 0..vec_size {
            for j in 0..(sub_vec_size - 1) {
                let mut count = 0;
                for k in 0..block_size.to_usize() {
                    count += bv.get_u8(BVSize(
                        i * super_block_size.to_usize() + j * block_size.to_usize() + k,
                    )) as u64;
                }
                blocks[i][j + 1] = count;
            }
        }
        println!("Blocks: {:?}", blocks);
        blocks
            .into_iter()
            .map(|v| {
                v.into_iter()
                    .map(|x| BitVec::from_u64(x.clone(), block_space))
                    .collect()
            })
            .collect()
    }
    fn compute_rp(bv: &BitVec) -> BitVec {
        BitVec::new(BVSize(0))
    }
}
impl<'bv> RankSupport<'bv> {
    pub fn dummy_rank1(&self, i: u64) -> u64 {
        unimplemented!()
    }
    pub fn dummy_rankn(&self, i: usize) -> u64 {
        let mut rank = 0;
        for ind in 0..i {
            rank += self.bv.get(BVSize(ind)) as u64;
        }
        rank
    }
    pub fn rank1(self, i: u64) -> u64 {
        unimplemented!()
    }
    pub fn overhead() -> u64 {
        unimplemented!()
    }
    pub fn save(file_name: &str) {
        unimplemented!()
    }
    pub fn load(file_name: &str) {
        unimplemented!()
    }
}
