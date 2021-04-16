use num::PrimInt;
use num_traits::NumAssign;
use std::{cmp::Reverse, collections::BinaryHeap};
#[derive(Debug, Clone)]
pub struct MinCostFlowEdge<Flow, Cost> {
    src: usize,
    dst: usize,
    flow: Flow,
    cap: Flow,
    cost: Cost,
    rev: usize,
}
impl<Flow, Cost> MinCostFlowEdge<Flow, Cost>
where
    Flow: PrimInt,
    Cost: PrimInt,
{
    pub fn residual_cap(&self) -> Flow {
        self.cap - self.flow
    }
    pub fn src(&self) -> usize {
        self.src
    }
    pub fn dst(&self) -> usize {
        self.dst
    }
    pub fn flow(&self) -> Flow {
        self.flow
    }
}
pub struct MinCostFlowGraph<Flow, Cost> {
    n: usize,
    g: Vec<Vec<MinCostFlowEdge<Flow, Cost>>>,
    b: Vec<Flow>,
    poterntial: Vec<Cost>,
    dist: Vec<Cost>,
    parent: Vec<Option<(usize, usize)>>,
    pos: Vec<(usize, usize)>,
}
impl<Flow, Cost> MinCostFlowGraph<Flow, Cost>
where
    Flow: PrimInt + NumAssign + std::ops::Neg<Output = Flow>,
    Cost: PrimInt + NumAssign + std::ops::Neg<Output = Cost> + std::convert::From<Flow>,
{
    pub fn new(n: usize) -> Self {
        Self {
            n,
            g: vec![vec![]; n],
            b: vec![Flow::zero(); n],
            poterntial: vec![Cost::zero(); n],
            dist: vec![Cost::max_value(); n],
            parent: vec![None; n],
            pos: vec![],
        }
    }
    pub fn add_edge(&mut self, src: usize, dst: usize, lower: Flow, upper: Flow, cost: Cost) {
        assert!(src < self.n);
        assert!(dst < self.n);
        assert!(lower <= upper);
        let e = self.g[src].len();
        let re = if src == dst { e + 1 } else { self.g[dst].len() };
        self.g[src].push(MinCostFlowEdge {
            src: src,
            dst: dst,
            flow: Flow::zero(),
            cap: upper,
            cost: cost,
            rev: re,
        });
        self.g[dst].push(MinCostFlowEdge {
            src: dst,
            dst: src,
            flow: Flow::zero(),
            cap: lower.neg(),
            cost: cost.neg(),
            rev: e,
        });
        self.pos.push((src, re));
    }
    pub fn add_supply(&mut self, v: usize, amount: Flow) {
        self.b[v] += amount;
    }
    pub fn add_demand(&mut self, v: usize, amount: Flow) {
        self.b[v] -= amount;
    }
    pub fn solve(&mut self) -> Cost {
        for v in 0..self.g.len() {
            for e in 0..self.g[v].len() {
                let rcap = self.g[v][e].residual_cap();
                if rcap < Flow::zero() {
                    self.push(v, e, rcap);
                    let src = self.g[v][e].src();
                    let dst = self.g[v][e].dst();
                    self.b[src] -= rcap;
                    self.b[dst] += rcap;
                }
            }
        }
        let mut inf_flow = Flow::one();
        for v in 0..self.g.len() {
            for e in 0..self.g[v].len() {
                inf_flow = inf_flow.max(self.g[v][e].residual_cap());
            }
        }
        let mut delta = Flow::one();
        while delta <= inf_flow {
            delta = delta << 1;
        }
        delta = delta >> 1;
        while delta > Flow::zero() {
            let mut excess_vs = vec![];
            let mut deficit_vs = vec![];
            for v in 0..self.g.len() {
                for e in 0..self.g[v].len() {
                    let rcap = self.g[v][e].residual_cap();
                    let rcost = self.residual_cost(v, e);
                    if rcost < Cost::zero() && rcap >= delta {
                        self.push(v, e, rcap);
                        let src = self.g[v][e].src;
                        let dst = self.g[v][e].dst;
                        self.b[src] -= rcap;
                        self.b[dst] += rcap;
                    }
                }
            }
            for v in 0..self.n {
                if self.b[v] > Flow::zero() {
                    excess_vs.push(v);
                } else if self.b[v] < Flow::zero() {
                    deficit_vs.push(v);
                }
            }
            while let Some(farthest) = self.dual(delta, &mut excess_vs, &mut deficit_vs) {
                self.primal(delta, farthest, &deficit_vs);
            }
            delta = delta >> 1;
        }
        let mut value = Cost::zero();
        for v in 0..self.g.len() {
            for e in 0..self.g[v].len() {
                value += self.g[v][e].cost * std::convert::From::from(self.g[v][e].flow);
            }
        }
        value >> 1
    }
    fn push(&mut self, v: usize, e: usize, amount: Flow) {
        self.g[v][e].flow += amount;
        let dst = self.g[v][e].dst;
        let rev = self.g[v][e].rev;
        self.g[dst][rev].flow -= amount;
    }
    fn residual_cost(&self, v: usize, e: usize) -> Cost {
        let src = self.g[v][e].src;
        let dst = self.g[v][e].dst;
        self.g[v][e].cost + self.poterntial[src] - self.poterntial[dst]
    }
    fn dual(
        &mut self,
        delta: Flow,
        excess_vs: &mut Vec<usize>,
        deficit_vs: &mut Vec<usize>,
    ) -> Option<Cost> {
        self.dist = self.dist.iter().map(|_| Cost::max_value()).collect();
        self.parent = self.parent.iter().map(|_| None).collect();
        excess_vs.retain(|&v| self.b[v] >= delta);
        deficit_vs.retain(|&v| self.b[v] <= delta.neg());
        let mut heap = BinaryHeap::new();
        for &mut v in excess_vs {
            self.dist[v] = Cost::zero();
            heap.push((Reverse(Cost::zero()), v));
        }
        let mut farthest = Cost::zero();
        let mut deficit_count = 0;
        while let Some((Reverse(d), u)) = heap.pop() {
            if self.dist[u] < d {
                continue;
            }
            farthest = d;
            if self.b[u] <= delta.neg() {
                deficit_count += 1;
            }
            if deficit_count >= deficit_vs.len() {
                break;
            }
            for e in 0..self.g[u].len() {
                if self.g[u][e].residual_cap() < delta {
                    continue;
                }
                let v = self.g[u][e].dst;
                let new_dist = d + self.residual_cost(u, e);
                if new_dist >= self.dist[v] {
                    continue;
                }
                self.dist[v] = new_dist;
                heap.push((Reverse(new_dist), v));
                self.parent[v] = Some((u, e));
            }
        }
        for v in 0..self.n {
            self.poterntial[v] += farthest.min(self.dist[v]);
        }
        if deficit_count > 0 {
            Some(farthest)
        } else {
            None
        }
    }
    fn primal(&mut self, delta: Flow, farthest: Cost, deficit_vs: &Vec<usize>) {
        for &t in deficit_vs {
            if self.dist[t] > farthest {
                continue;
            }
            let mut f = self.b[t].neg();
            let mut v = t;
            while let Some((pv, e)) = self.parent[v] {
                f = f.min(self.g[pv][e].residual_cap());
                v = pv;
            }
            f = f.min(self.b[v]);
            if f < delta {
                continue;
            }
            v = t;
            while let Some((pv, e)) = self.parent[v] {
                self.push(pv, e, f);
                self.parent[v] = None;
                v = pv;
            }
            self.b[t] += f;
            self.b[v] -= f;
        }
    }
}
