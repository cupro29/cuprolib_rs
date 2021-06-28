// verification-helper: PROBLEM https://onlinejudge.u-aizu.ac.jp/courses/library/5/GRL/all/GRL_6_B

use lib_rs::mincostflow::MinCostFlowGraph;
use proconio::input;

fn main() {
    input! {
        n: usize, m: usize, f: i32,
        uvcd: [(usize, usize, i32, i32); m],
    }
    let mut mcfg = MinCostFlowGraph::new(n);
    for &(u, v, c, d) in uvcd.iter() {
        mcfg.add_edge(u, v, c, d);
    }
    println!(
        "{}",
        if let Some(ans) = mcfg.min_cost_flow(0, n - 1, f) {
            ans
        } else {
            -1
        }
    );
}
