// verification-helper: PROBLEM https://judge.yosupo.jp/problem/scc

use lib_rs::graph::{adj_list_graph::AdjListGraph, strongly_connected_components::scc};
use proconio::input;

fn main() {
    input! {
        n: usize, m: usize,
        ab: [(usize, usize); m],
    }
    let mut g = AdjListGraph::new(n);
    for &(a, b) in ab.iter() {
        g.add_edge(a, b);
    }
    let scc = scc(&g);
    println!("{}", scc.len());
    for edge in scc.into_iter() {
        print!("{}", edge.len());
        for v in edge.iter() {
            print!(" {}", v);
        }
        println!();
    }
}
