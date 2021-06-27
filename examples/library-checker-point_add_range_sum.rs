// verification-helper: PROBLEM https://judge.yosupo.jp/problem/point_add_range_sum

use lib_rs::binaryindexedtree::BIT;
use proconio::input;

fn main() {
    input! {
        n: usize, q: usize,
        a: [u64; n],
        query: [(u32, usize, usize); q],
    }
    let mut bit = BIT::new(n);
    for i in 0..n {
        bit.add(i, a[i]);
    }
    for &(t, x, y) in query.iter() {
        if t == 0 {
            bit.add(x, y as u64);
        } else {
            println!("{}", bit.range(x, y));
        }
    }
}
