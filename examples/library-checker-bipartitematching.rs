// verification-helper: PROBLEM https://judge.yosupo.jp/problem/bipartitematching

use lib_rs::maxflow::MaxFlowGraph;
use proconio::input;

fn main() {
    input! {
        l: usize, r: usize, m: usize,
        ab: [(usize, usize); m],
    }
    let mut mfg = MaxFlowGraph::new(l + r + 2);
    let (s, t) = (l + r, l + r + 1);
    for i in 0..l {
        mfg.add_edge(s, i, 1);
    }
    for i in 0..r {
        mfg.add_edge(i + l, t, 1);
    }
    for &(a, b) in ab.iter() {
        mfg.add_edge(a, b + l, 1);
    }
    println!("{}", mfg.maxflow(s, t, l.max(r)));
    let edges = mfg
        .edges()
        .into_iter()
        .filter(|e| e.flow == 1 && e.from != s && e.to != t);
    for e in edges {
        println!("{} {}", e.from, e.to - l);
    }
}
