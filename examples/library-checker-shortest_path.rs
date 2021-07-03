// verification-helper: PROBLEM https://judge.yosupo.jp/problem/shortest_path

use lib_rs::graph::{adj_list_graph::WeightedAdjListGraph, shortest_path::dijkstra};
use proconio::input;

fn main() {
    input! {
        n: usize, m: usize, s: usize, t: usize,
        abc: [(usize, usize, u64); m],
    }
    let mut g = WeightedAdjListGraph::new(n);
    for &(a, b, c) in abc.iter() {
        g.add_edge(a, b, c);
    }
    let ans = dijkstra(&g, s);
    if let Some(dist) = ans[t] {
        let mut path = vec![];
        let mut v = t;
        while let Some(u) = ans[v].unwrap().1 {
            path.push((u, v));
            v = u;
        }
        let l = path.len();
        println!("{} {}", dist.0, l);
        for i in (0..l).rev() {
            println!("{} {}", path[i].0, path[i].1);
        }
    } else {
        println!("-1");
    }
}
