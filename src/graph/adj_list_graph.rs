use super::{DirectedEdge, Graph, WeigtedEdge};

#[derive(Debug, Clone)]
pub struct Edge {
    to: usize,
}
pub struct AdjListGraph {
    size: usize,
    graph: Vec<Vec<Edge>>,
}
impl DirectedEdge for Edge {
    fn to(&self) -> usize {
        self.to
    }
}
impl<'a> Graph<'a, Edge> for AdjListGraph {
    type GraphDIter = std::slice::Iter<'a, Edge>;
    fn size(&self) -> usize {
        self.size
    }
    fn adjacencies(&'a self, v: usize) -> Self::GraphDIter {
        self.graph[v].iter()
    }
}
impl AdjListGraph {
    pub fn new(size: usize) -> Self {
        Self {
            size,
            graph: vec![vec![]; size],
        }
    }
    pub fn add_edge(&mut self, from: usize, to: usize) {
        self.graph[from].push(Edge { to });
    }
}

#[derive(Debug, Clone)]
pub struct WEdge<T> {
    to: usize,
    cost: T,
}
pub struct WeightedAdjListGraph<T> {
    size: usize,
    graph: Vec<Vec<WEdge<T>>>,
}
impl<T> DirectedEdge for WEdge<T> {
    fn to(&self) -> usize {
        self.to
    }
}
impl<T: Copy> WeigtedEdge<T> for WEdge<T> {
    fn cost(&self) -> T {
        self.cost
    }
}
impl<'a, T: 'a> Graph<'a, WEdge<T>> for WeightedAdjListGraph<T> {
    type GraphDIter = std::slice::Iter<'a, WEdge<T>>;
    fn size(&self) -> usize {
        self.size
    }
    fn adjacencies(&'a self, v: usize) -> Self::GraphDIter {
        self.graph[v].iter()
    }
}
impl<T> WeightedAdjListGraph<T>
where
    T: Clone + Copy,
{
    pub fn new(size: usize) -> Self {
        Self {
            size,
            graph: vec![vec![]; size],
        }
    }
    pub fn add_edge(&mut self, from: usize, to: usize, cost: T) {
        self.graph[from].push(WEdge { to, cost });
    }
}
