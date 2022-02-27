#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]

mod bit_vec;
mod rank_support;

use crate::bit_vec::*;
use rank_support::*;
use std::fmt::{Display, Formatter};
use std::vec::Vec;

fn main() {
    let mut b = BitVec::new_with_random(256);
    let bp = BitVec::new_with_vec(vec![0b00110000; 2]);
    let r = RankSupport::new(&b);
    let i = 8;
    let res = RankSupport::dummy_rankn(&b, i);
    let res2 = r.rank1(i as u64);
    println!("BV as binary {}", b);
    // println!("BV as decimal {:?}", b);
    // println!("Rb {:?}", r.rs);
    // println!("Rr {:?}", r.rb);
    // println!("Rp {:?}", r.rp);
    println!("Rank {} is {}", i, res);
    println!("Rank {} is {}", i, res2);
    println!("Size {}", r.overhead())
}

// #[cfg(test)]
// mod from_u64_tests {
//     use crate::Transform;
//     use bitvec::order::{Lsb0, Msb0};
//     use bitvec::vec as bv;
//     #[test]
//     fn from_u64_simple_test() {
//         for i in 0..255 {
//             let fst = i as u64;
//             let snd: [u8; 1] = [i];
//             assert_eq!(
//                 bv::BitVec::from_u64(fst, 8),
//                 bv::BitVec::<_, Msb0>::from_slice(&snd)
//             )
//         }
//     }
//     #[test]
//     fn from_u64_still_simple_test() {
//         for i in 0_u8..255 {
//             for j in 0_u8..255 {
//                 let fst = (((i as u32) << 8) + j as u32) as u64;
//                 let snd: [u8; 2] = [i, j];
//                 // println!("i: {}, j: {:}", i, j);
//                 // println!("fst: {}, snd: {:?}", fst, snd);
//                 assert_eq!(
//                     bv::BitVec::from_u64(fst, 16),
//                     bv::BitVec::<_, Msb0>::from_slice(&snd)
//                 )
//             }
//         }
//     }
// }
//
// #[cfg(test)]
// mod rank_test {
//     use crate::bit_vector;
//     use crate::rank_support;
//     use crate::Transform;
//     use std::path::Component::Prefix;
//     #[test]
//     fn dummy_rank_equals_first_superblock() {
//         let mut b = bit_vector::new(1025);
//         b.set_random();
//         let mut r = rank_support::new(&b);
//         let i = 51;
//         let res = r.dummy_rankn(i);
//         let mut rbv = r.rb[1].bit_vec.clone();
//         println!("Rb : {:?}", r.rb);
//         println!("rbv: {:?}", rbv);
//         assert_eq!(res, rbv.to_u64())
//     }
// }
