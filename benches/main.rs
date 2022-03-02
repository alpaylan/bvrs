// pub mod group;
// pub mod single;
pub mod sparse_array;
criterion::criterion_main! {
    // group::benches,
    // single::benches,
    sparse_array::benches,
}
