#![allow(dead_code)]
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::ops::Add;

#[derive(Serialize, Deserialize, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub(in crate) struct ByteSize(pub usize);

impl From<BitVecSize> for ByteSize {
    fn from(b: BitVecSize) -> Self {
        let BitVecSize(u) = b;
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

#[derive(Serialize, Deserialize, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub(in crate) struct BitVecSize(pub usize);

impl From<ByteSize> for BitVecSize {
    fn from(b: ByteSize) -> Self {
        let ByteSize(u) = b;
        BitVecSize(u * 8)
    }
}
impl BitVecSize {
    pub fn to_usize(&self) -> usize {
        self.0
    }
    pub fn to_u64(&self) -> u64 {
        self.0 as u64
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialOrd, PartialEq, Ord, Eq)]
pub struct BitVec {
    pub bv: Vec<u8>,
    pub size: usize,
}

impl BitVec /* Essentials */ {
    pub fn new(size: usize) -> Self {
        BitVec::new_with_zeros(size)
    }
    pub fn new_with_zeros(size: usize) -> Self {
        let byte_size = ByteSize((size as f64 / 8.0_f64).ceil() as usize);
        let mut bv = Vec::with_capacity(byte_size.to_usize());
        for _ in 0..byte_size.0 {
            bv.push(0);
        }
        BitVec { bv, size }
    }
    pub fn new_with_vec(vec: Vec<u8>) -> Self {
        let size = BitVecSize::from(ByteSize(vec.len())).to_usize();
        let bv = vec;
        BitVec { bv, size }
    }
    pub fn new_with_random(size: usize) -> Self {
        let byte_size = ByteSize::from(BitVecSize(size));
        let mut bv = Vec::with_capacity(byte_size.to_usize());
        for _ in 0..byte_size.to_usize() {
            bv.push(rand::random());
        }
        BitVec { bv, size }
    }
}
impl BitVec /* Operations */ {
    // Getters
    pub fn get(&self, i: usize) -> bool {
        self.get_bool(i)
    }
    fn get_bool(&self, i: usize) -> bool {
        let pri_ind = i / 8;
        let snd_ind = i % 8;
        let padding = if pri_ind == 0 { (self.size - 1) % 8 } else { 7 };
        if let Some(b) = self.bv.get(pri_ind) {
            (b & (0b00000001 << (padding - snd_ind))) != 0
        } else {
            // println!("Rank Position Exceeds Size!");
            // println!("(Position: {:?}, Size: {:?})", i, self.size);
            false
        }
    }
    pub fn get_u8(&self, i: usize) -> u8 {
        self.get_bool(i) as u8
    }
    // Setters
    pub fn set(&mut self, i: usize) {
        self.set_bool(i);
    }
    pub(crate) fn set_bool(&mut self, i: usize) {
        let pri_ind = i / 8;
        let snd_ind = i % 8;
        let padding = if pri_ind == 0 { (self.size - 1) % 8 } else { 7 };
        if let Some(b) = self.bv.get(pri_ind) {
            self.bv[pri_ind] = b | (0b00000001 << (padding - snd_ind));
        }
    }
    // Slicing
    fn concat(&self, other: &Self) -> Self {
        let bv = [self.clone().bv, other.clone().bv].concat();
        let size = BitVecSize::from(ByteSize(bv.len())).to_usize();
        Self { bv, size }
    }
    pub fn extract(&self, left: usize, right: usize) -> Self {
        if left > right {
            panic!()
        }
        let mut val = 0;
        for i in left..right {
            val += 2_i32.pow((right - i - 1) as u32) * self.get_u8(i) as i32;
        }
        BitVec::from_u64(val as u64, right - left)
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
            let zeros: BitVec =
                BitVec::new_with_zeros(BitVecSize::from(ByteSize(remain)).to_usize());
            BitVec::concat(&zeros, &self)
        }
    }
    pub fn to_u64(&self) -> u64 {
        let bv = self.zero_extend();
        let bytes = &bv.bv[..];
        u64::from_be_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
        ])
    }
    pub fn from_u64(u: u64, size: usize) -> Self {
        let bytes = u.to_be_bytes();
        let bv: Vec<u8> = bytes.to_vec();
        let vec_size = bv.len();
        let empty_bytes = vec_size - ByteSize::from(BitVecSize(size)).to_usize();
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
        write!(f, "BitVec({}): [", self.size)?;
        for byte in &self.bv[..self.bv.len() - 1] {
            write!(f, "{:08b} ", byte)?;
        }
        write!(f, "{:08b}]", self.bv[self.bv.len() - 1])
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
