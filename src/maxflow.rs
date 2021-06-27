use num_traits::NumAssign;
use std::collections::VecDeque;
#[derive(Debug, Clone)]
pub struct MaxFlowEdge<Cap> {
    pub from: usize,
    pub to: usize,
    pub flow: Cap,
}
#[derive(Debug, Clone)]
struct _Edge<Cap> {
    to: usize,
    rev: usize,
    cap: Cap,
}
pub struct MaxFlowGraph<Cap> {
    n: usize,
    g: Vec<Vec<_Edge<Cap>>>,
    pos: Vec<(usize, usize)>,
}
impl<Cap> MaxFlowGraph<Cap>
where
    Cap: NumAssign + Ord + Copy,
{
    pub fn new(n: usize) -> Self {
        Self {
            n,
            g: vec![vec![]; n],
            pos: vec![],
        }
    }
    pub fn add_edge(&mut self, from: usize, to: usize, cap: Cap) {
        let from_len = self.g[from].len();
        let to_len = self.g[to].len() + (if from == to { 1 } else { 0 });
        self.pos.push((from, from_len));
        self.g[from].push(_Edge {
            to,
            rev: to_len,
            cap,
        });
        self.g[to].push(_Edge {
            to: from,
            rev: from_len,
            cap: Cap::zero(),
        });
    }
    pub fn maxflow(&mut self, s: usize, t: usize, flow_limit: Cap) -> Cap {
        let mut flow = Cap::zero();
        while flow < flow_limit {
            let mut level = vec![-1; self.n];
            self.bfs(s, &mut level);
            if level[t] < 0 {
                break;
            }
            let mut iter = vec![0; self.n];
            while flow < flow_limit {
                let f = self.dfs(s, t, flow_limit - flow, &mut iter, &level);
                if f == Cap::zero() {
                    break;
                }
                flow += f;
            }
        }
        flow
    }
    pub fn edges(&self) -> Vec<MaxFlowEdge<Cap>> {
        let get_edge = |from: usize, e: usize| {
            let to = self.g[from][e].to;
            let rev = self.g[from][e].rev;
            let flow = self.g[to][rev].cap;
            MaxFlowEdge { from, to, flow }
        };
        self.pos
            .iter()
            .map(|&(from, e)| get_edge(from, e))
            .collect()
    }
    fn bfs(&self, s: usize, level: &mut Vec<i32>) {
        let mut que = VecDeque::new();
        level[s] = 0;
        que.push_back(s);
        while let Some(v) = que.pop_front() {
            for e in self.g[v].iter() {
                if e.cap > Cap::zero() && level[e.to] < 0 {
                    level[e.to] = level[v] + 1;
                    que.push_back(e.to);
                }
            }
        }
    }
    fn dfs(&mut self, v: usize, t: usize, up: Cap, iter: &mut Vec<usize>, level: &Vec<i32>) -> Cap {
        if v == t {
            return up;
        }
        let s = iter[v];
        for i in s..self.g[v].len() {
            iter[v] += 1;
            let e = self.g[v][i].clone();
            if e.cap > Cap::zero() && level[v] < level[e.to] {
                let d = self.dfs(e.to, t, up.min(e.cap), iter, level);
                if d > Cap::zero() {
                    self.g[v][i].cap -= d;
                    let to = self.g[v][i].to;
                    let rev = self.g[v][i].rev;
                    self.g[to][rev].cap += d;
                    return d;
                }
            }
        }
        Cap::zero()
    }
}
