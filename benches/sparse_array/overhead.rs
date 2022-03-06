use bvrs::SparseArray;

pub fn benchmark() {
    let mut overheads = vec![];
    for i in 1..=84 {
        let size = i * i * 8;
        let sa = SparseArray::new(size);
        let overhead = r.overhead();
        overheads.push((size, overhead));
    }
    println!("Overheads: {:?}", overheads);
}
