use std::collections::HashSet;


/// Graph datastructure implemented as a set of edges.
/// The graph is undirected and unweighted - only the connectivity pattern of
/// the vertices is captured. Multiple edges are also disallowed.
///
/// Vertices and edges may not be removed.
pub struct Graph {
    edges: HashSet<(usize, usize)>,
}

impl Graph {
    /// Constructs a new graph
    pub fn new() -> Self {
        Graph { edges: HashSet::new() }
    }

    /// Add edge to the graph
    /// add_edge(u,v) has the same effect as add_edge(v,u)
    /// as the graph captures undirected edges.
    /// Adding an edge that already exists has no effect.
    pub fn add_edge(&mut self, mut u: usize, mut v: usize)  {
        if u < v {
            let t = u;
            u = v;
            v = t;
        }
        self.edges.insert((u,v));
    }

    /// Queries whether an edge exists in the graph.
    pub fn has_edge(&self, mut u: usize, mut v: usize) -> bool {
        if u < v {
            let t = u;
            u = v;
            v = t;
        }

        self.edges.contains(&(u,v))
    }
}
