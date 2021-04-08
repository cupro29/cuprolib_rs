type Cap = i32;
#[derive(Debug, Clone)]
struct MfEdge {
    pub to: usize,
    pub cap: Cap,
    pub rev: usize,
}
struct MfGraph {
    pub n: usize,
    pub g: Vec<Vec<MfEdge>>,
}
impl MfGraph {
    pub fn new(n: usize) -> Self {
        Self {
            n,
            g: vec![vec![]; n],
        }
    }
    pub fn add_edge(&mut self, from: usize, to: usize, cap: Cap) {
        let from_len = self.g[from].len();
        let to_len = self.g[to].len();
        self.g[from].push(MfEdge {
            to: to,
            cap: cap,
            rev: to_len,
        });
        self.g[to].push(MfEdge {
            to: from,
            cap: 0,
            rev: from_len,
        });
    }
    pub fn maxflow(&mut self, s: usize, t: usize, flow_limit: Cap) -> Cap {
        let mut flow = 0;
        loop {
            let mut level = vec![-1; self.n];
            self.bfs(s, &mut level);
            if level[t] < 0 {
                break;
            }
            let mut iter = vec![0; self.n];
            while flow < flow_limit {
                let f = self.dfs(s, t, flow_limit - flow, &mut iter, &level);
                if f == 0 {
                    break;
                }
                flow += f;
            }
        }
        flow
    }
    fn bfs(&self, s: usize, level: &mut Vec<i32>) {
        let mut que = std::collections::VecDeque::new();
        level[s] = 0;
        que.push_back(s);
        while let Some(v) = que.pop_front() {
            for e in self.g[v].iter() {
                if e.cap > 0 && level[e.to] < 0 {
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
        for i in iter[v]..self.g[v].len() {
            let e = self.g[v][i].clone();
            if e.cap > 0 && level[v] + 1 == level[e.to] {
                let d = self.dfs(e.to, t, std::cmp::min(up, e.cap), iter, level);
                if d > 0 {
                    self.g[v][i].cap -= d;
                    let to = self.g[v][i].to;
                    let rev = self.g[v][i].rev;
                    self.g[to][rev].cap += d;
                    return d;
                }
            }
        }
        0
    }
}
