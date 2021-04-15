use num::Bounded;
use num_traits::NumAssign;
use std::{cmp::Reverse, collections::BinaryHeap};
#[derive(Debug, Clone)]
pub struct MinCostFlowEdge<T, U> {
    to: usize,
    rev: usize,
    cap: T,
    cost: U,
}
pub struct MinCostFlowGraph<T, U> {
    n: usize,
    g: Vec<Vec<MinCostFlowEdge<T, U>>>,
}
impl<T, U> MinCostFlowGraph<T, U>
where
    T: NumAssign + Ord + Copy,
    U: NumAssign + Ord + Copy + std::ops::Neg<Output = U> + Bounded + From<T>,
{
    pub fn new(n: usize) -> Self {
        Self {
            n,
            g: vec![vec![]; n],
        }
    }
    pub fn add_edge(&mut self, from: usize, to: usize, cap: T, cost: U) {
        assert!(from < self.n);
        assert!(to < self.n);
        assert!(cost >= U::zero());
        let rev = self.g[to].len() + (if from == to { 1 } else { 0 });
        self.g[from].push(MinCostFlowEdge { to, rev, cap, cost });
        let rev = self.g[from].len() - 1;
        self.g[to].push(MinCostFlowEdge {
            to: from,
            rev,
            cap: T::zero(),
            cost: cost.neg(),
        })
    }
    pub fn min_cost_flow(&mut self, s: usize, t: usize, mut flow_limit: T) -> U {
        assert!(s < self.n);
        assert!(t < self.n);
        let mut res = U::zero();
        let mut h = vec![U::zero(); self.n];
        let mut prevv = vec![s; self.n];
        let mut preve = vec![0; self.n];
        while flow_limit > T::zero() {
            let mut heap = BinaryHeap::new();
            let mut dist = vec![U::max_value(); self.n];
            dist[s] = U::zero();
            heap.push((Reverse(U::zero()), s));
            while let Some((Reverse(c), v)) = heap.pop() {
                if dist[v] < c {
                    continue;
                }
                for i in 0..self.g[v].len() {
                    let e = &self.g[v][i];
                    if e.cap > T::zero() && dist[e.to] > dist[v] + e.cost + h[v] - h[e.to] {
                        dist[e.to] = dist[v] + e.cost + h[v] - h[e.to];
                        prevv[e.to] = v;
                        preve[e.to] = i;
                        heap.push((Reverse(dist[e.to]), e.to));
                    }
                }
            }
            if dist[t] == U::max_value() {
                return U::one().neg();
            }
            for i in 0..self.n {
                h[i] += dist[i];
            }
            let mut d = flow_limit;
            {
                let mut v = t;
                while v != s {
                    d = d.min(self.g[prevv[v]][preve[v]].cap);
                    v = prevv[v];
                }
            }
            flow_limit -= d;
            res += h[t] * U::from(d);
            {
                let mut v = t;
                while v != s {
                    self.g[prevv[v]][preve[v]].cap -= d;
                    let rev = self.g[prevv[v]][preve[v]].rev;
                    self.g[v][rev].cap += d;
                    v = prevv[v];
                }
            }
        }
        res
    }
}
