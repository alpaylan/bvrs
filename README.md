<div align="center">
  <h1>BVRS</h1>
  A lightweight rust library for BitVector Rank&Select operations, coupled with a generic Sparse Array implementation.
</div>


## Description

This library was written as part of CMSC858D - Algorithms, Data Structures and Inference for High-throughput Genomics Class Homework.

The official version of the library is hosted on github at this [repository](https://github.com/alpaylan/bvrs), while crates.io version is hosted at this [page](https://crates.io/crates/bvrs/).

## Installation

You can download the source code from github and build the project using `cargo build` or add it to your project as a dependency from crates.io.

## Usage
Below are some ways to instantiate and use given constructs.
### BitVector
```rust
use bvrs::BitVec;

let size = 16;
let b1 = BitVec::new_with_random(size);
let b2 = BitVec::new_with_zeros(size);
let b3 = BitVec::new_with_vec(vec![0b10010001, 0b10000001]);
let b4 = BitVec::new(size); // uses new with zeros
let b5 = b1 + b2;
let b6 = b1.incr();
let b7 = b1.concat(&b2);
let b8 = b1.extract(0, 7);
let bit = b1.get(2);
```
### Rank Support
```rust
use bvrs::BitVec;
use bvrs::RankSupport;

let size = 16;
let b1 = BitVec::new_with_random(size);
let r = RankSupport::new(&b);
let index = 3;
let res = r.rank1(index);
let res_prime = RankSupport::dummy_rank(&b, index);
r.save("example.txt");
let r2 = RankSupport::load("example.txt".to_owned()).unwrap();
```

### Select Support
```rust
use bvrs::BitVec;
use bvrs::RankSupport;
use bvrs::SelectSupport;

let b = BitVec::new_with_random(size);
let r = RankSupport::new(&b);
let s = SelectSupport::new(Cow::Borrowed(&r));
let select = s.select1(3);
```


### Sparse Array
```rust
use bvrs::SparseArray;

let size = 32;
let mut sa: SparseArray<String> = SparseArray::new(size);
sa.append("alp".to_owned(), 3);
let res = sa.get_at_index(3).unwrap();
assert_eq!(*res, "alp".to_owned());

```
