// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DSL_1_B&lang=jp

use lib_rs::weighteduf::WeightedUnionFind;
use proconio::input;

fn main() {
    input! { n: usize, q: usize, }
    let mut wuf = WeightedUnionFind::new(n);
    for _ in 0..q {
        input! { t: u32, }
        if t == 0 {
            input! { x: usize, y: usize, z: i32, }
            wuf.merge(x, y, z);
        } else {
            input! { x: usize, y: usize, }
            if let Some(d) = wuf.diff(x, y) {
                println!("{}", d);
            } else {
                println!("?");
            }
        }
    }
}
