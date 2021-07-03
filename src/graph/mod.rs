pub mod adj_list_graph;
pub mod shortest_path;
pub mod strongly_connected_components;

pub trait Graph<'a, E: 'a + DirectedEdge> {
    type GraphDIter: Iterator<Item = &'a E>;
    fn size(&self) -> usize;
    fn adjacencies(&'a self, v: usize) -> Self::GraphDIter;
}

pub trait DirectedEdge {
    fn to(&self) -> usize;
}
pub trait WeigtedEdge<T>: DirectedEdge {
    fn cost(&self) -> T;
}
