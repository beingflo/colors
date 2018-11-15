use std::collections::HashMap;
use graph::Graph;

/// Coloring type.
/// This maps from vertices to colors.
pub type Coloring = HashMap<usize, usize>;

pub fn check_coloring(graph: &Graph, coloring: &Coloring) -> bool {
    for (u,v) in graph.edges() {
        if coloring[u] != coloring[v] {
            return false;
        }
    }

    return true;
}
