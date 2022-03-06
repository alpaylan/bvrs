// pub mod group;
// pub mod single;
// pub mod sparse_array_generation;
mod sparse_array;

use sparse_array::index;
// use sparse_array::rank;
criterion::criterion_main! {
    // group::benches,
    // single::benches,
    // sparse_array_generation::benches,
    // rank::benches,
    index::benches
}
