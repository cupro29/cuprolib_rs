// verification-helper: PROBLEM https://judge.yosupo.jp/problem/min_cost_b_flow

use lib_rs::mincostflow_capacity_scaling::MinCostFlowGraph;
use proconio::input;

fn main() {
    input! {
        n: usize, m: usize,
        b: [i128; n],
        stluc: [(usize, usize, i128, i128, i128); m],
    }
    let mut mcfg = MinCostFlowGraph::new(n);
    for i in 0..n {
        mcfg.add_supply(i, b[i]);
    }
    for &(s, t, l, u, c) in stluc.iter() {
        mcfg.add_edge(s, t, l, u, c);
    }
    if let Ok(cost) = mcfg.solve() {
        println!("{}", cost);
        let potential = mcfg.potential();
        for &p in potential.iter() {
            println!("{}", p);
        }
        let edges = mcfg.edges();
        for e in edges.into_iter() {
            println!("{}", e.flow);
        }
    } else {
        println!("infeasible");
    }
}
