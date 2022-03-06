use bvrs::SparseArray;
use rand;
use rand::distributions::{Distribution, Standard};
use rand::Rng;
use std::collections::HashSet;
use std::ops::Range;

pub struct SparseArrayUtilities;

impl SparseArrayUtilities {
    pub fn create_sparse_array_elements<Element>(
        size: usize,
        sparsity: f64,
    ) -> Vec<(usize, Element)>
    where
        Element: Copy,
        Standard: Distribution<Element>,
    {
        let actual_size = (size as f64 * sparsity) as usize;
        let mut positions_hashset: HashSet<usize> =
            std::collections::HashSet::with_capacity(actual_size);
        let mut rng = rand::thread_rng();
        for _ in 0..actual_size {
            positions_hashset.insert(rng.gen_range(0..size));
        }
        let mut positions: Vec<usize> = positions_hashset.into_iter().map(|x| x).collect();
        positions.sort();
        let mut elements = vec![];
        for i in positions {
            elements.push((i, rand::random()));
        }
        elements
    }

    pub fn create_sparse_array<T: Copy>(elements: &[(usize, T)]) -> SparseArray<'static, T> {
        let mut sparse_array: SparseArray<T> = SparseArray::new(elements.len());
        for (pos, elem) in elements {
            sparse_array.append(*elem, *pos);
        }
        sparse_array
    }

    pub fn uniform_sample_range(range: Range<usize>, size: usize) -> Vec<usize> {
        let mut positions: Vec<usize> = Vec::with_capacity(size);
        let mut rng = rand::thread_rng();
        for _ in 0..size {
            positions.push(rng.gen_range(range.clone()));
        }
        positions
    }
    pub fn get_rank_sparse_array<T>(sa: &SparseArray<T>, positions: &Vec<usize>) {
        for p in positions {
            sa.get_at_rank(*p);
        }
    }
    pub fn get_index_sparse_array<T>(sa: &SparseArray<T>, positions: &Vec<usize>) {
        for p in positions {
            sa.get_at_index(*p);
        }
    }
}
