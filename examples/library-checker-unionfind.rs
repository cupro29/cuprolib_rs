// verification-helper: PROBLEM https://judge.yosupo.jp/problem/unionfind

use lib_rs::unionfind::UnionFind;
use proconio::input;

fn main() {
    input! {
        n: usize, q: usize,
        tuq: [(u32, usize, usize); q],
    }
    let mut uf = UnionFind::new(n);
    for &(t, u, v) in tuq.iter() {
        if t == 0 {
            uf.merge(u, v);
        } else {
            println!("{}", if uf.same(u, v) { 1 } else { 0 });
        }
    }
}
