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

    pub fn add_edge(&mut self, mut u: usize, mut v: usize)  {
        if u < v {
            let t = u;
            u = v;
            v = t;
        }
        self.edges.insert((u,v));
    }

    pub fn has_edge(&self, mut u: usize, mut v: usize) -> bool {
        if u < v {
            let t = u;
            u = v;
            v = t;
        }

        self.edges.contains(&(u,v))
    }
}

