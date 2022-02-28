use bitvec::order::Msb0;
use bitvec::vec as BitVecHelp;
use std::fmt::{Display, Formatter};
use std::ops::Add;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct ByteSize(pub usize);
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct BVSize(pub usize);
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Byte(pub u8);

impl Byte {
    pub fn get_bits(&self) -> u8 {
        self.0
    }
}
impl From<ByteSize> for BVSize {
    fn from(b: ByteSize) -> Self {
        let ByteSize(u) = b;
        BVSize(u * 8)
    }
}
impl BVSize {
    pub fn to_usize(&self) -> usize {
        self.0
    }
    pub fn to_u64(&self) -> u64 {
        self.0 as u64
    }
}
impl From<BVSize> for ByteSize {
    fn from(b: BVSize) -> Self {
        let BVSize(u) = b;
        ByteSize((u as f64 / 8.0_f64).ceil() as usize)
    }
}
impl ByteSize {
    pub fn to_usize(&self) -> usize {
        self.0
    }
    pub fn to_u64(&self) -> u64 {
        self.0 as u64
    }
}
#[derive(Debug, Clone, PartialOrd, PartialEq, Ord, Eq)]
pub struct BitVec {
    pub bv: Vec<Byte>,
    pub size: BVSize,
}

impl BitVec /* Essentials */ {
    pub fn new(size: BVSize) -> Self {
        BitVec::new_with_zeros(size)
    }
    fn new_with_zeros(size: BVSize) -> Self {
        let byte_size = ByteSize((size.to_usize() as f64 / 8.0_f64).ceil() as usize);
        let mut bv = Vec::with_capacity(byte_size.0);
        for _ in 0..byte_size.0 {
            bv.push(Byte(0));
        }
        BitVec { bv, size }
    }
    pub fn new_with_vec(vec: Vec<u8>) -> Self {
        let size = BVSize(vec.len() * 8);
        let bv = vec.into_iter().map(|x| Byte(x)).collect();
        BitVec { bv, size }
    }
    pub fn new_with_random(size: usize) -> Self {
        BitVec::new_with_random_(BVSize(size))
    }
    fn new_with_random_(size: BVSize) -> Self {
        let byte_size = ByteSize::from(size);
        let mut bv = Vec::with_capacity(byte_size.to_usize());
        for _ in 0..byte_size.to_usize() {
            bv.push(Byte(rand::random()));
        }
        BitVec { bv, size }
    }
}

impl BitVec /* Operations */ {
    // Getters
    pub fn get(&self, i: BVSize) -> bool {
        self.get_bool(i)
    }
    fn get_bool(&self, i: BVSize) -> bool {
        let pri_ind = i.to_usize() / 8;
        let snd_ind = i.to_usize() % 8;
        let padding = if pri_ind == 0 {
            (self.size.to_usize() - 1) % 8
        } else {
            7
        };
        if let Some(b) = self.bv.get(pri_ind) {
            (b.0 & (0b00000001 << (padding - snd_ind))) != 0
        } else {
            // println!("Rank Position Exceeds Size!");
            // println!("(Position: {:?}, Size: {:?})", i, self.size);
            false
        }
    }
    pub fn get_u8(&self, i: BVSize) -> u8 {
        self.get_bool(i) as u8
    }
    // Slicing
    fn concat(&self, other: &Self) -> Self {
        let bv = [self.clone().bv, other.clone().bv].concat();
        let size = BVSize::from(ByteSize(bv.len()));
        Self { bv, size }
    }
    pub fn extract(&self, left: usize, right: usize) -> Self {
        if left > right
        /* || right > self.size.to_usize() */
        {
            panic!()
        }
        let mut val = 0;
        for i in left..right {
            val += 2_i32.pow((right - i - 1) as u32) * self.get_u8(BVSize(i)) as i32;
        }
        BitVec::from_u64(val as u64, BVSize(right - left))
    }
    // Ops
    fn incr(self) -> Self {
        BitVec::from_u64(1, self.size) + self
    }
}

impl BitVec /* Transform to u64 */ {
    fn zero_extend(&self) -> Self {
        let size = self.bv.len();
        if size >= 8 {
            self.clone()
        } else {
            let remain = 8 - size;
            let mut zeros: BitVec = BitVec::new_with_zeros(ByteSize(remain).into());
            BitVec::concat(&zeros, &self)
        }
    }
    pub fn to_u64(&self) -> u64 {
        let bv = self.zero_extend();
        let bytes = &bv.bv[..];
        let bytes_as_u8: Vec<u8> = bytes.into_iter().map(|x| x.get_bits()).collect();

        u64::from_be_bytes(bytes_as_u8.try_into().unwrap())
    }
    pub fn from_u64(u: u64, size: BVSize) -> Self {
        let bytes = u.to_be_bytes();
        let bv: Vec<Byte> = bytes.to_vec().into_iter().map(|x| Byte(x)).collect();
        let vec_size = bv.len();
        let empty_bytes = vec_size - ByteSize::from(size).0;
        let bv = bv[empty_bytes..].to_vec();
        BitVec { bv, size }
    }
}

impl Add for BitVec {
    type Output = Self;

    fn add(self, rhs: BitVec) -> Self {
        BitVec::from_u64(self.to_u64() + rhs.to_u64(), self.size)
    }
}

impl Display for BitVec {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "BitVec({}): [", self.size.to_usize());
        for byte in &self.bv[..self.bv.len() - 1] {
            write!(f, "{:08b} ", byte.get_bits());
        }
        write!(f, "{:08b}]", self.bv[self.bv.len() - 1].get_bits())
    }
}
#[cfg(test)]
mod essentials_test {
    #[test]
    fn test_new() {}
    #[test]
    fn test_new_with_vec() {}
    #[test]
    fn test_new_with_random() {}
}

#[cfg(test)]
mod operations_test {
    use crate::*;
    #[test]
    fn test_get_bool() {}
    #[test]
    fn test_get_u8() {}
    #[test]
    fn test_concat() {}
    #[test]
    fn test_incr() {
        let bv1 = BitVec::new_with_vec(vec![0b10010001, 0b10000001]);
        let bv2 = bv1.incr();
        assert_eq!(bv2, BitVec::new_with_vec(vec![0b10010001, 0b10000010]));
        let bv3 = bv2.incr();
        assert_eq!(bv3, BitVec::new_with_vec(vec![0b10010001, 0b10000011]));
        let bv4 = bv3.incr();
        assert_eq!(bv4, BitVec::new_with_vec(vec![0b10010001, 0b10000100]));
    }
    #[test]
    fn test_add() {
        let bv1 = BitVec::new_with_vec(vec![0b10010001, 0b10000001]);
        let bv2 = BitVec::new_with_vec(vec![0b10000001, 0b10000001]);
        let addition = bv1 + bv2;
        assert_eq!(addition, BitVec::new_with_vec(vec![0b00010011, 0b00000010]));
    }
}
