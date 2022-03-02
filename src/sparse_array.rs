#![allow(dead_code)]
use crate::rank_support::RankSupport;
use crate::select_support::SelectSupport;
use crate::BitVec;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::fs::File;
use std::io::{Read, Write};

#[derive(PartialEq, Serialize, Deserialize)]
pub struct SparseArray<'bv, T> {
    s: SelectSupport<'bv>,
    v: Vec<T>,
}
impl<'bv, T> SparseArray<'bv, T> {
    fn compute_index(&mut self) {
        self.s.compute_index();
    }
    fn set_bv_index(&mut self, i: usize) {
        self.s.set(i);
    }
}

impl<'bv, T: Serialize + DeserializeOwned> SparseArray<'bv, T> /* Public API */ {
    pub fn new(size: usize) -> SparseArray<'bv, T> {
        let bv = Cow::Owned(BitVec::new(size));
        let r = Cow::Owned(RankSupport::new_with_index_computation(bv));
        let s = SelectSupport::new(r);
        let v = vec![];
        SparseArray { s, v }
    }
    pub fn append(&mut self, elem: T, pos: usize) {
        self.v.push(elem);
        self.set_bv_index(pos);
        self.compute_index();
    }
    pub fn get_at_rank(&self, u: usize) -> Option<&T> {
        if let Some(elem_) = self.v.get(u - 1) {
            Some(elem_)
        } else {
            None
        }
    }
    pub fn get_at_index(&mut self, u: usize) -> Option<&T> {
        if self.s.r.bv.get(u) {
            let rank = self.s.rank1(u as u64);
            if let Some(elem) = self.v.get(rank as usize - 1) {
                return Some(elem);
            }
        }
        None
    }
    pub fn num_elem_at(&self, u: u64) -> u64 {
        self.s.rank1(u)
    }
    pub fn size(&self) -> usize {
        self.s.get_size()
    }
    pub fn num_elem(&self) -> usize {
        self.v.len()
    }
    pub fn save(&self, file_name: &str) -> std::io::Result<()> {
        let serialized = serde_json::to_string(&self)?;
        let mut file = File::create(file_name)?;
        file.write_all(serialized.as_bytes())?;
        Ok(())
    }
    pub fn load(file_name: String) -> std::io::Result<SparseArray<'bv, T>> {
        let mut file = File::open(file_name)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let deserialized: SparseArray<'bv, T> = serde_json::from_str(&contents)?;
        Ok(deserialized)
    }
}

#[cfg(test)]
mod sparse_array_tests {
    use crate::sparse_array::SparseArray;
    use serde::{Deserialize, Serialize};
    use std::string::String;

    #[test]
    fn single_element_get_at_index_tests() {
        for i in 1..=32 {
            let size = i * 32;
            let mut sa: SparseArray<String> = SparseArray::new(size);
            sa.append("alp".to_owned(), 3);
            let res = sa.get_at_index(3).unwrap();
            assert_eq!(*res, "alp".to_owned());
        }
    }
    #[test]
    fn fail_get_at_index_tests() {
        for i in 1..=32 {
            let size = i * 32;
            let mut sa: SparseArray<String> = SparseArray::new(size);
            sa.append("alp".to_owned(), 3);
            let res = sa.get_at_index(4);
            assert_eq!(res, None);
        }
    }
    #[test]
    fn single_element_get_at_rank_tests() {
        for i in 1..=32 {
            let size = i * 32;
            let mut sa: SparseArray<String> = SparseArray::new(size);
            sa.append("alp".to_owned(), 3);
            let res = sa.get_at_rank(1).unwrap();
            assert_eq!(*res, "alp".to_owned());
        }
    }
    #[test]
    fn fail_get_at_rank_tests() {
        for i in 1..=32 {
            let size = i * 32;
            let mut sa: SparseArray<String> = SparseArray::new(size);
            sa.append("alp".to_owned(), 3);
            let res = sa.get_at_rank(2);
            assert_eq!(res, None);
        }
    }

    #[test]
    fn test_generic_construction() {
        #[derive(PartialEq, Debug, Serialize, Deserialize)]
        struct Point {
            x: i64,
            y: i64,
        }
        for i in 1..=32 {
            let size = i * 32;
            let mut sa: SparseArray<Point> = SparseArray::new(size);
            sa.append(Point { x: 3, y: -4 }, 3);
            let res = sa.get_at_index(3).unwrap();
            assert_eq!(*res, Point { x: 3, y: -4 });
        }
        for i in 1..=32 {
            let size = i * 32;
            let mut sa: SparseArray<Point> = SparseArray::new(size);
            sa.append(Point { x: 3, y: -4 }, 3);
            let res = sa.get_at_index(4);
            assert_eq!(res, None);
        }
    }

    #[test]
    fn test_generic_save_load() -> std::io::Result<()> {
        #[derive(PartialEq, Debug, Serialize, Deserialize)]
        struct Point {
            x: i64,
            y: i64,
        }
        let size = 128;
        let mut sa: SparseArray<Point> = SparseArray::new(size);
        sa.append(Point { x: 3, y: -4 }, 3);
        sa.save("example_sparse_array.txt")?;
        let mut sa2: SparseArray<Point> =
            SparseArray::load("example_sparse_array.txt".to_owned()).unwrap();
        let res = sa2.get_at_index(3).unwrap();
        assert_eq!(*res, Point { x: 3, y: -4 });
        Ok(())
    }
}
