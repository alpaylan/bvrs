mod bit_vec;
mod rank_support;
mod select_support;
mod sparse_array;

pub use bit_vec::BitVec;
pub use rank_support::RankSupport;
pub use select_support::SelectSupport;
pub use sparse_array::SparseArray;

pub fn fibonacci(n: u32) -> u32 {
    if n <= 1 {
        1
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}
