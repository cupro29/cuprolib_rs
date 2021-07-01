// verification-helper: PROBLEM https://onlinejudge.u-aizu.ac.jp/courses/library/3/DSL/all/DSL_2_D

use lib_rs::dualsegtree::DualSegtree;
use proconio::input;

fn main() {
    const ID: i32 = std::i32::MAX;
    input! { n: usize, q: usize, }
    let mut seg = DualSegtree::new(n, |x, y| if x == ID { y } else { x }, ID);
    for _ in 0..q {
        input! { t: u32, }
        if t == 0 {
            input! { l: usize, r: usize, x: i32 }
            seg.range_apply(l, r + 1, x);
        } else {
            input! { i: usize, }
            println!("{}", seg.get(i));
        }
    }
}
