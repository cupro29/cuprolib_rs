use super::{DirectedEdge, Graph};

pub fn scc<'a, G, E>(graph: &'a G) -> Vec<Vec<usize>>
where
    E: 'a + DirectedEdge,
    G: Graph<'a, E>,
{
    let size = graph.size();
    let mut now_ord = 0;
    let mut group_num = 0;
    let mut visited = Vec::with_capacity(size);
    let mut low = vec![0; size];
    let mut ord = vec![None; size];
    let mut ids = vec![0; size];
    for i in 0..size {
        if ord[i].is_none() {
            dfs(
                graph,
                i,
                &mut now_ord,
                &mut group_num,
                &mut visited,
                &mut low,
                &mut ord,
                &mut ids,
            );
        }
    }
    let mut groups = vec![vec![]; group_num];
    for i in 0..size {
        groups[group_num - 1 - ids[i]].push(i);
    }
    groups
}

fn dfs<'a, G, E>(
    graph: &'a G,
    v: usize,
    now_ord: &mut usize,
    group_num: &mut usize,
    visited: &mut Vec<usize>,
    low: &mut Vec<usize>,
    ord: &mut Vec<Option<usize>>,
    ids: &mut Vec<usize>,
) where
    E: 'a + DirectedEdge,
    G: Graph<'a, E>,
{
    *now_ord += 1;
    low[v] = *now_ord;
    ord[v] = Some(*now_ord);
    visited.push(v);
    for e in graph.adjacencies(v) {
        let u = e.to();
        if let Some(k) = ord[u] {
            low[v] = low[v].min(k);
        } else {
            dfs(graph, u, now_ord, group_num, visited, low, ord, ids);
            low[v] = low[v].min(low[u]);
        }
    }
    if low[v] == ord[v].unwrap() {
        loop {
            let u = visited.pop().unwrap();
            ord[u] = Some(graph.size());
            ids[u] = *group_num;
            if u == v {
                break;
            }
        }
        *group_num += 1;
    }
}
