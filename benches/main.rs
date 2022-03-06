mod rank_support;
mod select_support;
mod sparse_array;

criterion::criterion_main! {
//     sparse_array::generation::benches,
//     sparse_array::rank::benches,
//     sparse_array::index::benches,
    // rank_support::generation::benches,
    rank_support::rank::benches,
    rank_support::dummy_rank::benches,
    select_support::select::benches,
}
