use super::{DirectedEdge, Graph, WeigtedEdge};
use std::cmp::Reverse;
use std::collections::{BinaryHeap, VecDeque};

pub fn dijkstra<'a, G, E, T>(graph: &'a G, start: usize) -> Vec<Option<(T, Option<usize>)>>
where
    T: Ord + Copy + std::ops::Add<Output = T> + Default,
    E: 'a + WeigtedEdge<T>,
    G: Graph<'a, E>,
{
    let mut res = vec![None; graph.size()];
    let mut heap = BinaryHeap::new();
    res[start] = Some((T::default(), None));
    heap.push((Reverse(T::default()), start));
    while let Some((Reverse(dist), v)) = heap.pop() {
        if let Some(vdist) = res[v] {
            if vdist.0 < dist {
                continue;
            }
        }
        for edge in graph.adjacencies(v) {
            let (to, cost) = (edge.to(), edge.cost());
            let new_dist = cost + dist;
            match res[to] {
                Some((to_dist, _)) if to_dist <= new_dist => continue,
                _ => {
                    heap.push((Reverse(new_dist), to));
                    res[to] = Some((new_dist, Some(v)));
                }
            }
        }
    }
    res
}

pub fn bfs<'a, G, E>(graph: &'a G, start: usize) -> Vec<Option<(usize, Option<usize>)>>
where
    E: 'a + DirectedEdge,
    G: Graph<'a, E>,
{
    let mut res = vec![None; graph.size()];
    let mut deq = VecDeque::new();
    res[start] = Some((0, None));
    deq.push_back(start);
    while let Some(v) = deq.pop_front() {
        let dist = res[v].unwrap().0;
        for edge in graph.adjacencies(v) {
            let u = edge.to();
            if res[u].is_none() {
                res[u] = Some((dist + 1, Some(v)));
                deq.push_back(u);
            }
        }
    }
    res
}
