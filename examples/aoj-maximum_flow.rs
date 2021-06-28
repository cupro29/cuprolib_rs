// verification-helper: PROBLEM https://onlinejudge.u-aizu.ac.jp/courses/library/5/GRL/all/GRL_6_A

use lib_rs::maxflow::MaxFlowGraph;
use proconio::input;

fn main() {
    input! {
        n: usize, m: usize,
        uvc: [(usize, usize, u32); m],
    }
    let mut mfg = MaxFlowGraph::new(n);
    for &(u, v, c) in uvc.iter() {
        mfg.add_edge(u, v, c);
    }
    println!("{}", mfg.maxflow(0, n - 1, std::u32::MAX));
}
