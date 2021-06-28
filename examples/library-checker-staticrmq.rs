// verification-helper: PROBLEM https://judge.yosupo.jp/problem/staticrmq

use lib_rs::segtree::Segtree;
use proconio::input;
use std::cmp::min;

fn main() {
    input! {
        n: usize, q: usize,
        a: [u32; n],
        lr: [(usize, usize); q],
    }
    let seg = Segtree::from_vec(&a, min, std::u32::MAX);
    for &(l, r) in lr.iter() {
        println!("{}", seg.prod(l, r));
    }
}
