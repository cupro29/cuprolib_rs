// verification-helper: PROBLEM https://judge.yosupo.jp/problem/shortest_path

use lib_rs::graph::{adj_list_graph::WeightedAdjListGraph, shortest_path::dijkstra};
use proconio::input;

fn main() {
    input! {
        n: usize, m: usize, s: usize, mut t: usize,
        abc: [(usize, usize, u64); m],
    }
    let mut g = WeightedAdjListGraph::new(n);
    for &(a, b, c) in abc.iter() {
        g.add_edge(a, b, c);
    }
    let ans = dijkstra(&g, s);
    if let Some(dist) = ans[t] {
        let mut path = vec![];
        while t != s {
            let p = ans[t].unwrap().1.unwrap();
            path.push((p, t));
            t = p;
        }
        let l = path.len();
        println!("{} {}", dist.0, l);
        for &(from, to) in path.iter().rev() {
            println!("{} {}", from, to);
        }
    } else {
        println!("-1");
    }
}
