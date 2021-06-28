// verification-helper: PROBLEM https://judge.yosupo.jp/problem/static_range_sum

use lib_rs::binaryindexedtree::BIT;
use proconio::input;

fn main() {
    input! {
        n: usize, q: usize,
        a: [u64; n],
        lr: [(usize, usize); q],
    }
    let mut bit = BIT::new(n);
    for i in 0..n {
        bit.add(i, a[i]);
    }
    for &(l, r) in lr.iter() {
        println!("{}", bit.range(l, r));
    }
}
